use ndarray::Array2;
use effects::Effect;
use entities::Entity;
use entities::structure::Structure;

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

    //returns previous cell state
    fn update(&mut self, at: (usize, usize), with_cell: Option<Entity>) -> CellState {
        let cell_state = self.cell_state(at);

        if cell_state != CellState::OutOfBounds {
            self.cells[at] = with_cell;
        }

        cell_state
    }

    pub fn add(&mut self, at: (usize, usize), entity: Entity) -> CellState {
        self.update(at, Some(entity))
    }

    pub fn remove(&mut self, at: (usize, usize)) -> CellState {
        self.update(at, None)
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
        for ((x, y), cell) in self.cells.indexed_iter() {
            f((x, y), cell)
        }
    }
}

pub struct GridCursor {
    cell: (usize, usize),
    direction: Direction,
    range: usize
}

impl GridCursor {
    pub fn new(range: usize, direction: Direction, start: (usize, usize)) -> GridCursor {
        GridCursor {
            cell: start,
            direction,
            range
        }
    }

    pub fn position(&self) -> (usize, usize) {
        self.cell
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

        let (next_cell_x, next_cell_y) = match self.direction {
            //cursor moves up and left
            Direction::Up => {
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
            Direction::Down => {
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
            Direction::Left => {
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
            Direction::Right => {
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

        //resets the cursor position
        self.cell = (next_cell_x as usize, next_cell_y as usize);
    }
}
