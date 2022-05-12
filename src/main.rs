
mod tile;
use crate::tile::*;

mod rule;

mod grid;
use crate::grid::*;

mod generation;

mod input;
use crate::input::*;


fn main() {

    let mut base_grid = match import_base_grid("./base.txt"){
        Some(x) => x,
        None => panic!("Failed to load base grid!"),
    };

    let rules_vec = match import_rules_folder(){
        None => panic!("Failed to get input!"),
        Some(x) => x,
    };

    for _i in 0..150{
        base_grid.apply_random_rule(&rules_vec);
    }

    base_grid.print();
}
