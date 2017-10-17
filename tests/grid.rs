extern crate owe;
extern crate uuid;

mod setup;

use owe::grid::Direction;

fn sort_cells(cells: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut result = cells.clone();
    result.sort();
    result
}

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
fn grid_without_entities_should_report_correct_cell_neighbors() {
    let g = setup::grid::grid_empty();

    assert_eq!(
        sort_cells(&g.neighbors_of((0, 0))),
        sort_cells(&vec![(1, 0), (0, 1), (1, 1)])
    );

    assert_eq!(
        sort_cells(&g.neighbors_of((1, 0))),
        sort_cells(&vec![(0, 0), (2, 0), (0, 1), (1, 1), (2, 1)])
    );

    assert_eq!(
        sort_cells(&g.neighbors_of((2, 0))),
        sort_cells(&vec![(1, 0), (1, 1), (2, 1)])
    );

    assert_eq!(
        sort_cells(&g.neighbors_of((0, 1))),
        sort_cells(&vec![(0, 0), (1, 0), (1, 1), (0, 2), (1, 2)])
    );

    assert_eq!(
        sort_cells(&g.neighbors_of((1, 1))),
        sort_cells(&vec![(0, 0), (1, 0), (2, 0), (0, 1), (2, 1), (0, 2), (1, 2), (2, 2)])
    );

    assert_eq!(
        sort_cells(&g.neighbors_of((2, 1))),
        sort_cells(&vec![(1, 0), (2, 0), (1, 1), (1, 2), (2, 2)])
    );

    assert_eq!(
        sort_cells(&g.neighbors_of((0, 2))),
        sort_cells(&vec![(0, 1), (1, 1), (1, 2)])
    );

    assert_eq!(
        sort_cells(&g.neighbors_of((1, 2))),
        sort_cells(&vec![(0, 1), (1, 1), (2, 1), (0, 2), (2, 2)])
    );

    assert_eq!(
        sort_cells(&g.neighbors_of((2, 2))),
        sort_cells(&vec![(1, 1), (2, 1), (1, 2)])
    );
}

#[test]
fn grid_with_entities_should_report_correct_cell_neighbors() {
    let g = setup::grid::grid_default();

    assert_eq!(
        sort_cells(&g.neighbors_of((0, 0))),
        sort_cells(&vec![(1, 1)])
    );

    assert_eq!(
        sort_cells(&g.neighbors_of((1, 0))),
        sort_cells(&vec![(1, 1)])
    );

    assert_eq!(
        sort_cells(&g.neighbors_of((2, 0))),
        sort_cells(&vec![(1, 1)])
    );

    assert_eq!(
        sort_cells(&g.neighbors_of((0, 1))),
        sort_cells(&vec![(1, 1), (1, 2)])
    );

    assert_eq!(
        sort_cells(&g.neighbors_of((1, 1))),
        sort_cells(&vec![(1, 2), (2, 2)])
    );

    assert_eq!(
        sort_cells(&g.neighbors_of((2, 1))),
        sort_cells(&vec![(1, 1), (1, 2), (2, 2)])
    );

    assert_eq!(
        sort_cells(&g.neighbors_of((0, 2))),
        sort_cells(&vec![(1, 1), (1, 2)])
    );

    assert_eq!(
        sort_cells(&g.neighbors_of((1, 2))),
        sort_cells(&vec![(1, 1), (2, 2)])
    );

    assert_eq!(
        sort_cells(&g.neighbors_of((2, 2))),
        sort_cells(&vec![(1, 1), (1, 2)])
    );
}

#[test]
fn grid_without_entities_should_calculate_paths_between_cells() {
    let g = setup::grid::grid_empty();

    assert_eq!(
        g.path_between((0, 0), (0, 0)),
        Some((vec![(0, 0)], 0))
    );

    assert_eq!(
        g.path_between((0, 0), (1, 0)),
        Some((vec![(0, 0), (1, 0)], 1))
    );

    assert_eq!(
        g.path_between((0, 0), (2, 0)),
        Some((vec![(0, 0), (1, 1), (2, 0)], 2))
    );

    assert_eq!(
        g.path_between((0, 0), (0, 1)),
        Some((vec![(0, 0), (0, 1)], 1))
    );

    assert_eq!(
        g.path_between((0, 0), (1, 1)),
        Some((vec![(0, 0), (1, 1)], 1))
    );

    assert_eq!(
        g.path_between((0, 0), (2, 1)),
        Some((vec![(0, 0), (1, 1), (2, 1)], 2))
    );

    assert_eq!(
        g.path_between((0, 0), (0, 2)),
        Some((vec![(0, 0), (0, 1), (0, 2)], 2))
    );

    assert_eq!(
        g.path_between((0, 0), (1, 2)),
        Some((vec![(0, 0), (0, 1), (1, 2)], 2))
    );

    assert_eq!(
        g.path_between((0, 0), (2, 2)),
        Some((vec![(0, 0), (1, 1), (2, 2)], 2))
    );
}

#[test]
fn grid_with_entities_should_calculate_paths_between_cells() {
    let mut g = setup::grid::grid_large();

    assert_eq!(
        g.path_between((0, 0), (4, 0)),
        Some((vec![(0, 0), (0, 1), (0, 2), (1, 3), (2, 4), (3, 4), (4, 3), (3, 2), (3, 1), (4, 0)], 9))
    );

    g.remove((2, 2));

    assert_eq!(
        g.path_between((0, 0), (4, 0)),
        Some((vec![(0, 0), (1, 1), (2, 2), (3, 1), (4, 0)], 4))
    );

    assert_eq!(
        g.path_between((0, 0), (3, 4)),
        Some((vec![(0, 0), (1, 1), (2, 2), (3, 2), (4, 3), (3, 4)], 5))
    );

    assert_eq!(
        g.path_between((0, 0), (2, 4)),
        Some((vec![(0, 0), (0, 1), (0, 2), (1, 3), (2, 4)], 4))
    );
}

#[test]
fn cursor_should_move_up() {
    let (g, mut gc) = setup::grid::grid_with_direction_from(Direction::Up, (2, 2));

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
fn cursor_should_move_down() {
    let (g, mut gc) = setup::grid::grid_with_direction_from(Direction::Down, (0, 0));

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
fn cursor_should_move_left() {
    let (g, mut gc) = setup::grid::grid_with_direction_from(Direction::Left, (2, 2));

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
fn cursor_should_move_right() {
    let (g, mut gc) = setup::grid::grid_with_direction_from(Direction::Right, (0, 0));

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
fn cursor_should_process_effects() {
    //TODO - implement
}

#[test]
fn cursor_should_process_enqueued_actions() {
    //TODO - implement
}

#[test]
fn cursor_should_process_resource_production() {
    //TODO - implement
}

#[test]
fn cursor_should_process_walker_production() {
    //TODO - implement
}
