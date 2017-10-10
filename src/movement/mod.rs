use entities::Entity;

//trait implemented on an entity that governs its movement
pub trait Movable {
    fn can_traverse(&self, entity: &Option<Entity>) -> bool;
    fn destination(&self) -> &Option<(usize, usize)>;
}
