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

#[derive(PartialEq, Clone, Debug)]
pub enum Entity {
    Road,

    Roadblock,

    Doodad { props: doodad::Doodad },

    Resource {
        props: resource::ResourceProperties,
        state: resource::ResourceState,
        producer: Option<Box<Producer>>,
    },

    Structure {
        props: structure::StructureProperties,
        state: structure::StructureState,
        producer: Option<Box<Producer>>,
    },

    Walker {
        props: walker::WalkerProperties,
        state: walker::WalkerState,
    },
}
