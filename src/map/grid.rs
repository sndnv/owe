use std::f64;
use std::rc::Rc;
use std::cmp::Ordering;
use ndarray::Array2;
use pathfinding::dijkstra;
use entities::{Entity, NamedEntityType};
use entities::structure;
use effects::Effect;
use map::{Grid, GridError, Cell, CellState};

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

    pub fn add_entity(&mut self, at: (usize, usize), entity: Entity) -> Result<CellState, GridError> {
        match self.cell_state(at) {
            CellState::Empty => {
                let entity_rc = Rc::new(entity);

                match *entity_rc {
                    Entity::Structure { ref props, .. } => {
                        let cells = Self::entity_cells(&props.size, at);

                        if cells.iter().all(|c| self.cell_state(*c) == CellState::Empty) {
                            for cell in cells {
                                let cell_data = &mut self.cells[cell];
                                cell_data.entity = Some(entity_rc.clone());
                                cell_data.parent = Some(at);
                            }

                            Ok(CellState::Empty)
                        } else {
                            Err(GridError::CellUnavailable)
                        }
                    }

                    _ => {
                        self.cells[at].entity = Some(entity_rc.clone());
                        Ok(CellState::Empty)
                    }
                }
            }

            //TODO - support one/multiple walkers & roads/roadblocks/doodads in same cell

            _ => {
                Err(GridError::CellUnavailable)
            }
        }
    }

    pub fn remove_entity(&mut self, at: (usize, usize)) -> Result<CellState, GridError> {
        match self.cell_state(at) {
            CellState::Occupied => {
                let entity = self.cells[at].entity.clone().unwrap();
                let parent = self.cells[at].parent.unwrap_or(at);

                match *entity {
                    Entity::Structure { ref props, .. } if props.size.width * props.size.height > 1 => {
                        let cells = Self::entity_cells(&props.size, parent);

                        for cell in cells {
                            let cell_data = &mut self.cells[cell];
                            cell_data.entity = None;
                            cell_data.parent = None;
                        }

                        Ok(CellState::Occupied)
                    }

                    _ => {
                        self.cells[at].entity = None;
                        Ok(CellState::Occupied)
                    }
                }
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

    pub fn entity(&self, at: (usize, usize)) -> Option<Rc<Entity>> {
        self.cells.get(at)
            .and_then(|cell| {
                cell.entity.as_ref().map(|entity| entity.clone())
            })
    }

    pub fn find_first_adjacent_road(&self, next_to: (usize, usize)) -> Option<(usize, usize)> {
        self.cells.get(next_to)
            .and_then(|cell: &Cell| {
                cell.entity.as_ref().and_then(|entity| {
                    match **entity {
                        Entity::Structure { ref props, .. } => Some(Self::entity_cells(&props.size, cell.parent.unwrap())),
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
                                cell.entity.as_ref().and_then(|entity| {
                                    match **entity {
                                        Entity::Road => Some(neighbour),
                                        _ => None
                                    }
                                })
                            })
                            .is_some()
                    })
                    .map(|&neighbour| neighbour)
            })
    }

    pub fn distance_between(&(x1, y1): &(usize, usize), &(x2, y2): &(usize, usize)) -> f64 {
        let x = if x1 < x2 { (x2 - x1) } else { x1 - x2 };
        let y = if y1 < y2 { (y2 - y1) } else { y1 - y2 };

        ((x * x + y * y) as f64).sqrt()
    }

    pub fn find_closest_named_entity(&self, entity_type: NamedEntityType, with_name: String, close_to: &(usize, usize)) -> Option<((usize, usize), f64)> {
        let closest: (Option<(usize, usize)>, f64) = self.find_named_entities(entity_type, with_name).iter().fold(
            (None, f64::MAX),
            |acc, entity: &(usize, usize)| {
                let distance = Self::distance_between(close_to, entity);
                if distance < acc.1 { (Some(*entity), distance) } else { acc }
            });

        closest.0.map(|cell| (cell, closest.1))
    }

    pub fn find_named_entities(&self, entity_type: NamedEntityType, with_name: String) -> Vec<(usize, usize)> {
        self.cells.indexed_iter().fold(vec![], |mut acc: Vec<(usize, usize)>, (index, cell)| {
            match cell.entity.clone() {
                Some(entity) => {
                    match *entity {
                        Entity::Doodad { ref props, .. } if entity_type == NamedEntityType::Doodad && props.name == with_name => acc.push(index),
                        Entity::Resource { ref props, .. } if entity_type == NamedEntityType::Resource && props.name == with_name => acc.push(index),
                        Entity::Structure { ref props, .. } if entity_type == NamedEntityType::Structure && props.name == with_name => acc.push(index),
                        Entity::Walker { ref props, .. } if entity_type == NamedEntityType::Walker && props.name == with_name => acc.push(index),
                        _ => ()
                    }
                }

                None => ()
            }

            acc
        })
    }

    pub fn cell_state(&self, at: (usize, usize)) -> CellState {
        match self.cells.get(at) {
            Some(cell) =>
                match cell.entity {
                    Some(_) => CellState::Occupied,
                    None => CellState::Empty
                },

            None => CellState::OutOfBounds
        }
    }

    pub fn is_cell_in_grid(&self, cell: (usize, usize)) -> bool {
        self.width > cell.0 && self.height > cell.1
    }

    pub fn is_cell_passable(&self, cell: (usize, usize)) -> bool {
        self.is_cell_in_grid(cell) && match self.cells[cell].entity.clone() {
            Some(entity_cell) => match *entity_cell {
                Entity::Road => true,
                Entity::Roadblock => true,
                Entity::Walker { .. } => true,
                _ => false
            },

            None => true //cell is empty
        }
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
