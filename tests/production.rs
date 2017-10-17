extern crate owe;
extern crate uuid;

mod setup;

use std::rc::Rc;
use owe::entities::Entity;
use owe::production::Commodity;
use owe::production::exchange::CommodityState;

fn sort_entities(entities: &Vec<Rc<Entity>>) -> Vec<Rc<Entity>> {
    let mut result = entities.clone();

    result.sort_by_key(|&ref e| -> String {
        match *e.clone() {
            Entity::Structure { ref props, .. } => &props.name,
            Entity::Walker { ref props, .. } => &props.name,
            Entity::Resource { ref props, .. } => &props.name,
            Entity::Doodad { ref props, .. } => &props.name,
            _ => "none"
        }.to_owned()
    });

    result
}

#[test]
fn exchange_should_add_and_remove_producers() {
    let mut exchange = setup::production::exchange_default();
    let commodities = setup::production::commodities_default();
    let mut entities = setup::production::entities_default();

    assert!(exchange.producers_of(&commodities[0].name).is_empty());
    assert!(exchange.producers_of(&commodities[1].name).is_empty());
    assert!(exchange.producers_of(&commodities[2].name).is_empty());

    exchange.add_producer(entities[0].clone(), &commodities[0].name);
    exchange.add_producer(entities[0].clone(), &commodities[1].name);
    exchange.add_producer(entities[0].clone(), &commodities[2].name);
    exchange.add_producer(entities[1].clone(), &commodities[2].name);

    assert_eq!(sort_entities(&exchange.producers_of(&commodities[0].name)), vec![entities[0].clone()]);
    assert_eq!(sort_entities(&exchange.producers_of(&commodities[1].name)), vec![entities[0].clone()]);
    assert_eq!(sort_entities(&exchange.producers_of(&commodities[2].name)), vec![entities[0].clone(), entities[1].clone()]);

    entities.remove(0);

    assert!(sort_entities(&exchange.producers_of(&commodities[0].name)).is_empty());
    assert!(sort_entities(&exchange.producers_of(&commodities[1].name)).is_empty());
    assert_eq!(sort_entities(&exchange.producers_of(&commodities[2].name)), vec![entities[0].clone()]);
}

#[test]
fn exchange_should_add_and_remove_consumers() {
    let mut exchange = setup::production::exchange_default();
    let commodities = setup::production::commodities_default();
    let mut entities = setup::production::entities_default();

    assert!(exchange.consumers_of(&commodities[0].name).is_empty());
    assert!(exchange.consumers_of(&commodities[1].name).is_empty());
    assert!(exchange.consumers_of(&commodities[2].name).is_empty());

    exchange.add_consumer(entities[0].clone(), &commodities[2].name);
    exchange.add_consumer(entities[1].clone(), &commodities[0].name);
    exchange.add_consumer(entities[1].clone(), &commodities[1].name);

    assert_eq!(sort_entities(&exchange.consumers_of(&commodities[0].name)), vec![entities[1].clone()]);
    assert_eq!(sort_entities(&exchange.consumers_of(&commodities[1].name)), vec![entities[1].clone()]);
    assert_eq!(sort_entities(&exchange.consumers_of(&commodities[2].name)), vec![entities[0].clone()]);

    entities.remove(0);

    assert_eq!(sort_entities(&exchange.consumers_of(&commodities[0].name)), vec![entities[0].clone()]);
    assert_eq!(sort_entities(&exchange.consumers_of(&commodities[1].name)), vec![entities[0].clone()]);
    assert!(sort_entities(&exchange.consumers_of(&commodities[2].name)).is_empty());
}

#[test]
#[should_panic(expected = "Unexpected entity supplied")]
fn exchange_should_not_add_road_as_producer() {
    let mut exchange = setup::production::exchange_default();
    let entities = setup::production::entities_default();

    exchange.add_producer(entities[4].clone(), "c0");
}

#[test]
#[should_panic(expected = "Unexpected entity supplied")]
fn exchange_should_not_add_roadblock_as_producer() {
    let mut exchange = setup::production::exchange_default();
    let entities = setup::production::entities_default();

    exchange.add_producer(entities[6].clone(), "c0");
}

#[test]
#[should_panic(expected = "Unexpected entity supplied")]
fn exchange_should_not_add_doodad_as_producer() {
    let mut exchange = setup::production::exchange_default();
    let entities = setup::production::entities_default();

    exchange.add_producer(entities[8].clone(), "c0");
}

#[test]
#[should_panic(expected = "Unexpected entity supplied")]
fn exchange_should_not_add_resource_as_producer() {
    let mut exchange = setup::production::exchange_default();
    let entities = setup::production::entities_default();

    exchange.add_producer(entities[10].clone(), "c0");
}

#[test]
#[should_panic(expected = "Unexpected entity supplied")]
fn exchange_should_not_add_walker_as_producer() {
    let mut exchange = setup::production::exchange_default();
    let entities = setup::production::entities_default();

    exchange.add_producer(entities[2].clone(), "c0");
}

#[test]
#[should_panic(expected = "Unexpected entity supplied")]
fn exchange_should_not_add_road_as_consumer() {
    let mut exchange = setup::production::exchange_default();
    let entities = setup::production::entities_default();

    exchange.add_consumer(entities[4].clone(), "c0");
}

#[test]
#[should_panic(expected = "Unexpected entity supplied")]
fn exchange_should_not_add_roadblock_as_consumer() {
    let mut exchange = setup::production::exchange_default();
    let entities = setup::production::entities_default();

    exchange.add_consumer(entities[6].clone(), "c0");
}

#[test]
#[should_panic(expected = "Unexpected entity supplied")]
fn exchange_should_not_add_doodad_as_consumer() {
    let mut exchange = setup::production::exchange_default();
    let entities = setup::production::entities_default();

    exchange.add_consumer(entities[8].clone(), "c0");
}

#[test]
#[should_panic(expected = "Unexpected entity supplied")]
fn exchange_should_not_add_resource_as_consumer() {
    let mut exchange = setup::production::exchange_default();
    let entities = setup::production::entities_default();

    exchange.add_consumer(entities[10].clone(), "c0");
}

#[test]
#[should_panic(expected = "Unexpected entity supplied")]
fn exchange_should_not_add_walker_as_consumer() {
    let mut exchange = setup::production::exchange_default();
    let entities = setup::production::entities_default();

    exchange.add_consumer(entities[2].clone(), "c0");
}

#[test]
#[should_panic(expected = "Producer already added")]
fn exchange_should_not_add_duplicate_producers() {
    let mut exchange = setup::production::exchange_default();
    let commodities = setup::production::commodities_default();
    let entities = setup::production::entities_default();

    assert!(exchange.producers_of(&commodities[0].name).is_empty());
    assert!(exchange.producers_of(&commodities[1].name).is_empty());
    assert!(exchange.producers_of(&commodities[2].name).is_empty());

    exchange.add_producer(entities[0].clone(), &commodities[0].name);
    exchange.add_producer(entities[0].clone(), &commodities[0].name);
}

#[test]
#[should_panic(expected = "Consumer already added")]
fn exchange_should_not_add_duplicate_consumers() {
    let mut exchange = setup::production::exchange_default();
    let commodities = setup::production::commodities_default();
    let entities = setup::production::entities_default();

    assert!(exchange.consumers_of(&commodities[0].name).is_empty());
    assert!(exchange.consumers_of(&commodities[1].name).is_empty());
    assert!(exchange.consumers_of(&commodities[2].name).is_empty());

    exchange.add_consumer(entities[0].clone(), &commodities[2].name);
    exchange.add_consumer(entities[0].clone(), &commodities[2].name);
}

#[test]
fn exchange_should_update_structures_state() {
    let mut exchange = setup::production::exchange_default();
    let commodities = setup::production::commodities_default();
    let mut entities = setup::production::entities_default();

    assert!(exchange.entities_that_need(&commodities[0].name).is_empty());
    assert!(exchange.entities_that_need(&commodities[1].name).is_empty());
    assert!(exchange.entities_that_need(&commodities[2].name).is_empty());

    assert!(exchange.entities_that_have(&commodities[0].name).is_empty());
    assert!(exchange.entities_that_have(&commodities[1].name).is_empty());
    assert!(exchange.entities_that_have(&commodities[2].name).is_empty());

    assert_eq!(exchange.amount_required_of(&commodities[0].name), 0);
    assert_eq!(exchange.amount_required_of(&commodities[1].name), 0);
    assert_eq!(exchange.amount_required_of(&commodities[2].name), 0);

    assert_eq!(exchange.amount_available_of(&commodities[0].name), 0);
    assert_eq!(exchange.amount_available_of(&commodities[1].name), 0);
    assert_eq!(exchange.amount_available_of(&commodities[2].name), 0);

    assert_eq!(exchange.amount_used_of(&commodities[0].name), 0);
    assert_eq!(exchange.amount_used_of(&commodities[1].name), 0);
    assert_eq!(exchange.amount_used_of(&commodities[2].name), 0);

    exchange.update_state(entities[0].clone(), &commodities[2], CommodityState::Required);
    exchange.update_state(entities[1].clone(), &commodities[0], CommodityState::Required);
    exchange.update_state(entities[1].clone(), &commodities[1], CommodityState::Required);
    exchange.update_state(entities[0].clone(), &commodities[0], CommodityState::Available);
    exchange.update_state(entities[0].clone(), &commodities[1], CommodityState::Available);
    exchange.update_state(entities[0].clone(), &commodities[2], CommodityState::Available);
    exchange.update_state(entities[1].clone(), &commodities[0], CommodityState::Available);
    exchange.update_state(entities[0].clone(), &commodities[1], CommodityState::Used);
    exchange.update_state(entities[1].clone(), &commodities[2], CommodityState::Used);

    assert_eq!(sort_entities(&exchange.entities_that_need(&commodities[0].name)), vec![entities[1].clone()]);
    assert_eq!(sort_entities(&exchange.entities_that_need(&commodities[1].name)), vec![entities[1].clone()]);
    assert_eq!(sort_entities(&exchange.entities_that_need(&commodities[2].name)), vec![entities[0].clone()]);

    assert_eq!(sort_entities(&exchange.entities_that_have(&commodities[0].name)), vec![entities[0].clone(), entities[1].clone()]);
    assert_eq!(sort_entities(&exchange.entities_that_have(&commodities[1].name)), vec![entities[0].clone()]);
    assert_eq!(sort_entities(&exchange.entities_that_have(&commodities[2].name)), vec![entities[0].clone()]);

    assert_eq!(exchange.amount_required_of(&commodities[0].name), commodities[0].amount as usize);
    assert_eq!(exchange.amount_required_of(&commodities[1].name), commodities[1].amount as usize);
    assert_eq!(exchange.amount_required_of(&commodities[2].name), commodities[2].amount as usize);

    assert_eq!(exchange.amount_available_of(&commodities[0].name), (commodities[0].amount * 2) as usize);
    assert_eq!(exchange.amount_available_of(&commodities[1].name), commodities[1].amount as usize);
    assert_eq!(exchange.amount_available_of(&commodities[2].name), commodities[2].amount as usize);

    assert_eq!(exchange.amount_used_of(&commodities[0].name), 0);
    assert_eq!(exchange.amount_used_of(&commodities[1].name), commodities[1].amount as usize);
    assert_eq!(exchange.amount_used_of(&commodities[2].name), commodities[2].amount as usize);

    exchange.update_state(entities[0].clone(), &Commodity { amount: 0, name: "c2".to_owned() }, CommodityState::Required);
    exchange.update_state(entities[0].clone(), &Commodity { amount: 0, name: "c2".to_owned() }, CommodityState::Available);

    assert_eq!(sort_entities(&exchange.entities_that_need(&commodities[0].name)), vec![entities[1].clone()]);
    assert_eq!(sort_entities(&exchange.entities_that_need(&commodities[1].name)), vec![entities[1].clone()]);
    assert!(sort_entities(&exchange.entities_that_need(&commodities[2].name)).is_empty());

    assert_eq!(sort_entities(&exchange.entities_that_have(&commodities[0].name)), vec![entities[0].clone(), entities[1].clone()]);
    assert_eq!(sort_entities(&exchange.entities_that_have(&commodities[1].name)), vec![entities[0].clone()]);
    assert!(sort_entities(&exchange.entities_that_have(&commodities[2].name)).is_empty());

    assert_eq!(exchange.amount_required_of(&commodities[0].name), commodities[0].amount as usize);
    assert_eq!(exchange.amount_required_of(&commodities[1].name), commodities[1].amount as usize);
    assert_eq!(exchange.amount_required_of(&commodities[2].name), 0);

    assert_eq!(exchange.amount_available_of(&commodities[0].name), (commodities[0].amount * 2) as usize);
    assert_eq!(exchange.amount_available_of(&commodities[1].name), commodities[1].amount as usize);
    assert_eq!(exchange.amount_available_of(&commodities[2].name), 0);

    assert_eq!(exchange.amount_used_of(&commodities[0].name), 0);
    assert_eq!(exchange.amount_used_of(&commodities[1].name), commodities[1].amount as usize);
    assert_eq!(exchange.amount_used_of(&commodities[2].name), commodities[2].amount as usize);

    entities.remove(0);

    assert_eq!(sort_entities(&exchange.entities_that_need(&commodities[0].name)), vec![entities[0].clone()]);
    assert_eq!(sort_entities(&exchange.entities_that_need(&commodities[1].name)), vec![entities[0].clone()]);
    assert!(sort_entities(&exchange.entities_that_need(&commodities[2].name)).is_empty());

    assert_eq!(sort_entities(&exchange.entities_that_have(&commodities[0].name)), vec![entities[0].clone()]);
    assert!(sort_entities(&exchange.entities_that_have(&commodities[1].name)).is_empty());
    assert!(sort_entities(&exchange.entities_that_have(&commodities[2].name)).is_empty());

    assert_eq!(exchange.amount_required_of(&commodities[0].name), commodities[0].amount as usize);
    assert_eq!(exchange.amount_required_of(&commodities[1].name), commodities[1].amount as usize);
    assert_eq!(exchange.amount_required_of(&commodities[2].name), 0);

    assert_eq!(exchange.amount_available_of(&commodities[0].name), commodities[0].amount as usize);
    assert_eq!(exchange.amount_available_of(&commodities[1].name), 0);
    assert_eq!(exchange.amount_available_of(&commodities[2].name), 0);

    assert_eq!(exchange.amount_used_of(&commodities[0].name), 0);
    assert_eq!(exchange.amount_used_of(&commodities[1].name), commodities[1].amount as usize);
    assert_eq!(exchange.amount_used_of(&commodities[2].name), commodities[2].amount as usize);
}

#[test]
fn exchange_should_update_walkers_state() {
    let mut exchange = setup::production::exchange_default();
    let commodities = setup::production::commodities_default();
    let mut entities = setup::production::entities_default();

    assert!(exchange.entities_transporting(&commodities[0].name).is_empty());
    assert!(exchange.entities_transporting(&commodities[1].name).is_empty());
    assert!(exchange.entities_transporting(&commodities[2].name).is_empty());

    assert_eq!(exchange.amount_in_transit_of(&commodities[0].name), 0);
    assert_eq!(exchange.amount_in_transit_of(&commodities[1].name), 0);
    assert_eq!(exchange.amount_in_transit_of(&commodities[2].name), 0);

    assert_eq!(exchange.amount_lost_of(&commodities[0].name), 0);
    assert_eq!(exchange.amount_lost_of(&commodities[1].name), 0);
    assert_eq!(exchange.amount_lost_of(&commodities[2].name), 0);

    exchange.update_state(entities[2].clone(), &commodities[0], CommodityState::InTransit);
    exchange.update_state(entities[3].clone(), &commodities[1], CommodityState::InTransit);
    exchange.update_state(entities[3].clone(), &commodities[2], CommodityState::InTransit);

    assert_eq!(exchange.amount_in_transit_of(&commodities[0].name), commodities[0].amount as usize);
    assert_eq!(exchange.amount_in_transit_of(&commodities[1].name), commodities[1].amount as usize);
    assert_eq!(exchange.amount_in_transit_of(&commodities[2].name), commodities[2].amount as usize);

    assert_eq!(exchange.amount_lost_of(&commodities[0].name), 0);
    assert_eq!(exchange.amount_lost_of(&commodities[1].name), 0);
    assert_eq!(exchange.amount_lost_of(&commodities[2].name), 0);

    assert_eq!(sort_entities(&exchange.entities_transporting(&commodities[0].name)), vec![entities[2].clone()]);
    assert_eq!(sort_entities(&exchange.entities_transporting(&commodities[1].name)), vec![entities[3].clone()]);
    assert_eq!(sort_entities(&exchange.entities_transporting(&commodities[2].name)), vec![entities[3].clone()]);

    exchange.update_state(entities[2].clone(), &Commodity { amount: 0, name: "c0".to_owned() }, CommodityState::InTransit);
    exchange.update_state(entities[2].clone(), &commodities[0], CommodityState::Lost);

    assert!(sort_entities(&exchange.entities_transporting(&commodities[0].name)).is_empty());
    assert_eq!(sort_entities(&exchange.entities_transporting(&commodities[1].name)), vec![entities[3].clone()]);
    assert_eq!(sort_entities(&exchange.entities_transporting(&commodities[2].name)), vec![entities[3].clone()]);

    assert_eq!(exchange.amount_in_transit_of(&commodities[0].name), 0);
    assert_eq!(exchange.amount_in_transit_of(&commodities[1].name), commodities[1].amount as usize);
    assert_eq!(exchange.amount_in_transit_of(&commodities[2].name), commodities[2].amount as usize);

    assert_eq!(exchange.amount_lost_of(&commodities[0].name), commodities[0].amount as usize);
    assert_eq!(exchange.amount_lost_of(&commodities[1].name), 0);
    assert_eq!(exchange.amount_lost_of(&commodities[2].name), 0);

    entities.remove(3);

    assert!(sort_entities(&exchange.entities_transporting(&commodities[0].name)).is_empty());
    assert!(sort_entities(&exchange.entities_transporting(&commodities[1].name)).is_empty());
    assert!(sort_entities(&exchange.entities_transporting(&commodities[2].name)).is_empty());

    assert_eq!(exchange.amount_in_transit_of(&commodities[0].name), 0);
    assert_eq!(exchange.amount_in_transit_of(&commodities[1].name), 0);
    assert_eq!(exchange.amount_in_transit_of(&commodities[2].name), 0);

    assert_eq!(exchange.amount_lost_of(&commodities[0].name), commodities[0].amount as usize);
    assert_eq!(exchange.amount_lost_of(&commodities[1].name), 0);
    assert_eq!(exchange.amount_lost_of(&commodities[2].name), 0);
}

#[test]
#[should_panic(expected = "Unexpected entity supplied for state update")]
fn exchange_should_not_update_state_for_road_entity() {
    let mut exchange = setup::production::exchange_default();
    let commodities = setup::production::commodities_default();
    let entities = setup::production::entities_default();

    exchange.update_state(entities[4].clone(), &commodities[0], CommodityState::Available);
}

#[test]
#[should_panic(expected = "Unexpected entity supplied for state update")]
fn exchange_should_not_update_state_for_roadblock_entity() {
    let mut exchange = setup::production::exchange_default();
    let commodities = setup::production::commodities_default();
    let entities = setup::production::entities_default();

    exchange.update_state(entities[6].clone(), &commodities[0], CommodityState::Available);
}

#[test]
#[should_panic(expected = "Unexpected entity supplied for state update")]
fn exchange_should_not_update_state_for_doodad_entity() {
    let mut exchange = setup::production::exchange_default();
    let commodities = setup::production::commodities_default();
    let entities = setup::production::entities_default();

    exchange.update_state(entities[8].clone(), &commodities[0], CommodityState::Available);
}

#[test]
#[should_panic(expected = "Unexpected entity supplied for state update")]
fn exchange_should_not_update_state_for_resource_entity() {
    let mut exchange = setup::production::exchange_default();
    let commodities = setup::production::commodities_default();
    let entities = setup::production::entities_default();

    exchange.update_state(entities[10].clone(), &commodities[0], CommodityState::Available);
}
