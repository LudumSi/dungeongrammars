
mod tile;
use crate::tile::*;

mod rule;

mod grid;
use crate::grid::*;

mod generation;

mod input;
use crate::input::*;


fn main() {

    let mut test_grid = Grid::new(50,3,Tile::Blank);
    test_grid.init_tjuction();

    let rules_vec = match import_rules_folder(){
        None => panic!("Failed to get input!"),
        Some(x) => x,
    };

    for _i in 0..150{
        test_grid.apply_random_rule(&rules_vec);
    }

    test_grid.print();
}
