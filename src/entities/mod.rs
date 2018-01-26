use uuid::Uuid;
use production::Producer;

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

#[derive(PartialEq, Clone, Debug)]
pub enum NamedEntityType {
    Doodad,
    Resource,
    Structure,
    Walker
}

#[derive(PartialEq, Clone, Debug)]
pub enum Entity {
    Road,

    Roadblock,

    Doodad { props: doodad::Doodad },

    Resource {
        id: Uuid,
        props: resource::ResourceProperties,
        state: resource::ResourceState,
        producer: Option<Box<Producer>>
    },

    Structure {
        id: Uuid,
        props: structure::StructureProperties,
        state: structure::StructureState,
        producer: Option<Box<Producer>>
    },

    Walker {
        id: Uuid,
        props: walker::WalkerProperties,
        state: walker::WalkerState
    }
}
