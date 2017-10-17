use ndarray::Array2;
use pathfinding::dijkstra;
use entities::Entity;
use entities::structure;

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

#[derive(Debug)]
pub struct Grid {
    cells: Array2<Option<Entity>>,
    width: usize,
    height: usize
}

impl Grid {
    //TODO - limit grid size to prevent cursor overflow when casting coords to isize (?)
    pub fn new(size: usize) -> Grid {
        Grid {
            cells: Array2::from_shape_fn((size, size), |_| None),
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

    pub fn add(&mut self, at: (usize, usize), entity: Entity) -> (CellState, bool) {
        match self.cell_state(at) {
            CellState::Empty => {
                //TODO - handle overlapping entities
                match entity {
                    Entity::Structure { ref props, .. } if props.size.width * props.size.height > 1 => {
                        //TODO - check if all cells are in grid
                        //TODO - check if all cells are empty
                        (CellState::Empty, true) //TODO
                    }

                    _ => {
                        self.cells[at] = Some(entity);
                        (CellState::Empty, true)
                    }
                }
            }

            CellState::Occupied => {
                (CellState::Occupied, false)
            }

            CellState::OutOfBounds => {
                panic!("Cell [{:?}] is not in grid [{:?}; {:?}]", at, self.width, self.height)
            }
        }
    }

    pub fn remove(&mut self, at: (usize, usize)) -> (CellState, bool) {
        match self.cell_state(at) {
            CellState::Empty => {
                (CellState::Empty, false)
            }

            CellState::Occupied => {
                //TODO - handle entities with size > (1, 1)
                //TODO - handle overlapping entities
                self.cells[at] = None;
                (CellState::Occupied, true)
            }

            CellState::OutOfBounds => {
                panic!("Cell [{:?}] is not in grid [{:?}; {:?}]", at, self.width, self.height)
            }
        }
    }

    pub fn cell_state(&self, at: (usize, usize)) -> CellState {
        match self.cells.get(at) {
            Some(cell) =>
                match cell {
                    &Some(_) => CellState::Occupied,
                    &None => CellState::Empty
                },

            None => CellState::OutOfBounds
        }
    }

    pub fn foreach(&self, f: &Fn((usize, usize), &Option<Entity>)) -> () {
        for ((x, y), entity) in self.cells.indexed_iter() {
            f((x, y), entity)
        }
    }

    pub fn is_cell_in_grid(&self, cell: (usize, usize)) -> bool {
        self.width > cell.0 && self.height > cell.1
    }

    pub fn is_cell_passable(&self, cell: (usize, usize)) -> bool {
        self.is_cell_in_grid(cell) && match self.cells[cell] {
            Some(ref entity) => match entity {
                &Entity::Road => true,
                &Entity::Roadblock => true,
                &Entity::Walker { .. } => true,
                _ => false
            },
            None => true //cell is empty
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

#[derive(Debug)]
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
    pub fn process_and_advance(&mut self, grid: &Grid) -> () {
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

        let effect_area = grid.cells.slice(s![rows, cols]);

        //TODO - process entity-local effects
        //TODO - process global effects
        //TODO - process resource production
        //TODO - process walker production
        //TODO - process action queue
        //TODO - process movement

        //resets the cursor position
        self.cell = Self::calculate_next_cell(cell_x, cell_y, grid_width, grid_height, &self.direction);
    }
}
