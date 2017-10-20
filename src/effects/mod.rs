use entities::Entity;

pub trait Effect {
    fn apply(&self, entity: &mut Entity) -> ();
}
