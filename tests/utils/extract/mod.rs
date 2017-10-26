pub mod doodad {
    use std::rc::Rc;
    use owe::entities::Entity;

    pub fn name(entity: Option<Rc<Entity>>) -> Option<String> {
        entity.and_then(|entity| {
            match *entity {
                Entity::Doodad { ref props } => {
                    Some(props.name.clone())
                }
                _ => None
            }
        })
    }
}

pub mod structure {
    use std::rc::Rc;
    use owe::entities::Entity;
    use owe::entities::structure;

    pub fn employees(entity: Option<Rc<Entity>>) -> Option<u8> {
        entity.and_then(|entity| {
            match *entity {
                Entity::Structure { ref state, .. } => {
                    Some(state.current_employees)
                }
                _ => None
            }
        })
    }

    pub fn risk(entity: Option<Rc<Entity>>) -> Option<structure::Risk> {
        entity.and_then(|entity| {
            match *entity {
                Entity::Structure { ref state, .. } => {
                    Some(state.risk.clone())
                }
                _ => None
            }
        })
    }
}

pub mod resource {
    use std::rc::Rc;
    use owe::entities::Entity;

    pub fn level(entity: Option<Rc<Entity>>) -> Option<u32> {
        entity.and_then(|entity| {
            match *entity {
                Entity::Resource { ref state, .. } => {
                    Some(state.current_amount)
                }
                _ => None
            }
        })
    }
}

pub mod walker {
    use std::rc::Rc;
    use owe::entities::Entity;

    pub fn life(entity: Option<Rc<Entity>>) -> Option<Option<u16>> {
        entity.and_then(|entity| {
            match *entity {
                Entity::Walker { ref state, .. } => {
                    Some(state.current_life)
                }
                _ => None
            }
        })
    }
}
