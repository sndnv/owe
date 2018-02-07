use effects::Effect;
use entities::Entity;
use ndarray::Array2;
use production::exchange::ExchangeError;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use uuid::Uuid;

pub mod cursor;
pub mod grid;

#[derive(Clone, Debug)]
struct GridEntity {
    entity: Rc<Entity>,
    parent: (usize, usize),
}

impl GridEntity {
    pub fn replace_entity(&mut self, entity: Entity) {
        self.entity = Rc::new(entity);
    }

    pub fn replace_ref(&mut self, entity: Rc<Entity>) {
        self.entity = entity;
    }
}

#[derive(Clone)]
struct Cell {
    entities: HashMap<Uuid, GridEntity>,
    desirability: i8,
    active_effects: Vec<Rc<Effect>>,
}

impl Cell {
    fn empty() -> Cell {
        Cell { entities: HashMap::new(), desirability: 0, active_effects: Vec::new() }
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Cell {{ entities: {:?}, desirability: {}, active_effects: {} }}",
            self.entities, self.desirability, self.active_effects.len()
        )
    }
}

#[derive(PartialEq, Debug)]
pub enum CellState {
    AvailableEmpty,
    AvailableOccupied,
    UnavailableOccupied,
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
    EntityMissing,
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