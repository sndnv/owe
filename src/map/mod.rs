use std::rc::Rc;
use std::fmt;
use ndarray::Array2;
use entities::Entity;
use effects::Effect;
use production::exchange::ExchangeError;

pub mod cursor;
pub mod grid;

#[derive(Clone)]
struct Cell {
    entity: Option<Rc<Entity>>,
    parent: Option<(usize, usize)>,
    desirability: i8,
    active_effects: Vec<Rc<Effect>>,
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
    OutOfBounds,
}

#[derive(Debug)]
pub enum TraversalType {
    RoadOnly,
    EmptyOnly,
    RoadOrEmpty,
}

#[derive(Eq, PartialEq, Debug)]
pub enum GridError {
    CellUnavailable,
    EffectPresent,
    EffectMissing,
}

pub struct Grid {
    cells: Array2<Cell>,
    active_effects: Vec<Rc<Effect>>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, PartialEq, Debug)]
pub enum CursorError {
    ForGrid { e: GridError },
    ForExchange { errors: Vec<ExchangeError> },
}

pub struct Cursor {
    cell: (usize, usize),
    direction: Direction,
    range: usize,
}