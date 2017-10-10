use owe::effects;
use owe::entities::Entity;

struct TestEffect1 {}

struct TestEffect2 {}

struct TestEffect3 {}

impl effects::Effect for TestEffect1 {
    fn apply(&self, other_entity: &mut Entity) -> () {
        match other_entity {
            &mut Entity::Structure { ref mut data } => {
                data.cost = 9001;
                if data.employees.current < data.employees.required {
                    data.employees.current += 1;
                }
            }
            &mut Entity::Doodad { ref mut data } => {
                data.name = "updated doodad name".to_owned();
            }
            _ => ()//does nothing
        }
    }
}

impl effects::Effect for TestEffect2 {
    fn apply(&self, other_entity: &mut Entity) -> () {
        match other_entity {
            &mut Entity::Structure { ref mut data } => {
                data.risk.fire += 5;
                if data.desirability.3 > 0 {
                    data.desirability.3 = -5;
                }
                if data.risk.damage > 0 {
                    data.risk.damage -= 1;
                }

            }
            _ => ()//does nothing
        }
    }
}

impl effects::Effect for TestEffect3 {
    fn apply(&self, other_entity: &mut Entity) -> () {
        match other_entity {
            &mut Entity::Resource { ref mut data } => {
                if data.level.current > 0 {
                    data.level.current -= 5;
                }
            }
            &mut Entity::Walker { ref mut data } => {
                match data.life {
                    Some(level) => if level > 0 { data.life = Some(level - 1) },
                    None => data.life = Some(100)
                };
            }
            _ => ()//does nothing
        }
    }
}
