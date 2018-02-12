use actix::{ResponseType, SyncAddress};
use effects::Effect;
use entities::{Entity, EntityActor, EntityAddress};
use ndarray::Array2;
use production::{CommodityProductionData, CommodityProductionError, WalkerProductionData, WalkerProductionError};
use production::exchange::ExchangeError;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

pub mod cursor;
pub mod grid;

pub struct ProcessTick {
    pub tick_size: u32
}

pub enum TickResult {
    Structure {
        commodity_data: Option<CommodityProductionData>,
        walker_data: Option<WalkerProductionData>,
        //TODO - effects
    },

    Walker {
        //TODO - effects
    },

    Resource {
        commodity_data: Option<CommodityProductionData>,
    },
}

pub enum TickError {
    Structure {
        commodity_error: Option<CommodityProductionError>,
        walker_error: Option<WalkerProductionError>,
    },

    Walker {
        //TODO
    },

    Resource {
        commodity_error: CommodityProductionError,
    },
}

impl ResponseType for ProcessTick {
    type Item = TickResult;
    type Error = TickError;
}

struct GridEntity {
    entity_type: EntityType,
    entity_address: Option<EntityAddress>,
    parent: (usize, usize),
}

impl GridEntity {
    //TODO
    pub fn replace_entity(&mut self, entity_address: EntityAddress) {
        self.entity = Rc::new(entity);
    }

    //TODO
    pub fn replace_ref(&mut self, entity_address: EntityAddress) {
        self.entity = entity;
    }
}

struct GridCell {
    entities: HashMap<Uuid, GridEntity>,
    desirability: i8,
    active_effects: Vec<Effect>,
    ground_fertility: u8,
    water_availability: u8,
    construction_allowed: bool,
}

impl GridCell {
    fn empty() -> GridCell {
        GridCell {
            entities: HashMap::new(),
            desirability: 0,
            active_effects: Vec::new(),
            ground_fertility: 0,
            water_availability: 0,
            construction_allowed: true,
        }
    }
}

#[derive(PartialEq)]
pub enum GridCellState {
    AvailableEmpty,
    AvailableOccupied,
    UnavailableOccupied,
    OutOfBounds,
}

pub enum TraversalType {
    RoadOnly,
    EmptyOnly,
    RoadOrEmpty,
}

#[derive(Eq, PartialEq)]
pub enum GridError {
    GridCellUnavailable,
    EntityMissing,
    EffectPresent,
    EffectMissing,
}

pub struct Grid {
    cells: Array2<GridCell>,
    active_effects: Vec<Effect>,
    width: usize,
    height: usize,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, PartialEq)]
pub enum CursorError {
    ForGrid { e: GridError },
    ForExchange { errors: Vec<ExchangeError> },
}

pub struct Cursor {
    cell: (usize, usize),
    direction: Direction,
    range: usize,
}
