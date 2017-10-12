use production::Commodity;
use entities::Entity;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::rc::{Weak, Rc};
use uuid::Uuid;

type EntityStatsMap = HashMap<String, HashMap<Uuid, (Weak<Entity>, u32)>>;

pub enum CommodityState {
    Required,
    Available,
    InTransit,
    Used,
    Lost
}

pub struct CommodityExchange {
    required: EntityStatsMap,
    available: EntityStatsMap,
    in_transit: EntityStatsMap,

    producers: HashMap<String, Vec<Weak<Entity>>>,
    consumers: HashMap<String, Vec<Weak<Entity>>>,

    used: HashMap<String, usize>,
    lost: HashMap<String, usize>
}

impl CommodityExchange {
    pub fn new() -> CommodityExchange {
        CommodityExchange {
            required: HashMap::new(),
            available: HashMap::new(),
            in_transit: HashMap::new(),
            producers: HashMap::new(),
            consumers: HashMap::new(),
            used: HashMap::new(),
            lost: HashMap::new()
        }
    }

    //adds a new commodity producer to the exchange; removal is not needed
    pub fn add_producer(&mut self, producer: &Rc<Entity>, commodity: &str) -> () {
        let entity_ptr = Rc::downgrade(producer);

        match self.producers.entry(commodity.to_string()) {
            Entry::Occupied(entry) => {
                let commodity_producers = entry.into_mut();
                commodity_producers.retain(|e| e.upgrade().is_some());
                commodity_producers.push(entity_ptr);
            }

            Entry::Vacant(entry) => {
                entry.insert(vec![entity_ptr]);
            }
        }
    }

    //adds a new commodity consumer to the exchange; removal is not needed
    pub fn add_consumer(&mut self, consumer: &Rc<Entity>, commodity: &str) -> () {
        let entity_ptr = Rc::downgrade(consumer);

        match self.consumers.entry(commodity.to_string()) {
            Entry::Occupied(entry) => {
                let commodity_consumers = entry.into_mut();
                commodity_consumers.retain(|e| e.upgrade().is_some());
                commodity_consumers.push(entity_ptr);
            }

            Entry::Vacant(entry) => {
                entry.insert(vec![entity_ptr]);
            }
        }
    }

    fn do_update(entity_map: &mut EntityStatsMap, entity: Rc<Entity>, commodity: &Commodity) -> () {
        let entity_id = match *entity {
            Entity::Structure { id, .. } => id,
            Entity::Walker { id, .. } => id,
            ref other => panic!("Unexpected entity supplied: [{:?}]", other)
        };

        match entity_map
            .entry(commodity.name.clone()).or_insert_with(|| HashMap::new())
            .entry(entity_id) {
            Entry::Occupied(entry) => {
                entry.into_mut().1 = commodity.amount;
            }

            Entry::Vacant(entry) => {
                entry.insert((Rc::downgrade(&entity), commodity.amount));
            }
        }

        entity_map
            .get_mut(&commodity.name).unwrap()
            .retain(|_, v| v.0.upgrade().is_some())
    }

    pub fn update_state(&mut self, entity: Rc<Entity>, commodity: &Commodity, state: CommodityState) -> () {
        match state {
            CommodityState::Required => {
                Self::do_update(&mut self.required, entity, commodity);
            }

            CommodityState::Available => {
                Self::do_update(&mut self.available, entity, commodity);
            }

            CommodityState::InTransit => {
                Self::do_update(&mut self.in_transit, entity, commodity);
            }

            CommodityState::Used => {
                let amount = self.used.entry(commodity.name.clone()).or_insert(0);
                *amount += commodity.amount as usize;
            }

            CommodityState::Lost => {
                let amount = self.lost.entry(commodity.name.clone()).or_insert(0);
                *amount += commodity.amount as usize;
            }
        }
    }
}
