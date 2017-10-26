use std::rc::Rc;
use ndarray::Array2;
use pathfinding::dijkstra;
use entities::Entity;
use entities::structure;
use effects::Effect;
use production::exchange::{CommodityExchange, CommodityState, ExchangeError};
use std::fmt;

#[derive(Clone)]
struct Cell {
    entity: Option<Rc<Entity>>,
    parent: Option<(usize, usize)>,
    desirability: i8,
    active_effects: Vec<Rc<Effect>>
}

impl Cell {
    fn empty() -> Cell {
        Cell { entity: None, parent: None, desirability: 0, active_effects: Vec::new() }
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Cell {{ entity: {:?}, parent: {:?}, desirability: {}, active_effects: {} }}",
            self.entity, self.parent, self.desirability, self.active_effects.len()
        )
    }
}

#[derive(PartialEq, Debug)]
pub enum CellState {
    Empty,
    Occupied,
    OutOfBounds
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
pub enum TraversalType {
    RoadOnly,
    EmptyOnly,
    RoadOrEmpty
}

#[derive(Eq, PartialEq, Debug)]
pub enum GridError {
    CellUnavailable,
    EffectPresent,
    EffectMissing
}

pub struct Grid {
    cells: Array2<Cell>,
    active_effects: Vec<Rc<Effect>>,
    width: usize,
    height: usize
}

impl Grid {
    //TODO - limit grid size to prevent cursor overflow when casting coords to isize (?)
    pub fn new(size: usize) -> Grid {
        Grid {
            cells: Array2::from_shape_fn((size, size), |_| Cell::empty()),
            active_effects: Vec::new(),
            width: size,
            height: size
        }
    }

    pub fn with_global_effects(size: usize, effects: Vec<Rc<Effect>>) -> Grid {
        Grid {
            cells: Array2::from_shape_fn((size, size), |_| Cell::empty()),
            active_effects: effects,
            width: size,
            height: size
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
                    Entity::Structure { ref props, .. } if props.size.width * props.size.height > 1 => {
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

    pub fn neighbors_of(&self, cell: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = cell;

        //TODO - allow corner neighbors only for specific walkers that don't need roads
        let neighbors = vec![
            if x > 0 { Some((x - 1, y + 1)) } else { None },
            Some((x, y + 1)),
            Some((x + 1, y + 1)),
            if x > 0 { Some((x - 1, y)) } else { None },
            Some((x + 1, y)),
            if x > 0 && y > 0 { Some((x - 1, y - 1)) } else { None },
            if y > 0 { Some((x, y - 1)) } else { None },
            if y > 0 { Some((x + 1, y - 1)) } else { None }
        ];

        neighbors.into_iter()
            .filter(|opt| opt.map_or(false, |c| self.is_cell_passable(c)))
            .map(|opt| opt.unwrap())
            .collect()
    }

    pub fn path_between(&self, start: (usize, usize), end: (usize, usize)) -> Option<(Vec<(usize, usize)>, usize)> {
        if self.is_cell_in_grid(start) && self.is_cell_in_grid(end) {
            dijkstra(
                &start,
                |cell| self.neighbors_of(*cell).into_iter().map(|c| (c, 1)),
                |cell| *cell == end
            )
        } else {
            None
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum CursorError {
    ForGrid { e: GridError },
    ForExchange { errors: Vec<ExchangeError> }
}

pub struct Cursor {
    cell: (usize, usize),
    direction: Direction,
    range: usize
}

impl Cursor {
    pub fn new(range: usize, direction: Direction, start: (usize, usize)) -> Cursor {
        Cursor {
            cell: start,
            direction,
            range
        }
    }

    pub fn position(&self) -> (usize, usize) {
        (self.cell.0, self.cell.1)
    }

    fn calculate_next_cell(cell_x: isize, cell_y: isize, grid_width: isize, grid_height: isize, direction: &Direction) -> (usize, usize) {
        let (next_cell_x, next_cell_y) = match direction {
            //cursor moves up and left
            &Direction::Up => {
                if cell_y == 0 {
                    //reached top row
                    (
                        if cell_x == 0 {
                            //reached left-most col
                            grid_width - 1
                        } else {
                            //moves one col to the left
                            cell_x - 1
                        },
                        //resets to bottom row
                        grid_height - 1
                    )
                } else {
                    //moves one row up
                    (
                        cell_x,
                        cell_y - 1
                    )
                }
            }

            //cursor moves down and right
            &Direction::Down => {
                if cell_y + 1 == grid_height {
                    //reached bottom row
                    (
                        if cell_x + 1 == grid_width {
                            //reached right-most col
                            0
                        } else {
                            //moves one col to the right
                            cell_x + 1
                        },
                        //resets to top row
                        0
                    )
                } else {
                    //moves one row down
                    (
                        cell_x,
                        cell_y + 1
                    )
                }
            }

            //cursor moves left and up
            &Direction::Left => {
                if cell_x == 0 {
                    //reached left-most col
                    (
                        //resets to last col
                        grid_width - 1,
                        if cell_y == 0 {
                            //reached top row
                            grid_height - 1
                        } else {
                            //moves one row up
                            cell_y - 1
                        }
                    )
                } else {
                    //moves one col to the left on the current row
                    (
                        cell_x - 1,
                        cell_y
                    )
                }
            }

            //cursor moves right & down
            &Direction::Right => {
                if cell_x + 1 == grid_width {
                    //reached right-most col
                    (
                        //resets to first col
                        0,
                        if cell_y + 1 == grid_height {
                            //reached bottom row
                            0
                        } else {
                            //moves one row down
                            cell_y + 1
                        }
                    )
                } else {
                    //moves one col to the right on the current row
                    (
                        cell_x + 1,
                        cell_y
                    )
                }
            }
        };

        (next_cell_x as usize, next_cell_y as usize)
    }

    //processes all effects for the current cell and moves to the next cell in the grid
    pub fn process_and_advance(&mut self, grid: &mut Grid, exchange: &mut CommodityExchange) -> Result<(), CursorError> {
        let cell_x = self.cell.0 as isize;
        let cell_y = self.cell.1 as isize;
        let effect_range = self.range as isize;
        let grid_width = grid.width as isize;
        let grid_height = grid.height as isize;

        let row_start = cell_x - effect_range;
        let row_start = if row_start > 0 { row_start } else { 0 };

        let row_end = cell_x + effect_range + 1;
        let row_end = if row_end > grid_width { grid_width } else { row_end };

        let col_start = cell_y - effect_range;
        let col_start = if col_start > 0 { col_start } else { 0 };

        let col_end = cell_y + effect_range + 1;
        let col_end = if col_end > grid_height { grid_height } else { col_end };

        let rows = row_start..row_end;
        let cols = col_start..col_end;

        let next_cell = Self::calculate_next_cell(cell_x, cell_y, grid_width, grid_height, &self.direction);

        {
            //applies cell effects
            let cell_effects = &grid.cells[self.cell].active_effects.clone();
            let mut effect_area = grid.cells.slice_mut(s![rows, cols]);

            for effect in cell_effects {
                for affected_cell in effect_area.iter_mut() {
                    affected_cell.entity.clone().map(|e| {
                        let mut updated_entity = (*e).clone();
                        effect.apply(&mut updated_entity);
                        affected_cell.entity = Some(Rc::new(updated_entity));
                    });
                }
            }
        }

        if next_cell == (0, 0) {
            //applies global effects
            for effect in &grid.active_effects {
                for affected_cell in grid.cells.iter_mut() {
                    affected_cell.entity.clone().map(|e| {
                        let mut updated_entity = (*e).clone();
                        effect.apply(&mut updated_entity);
                        affected_cell.entity = Some(Rc::new(updated_entity));
                    });
                }
            }

            //TODO - process movement
            //TODO - process action queue
            //TODO - process desirability changes for cells
        }

        let processing_result = {
            //process current cell production and state updates
            let affected_cell: &mut Cell = grid.cells.get_mut(self.cell).unwrap();
            affected_cell.entity.clone().map(|e| {
                let mut updated_entity = (*e).clone();

                let exchange_updates = match updated_entity {
                    Entity::Resource { ref props, ref mut producer, ref mut state, .. } => {
                        producer.as_mut().map(|mut p| {
                            let exchange_update = p.produce_commodity(&*e).map(|stage| {
                                if state.current_amount >= stage.commodity.amount {
                                    state.current_amount -= stage.commodity.amount;
                                } else {
                                    state.current_amount = 0;
                                }

                                vec![(stage.commodity, CommodityState::Available)]
                            });

                            props.replenish_amount.map(|amount| {
                                if state.current_amount + amount < props.max_amount {
                                    state.current_amount += amount;
                                } else {
                                    state.current_amount = props.max_amount;
                                }
                            });

                            exchange_update
                        }).unwrap_or(None)
                    }

                    Entity::Structure { ref mut producer, ref mut state, .. } => {
                        producer.as_mut().map(|mut p| {
                            let exchange_update = p.produce_commodity(&*e).map(|stage| {
                                let existing = state.commodities.entry(stage.commodity.name.clone()).or_insert(0);
                                *existing += stage.commodity.amount;

                                let mut updates = stage.required.into_iter().map(|c| (c, CommodityState::Required)).collect::<Vec<_>>();
                                updates.extend(stage.used.into_iter().map(|c| (c, CommodityState::Used)).collect::<Vec<_>>());
                                updates.push((stage.commodity, CommodityState::Available));

                                updates
                            });

                            p.produce_walker(&*e).map(|walker| {
                                //TODO - add walker to grid
                                //TODO - add walker effects to grid
                            });

                            //TODO - update current employees count

                            exchange_update
                        }).unwrap_or(None)
                    }

                    Entity::Walker { ref mut state, .. } => {
                        //TODO - update state
                        //TODO - process interaction with nearby entities
                        //       (work, attack, get/leave commodities)
                        None
                    }

                    _ => None //do nothing
                }.unwrap_or(Vec::new());

                let updated_entity = Rc::new(updated_entity);
                let failed_updates = exchange_updates.into_iter()
                    .map(|update| {
                        exchange
                            .update_state(updated_entity.clone(), &update.0, update.1)
                    })
                    .filter_map(|result| {
                        match result {
                            Ok(()) => None,
                            Err(e) => Some(e)
                        }
                    })
                    .collect::<Vec<_>>();

                affected_cell.entity = Some(updated_entity);

                if failed_updates.is_empty() {
                    Ok(())
                } else {
                    Err(CursorError::ForExchange { errors: failed_updates })
                }
            }).unwrap_or(Ok(()))
        };

        //resets the cursor position
        self.cell = next_cell;

        processing_result
    }
}
