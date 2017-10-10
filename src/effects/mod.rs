use entities::Entity;

//trait implemented on an entity that affects other entities in its AOE
pub trait Effect {
    fn apply(&self, other_entity: &mut Entity) -> ();
}
