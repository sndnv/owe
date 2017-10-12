use owe::grid;
use owe::entities::doodad;
use owe::entities::resource;
use owe::entities::structure;
use owe::entities::walker;
use owe::entities::Entity;
use std::collections::HashMap;
use uuid::Uuid;

pub fn default_grid() -> grid::Grid {
    let mut g = grid::Grid::new(3);

    let d0 = doodad::Doodad { name: "d0".to_owned(), is_removable: false };
    let d1 = doodad::Doodad { name: "d1".to_owned(), is_removable: false };

    let r0 = resource::Resource { level: resource::Level { current: 0, max: 5 }, name: "r0".to_owned(), replenish_time: Some(5) };
    let r1 = resource::Resource { level: resource::Level { current: 3, max: 5 }, name: "r1".to_owned(), replenish_time: None };

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

    g.add((0, 0), Entity::Doodad { data: d0 });
    g.add((1, 0), Entity::Doodad { data: d1 });
    g.add((2, 0), Entity::Resource { data: r0 });
    g.add((0, 1), Entity::Resource { data: r1 });
    g.add((2, 1), Entity::Structure { id: Uuid::new_v4(), data: s0 });
    g.add((0, 2), Entity::Structure { id: Uuid::new_v4(), data: s1 });
    g.add((1, 2), Entity::Walker { id: Uuid::new_v4(), data: w0 });
    g.add((2, 2), Entity::Walker { id: Uuid::new_v4(), data: w1 });

    g
}

pub fn with_direction_from(direction: grid::Direction, from: (usize, usize)) -> (grid::Grid, grid::GridCursor) {
    let g = default_grid();

    let gc = grid::GridCursor::new(1, direction, from);

    (g, gc)
}

pub fn with_effects() -> (grid::Grid, grid::GridCursor) {
    let g = default_grid();

    //TODO - create effects

    let gc = grid::GridCursor::new(1, grid::Direction::Right, (0, 0));

    (g, gc)
}
