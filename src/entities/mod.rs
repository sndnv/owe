use uuid::Uuid;

pub mod structure;
pub mod resource;
pub mod doodad;
pub mod walker;

#[derive(Debug)]
pub enum State {
    Active,
    Affected,
    Destroyed,
    Deleted
}

#[derive(PartialEq, Debug)]
pub enum Entity {
    Road,
    Roadblock,
    Doodad { props: doodad::Doodad },
    Resource { id: Uuid, props: resource::ResourceProperties, state: resource::ResourceState },
    Structure { id: Uuid, props: structure::StructureProperties, state: structure::StructureState },
    Walker { id: Uuid, props: walker::WalkerProperties, state: walker::WalkerState }
}
