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

    fn do_update(entity_map: &mut EntityStatsMap, entity: Rc<Entity>, commodity: &Commodity) -> () {
        let entity_id = match *entity {
            //TODO - support Entity::Resource as producer (?)
            Entity::Structure { id, .. } => id,
            Entity::Walker { id, .. } => id,
            ref other => panic!("Unexpected entity supplied for state update: [{:?}]", other)
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

    pub fn collect_entities(entity_map: &EntityStatsMap, commodity: &str) -> Vec<Rc<Entity>> {
        entity_map
            .get(commodity)
            .map_or_else(
                || Vec::new(),
                |commodity_map| {
                    commodity_map
                        .into_iter()
                        .filter_map(
                            |(_, entity_data)| {
                                match entity_data.0.upgrade() {
                                    Some(ref ptr) if entity_data.1 > 0 => Some(ptr.clone()),
                                    _ => None
                                }
                            }
                        )
                        .collect()
                }
            )
    }

    fn fold_commodity_value(entity_map: &EntityStatsMap, commodity: &str) -> usize {
        entity_map
            .get(commodity)
            .map_or_else(
                || 0usize,
                |commodity_map| {
                    commodity_map
                        .into_iter()
                        .filter_map(
                            |(_, entity_data)| {
                                entity_data.0.upgrade().map(|_| entity_data.1 as usize)
                            }
                        )
                        .fold(0usize, |acc, current| acc + current)
                }
            )
    }

    fn find_duplicate_entity(entities: &Vec<Weak<Entity>>, entity_id: Uuid) -> Option<Weak<Entity>> {
        entities.into_iter()
            .find(|e| {
                e.upgrade().map_or(
                    false,
                    |e| {
                        match *e {
                            Entity::Structure { id, .. } => id == entity_id,
                            _ => false
                        }
                    })
            })
            .map(|e| e.clone())
    }

    //adds a new commodity producer to the exchange; removal is not needed
    pub fn add_producer(&mut self, producer: Rc<Entity>, commodity: &str) -> () {
        let producer_id = match *producer {
            //TODO - support Entity::Resource as producer (?)
            Entity::Structure { id, .. } => id,
            ref other => panic!("Unexpected entity supplied: [{:?}]", other)
        };

        let entity_ptr = Rc::downgrade(&producer);

        match self.producers.entry(commodity.to_string()) {
            Entry::Occupied(entry) => {
                Self::find_duplicate_entity(entry.get(), producer_id)
                    .map(|existing| panic!("Producer already added: [{:?}]", existing.upgrade()));

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
    pub fn add_consumer(&mut self, consumer: Rc<Entity>, commodity: &str) -> () {
        let consumer_id = match *consumer {
            Entity::Structure { id, .. } => id,
            ref other => panic!("Unexpected entity supplied: [{:?}]", other)
        };

        let entity_ptr = Rc::downgrade(&consumer);

        match self.consumers.entry(commodity.to_string()) {
            Entry::Occupied(entry) => {
                Self::find_duplicate_entity(entry.get(), consumer_id)
                    .map(|existing| panic!("Consumer already added: [{:?}]", existing.upgrade()));

                let commodity_consumers = entry.into_mut();
                commodity_consumers.retain(|e| e.upgrade().is_some());
                commodity_consumers.push(entity_ptr);
            }

            Entry::Vacant(entry) => {
                entry.insert(vec![entity_ptr]);
            }
        }
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

    pub fn entities_that_need(&self, commodity: &str) -> Vec<Rc<Entity>> {
        Self::collect_entities(&self.required, commodity)
    }

    pub fn entities_that_have(&self, commodity: &str) -> Vec<Rc<Entity>> {
        Self::collect_entities(&self.available, commodity)
    }

    pub fn entities_transporting(&self, commodity: &str) -> Vec<Rc<Entity>> {
        Self::collect_entities(&self.in_transit, commodity)
    }

    pub fn producers_of(&self, commodity: &str) -> Vec<Rc<Entity>> {
        self.producers
            .get(commodity)
            .map_or_else(
                || Vec::new(),
                |v| {
                    v.into_iter().filter_map(|e| e.upgrade()).collect()
                }
            )
    }

    pub fn consumers_of(&self, commodity: &str) -> Vec<Rc<Entity>> {
        self.consumers
            .get(commodity)
            .map_or_else(
                || Vec::new(),
                |v| {
                    v.into_iter().filter_map(|e| e.upgrade()).collect()
                }
            )
    }

    pub fn amount_required_of(&self, commodity: &str) -> usize {
        Self::fold_commodity_value(&self.required, commodity)
    }

    pub fn amount_available_of(&self, commodity: &str) -> usize {
        Self::fold_commodity_value(&self.available, commodity)
    }

    pub fn amount_in_transit_of(&self, commodity: &str) -> usize {
        Self::fold_commodity_value(&self.in_transit, commodity)
    }

    pub fn amount_used_of(&self, commodity: &str) -> usize {
        self.used
            .get(commodity)
            .map_or_else(
                || 0,
                |v| *v
            )
    }

    pub fn amount_lost_of(&self, commodity: &str) -> usize {
        self.lost
            .get(commodity)
            .map_or_else(
                || 0,
                |v| *v
            )
    }
}
