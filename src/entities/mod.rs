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

#[derive(Debug)]
pub enum Entity {
    Road,
    Roadblock,
    Doodad { data: doodad::Doodad },
    Resource { data: resource::Resource },
    Structure { data: structure::Structure },
    Walker { data: walker::Walker }
}
