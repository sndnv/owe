use owe::grid;
use owe::entities::doodad;
use owe::entities::resource;
use owe::entities::structure;
use owe::entities::walker;
use owe::entities::Entity;
use owe::effects::Effect;
use owe::production::{Commodity, ProductionStage, Producer};
use owe::production::exchange;
use setup::effects::*;
use std::rc::Rc;
use std::collections::HashMap;
use uuid::Uuid;

pub fn grid_empty() -> grid::Grid {
    let g = grid::Grid::new(3);

    g
}

pub fn grid_default() -> grid::Grid {
    let mut g = grid::Grid::new(3);

    let d0 = doodad::Doodad { name: "d0".to_owned(), is_removable: false };
    let d1 = doodad::Doodad { name: "d1".to_owned(), is_removable: false };

    let r0 = resource::ResourceProperties { max_amount: 5, name: "r0".to_owned(), replenish_amount: Some(1) };
    let r1 = resource::ResourceProperties { max_amount: 5, name: "r1".to_owned(), replenish_amount: None };
    let r0_state = resource::ResourceState { current_amount: 2 };
    let r1_state = resource::ResourceState { current_amount: 5 };

    let s0 = structure::StructureProperties {
        name: "s0".to_owned(),
        size: structure::Size { width: 1, height: 1 },
        max_employees: 5,
        cost: 1000,
        desirability: (0, 0, 0, 0, 0, 0),
        structure_type: structure::Type::Housing
    };

    let s1 = structure::StructureProperties {
        name: "s1".to_owned(),
        size: structure::Size { width: 1, height: 1 },
        max_employees: 2,
        cost: 5000,
        desirability: (1, 2, 3, 4, 5, 6),
        structure_type: structure::Type::Industry
    };

    let s0_state = structure::StructureState {
        current_employees: 0,
        commodities: HashMap::new(),
        risk: structure::Risk { damage: 0, fire: 0 }
    };

    let s1_state = structure::StructureState {
        current_employees: 1,
        commodities: HashMap::new(),
        risk: structure::Risk { damage: 10, fire: 3 }
    };

    let w0 = walker::WalkerProperties {
        name: "w0".to_owned(),
        patrol: None,
        max_life: Some(3)
    };

    let w1 = walker::WalkerProperties {
        name: "w1".to_owned(),
        patrol: Some(5),
        max_life: Some(1)
    };

    let w0_state = walker::WalkerState {
        current_life: None,
        commodities: HashMap::new()
    };

    let w1_state = walker::WalkerState {
        current_life: Some(1),
        commodities: HashMap::new()
    };

    let _ = g.add_entity((0, 0), Entity::Doodad { props: d0 });
    let _ = g.add_entity((1, 0), Entity::Doodad { props: d1 });

    let _ = g.add_entity((2, 0), Entity::Resource {
        id: Uuid::new_v4(),
        props: r0,
        state: r0_state,
        producer: Some(Box::new(TestCommodityProducer0 {}))
    });

    let _ = g.add_entity((0, 1), Entity::Resource {
        id: Uuid::new_v4(),
        props: r1,
        state: r1_state,
        producer: Some(Box::new(TestCommodityProducer1 { max_progress: 100, current_progress: 0 }))
    });

    let _ = g.add_entity((2, 1), Entity::Structure {
        id: Uuid::new_v4(),
        props: s0,
        state: s0_state,
        producer: None
    });

    let _ = g.add_entity((0, 2), Entity::Structure {
        id: Uuid::new_v4(),
        props: s1,
        state: s1_state,
        producer: None
    });

    let _ = g.add_entity((1, 2), Entity::Walker { id: Uuid::new_v4(), props: w0, state: w0_state });
    let _ = g.add_entity((2, 2), Entity::Walker { id: Uuid::new_v4(), props: w1, state: w1_state });

    g
}

pub fn grid_large() -> grid::Grid {
    let mut g = grid::Grid::new(5);

    let d0 = doodad::Doodad { name: "d0".to_owned(), is_removable: false };
    let d1 = doodad::Doodad { name: "d1".to_owned(), is_removable: false };
    let d2 = doodad::Doodad { name: "d2".to_owned(), is_removable: false };

    let r0 = resource::ResourceProperties { max_amount: 5, name: "r0".to_owned(), replenish_amount: Some(5) };
    let r1 = resource::ResourceProperties { max_amount: 5, name: "r1".to_owned(), replenish_amount: None };
    let r2 = resource::ResourceProperties { max_amount: 10, name: "r2".to_owned(), replenish_amount: Some(1) };
    let r0_state = resource::ResourceState { current_amount: 0 };
    let r1_state = resource::ResourceState { current_amount: 3 };
    let r2_state = resource::ResourceState { current_amount: 10 };

    let s0 = structure::StructureProperties {
        name: "s0".to_owned(),
        size: structure::Size { width: 1, height: 1 },
        max_employees: 5,
        cost: 1000,
        desirability: (0, 0, 0, 0, 0, 0),
        structure_type: structure::Type::Housing
    };

    let s1 = structure::StructureProperties {
        name: "s1".to_owned(),
        size: structure::Size { width: 1, height: 1 },
        max_employees: 2,
        cost: 5000,
        desirability: (1, 2, 3, 4, 5, 6),
        structure_type: structure::Type::Industry
    };

    let s2 = structure::StructureProperties {
        name: "s2".to_owned(),
        size: structure::Size { width: 1, height: 1 },
        max_employees: 10,
        cost: 500,
        desirability: (1, 2, 3, 4, 5, 6),
        structure_type: structure::Type::CivilService
    };

    let s3 = structure::StructureProperties {
        name: "s3".to_owned(),
        size: structure::Size { width: 1, height: 1 },
        max_employees: 1,
        cost: 1,
        desirability: (1, 2, 3, 4, 5, 6),
        structure_type: structure::Type::Religion
    };

    let s0_state = structure::StructureState {
        current_employees: 0,
        commodities: HashMap::new(),
        risk: structure::Risk { damage: 0, fire: 0 }
    };

    let s1_state = structure::StructureState {
        current_employees: 1,
        commodities: HashMap::new(),
        risk: structure::Risk { damage: 10, fire: 3 }
    };

    let s2_state = structure::StructureState {
        current_employees: 1,
        commodities: HashMap::new(),
        risk: structure::Risk { damage: 1, fire: 1 }
    };

    let s3_state = structure::StructureState {
        current_employees: 1,
        commodities: HashMap::new(),
        risk: structure::Risk { damage: 10, fire: 10 }
    };

    let w0 = walker::WalkerProperties {
        name: "w0".to_owned(),
        patrol: None,
        max_life: None
    };

    let w1 = walker::WalkerProperties {
        name: "w1".to_owned(),
        patrol: Some(5),
        max_life: None
    };

    let w0_state = walker::WalkerState {
        current_life: None,
        commodities: HashMap::new()
    };

    let w1_state = walker::WalkerState {
        current_life: None,
        commodities: HashMap::new()
    };

    let _ = g.add_entity((1, 0), Entity::Doodad { props: d0 });
    let _ = g.add_entity((2, 3), Entity::Doodad { props: d1 });
    let _ = g.add_entity((3, 3), Entity::Doodad { props: d2 });

    let _ = g.add_entity((0, 3), Entity::Resource {
        id: Uuid::new_v4(),
        props: r0,
        state: r0_state,
        producer: Some(Box::new(TestCommodityProducer0 {}))
    });

    let _ = g.add_entity((0, 4), Entity::Resource {
        id: Uuid::new_v4(),
        props: r1,
        state: r1_state,
        producer: Some(Box::new(TestCommodityProducer1 { max_progress: 100, current_progress: 0 }))
    });

    let _ = g.add_entity((1, 4), Entity::Resource {
        id: Uuid::new_v4(),
        props: r2,
        state: r2_state,
        producer: None
    });

    let _ = g.add_entity((2, 0), Entity::Structure {
        id: Uuid::new_v4(),
        props: s0,
        state: s0_state,
        producer: None
    });

    let _ = g.add_entity((2, 1), Entity::Structure {
        id: Uuid::new_v4(),
        props: s1,
        state: s1_state,
        producer: None
    });

    let _ = g.add_entity((2, 2), Entity::Structure {
        id: Uuid::new_v4(),
        props: s2,
        state: s2_state,
        producer: None
    });

    let _ = g.add_entity((4, 1), Entity::Structure {
        id: Uuid::new_v4(),
        props: s3,
        state: s3_state,
        producer: None
    });

    let _ = g.add_entity((0, 2), Entity::Walker { id: Uuid::new_v4(), props: w0, state: w0_state });
    let _ = g.add_entity((4, 4), Entity::Walker { id: Uuid::new_v4(), props: w1, state: w1_state });

    g
}

pub fn grid_with_direction_from(direction: grid::Direction, from: (usize, usize)) -> (grid::Grid, grid::Cursor, exchange::CommodityExchange) {
    let g = grid_default();
    let gc = grid::Cursor::new(1, direction, from);
    let e = exchange::CommodityExchange::new();

    (g, gc, e)
}

pub fn grid_with_effects() -> (grid::Grid, grid::Cursor, exchange::CommodityExchange, Vec<Rc<Effect>>) {
    let g = grid_default();
    let gc = grid::Cursor::new(1, grid::Direction::Right, (0, 0));
    let e = exchange::CommodityExchange::new();
    let effects = effects_default();

    (g, gc, e, effects)
}

pub fn grid_with_production() -> (grid::Grid, grid::Cursor, exchange::CommodityExchange) {
    let mut g = grid::Grid::new(3);
    let gc = grid::Cursor::new(1, grid::Direction::Right, (0, 0));
    let mut e = exchange::CommodityExchange::new();

    let r0 = resource::ResourceProperties { max_amount: 5, name: "r0".to_owned(), replenish_amount: Some(1) };
    let r1 = resource::ResourceProperties { max_amount: 5, name: "r1".to_owned(), replenish_amount: None };
    let r0_state = resource::ResourceState { current_amount: 2 };
    let r1_state = resource::ResourceState { current_amount: 5 };

    let s0 = structure::StructureProperties {
        name: "s0".to_owned(),
        size: structure::Size { width: 1, height: 1 },
        max_employees: 5,
        cost: 1000,
        desirability: (0, 0, 0, 0, 0, 0),
        structure_type: structure::Type::Housing
    };

    let s1 = structure::StructureProperties {
        name: "s1".to_owned(),
        size: structure::Size { width: 1, height: 1 },
        max_employees: 2,
        cost: 5000,
        desirability: (1, 2, 3, 4, 5, 6),
        structure_type: structure::Type::Industry
    };

    let s0_state = structure::StructureState {
        current_employees: 5,
        commodities: HashMap::new(),
        risk: structure::Risk { damage: 0, fire: 0 }
    };

    let s1_state = structure::StructureState {
        current_employees: 1,
        commodities: HashMap::new(),
        risk: structure::Risk { damage: 10, fire: 3 }
    };

    let w0 = walker::WalkerProperties {
        name: "w0".to_owned(),
        patrol: None,
        max_life: Some(3)
    };

    let w1 = walker::WalkerProperties {
        name: "w1".to_owned(),
        patrol: Some(5),
        max_life: Some(1)
    };

    let w0_state = walker::WalkerState {
        current_life: None,
        commodities: HashMap::new()
    };

    let w1_state = walker::WalkerState {
        current_life: Some(1),
        commodities: HashMap::new()
    };

    let _ = g.add_entity((2, 0), Entity::Resource {
        id: Uuid::new_v4(),
        props: r0,
        state: r0_state,
        producer: Some(Box::new(TestCommodityProducer2 {}))
    });

    let _ = g.add_entity((0, 1), Entity::Resource {
        id: Uuid::new_v4(),
        props: r1,
        state: r1_state,
        producer: None
    });

    let _ = g.add_entity((2, 1), Entity::Structure {
        id: Uuid::new_v4(),
        props: s0,
        state: s0_state,
        producer: Some(Box::new(TestCommodityProducer0 {}))
    });

    let _ = g.add_entity((0, 2), Entity::Structure {
        id: Uuid::new_v4(),
        props: s1,
        state: s1_state,
        producer: Some(Box::new(TestCommodityProducer1 { max_progress: 100, current_progress: 0 }))
    });

    let _ = g.add_entity((1, 2), Entity::Walker { id: Uuid::new_v4(), props: w0, state: w0_state });
    let _ = g.add_entity((2, 2), Entity::Walker { id: Uuid::new_v4(), props: w1, state: w1_state });

    let _ = e.add_producer(g.entity((2, 1)).unwrap(), "c0");
    let _ = e.add_producer(g.entity((0, 2)).unwrap(), "c1");
    let _ = e.add_producer(g.entity((2, 0)).unwrap(), "c2");

    (g, gc, e)
}

#[derive(Clone)]
pub struct TestCommodityProducer0 {}

impl Producer for TestCommodityProducer0 {
    fn produce_commodity(&mut self, entity: &Entity) -> Option<ProductionStage> {
        match entity {
            &Entity::Structure { ref state, ref props, .. } => {
                if props.max_employees == state.current_employees {
                    Some(ProductionStage {
                        commodity: Commodity { name: "c0".to_owned(), amount: 100 },
                        required: vec![Commodity { name: "c2".to_owned(), amount: 3 }],
                        used: Vec::new()
                    })
                } else {
                    None
                }
            }

            _ => None //does nothing
        }
    }

    fn produce_walker(&mut self, _: &Entity) -> Option<walker::WalkerProperties> {
        None //no walker is produced
    }

    fn clone_boxed(&self) -> Box<Producer> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct TestCommodityProducer1 {
    max_progress: u8,
    current_progress: u8
}

impl Producer for TestCommodityProducer1 {
    fn produce_commodity(&mut self, entity: &Entity) -> Option<ProductionStage> {
        match entity {
            &Entity::Structure { .. } => {
                if self.max_progress < self.current_progress {
                    self.current_progress += 1;
                    None
                } else {
                    self.current_progress = 0;
                    Some(ProductionStage {
                        commodity: Commodity { name: "c1".to_owned(), amount: 1 },
                        required: Vec::new(),
                        used: vec![Commodity { name: "c0".to_owned(), amount: 2 }]
                    })
                }
            }

            _ => None //does nothing
        }
    }

    fn produce_walker(&mut self, _: &Entity) -> Option<walker::WalkerProperties> {
        None //no walker is produced
    }

    fn clone_boxed(&self) -> Box<Producer> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct TestCommodityProducer2 {}

impl Producer for TestCommodityProducer2 {
    fn produce_commodity(&mut self, entity: &Entity) -> Option<ProductionStage> {
        match entity {
            &Entity::Resource { .. } => {
                Some(ProductionStage {
                    commodity: Commodity { name: "c2".to_owned(), amount: 1 },
                    required: Vec::new(),
                    used: Vec::new()
                })
            }

            _ => None //does nothing
        }
    }

    fn produce_walker(&mut self, _: &Entity) -> Option<walker::WalkerProperties> {
        None //no walker is produced
    }

    fn clone_boxed(&self) -> Box<Producer> {
        Box::new(self.clone())
    }
}
