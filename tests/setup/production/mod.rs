use owe::entities::doodad;
use owe::entities::resource;
use owe::entities::structure;
use owe::entities::walker;
use owe::entities::Entity;
use owe::production::Commodity;
use owe::production::exchange::CommodityExchange;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

pub fn exchange_default() -> CommodityExchange {
    CommodityExchange::new()
}

pub fn commodities_default() -> Vec<Commodity> {
    let c0 = Commodity { name: "c0".to_owned(), amount: 1 };
    let c1 = Commodity { name: "c1".to_owned(), amount: 3 };
    let c2 = Commodity { name: "c2".to_owned(), amount: 10 };

    vec![c0, c1, c2]
}

pub fn entities_default() -> Vec<Rc<Entity>> {
    let s0 = structure::Structure {
        name: "s0".to_owned(),
        size: (1, 1),
        employees: structure::Employees { current: 0, required: 5 },
        cost: 1000,
        desirability: (0, 0, 0, 0, 0, 0),
        risk: structure::Risk { damage: 0, fire: 0 },
        structure_type: structure::Type::Housing,
        commodities: HashMap::new()
    };

    let s1 = structure::Structure {
        name: "s1".to_owned(),
        size: (3, 1),
        employees: structure::Employees { current: 1, required: 2 },
        cost: 5000,
        desirability: (1, 2, 3, 4, 5, 6),
        risk: structure::Risk { damage: 10, fire: 3 },
        structure_type: structure::Type::Industry,
        commodities: HashMap::new()
    };


    let w0 = walker::Walker {
        name: "w0".to_owned(),
        patrol: None,
        life: None,
        commodities: HashMap::new()
    };

    let w1 = walker::Walker {
        name: "w1".to_owned(),
        patrol: Some(5),
        life: None,
        commodities: HashMap::new()
    };

    let e0 = Rc::new(Entity::Structure { id: Uuid::new_v4(), data: s0 });
    let e1 = Rc::new(Entity::Structure { id: Uuid::new_v4(), data: s1 });
    let e2 = Rc::new(Entity::Walker { id: Uuid::new_v4(), data: w0 });
    let e3 = Rc::new(Entity::Walker { id: Uuid::new_v4(), data: w1 });

    vec![e0, e1, e2, e3]
}
