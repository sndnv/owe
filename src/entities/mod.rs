use actix::{Actor, Context, SyncAddress};

pub mod structure;
pub mod resource;
pub mod doodad;
pub mod walker;

#[derive(Debug)]
pub enum State {
    Active,
    Affected,
    Destroyed,
    Deleted,
}

#[derive(PartialEq, Clone, Debug)]
pub enum EntityType {
    Road,
    Roadblock,
    Doodad,
    Resource,
    Structure,
    Walker,
}

pub trait Entity<Properties, State>: Sized + 'static {
    fn get_props(&self) -> Properties;
    fn get_state(&self) -> State;
    fn set_state(&mut self, new_state: State) -> ();
}

pub struct EntityActor<T: Entity> {
    entity: T
}

impl<T: Entity> EntityActor<T> {
    pub fn new(entity: T) -> EntityActor<T> {
        EntityActor { entity }
    }
}

impl<T: Entity> Actor for EntityActor<T> {
    type Context = Context<Self>;
}
