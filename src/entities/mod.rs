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

//TODO - Move state data from each struct into the enums
//TODO - * original structs become simple descriptors/props
//TODO - * all runtime state is stored here
//TODO - * data field -> props field that references static (?) props/descriptors
#[derive(PartialEq, Debug)]
pub enum Entity {
    Road,
    Roadblock,
    Doodad { data: doodad::Doodad },
    Resource { data: resource::Resource },
    Structure { id: Uuid, data: structure::Structure },
    Walker { id: Uuid, data: walker::Walker }
}
