extern crate owe;

mod setup;

use owe::grid::Direction;

#[test]
fn grid_should_add_entities() {
    //TODO - implement
}

#[test]
fn grid_should_not_add_overlapping_entities() {
    //TODO - implement
}

#[test]
fn grid_should_remove_entities_from_cell() {
    //TODO - implement
}

#[test]
fn grid_should_not_remove_entities_not_in_cell() {
    //TODO - implement
}

#[test]
fn grid_cursor_should_move_up() {
    let (g, mut gc) = setup::grid::with_direction_from(Direction::Up, (2, 2));

    assert_eq!(gc.position(), (2, 2));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (2, 1));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (2, 0));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (1, 2));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (1, 1));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (1, 0));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (0, 2));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (0, 1));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (0, 0));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (2, 2));
}

#[test]
fn grid_cursor_should_move_down() {
    let (g, mut gc) = setup::grid::with_direction_from(Direction::Down, (0, 0));

    assert_eq!(gc.position(), (0, 0));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (0, 1));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (0, 2));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (1, 0));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (1, 1));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (1, 2));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (2, 0));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (2, 1));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (2, 2));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (0, 0));
}

#[test]
fn grid_cursor_should_move_left() {
    let (g, mut gc) = setup::grid::with_direction_from(Direction::Left, (2, 2));

    assert_eq!(gc.position(), (2, 2));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (1, 2));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (0, 2));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (2, 1));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (1, 1));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (0, 1));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (2, 0));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (1, 0));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (0, 0));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (2, 2));
}

#[test]
fn grid_cursor_should_move_right() {
    let (g, mut gc) = setup::grid::with_direction_from(Direction::Right, (0, 0));

    assert_eq!(gc.position(), (0, 0));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (1, 0));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (2, 0));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (0, 1));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (1, 1));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (2, 1));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (0, 2));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (1, 2));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (2, 2));
    gc.process_and_advance(&g);
    assert_eq!(gc.position(), (0, 0));
}

#[test]
fn grid_cursor_should_process_effects() {
    //TODO - implement
}

#[test]
fn grid_cursor_should_process_enqueued_actions() {
    //TODO - implement
}

#[test]
fn grid_cursor_should_process_resource_production() {
    //TODO - implement
}

#[test]
fn grid_cursor_should_process_walker_production() {
    //TODO - implement
}
