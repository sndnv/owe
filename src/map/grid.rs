use effects::Effect;
use entities::{Entity, EntityType};
use entities::structure;
use map::{Cell, CellState, Grid, GridEntity, GridError};
use ndarray::Array2;
use pathfinding::dijkstra;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::f64;
use std::rc::Rc;
use uuid::Uuid;

impl Grid {
    //TODO - limit grid size to prevent cursor overflow when casting coords to isize (?)
    pub fn new(size: usize) -> Grid {
        Grid {
            cells: Array2::from_shape_fn((size, size), |_| Cell::empty()),
            active_effects: Vec::new(),
            width: size,
            height: size,
        }
    }

    pub fn with_global_effects(size: usize, effects: Vec<Rc<Effect>>) -> Grid {
        Grid {
            cells: Array2::from_shape_fn((size, size), |_| Cell::empty()),
            active_effects: effects,
            width: size,
            height: size,
        }
    }

    fn entity_cells(entity_size: &structure::Size, cell: (usize, usize)) -> Vec<(usize, usize)> {
        let cells: Vec<Vec<(usize, usize)>> = (cell.0..(cell.0 + entity_size.width as usize))
            .map(|x| {
                (cell.1..(cell.1 + entity_size.height as usize)).map(|y| {
                    (x, y)
                }).collect()
            }).collect();

        cells.into_iter().fold(vec![], |mut acc, vec| {
            acc.extend(vec);
            acc
        })
    }

    pub fn neighbours_of(cell: &(usize, usize), with_corner_neighbours: bool) -> Vec<Option<(usize, usize)>> {
        let &(x, y) = cell;

        vec![
            /* top    left   */ if with_corner_neighbours && x > 0 && y > 0 { Some((x - 1, y - 1)) } else { None },
            /* top    center */ if y > 0 { Some((x, y - 1)) } else { None },
            /* top    right  */ if with_corner_neighbours && y > 0 { Some((x + 1, y - 1)) } else { None },
            /* middle left   */ if x > 0 { Some((x - 1, y)) } else { None },
            /* middle right  */ Some((x + 1, y)),
            /* bottom left   */ if with_corner_neighbours && x > 0 { Some((x - 1, y + 1)) } else { None },
            /* bottom center */ Some((x, y + 1)),
            /* bottom right  */ if with_corner_neighbours { Some((x + 1, y + 1)) } else { None },
        ]
    }

    pub fn distance_between(&(x1, y1): &(usize, usize), &(x2, y2): &(usize, usize)) -> f64 {
        let x = if x1 < x2 { (x2 - x1) } else { x1 - x2 };
        let y = if y1 < y2 { (y2 - y1) } else { y1 - y2 };

        ((x * x + y * y) as f64).sqrt()
    }

    pub fn add_entity(&mut self, at: (usize, usize), entity: Entity) -> Result<(Uuid, CellState), GridError> {
        let cell_state = self.cell_state(at);

        match cell_state {
            CellState::AvailableEmpty | CellState::AvailableOccupied => {
                let entity_ref = Rc::new(entity);
                let entity_id = Uuid::new_v4();

                match *entity_ref {
                    Entity::Structure { ref props, .. } => {
                        let cells = Self::entity_cells(&props.size, at);

                        if cells.iter().all(|c| {
                            let current_cell_state = self.cell_state(*c);
                            current_cell_state == CellState::AvailableEmpty || current_cell_state == CellState::AvailableOccupied
                        }) {
                            for cell in cells {
                                let cell_data = &mut self.cells[cell];
                                cell_data.entities.insert(entity_id, GridEntity { entity: entity_ref.clone(), parent: at });
                            }

                            Ok((entity_id, cell_state))
                        } else {
                            Err(GridError::CellUnavailable)
                        }
                    }

                    _ => {
                        self.cells[at].entities.insert(entity_id, GridEntity { entity: entity_ref.clone(), parent: at });
                        Ok((entity_id, cell_state))
                    }
                }
            }

            _ => {
                Err(GridError::CellUnavailable)
            }
        }
    }

    pub fn remove_entity(&mut self, at: (usize, usize), id: &Uuid) -> Result<CellState, GridError> {
        let cell_state = self.cell_state(at);

        match cell_state {
            CellState::AvailableOccupied | CellState::UnavailableOccupied => {
                let grid_entity = self.cells[at].entities.get(id).map(|grid_entity| {
                    (grid_entity.entity.clone(), grid_entity.parent)
                });

                grid_entity.map(|(entity, parent)| {
                    match *entity {
                        Entity::Structure { ref props, .. } if props.size.width * props.size.height > 1 => {
                            let cells = Self::entity_cells(&props.size, parent);

                            for cell in cells {
                                let cell_data = &mut self.cells[cell];
                                cell_data.entities.remove(id);
                            }

                            Ok(cell_state)
                        }

                        _ => {
                            self.cells[at].entities.remove(id);
                            Ok(cell_state)
                        }
                    }
                }).unwrap_or(Err(GridError::EntityMissing))
            }

            _ => {
                Err(GridError::CellUnavailable)
            }
        }
    }

    pub fn add_cell_effect(&mut self, at: (usize, usize), effect: Rc<Effect>) -> Result<CellState, GridError> {
        match self.cell_state(at) {
            CellState::OutOfBounds => {
                Err(GridError::CellUnavailable)
            }

            state => {
                if self.is_effect_in_cell(at, &effect) {
                    Err(GridError::EffectPresent)
                } else {
                    self.cells[at].active_effects.push(effect);
                    Ok(state)
                }
            }
        }
    }

    pub fn remove_cell_effect(&mut self, at: (usize, usize), effect: &Rc<Effect>) -> Result<CellState, GridError> {
        match self.cell_state(at) {
            CellState::OutOfBounds => {
                Err(GridError::CellUnavailable)
            }

            state => {
                match self.cells[at].active_effects.iter()
                    .position(|e| {
                        Rc::ptr_eq(e, effect)
                    })
                    .map(|i| {
                        self.cells[at].active_effects.remove(i)
                    }) {
                    Some(_) => Ok(state),
                    None => Err(GridError::EffectMissing)
                }
            }
        }
    }

    pub fn clear_cell_effects(&mut self, at: (usize, usize)) -> Result<CellState, GridError> {
        match self.cell_state(at) {
            CellState::OutOfBounds => {
                Err(GridError::CellUnavailable)
            }

            state => {
                self.cells[at].active_effects.clear();

                Ok(state)
            }
        }
    }

    pub fn add_global_effect(&mut self, effect: Rc<Effect>) -> Result<(), GridError> {
        if self.is_effect_global(&effect) {
            Err(GridError::EffectPresent)
        } else {
            self.active_effects.push(effect);
            Ok(())
        }
    }

    pub fn remove_global_effect(&mut self, effect: &Rc<Effect>) -> Result<(), GridError> {
        match self.active_effects.iter()
            .position(|e| {
                Rc::ptr_eq(e, effect)
            })
            .map(|i| {
                self.active_effects.remove(i)
            }) {
            Some(_) => Ok(()),
            None => Err(GridError::EffectMissing)
        }
    }

    pub fn clear_global_effects(&mut self) -> () {
        self.active_effects.clear()
    }

    pub fn entity(&self, at: (usize, usize), id: &Uuid) -> Option<Rc<Entity>> {
        self.cells.get(at)
            .and_then(|cell| {
                cell.entities.get(id).as_ref().map(|grid_entity| grid_entity.entity.clone())
            })
    }

    pub fn find_first_adjacent_road(&self, next_to: (usize, usize), id: &Uuid) -> Option<(usize, usize)> {
        self.cells.get(next_to)
            .and_then(|cell: &Cell| {
                cell.entities.get(id).and_then(|grid_entity| {
                    match *grid_entity.entity {
                        Entity::Structure { ref props, .. } => Some(Self::entity_cells(&props.size, grid_entity.parent)),
                        _ => None
                    }
                })
            })
            .and_then(|entity_cells: Vec<(usize, usize)>| {
                let mut neighbours: Vec<(usize, usize)> = entity_cells
                    .iter()
                    .fold(
                        vec![],
                        |mut acc: Vec<(usize, usize)>, cell| {
                            let mut neighbours: Vec<(usize, usize)> = Self::neighbours_of(cell, false).into_iter()
                                .filter(|cell| cell.is_some())
                                .map(|cell| cell.unwrap()).collect();

                            acc.append(&mut neighbours);
                            acc
                        });

                neighbours.retain(|cell| !entity_cells.contains(cell));

                neighbours.sort_unstable_by(|a, b| {
                    match a.1.cmp(&b.1) {
                        Ordering::Equal => a.0.cmp(&b.0),
                        ordering => ordering,
                    }
                });

                neighbours.dedup();

                neighbours.iter()
                    .find(|&&neighbour| {
                        self.cells.get(neighbour)
                            .and_then(|cell: &Cell| {
                                cell.entities.values().find(|grid_entity| {
                                    match *grid_entity.entity {
                                        Entity::Road => true,
                                        _ => false
                                    }
                                })
                            })
                            .is_some()
                    })
                    .map(|&neighbour| neighbour)
            })
    }

    pub fn find_closest_named_entity(
        &self,
        entity_type: EntityType,
        with_name: String,
        close_to: &(usize, usize),
    ) -> Option<((usize, usize), f64)> {
        let closest: (Option<(usize, usize)>, f64) = self.find_named_entities(entity_type, with_name).iter()
            .fold(
                (None, f64::MAX),
                |acc, entity: &(usize, usize)| {
                    let distance = Self::distance_between(close_to, entity);
                    if distance < acc.1 { (Some(*entity), distance) } else { acc }
                });

        closest.0.map(|cell| (cell, closest.1))
    }

    pub fn find_named_entities(&self, entity_type: EntityType, with_name: String) -> Vec<(usize, usize)> {
        self.cells.indexed_iter()
            .fold(
                HashMap::new(),
                |mut acc: HashMap<Uuid, (usize, usize)>, (index, cell)| {
                    cell.entities.iter().for_each(|(id, grid_entity)| {
                        match *grid_entity.entity {
                            Entity::Doodad { ref props, .. } if entity_type == EntityType::Doodad && props.name == with_name => {
                                acc.insert(*id, index);
                            }

                            Entity::Resource { ref props, .. } if entity_type == EntityType::Resource && props.name == with_name => {
                                acc.insert(*id, index);
                            }

                            Entity::Structure { ref props, .. } if entity_type == EntityType::Structure && props.name == with_name => {
                                acc.insert(*id, index);
                            }

                            Entity::Walker { ref props, .. } if entity_type == EntityType::Walker && props.name == with_name => {
                                acc.insert(*id, index);
                            }
                            _ => ()
                        };
                    });

                    acc
                })
            .values()
            .map(|v| *v)
            .collect()
    }

    pub fn cell_state(&self, at: (usize, usize)) -> CellState {
        match self.cells.get(at) {
            Some(cell) =>
                match cell.entities.iter().find(|&(_, grid_entity)| {
                    match *grid_entity.entity {
                        Entity::Doodad { .. } => true,
                        Entity::Resource { .. } => true,
                        Entity::Structure { .. } => true,
                        Entity::Walker { .. } => false,
                        Entity::Road => false,
                        Entity::Roadblock => false,
                    }
                }) {
                    Some(_) => CellState::UnavailableOccupied,
                    None if (!cell.entities.is_empty()) => CellState::AvailableOccupied,
                    _ => CellState::AvailableEmpty
                }

            None => CellState::OutOfBounds
        }
    }

    pub fn is_cell_in_grid(&self, cell: (usize, usize)) -> bool {
        self.width > cell.0 && self.height > cell.1
    }

    pub fn is_cell_passable(&self, cell: (usize, usize)) -> bool {
        self.is_cell_in_grid(cell) && self.cells[cell].entities.values().find(|grid_entity| {
            match *grid_entity.entity {
                Entity::Road => false,
                Entity::Roadblock => false,
                Entity::Walker { .. } => false,
                _ => true
            }
        }).is_none()
    }

    pub fn is_effect_in_cell(&self, cell: (usize, usize), effect: &Rc<Effect>) -> bool {
        match self.cell_state(cell) {
            CellState::OutOfBounds => {
                false
            }

            _ => {
                match self.cells[cell].active_effects.iter()
                    .position(|e| {
                        Rc::ptr_eq(e, &effect)
                    }) {
                    Some(_) => true,
                    None => false
                }
            }
        }
    }

    pub fn is_effect_global(&self, effect: &Rc<Effect>) -> bool {
        match self.active_effects.iter()
            .position(|e| {
                Rc::ptr_eq(e, effect)
            }) {
            Some(_) => true,
            None => false
        }
    }

    pub fn passable_neighbours_of(&self, cell: &(usize, usize)) -> Vec<(usize, usize)> {
        //TODO - allow corner neighbors only for specific walkers that don't need roads
        Self::neighbours_of(cell, true).into_iter()
            .filter(|opt| opt.map_or(false, |c| self.is_cell_passable(c)))
            .map(|opt| opt.unwrap())
            .collect()
    }

    pub fn path_between(&self, start: (usize, usize), end: (usize, usize)) -> Option<(Vec<(usize, usize)>, usize)> {
        if self.is_cell_in_grid(start) && self.is_cell_in_grid(end) {
            dijkstra(
                &start,
                |cell| self.passable_neighbours_of(cell).into_iter().map(|c| (c, 1)),
                |cell| *cell == end,
            )
        } else {
            None
        }
    }
}
