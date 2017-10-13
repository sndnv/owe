extern crate owe;
extern crate uuid;

mod setup;

use std::rc::{Rc, Weak};
use owe::entities::Entity;
use owe::production::exchange::{CommodityExchange, CommodityState};

fn exchange_entities_to_vec(entities: &Vec<Weak<Entity>>) -> Vec<Rc<Entity>> {
    entities
        .into_iter()
        .map(|e| e.upgrade().unwrap())
        .collect::<Vec<_>>()
}

#[test]
fn exchange_should_add_update_and_remove_producers() {
    let mut exchange = setup::production::exchange_default();
    let commodities = setup::production::commodities_default();
    let entities = setup::production::entities_default();

    assert!(exchange.producers_of(&commodities[0].name).is_empty());
    assert!(exchange.producers_of(&commodities[1].name).is_empty());
    assert!(exchange.producers_of(&commodities[2].name).is_empty());

    assert!(exchange.consumers_of(&commodities[0].name).is_empty());
    assert!(exchange.consumers_of(&commodities[1].name).is_empty());
    assert!(exchange.consumers_of(&commodities[2].name).is_empty());

    exchange.add_producer(entities[0].clone(), &commodities[0].name);
    exchange.add_producer(entities[0].clone(), &commodities[1].name);
    exchange.add_producer(entities[0].clone(), &commodities[2].name);
    exchange.add_producer(entities[1].clone(), &commodities[2].name);

    exchange.add_consumer(entities[0].clone(), &commodities[2].name);
    exchange.add_consumer(entities[1].clone(), &commodities[0].name);
    exchange.add_consumer(entities[1].clone(), &commodities[1].name);

    assert_eq!(
        exchange_entities_to_vec(&exchange.producers_of(&commodities[0].name)),
        vec![entities[0].clone()]
    );

    assert_eq!(
        exchange_entities_to_vec(&exchange.producers_of(&commodities[1].name)),
        vec![entities[0].clone()]
    );

    assert_eq!(
        exchange_entities_to_vec(&exchange.producers_of(&commodities[2].name)),
        vec![entities[0].clone(), entities[1].clone()]
    );

    assert_eq!(
        exchange_entities_to_vec(&exchange.consumers_of(&commodities[0].name)),
        vec![entities[1].clone()]
    );

    assert_eq!(
        exchange_entities_to_vec(&exchange.consumers_of(&commodities[1].name)),
        vec![entities[1].clone()]
    );

    assert_eq!(
        exchange_entities_to_vec(&exchange.consumers_of(&commodities[2].name)),
        vec![entities[0].clone()]
    );

    //TODO - remove some entities
    //TODO - verify query
}

#[test]
fn exchange_should_add_update_and_remove_consumers() {
    //TODO - implement
}

#[test]
#[should_panic]
fn exchange_should_not_add_invalid_producers() {
    //TODO
}

#[test]
#[should_panic]
fn exchange_should_not_add_invalid_consumers() {
    //TODO
}

#[test]
#[should_panic]
fn exchange_should_not_update_state_for_invalid_entities() {
    //TODO
}

#[test]
#[should_panic]
fn exchange_should_not_add_duplicate_producers() {
    //TODO - implement
}

#[test]
#[should_panic]
fn exchange_should_not_add_duplicate_consumers() {
    //TODO - implement
}

#[test]
fn exchange_should_update_structures_state() {
    let mut exchange = setup::production::exchange_default();
    let commodities = setup::production::commodities_default();
    let entities = setup::production::entities_default();

    assert!(exchange.entities_that_need(&commodities[0].name).is_empty());
    assert!(exchange.entities_that_need(&commodities[1].name).is_empty());
    assert!(exchange.entities_that_need(&commodities[2].name).is_empty());

    assert!(exchange.entities_that_have(&commodities[0].name).is_empty());
    assert!(exchange.entities_that_have(&commodities[1].name).is_empty());
    assert!(exchange.entities_that_have(&commodities[2].name).is_empty());

    exchange.update_state(entities[0].clone(), &commodities[2], CommodityState::Required);
    exchange.update_state(entities[1].clone(), &commodities[0], CommodityState::Required);
    exchange.update_state(entities[1].clone(), &commodities[1], CommodityState::Required);
    exchange.update_state(entities[0].clone(), &commodities[0], CommodityState::Available);
    exchange.update_state(entities[0].clone(), &commodities[1], CommodityState::Available);
    exchange.update_state(entities[0].clone(), &commodities[2], CommodityState::Available);
    exchange.update_state(entities[1].clone(), &commodities[0], CommodityState::Available);
    exchange.update_state(entities[0].clone(), &commodities[1], CommodityState::Used);
    exchange.update_state(entities[1].clone(), &commodities[2], CommodityState::Used);

    assert_eq!(
        exchange_entities_to_vec(&exchange.entities_that_need(&commodities[0].name)),
        vec![entities[1].clone()]
    );

    assert_eq!(
        exchange_entities_to_vec(&exchange.entities_that_need(&commodities[1].name)),
        vec![entities[1].clone()]
    );

    assert_eq!(
        exchange_entities_to_vec(&exchange.entities_that_need(&commodities[2].name)),
        vec![entities[0].clone()]
    );

    //TODO - fix order issues
    assert_eq!(
        exchange_entities_to_vec(&exchange.entities_that_have(&commodities[0].name)),
        vec![entities[0].clone(), entities[1].clone()]
    );

    assert_eq!(
        exchange_entities_to_vec(&exchange.entities_that_have(&commodities[1].name)),
        vec![entities[0].clone()]
    );

    assert_eq!(
        exchange_entities_to_vec(&exchange.entities_that_have(&commodities[2].name)),
        vec![entities[0].clone()]
    );

    assert_eq!(
        exchange_entities_to_vec(&exchange.entities_that_have(&commodities[0].name)),
        vec![entities[0].clone(), entities[1].clone()]
    );

    assert_eq!(
        exchange_entities_to_vec(&exchange.entities_that_have(&commodities[1].name)),
        vec![entities[0].clone()]
    );

    assert_eq!(
        exchange_entities_to_vec(&exchange.entities_that_have(&commodities[2].name)),
        vec![entities[0].clone()]
    );

    //TODO - update status
    //TODO - verify query
    //TODO - remove some entities
    //TODO - verify query
}

#[test]
fn exchange_should_update_walkers_state() {
    //TODO - implement
}
