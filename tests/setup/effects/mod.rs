use owe::effects;
use owe::entities::Entity;

struct TestEffect1 {}

struct TestEffect2 {}

struct TestEffect3 {}

impl effects::Effect for TestEffect1 {
    fn apply(&self, other_entity: &mut Entity) -> () {
        match other_entity {
            &mut Entity::Structure { ref mut state, ref props, .. } => {
                if state.current_employees < props.max_employees {
                    state.current_employees += 1;
                }
            }
            &mut Entity::Doodad { ref mut props } => {
                props.name = "updated doodad name".to_owned();
            }
            _ => () //does nothing
        }
    }
}

impl effects::Effect for TestEffect2 {
    fn apply(&self, other_entity: &mut Entity) -> () {
        match other_entity {
            &mut Entity::Structure { ref mut state, .. } => {
                state.risk.fire += 5;
                if state.risk.damage > 0 {
                    state.risk.damage -= 1;
                }
            }
            _ => () //does nothing
        }
    }
}

impl effects::Effect for TestEffect3 {
    fn apply(&self, other_entity: &mut Entity) -> () {
        match other_entity {
            &mut Entity::Resource { ref mut state, .. } => {
                if state.current_level > 0 {
                    state.current_level -= 5;
                }
            }
            &mut Entity::Walker { ref mut state, ref props, .. } => {
                match state.current_life {
                    Some(level) => if level > 0 { state.current_life = Some(level - 1) },
                    None => state.current_life = props.max_life
                };
            }
            _ => () //does nothing
        }
    }
}
