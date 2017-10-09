#[macro_use(s)]
extern crate ndarray;

mod grid;
mod entities;
use entities::doodad;
use entities::Entity;

fn main() {
    let mut g = grid::Grid::new(3);

    let d0 = doodad::Doodad { name: "d0".to_owned(), is_removable: false };
    let d1 = doodad::Doodad { name: "d1".to_owned(), is_removable: false };
    let d2 = doodad::Doodad { name: "d2".to_owned(), is_removable: false };
    let d3 = doodad::Doodad { name: "d3".to_owned(), is_removable: false };
    let d4 = doodad::Doodad { name: "d4".to_owned(), is_removable: false };
    let d5 = doodad::Doodad { name: "d5".to_owned(), is_removable: false };
    let d6 = doodad::Doodad { name: "d6".to_owned(), is_removable: false };
    let d7 = doodad::Doodad { name: "d7".to_owned(), is_removable: false };
    let d8 = doodad::Doodad { name: "d8".to_owned(), is_removable: false };

    g.add((0, 0), Entity::Doodad { data: d0 });
    g.add((1, 0), Entity::Doodad { data: d1 });
    g.add((2, 0), Entity::Doodad { data: d2 });
    g.add((0, 1), Entity::Doodad { data: d3 });
    g.add((1, 1), Entity::Doodad { data: d4 });
    g.add((2, 1), Entity::Doodad { data: d5 });
    g.add((0, 2), Entity::Doodad { data: d6 });
    g.add((1, 2), Entity::Doodad { data: d7 });
    g.add((2, 2), Entity::Doodad { data: d8 });

    let mut gc = grid::GridCursor::new(1, grid::Direction::Right, (0, 0));

    for _ in 0..9 {
        print!("({:?}, {:?}) => ", gc.get_position().0, gc.get_position().1);
        gc.process_and_advance(&g);
        println!("({:?}, {:?})", gc.get_position().0, gc.get_position().1);
    }
}
