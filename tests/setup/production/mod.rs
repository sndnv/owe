use owe::entities::doodad;
use owe::entities::Entity;
use owe::entities::resource;
use owe::entities::structure;
use owe::entities::walker;
use owe::production::Commodity;
use owe::production::exchange::CommodityExchange;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

#[allow(dead_code)]
pub fn exchange_default() -> CommodityExchange {
    CommodityExchange::new()
}

#[allow(dead_code)]
pub fn commodities_default() -> Vec<Commodity> {
    let c0 = Commodity { name: "c0".to_owned(), amount: 1 };
    let c1 = Commodity { name: "c1".to_owned(), amount: 3 };
    let c2 = Commodity { name: "c2".to_owned(), amount: 10 };

    vec![c0, c1, c2]
}

#[allow(dead_code)]
pub fn entities_default() -> Vec<(Uuid, Rc<Entity>)> {
    let s0 = structure::StructureProperties {
        name: "s0".to_owned(),
        size: structure::Size { width: 1, height: 1 },
        max_employees: 5,
        cost: 1000,
        desirability: (0, 0, 0, 0, 0, 0),
        structure_type: structure::Type::Housing,
    };

    let s1 = structure::StructureProperties {
        name: "s1".to_owned(),
        size: structure::Size { width: 3, height: 1 },
        max_employees: 2,
        cost: 5000,
        desirability: (1, 2, 3, 4, 5, 6),
        structure_type: structure::Type::Industry,
    };

    let s0_state = structure::StructureState {
        current_employees: 0,
        commodities: HashMap::new(),
        risk: structure::Risk { damage: 0, fire: 0 },
    };

    let s1_state = structure::StructureState {
        current_employees: 1,
        commodities: HashMap::new(),
        risk: structure::Risk { damage: 10, fire: 3 },
    };

    let w0 = walker::WalkerProperties {
        name: "w0".to_owned(),
        patrol: None,
        max_life: None,
    };

    let w1 = walker::WalkerProperties {
        name: "w1".to_owned(),
        patrol: Some(5),
        max_life: None,
    };

    let w0_state = walker::WalkerState {
        current_life: None,
        commodities: HashMap::new(),
    };

    let w1_state = walker::WalkerState {
        current_life: None,
        commodities: HashMap::new(),
    };

    let d0 = doodad::Doodad {
        name: "d0".to_owned(),
        is_removable: false,
    };

    let d1 = doodad::Doodad {
        name: "d1".to_owned(),
        is_removable: true,
    };

    let r0 = resource::ResourceProperties {
        name: "r0".to_owned(),
        max_amount: 10,
        replenish_amount: Some(15),
    };

    let r1 = resource::ResourceProperties {
        name: "r1".to_owned(),
        max_amount: 10,
        replenish_amount: None,
    };

    let r0_state = resource::ResourceState { current_amount: 0 };

    let r1_state = resource::ResourceState { current_amount: 5 };

    let e0 = Rc::new(Entity::Structure { props: s0, state: s0_state, producer: None });
    let e1 = Rc::new(Entity::Structure { props: s1, state: s1_state, producer: None });
    let e2 = Rc::new(Entity::Walker { props: w0, state: w0_state });
    let e3 = Rc::new(Entity::Walker { props: w1, state: w1_state });
    let e4 = Rc::new(Entity::Road);
    let e5 = Rc::new(Entity::Road);
    let e6 = Rc::new(Entity::Roadblock);
    let e7 = Rc::new(Entity::Roadblock);
    let e8 = Rc::new(Entity::Doodad { props: d0 });
    let e9 = Rc::new(Entity::Doodad { props: d1 });
    let e10 = Rc::new(Entity::Resource { props: r0, state: r0_state, producer: None });
    let e11 = Rc::new(Entity::Resource { props: r1, state: r1_state, producer: None });

    vec![
        (Uuid::new_v4(), e0),
        (Uuid::new_v4(), e1),
        (Uuid::new_v4(), e2),
        (Uuid::new_v4(), e3),
        (Uuid::new_v4(), e4),
        (Uuid::new_v4(), e5),
        (Uuid::new_v4(), e6),
        (Uuid::new_v4(), e7),
        (Uuid::new_v4(), e8),
        (Uuid::new_v4(), e9),
        (Uuid::new_v4(), e10),
        (Uuid::new_v4(), e11)
    ]
}
