use entities::Entity;

pub trait Effect {
    fn apply(&self, other_entity: &mut Entity) -> ();
}
