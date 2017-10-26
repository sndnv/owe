extern crate owe;
extern crate uuid;

mod setup;
mod utils;

use uuid::Uuid;
use std::collections::HashMap;
use owe::entities::Entity;
use owe::entities::{doodad, structure};
use owe::grid::{Direction, CellState, GridError};
use utils::extract;

fn sort_cells(cells: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut result = cells.clone();
    result.sort();
    result
}

#[test]
fn grid_should_add_entities_to_cell() {
    let mut g = setup::grid::grid_empty();

    let d0 = doodad::Doodad { name: "d0".to_owned(), is_removable: false };
    let d1 = doodad::Doodad { name: "d1".to_owned(), is_removable: false };
    let d2 = doodad::Doodad { name: "d2".to_owned(), is_removable: false };

    assert_eq!(g.add_entity((0, 0), Entity::Doodad { props: d0 }), Ok(CellState::Empty));
    assert_eq!(g.add_entity((1, 1), Entity::Doodad { props: d1 }), Ok(CellState::Empty));
    assert_eq!(g.add_entity((2, 2), Entity::Doodad { props: d2 }), Ok(CellState::Empty));

    assert_eq!(g.cell_state((0, 0)), CellState::Occupied);
    assert_eq!(g.cell_state((1, 0)), CellState::Empty);
    assert_eq!(g.cell_state((2, 0)), CellState::Empty);

    assert_eq!(g.cell_state((0, 1)), CellState::Empty);
    assert_eq!(g.cell_state((1, 1)), CellState::Occupied);
    assert_eq!(g.cell_state((2, 1)), CellState::Empty);

    assert_eq!(g.cell_state((0, 2)), CellState::Empty);
    assert_eq!(g.cell_state((1, 2)), CellState::Empty);
    assert_eq!(g.cell_state((2, 2)), CellState::Occupied);
}

#[test]
fn grid_should_remove_entities_from_cell() {
    let mut g = setup::grid::grid_default();

    assert_eq!(g.cell_state((0, 0)), CellState::Occupied);
    assert_eq!(g.cell_state((1, 0)), CellState::Occupied);
    assert_eq!(g.cell_state((2, 0)), CellState::Occupied);

    assert_eq!(g.cell_state((0, 1)), CellState::Occupied);
    assert_eq!(g.cell_state((1, 1)), CellState::Empty);
    assert_eq!(g.cell_state((2, 1)), CellState::Occupied);

    assert_eq!(g.cell_state((0, 2)), CellState::Occupied);
    assert_eq!(g.cell_state((1, 2)), CellState::Occupied);
    assert_eq!(g.cell_state((2, 2)), CellState::Occupied);

    assert_eq!(g.remove_entity((0, 1)), Ok(CellState::Occupied));
    assert_eq!(g.remove_entity((1, 2)), Ok(CellState::Occupied));
    assert_eq!(g.remove_entity((2, 1)), Ok(CellState::Occupied));
    assert_eq!(g.remove_entity((1, 0)), Ok(CellState::Occupied));

    assert_eq!(g.cell_state((0, 0)), CellState::Occupied);
    assert_eq!(g.cell_state((1, 0)), CellState::Empty);
    assert_eq!(g.cell_state((2, 0)), CellState::Occupied);

    assert_eq!(g.cell_state((0, 1)), CellState::Empty);
    assert_eq!(g.cell_state((1, 1)), CellState::Empty);
    assert_eq!(g.cell_state((2, 1)), CellState::Empty);

    assert_eq!(g.cell_state((0, 2)), CellState::Occupied);
    assert_eq!(g.cell_state((1, 2)), CellState::Empty);
    assert_eq!(g.cell_state((2, 2)), CellState::Occupied);
}

#[test]
fn grid_should_not_add_overlapping_entities() {
    let mut g = setup::grid::grid_empty();

    let s0 = structure::StructureProperties {
        name: "s0".to_owned(),
        size: structure::Size { width: 2, height: 3 },
        max_employees: 5,
        cost: 1000,
        desirability: (0, 0, 0, 0, 0, 0),
        structure_type: structure::Type::Housing
    };

    let s1 = structure::StructureProperties {
        name: "s1".to_owned(),
        size: structure::Size { width: 2, height: 2 },
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

    assert_eq!(
        g.add_entity((0, 0), Entity::Structure { id: Uuid::new_v4(), props: s0, state: s0_state, producer: None }),
        Ok(CellState::Empty)
    );

    assert_eq!(g.cell_state((0, 0)), CellState::Occupied);
    assert_eq!(g.cell_state((1, 0)), CellState::Occupied);
    assert_eq!(g.cell_state((2, 0)), CellState::Empty);

    assert_eq!(g.cell_state((0, 1)), CellState::Occupied);
    assert_eq!(g.cell_state((1, 1)), CellState::Occupied);
    assert_eq!(g.cell_state((2, 1)), CellState::Empty);

    assert_eq!(g.cell_state((0, 2)), CellState::Occupied);
    assert_eq!(g.cell_state((1, 2)), CellState::Occupied);
    assert_eq!(g.cell_state((2, 2)), CellState::Empty);

    assert_eq!(
        g.add_entity((1, 1), Entity::Structure { id: Uuid::new_v4(), props: s1, state: s1_state, producer: None }),
        Err(GridError::CellUnavailable)
    );
}

#[test]
fn grid_should_remove_entities_from_all_cells_they_use() {
    let mut g = setup::grid::grid_empty();

    let s0 = structure::StructureProperties {
        name: "s0".to_owned(),
        size: structure::Size { width: 2, height: 3 },
        max_employees: 5,
        cost: 1000,
        desirability: (0, 0, 0, 0, 0, 0),
        structure_type: structure::Type::Housing
    };

    let s1 = structure::StructureProperties {
        name: "s1".to_owned(),
        size: structure::Size { width: 2, height: 2 },
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

    assert_eq!(
        g.add_entity((0, 0), Entity::Structure { id: Uuid::new_v4(), props: s0, state: s0_state, producer: None }),
        Ok(CellState::Empty)
    );

    assert_eq!(g.cell_state((0, 0)), CellState::Occupied);
    assert_eq!(g.cell_state((1, 0)), CellState::Occupied);
    assert_eq!(g.cell_state((2, 0)), CellState::Empty);

    assert_eq!(g.cell_state((0, 1)), CellState::Occupied);
    assert_eq!(g.cell_state((1, 1)), CellState::Occupied);
    assert_eq!(g.cell_state((2, 1)), CellState::Empty);

    assert_eq!(g.cell_state((0, 2)), CellState::Occupied);
    assert_eq!(g.cell_state((1, 2)), CellState::Occupied);
    assert_eq!(g.cell_state((2, 2)), CellState::Empty);

    assert_eq!(
        g.add_entity((1, 1), Entity::Structure { id: Uuid::new_v4(), props: s1, state: s1_state, producer: None }),
        Err(GridError::CellUnavailable)
    );

    assert_eq!(g.remove_entity((0, 1)), Ok(CellState::Occupied));

    assert_eq!(g.cell_state((0, 0)), CellState::Empty);
    assert_eq!(g.cell_state((1, 0)), CellState::Empty);
    assert_eq!(g.cell_state((2, 0)), CellState::Empty);

    assert_eq!(g.cell_state((0, 1)), CellState::Empty);
    assert_eq!(g.cell_state((1, 1)), CellState::Empty);
    assert_eq!(g.cell_state((2, 1)), CellState::Empty);

    assert_eq!(g.cell_state((0, 2)), CellState::Empty);
    assert_eq!(g.cell_state((1, 2)), CellState::Empty);
    assert_eq!(g.cell_state((2, 2)), CellState::Empty);

    let s1_new = structure::StructureProperties {
        name: "s1".to_owned(),
        size: structure::Size { width: 2, height: 2 },
        max_employees: 2,
        cost: 5000,
        desirability: (1, 2, 3, 4, 5, 6),
        structure_type: structure::Type::Industry
    };

    let s1_new_state = structure::StructureState {
        current_employees: 1,
        commodities: HashMap::new(),
        risk: structure::Risk { damage: 10, fire: 3 }
    };

    assert_eq!(
        g.add_entity((1, 1), Entity::Structure { id: Uuid::new_v4(), props: s1_new, state: s1_new_state, producer: None }),
        Ok(CellState::Empty)
    );

    assert_eq!(g.cell_state((0, 0)), CellState::Empty);
    assert_eq!(g.cell_state((1, 0)), CellState::Empty);
    assert_eq!(g.cell_state((2, 0)), CellState::Empty);

    assert_eq!(g.cell_state((0, 1)), CellState::Empty);
    assert_eq!(g.cell_state((1, 1)), CellState::Occupied);
    assert_eq!(g.cell_state((2, 1)), CellState::Occupied);

    assert_eq!(g.cell_state((0, 2)), CellState::Empty);
    assert_eq!(g.cell_state((1, 2)), CellState::Occupied);
    assert_eq!(g.cell_state((2, 2)), CellState::Occupied);
}

#[test]
fn grid_should_not_remove_entities_not_in_cell() {
    let mut g = setup::grid::grid_empty();

    assert_eq!(g.cell_state((0, 0)), CellState::Empty);
    assert_eq!(g.cell_state((1, 0)), CellState::Empty);
    assert_eq!(g.cell_state((2, 0)), CellState::Empty);

    assert_eq!(g.cell_state((0, 1)), CellState::Empty);
    assert_eq!(g.cell_state((1, 1)), CellState::Empty);
    assert_eq!(g.cell_state((2, 1)), CellState::Empty);

    assert_eq!(g.cell_state((0, 2)), CellState::Empty);
    assert_eq!(g.cell_state((1, 2)), CellState::Empty);
    assert_eq!(g.cell_state((2, 2)), CellState::Empty);

    assert_eq!(g.remove_entity((0, 1)), Err(GridError::CellUnavailable));
    assert_eq!(g.remove_entity((1, 2)), Err(GridError::CellUnavailable));
    assert_eq!(g.remove_entity((2, 1)), Err(GridError::CellUnavailable));
    assert_eq!(g.remove_entity((1, 0)), Err(GridError::CellUnavailable));
}

#[test]
fn grid_should_not_add_entities_outside_of_bounds() {
    let mut g = setup::grid::grid_default();

    let d0 = doodad::Doodad { name: "d0".to_owned(), is_removable: false };

    assert_eq!(g.add_entity((12, 37), Entity::Doodad { props: d0 }), Err(GridError::CellUnavailable));
}

#[test]
fn grid_should_not_remove_entities_outside_of_bounds() {
    let mut g = setup::grid::grid_default();

    assert_eq!(g.remove_entity((12, 37)), Err(GridError::CellUnavailable));
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

    assert_eq!(g.remove_entity((2, 2)), Ok(CellState::Occupied));

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
fn grid_should_add_effects_to_cell() {
    let (mut g, _, _, effects) = setup::grid::grid_with_effects();

    assert_eq!(g.add_cell_effect((0, 0), effects[0].clone()), Ok(CellState::Occupied));
    assert_eq!(g.add_cell_effect((0, 0), effects[1].clone()), Ok(CellState::Occupied));
    assert_eq!(g.add_cell_effect((2, 0), effects[2].clone()), Ok(CellState::Occupied));
    assert_eq!(g.add_cell_effect((2, 1), effects[2].clone()), Ok(CellState::Occupied));
    assert_eq!(g.add_cell_effect((2, 2), effects[2].clone()), Ok(CellState::Occupied));

    assert_eq!(g.is_effect_in_cell((0, 0), &effects[0]), true);
    assert_eq!(g.is_effect_in_cell((0, 0), &effects[1]), true);
    assert_eq!(g.is_effect_in_cell((2, 0), &effects[2]), true);
    assert_eq!(g.is_effect_in_cell((2, 1), &effects[2]), true);
    assert_eq!(g.is_effect_in_cell((2, 2), &effects[2]), true);
}

#[test]
fn grid_should_remove_effects_from_cell() {
    let (mut g, _, _, effects) = setup::grid::grid_with_effects();

    assert_eq!(g.add_cell_effect((0, 0), effects[0].clone()), Ok(CellState::Occupied));
    assert_eq!(g.add_cell_effect((0, 0), effects[1].clone()), Ok(CellState::Occupied));
    assert_eq!(g.add_cell_effect((2, 0), effects[2].clone()), Ok(CellState::Occupied));
    assert_eq!(g.add_cell_effect((2, 1), effects[2].clone()), Ok(CellState::Occupied));
    assert_eq!(g.add_cell_effect((2, 2), effects[2].clone()), Ok(CellState::Occupied));

    assert_eq!(g.is_effect_in_cell((0, 0), &effects[0]), true);
    assert_eq!(g.is_effect_in_cell((0, 0), &effects[1]), true);
    assert_eq!(g.is_effect_in_cell((2, 0), &effects[2]), true);
    assert_eq!(g.is_effect_in_cell((2, 1), &effects[2]), true);
    assert_eq!(g.is_effect_in_cell((2, 2), &effects[2]), true);

    assert_eq!(g.remove_cell_effect((0, 0), &effects[1]), Ok(CellState::Occupied));
    assert_eq!(g.remove_cell_effect((2, 1), &effects[2]), Ok(CellState::Occupied));

    assert_eq!(g.is_effect_in_cell((0, 0), &effects[0]), true);
    assert_eq!(g.is_effect_in_cell((0, 0), &effects[1]), false);
    assert_eq!(g.is_effect_in_cell((2, 0), &effects[2]), true);
    assert_eq!(g.is_effect_in_cell((2, 1), &effects[2]), false);
    assert_eq!(g.is_effect_in_cell((2, 2), &effects[2]), true);
}

#[test]
fn grid_should_clear_effects_from_cell() {
    let (mut g, _, _, effects) = setup::grid::grid_with_effects();

    assert_eq!(g.add_cell_effect((0, 0), effects[0].clone()), Ok(CellState::Occupied));
    assert_eq!(g.add_cell_effect((0, 0), effects[1].clone()), Ok(CellState::Occupied));
    assert_eq!(g.add_cell_effect((2, 0), effects[2].clone()), Ok(CellState::Occupied));
    assert_eq!(g.add_cell_effect((2, 1), effects[2].clone()), Ok(CellState::Occupied));
    assert_eq!(g.add_cell_effect((2, 2), effects[2].clone()), Ok(CellState::Occupied));

    assert_eq!(g.is_effect_in_cell((0, 0), &effects[0]), true);
    assert_eq!(g.is_effect_in_cell((0, 0), &effects[1]), true);
    assert_eq!(g.is_effect_in_cell((2, 0), &effects[2]), true);
    assert_eq!(g.is_effect_in_cell((2, 1), &effects[2]), true);
    assert_eq!(g.is_effect_in_cell((2, 2), &effects[2]), true);

    assert_eq!(g.clear_cell_effects((0, 0)), Ok(CellState::Occupied));

    assert_eq!(g.is_effect_in_cell((0, 0), &effects[0]), false);
    assert_eq!(g.is_effect_in_cell((0, 0), &effects[1]), false);
    assert_eq!(g.is_effect_in_cell((2, 0), &effects[2]), true);
    assert_eq!(g.is_effect_in_cell((2, 1), &effects[2]), true);
    assert_eq!(g.is_effect_in_cell((2, 2), &effects[2]), true);
}

#[test]
fn grid_should_not_allow_duplicate_effects_for_cell() {
    let (mut g, _, _, effects) = setup::grid::grid_with_effects();

    assert_eq!(g.add_cell_effect((0, 0), effects[0].clone()), Ok(CellState::Occupied));
    assert_eq!(g.add_cell_effect((0, 0), effects[0].clone()), Err(GridError::EffectPresent));
}

#[test]
fn grid_should_not_remove_effects_not_in_cell() {
    let (mut g, _, _, effects) = setup::grid::grid_with_effects();

    assert_eq!(g.remove_cell_effect((0, 0), &effects[0]), Err(GridError::EffectMissing));
}

#[test]
fn grid_should_not_add_effects_outside_of_bounds() {
    let (mut g, _, _, effects) = setup::grid::grid_with_effects();

    assert_eq!(g.add_cell_effect((6, 42), effects[0].clone()), Err(GridError::CellUnavailable));
}

#[test]
fn grid_should_not_remove_effects_outside_of_bounds() {
    let (mut g, _, _, effects) = setup::grid::grid_with_effects();

    assert_eq!(g.remove_cell_effect((6, 42), &effects[0]), Err(GridError::CellUnavailable));
}

#[test]
fn grid_should_add_global_effects() {
    let (mut g, _, _, effects) = setup::grid::grid_with_effects();

    assert_eq!(g.add_global_effect(effects[0].clone()), Ok(()));
    assert_eq!(g.add_global_effect(effects[1].clone()), Ok(()));
    assert_eq!(g.add_global_effect(effects[2].clone()), Ok(()));

    assert_eq!(g.is_effect_global(&effects[0]), true);
    assert_eq!(g.is_effect_global(&effects[1]), true);
    assert_eq!(g.is_effect_global(&effects[2]), true);
}

#[test]
fn grid_should_remove_global_effects() {
    let (mut g, _, _, effects) = setup::grid::grid_with_effects();

    assert_eq!(g.add_global_effect(effects[0].clone()), Ok(()));
    assert_eq!(g.add_global_effect(effects[1].clone()), Ok(()));
    assert_eq!(g.add_global_effect(effects[2].clone()), Ok(()));

    assert_eq!(g.is_effect_global(&effects[0]), true);
    assert_eq!(g.is_effect_global(&effects[1]), true);
    assert_eq!(g.is_effect_global(&effects[2]), true);

    assert_eq!(g.remove_global_effect(&effects[1]), Ok(()));

    assert_eq!(g.is_effect_global(&effects[0]), true);
    assert_eq!(g.is_effect_global(&effects[1]), false);
    assert_eq!(g.is_effect_global(&effects[2]), true);
}

#[test]
fn grid_should_clear_global_effects() {
    let (mut g, _, _, effects) = setup::grid::grid_with_effects();

    assert_eq!(g.add_global_effect(effects[0].clone()), Ok(()));
    assert_eq!(g.add_global_effect(effects[1].clone()), Ok(()));
    assert_eq!(g.add_global_effect(effects[2].clone()), Ok(()));

    assert_eq!(g.is_effect_global(&effects[0]), true);
    assert_eq!(g.is_effect_global(&effects[1]), true);
    assert_eq!(g.is_effect_global(&effects[2]), true);

    g.clear_global_effects();

    assert_eq!(g.is_effect_global(&effects[0]), false);
    assert_eq!(g.is_effect_global(&effects[1]), false);
    assert_eq!(g.is_effect_global(&effects[2]), false);
}

#[test]
fn grid_should_not_allow_duplicate_global_effects() {
    let (mut g, _, _, effects) = setup::grid::grid_with_effects();

    assert_eq!(g.add_global_effect(effects[0].clone()), Ok(()));
    assert_eq!(g.add_global_effect(effects[0].clone()), Err(GridError::EffectPresent));
}

#[test]
fn grid_should_not_remove_nonexistent_global_effects() {
    let (mut g, _, _, effects) = setup::grid::grid_with_effects();

    assert_eq!(g.remove_global_effect(&effects[1]), Err((GridError::EffectMissing)));
}

#[test]
fn cursor_should_move_up() {
    let (mut g, mut gc, mut e) = setup::grid::grid_with_direction_from(Direction::Up, (2, 2));

    assert_eq!(gc.position(), (2, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 2));
}

#[test]
fn cursor_should_move_down() {
    let (mut g, mut gc, mut e) = setup::grid::grid_with_direction_from(Direction::Down, (0, 0));

    assert_eq!(gc.position(), (0, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 0));
}

#[test]
fn cursor_should_move_left() {
    let (mut g, mut gc, mut e) = setup::grid::grid_with_direction_from(Direction::Left, (2, 2));

    assert_eq!(gc.position(), (2, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 2));
}

#[test]
fn cursor_should_move_right() {
    let (mut g, mut gc, mut e) = setup::grid::grid_with_direction_from(Direction::Right, (0, 0));

    assert_eq!(gc.position(), (0, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 0));
}

#[test]
fn cursor_should_process_effects() {
    let (mut g, mut gc, mut e, effects) = setup::grid::grid_with_effects();

    assert_eq!(g.add_global_effect(effects[1].clone()), Ok(()));

    assert_eq!(g.add_cell_effect((0, 0), effects[0].clone()), Ok(CellState::Occupied));
    assert_eq!(g.add_cell_effect((2, 0), effects[2].clone()), Ok(CellState::Occupied));
    assert_eq!(g.add_cell_effect((2, 1), effects[2].clone()), Ok(CellState::Occupied));
    assert_eq!(g.add_cell_effect((2, 2), effects[2].clone()), Ok(CellState::Occupied));

    assert_eq!(gc.position(), (0, 0));
    assert_eq!(extract::doodad::name(g.entity((0, 0))), Some("d0".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0))), Some("d1".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1))), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2))), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1))), Some(structure::Risk { damage: 0, fire: 0 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2))), Some(structure::Risk { damage: 10, fire: 3 }));
    assert_eq!(extract::resource::level(g.entity((2, 0))), Some(2));
    assert_eq!(extract::resource::level(g.entity((0, 1))), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2))), Some(None));
    assert_eq!(extract::walker::life(g.entity((2, 2))), Some(Some(1)));

    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(())); //process (0, 0) and go to (1, 0)

    assert_eq!(gc.position(), (1, 0));
    assert_eq!(extract::doodad::name(g.entity((0, 0))), Some("updated doodad name".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0))), Some("updated doodad name".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1))), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2))), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1))), Some(structure::Risk { damage: 0, fire: 0 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2))), Some(structure::Risk { damage: 10, fire: 3 }));
    assert_eq!(extract::resource::level(g.entity((2, 0))), Some(2));
    assert_eq!(extract::resource::level(g.entity((0, 1))), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2))), Some(None));
    assert_eq!(extract::walker::life(g.entity((2, 2))), Some(Some(1)));

    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(())); //process (1, 0) and go to (2, 0)

    assert_eq!(gc.position(), (2, 0));
    assert_eq!(extract::doodad::name(g.entity((0, 0))), Some("updated doodad name".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0))), Some("updated doodad name".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1))), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2))), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1))), Some(structure::Risk { damage: 0, fire: 0 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2))), Some(structure::Risk { damage: 10, fire: 3 }));
    assert_eq!(extract::resource::level(g.entity((2, 0))), Some(2));
    assert_eq!(extract::resource::level(g.entity((0, 1))), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2))), Some(None));
    assert_eq!(extract::walker::life(g.entity((2, 2))), Some(Some(1)));

    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(())); //process (2, 0) and go to (0, 1)

    assert_eq!(gc.position(), (0, 1));
    assert_eq!(extract::doodad::name(g.entity((0, 0))), Some("updated doodad name".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0))), Some("updated doodad name".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1))), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2))), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1))), Some(structure::Risk { damage: 0, fire: 0 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2))), Some(structure::Risk { damage: 10, fire: 3 }));
    assert_eq!(extract::resource::level(g.entity((2, 0))), Some(1));
    assert_eq!(extract::resource::level(g.entity((0, 1))), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2))), Some(None));
    assert_eq!(extract::walker::life(g.entity((2, 2))), Some(Some(1)));

    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(())); //process (0, 1) and go to (1, 1)

    assert_eq!(gc.position(), (1, 1));
    assert_eq!(extract::doodad::name(g.entity((0, 0))), Some("updated doodad name".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0))), Some("updated doodad name".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1))), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2))), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1))), Some(structure::Risk { damage: 0, fire: 0 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2))), Some(structure::Risk { damage: 10, fire: 3 }));
    assert_eq!(extract::resource::level(g.entity((2, 0))), Some(1));
    assert_eq!(extract::resource::level(g.entity((0, 1))), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2))), Some(None));
    assert_eq!(extract::walker::life(g.entity((2, 2))), Some(Some(1)));

    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(())); //process (1, 1) and go to (2, 1)

    assert_eq!(gc.position(), (2, 1));
    assert_eq!(extract::doodad::name(g.entity((0, 0))), Some("updated doodad name".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0))), Some("updated doodad name".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1))), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2))), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1))), Some(structure::Risk { damage: 0, fire: 0 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2))), Some(structure::Risk { damage: 10, fire: 3 }));
    assert_eq!(extract::resource::level(g.entity((2, 0))), Some(1));
    assert_eq!(extract::resource::level(g.entity((0, 1))), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2))), Some(None));
    assert_eq!(extract::walker::life(g.entity((2, 2))), Some(Some(1)));

    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(())); //process (2, 1) and go to (0, 2)

    assert_eq!(gc.position(), (0, 2));
    assert_eq!(extract::doodad::name(g.entity((0, 0))), Some("updated doodad name".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0))), Some("updated doodad name".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1))), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2))), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1))), Some(structure::Risk { damage: 0, fire: 0 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2))), Some(structure::Risk { damage: 10, fire: 3 }));
    assert_eq!(extract::resource::level(g.entity((2, 0))), Some(1));
    assert_eq!(extract::resource::level(g.entity((0, 1))), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2))), Some(Some(3)));
    assert_eq!(extract::walker::life(g.entity((2, 2))), Some(Some(0)));

    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(())); //process (0, 2) and go to (1, 2)

    assert_eq!(gc.position(), (1, 2));
    assert_eq!(extract::doodad::name(g.entity((0, 0))), Some("updated doodad name".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0))), Some("updated doodad name".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1))), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2))), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1))), Some(structure::Risk { damage: 0, fire: 0 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2))), Some(structure::Risk { damage: 10, fire: 3 }));
    assert_eq!(extract::resource::level(g.entity((2, 0))), Some(1));
    assert_eq!(extract::resource::level(g.entity((0, 1))), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2))), Some(Some(3)));
    assert_eq!(extract::walker::life(g.entity((2, 2))), Some(Some(0)));

    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(())); //process (1, 2) and go to (2, 2)

    assert_eq!(gc.position(), (2, 2));
    assert_eq!(extract::doodad::name(g.entity((0, 0))), Some("updated doodad name".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0))), Some("updated doodad name".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1))), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2))), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1))), Some(structure::Risk { damage: 0, fire: 0 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2))), Some(structure::Risk { damage: 10, fire: 3 }));
    assert_eq!(extract::resource::level(g.entity((2, 0))), Some(1));
    assert_eq!(extract::resource::level(g.entity((0, 1))), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2))), Some(Some(3)));
    assert_eq!(extract::walker::life(g.entity((2, 2))), Some(Some(0)));

    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(())); //process (2, 2) and go to (0, 0)

    assert_eq!(gc.position(), (0, 0));
    assert_eq!(extract::doodad::name(g.entity((0, 0))), Some("updated doodad name".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0))), Some("updated doodad name".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1))), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2))), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1))), Some(structure::Risk { damage: 1, fire: 5 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2))), Some(structure::Risk { damage: 10, fire: 8 }));
    assert_eq!(extract::resource::level(g.entity((2, 0))), Some(1));
    assert_eq!(extract::resource::level(g.entity((0, 1))), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2))), Some(Some(2)));
    assert_eq!(extract::walker::life(g.entity((2, 2))), Some(Some(0)));

    for _ in 0..9 {
        assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    }

    assert_eq!(gc.position(), (0, 0));
    assert_eq!(extract::doodad::name(g.entity((0, 0))), Some("updated doodad name".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0))), Some("updated doodad name".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1))), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2))), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1))), Some(structure::Risk { damage: 2, fire: 10 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2))), Some(structure::Risk { damage: 10, fire: 13 }));
    assert_eq!(extract::resource::level(g.entity((2, 0))), Some(0));
    assert_eq!(extract::resource::level(g.entity((0, 1))), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2))), Some(Some(0)));
    assert_eq!(extract::walker::life(g.entity((2, 2))), Some(Some(0)));
}

#[test]
fn cursor_should_process_enqueued_actions() {
    //TODO - implement
}

#[test]
fn cursor_should_process_resource_production() {
    let (mut g, mut gc, mut e) = setup::grid::grid_with_production();

    assert_eq!(e.producers_of("c0").len(), 1); //at (2, 1)
    assert_eq!(e.producers_of("c1").len(), 1); //at (0, 2)
    assert_eq!(e.producers_of("c2").len(), 1); //at (2, 0)

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 0);
    assert_eq!(e.amount_available_of("c0"), 0);
    assert_eq!(e.amount_available_of("c1"), 0);
    assert_eq!(e.amount_available_of("c2"), 0);
    assert_eq!(e.amount_used_of("c0"), 0);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    assert_eq!(gc.position(), (0, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 0);
    assert_eq!(e.amount_available_of("c0"), 0);
    assert_eq!(e.amount_available_of("c1"), 0);
    assert_eq!(e.amount_available_of("c2"), 0);
    assert_eq!(e.amount_used_of("c0"), 0);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    assert_eq!(gc.position(), (1, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 0);
    assert_eq!(e.amount_available_of("c0"), 0);
    assert_eq!(e.amount_available_of("c1"), 0);
    assert_eq!(e.amount_available_of("c2"), 0);
    assert_eq!(e.amount_used_of("c0"), 0);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    assert_eq!(gc.position(), (2, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 0);
    assert_eq!(e.amount_available_of("c0"), 0);
    assert_eq!(e.amount_available_of("c1"), 0);
    assert_eq!(e.amount_available_of("c2"), 1);
    assert_eq!(e.amount_used_of("c0"), 0);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    assert_eq!(gc.position(), (0, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 0);
    assert_eq!(e.amount_available_of("c0"), 0);
    assert_eq!(e.amount_available_of("c1"), 0);
    assert_eq!(e.amount_available_of("c2"), 1);
    assert_eq!(e.amount_used_of("c0"), 0);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    assert_eq!(gc.position(), (1, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 0);
    assert_eq!(e.amount_available_of("c0"), 0);
    assert_eq!(e.amount_available_of("c1"), 0);
    assert_eq!(e.amount_available_of("c2"), 1);
    assert_eq!(e.amount_used_of("c0"), 0);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    assert_eq!(gc.position(), (2, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 3);
    assert_eq!(e.amount_available_of("c0"), 100);
    assert_eq!(e.amount_available_of("c1"), 0);
    assert_eq!(e.amount_available_of("c2"), 1);
    assert_eq!(e.amount_used_of("c0"), 0);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    assert_eq!(gc.position(), (0, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 3);
    assert_eq!(e.amount_available_of("c0"), 100);
    assert_eq!(e.amount_available_of("c1"), 1);
    assert_eq!(e.amount_available_of("c2"), 1);
    assert_eq!(e.amount_used_of("c0"), 2);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    assert_eq!(gc.position(), (1, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 3);
    assert_eq!(e.amount_available_of("c0"), 100);
    assert_eq!(e.amount_available_of("c1"), 1);
    assert_eq!(e.amount_available_of("c2"), 1);
    assert_eq!(e.amount_used_of("c0"), 2);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    assert_eq!(gc.position(), (2, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 3);
    assert_eq!(e.amount_available_of("c0"), 100);
    assert_eq!(e.amount_available_of("c1"), 1);
    assert_eq!(e.amount_available_of("c2"), 1);
    assert_eq!(e.amount_used_of("c0"), 2);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    assert_eq!(gc.position(), (0, 0));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 3);
    assert_eq!(e.amount_available_of("c0"), 100);
    assert_eq!(e.amount_available_of("c1"), 1);
    assert_eq!(e.amount_available_of("c2"), 1);
    assert_eq!(e.amount_used_of("c0"), 2);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    for _ in 0..9 {
        assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    }

    assert_eq!(gc.position(), (0, 0));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 3); //TODO
    assert_eq!(e.amount_available_of("c0"), 200); //TODO
    assert_eq!(e.amount_available_of("c1"), 2); //TODO
    assert_eq!(e.amount_available_of("c2"), 2); //TODO
    assert_eq!(e.amount_used_of("c0"), 4);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);
}

#[test]
fn cursor_should_process_walker_production() {
    //TODO - implement
}
