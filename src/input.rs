
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Read;

use crate::grid::*;
use crate::rule::*;
use crate::tile::*;

pub fn string_from_file(path: &Path)->Option<String>{
    //Get the rule file
    let display = path.display();

    let mut file = match File::open(&path){
        Err(why) => {
            println!("Couldn't open {}: {}", display, why);
            return None;
        }
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            println!("Couldn't read {}: {}", display, why);
            return None;
        }
        Ok(_) => (),
    }

    Some(s)
}

pub fn import_rule_file(path: &Path)->Option<Vec<Rule>>{

    //Get the string
    let s = match string_from_file(&path){
        None => return None,
        Some(x) => x,
    };

    let display = path.display();

    //Parse the string

    let lines = s.lines();
    let lines_vec = lines.collect::<Vec<&str>>();

    //Handle the first line
    if lines_vec.len() < 1 {
        println!("Nothing in {}!",display);
        return None;
    }
    let options = lines_vec[0].split(',');
    let options_vec = options.collect::<Vec<&str>>();

    //First two options are always columns and rows
    //Check for not enough options
    if options_vec.len() < 2 {
        println!("No coordinates in {}!",display);
        return None;
    }

    let rule_columns = options_vec[0].parse::<usize>().unwrap();
    let rule_rows = options_vec[1].parse::<usize>().unwrap();
    //println!("Columns and rows: {} {}",rule_columns,rule_rows);

    //Controls whether or not we flip and rotate
    let mut rotate = false;
    let mut fliphorizontal = false;
    let mut flipvertical = false;

    //Handle the rest of the options
    for i in 2..options_vec.len(){
        for char in options_vec[i].chars(){
            match char{
                'R' => {rotate = true;},
                'H' => {fliphorizontal = true;},
                'V' => {flipvertical = true;},
                _ => (),
            }
        }
    }

    //Handle the rest of the lines
    //These will either be lines full of data for grids or spacer lines
    let mut grids_vec: Vec<Grid> = Vec::new();

    let mut current_row = 0;
    let mut reset_grid = false;

    let mut current_grid = Grid::new(rule_rows,rule_columns,Tile::Unknown);

    for i in 1..lines_vec.len(){

        let chars_vec = lines_vec[i].chars().collect::<Vec<char>>();
        for j in 0..chars_vec.len(){
            if j == rule_columns{
                println!("Warning: Line {} in {} has too many columns",i,display);
                break;
            }

            match chars_vec[j]{
                '=' => {
                    reset_grid = true;
                    //println!("Resetting grid!");
                    break;
                },
                _ => {
                    if current_row == rule_rows{
                        println!("Grid with too many rows in {} at line {}", display,i);
                        return None;
                    }
                    current_grid.array[current_row][j] = import_tile(chars_vec[j]);
                },
            }
        }

        //Resets for the next grid
        if reset_grid{
            if current_row < rule_rows-1{
                println!("Grid with too few rows in {} at line {}", display,i);
                return None;
            }
            current_row = 0;
            reset_grid = false;
            grids_vec.push(current_grid);
            current_grid = Grid::new(rule_rows,rule_columns,Tile::Unknown);
        }else{
            current_row += 1;
        }
    }

    grids_vec.push(current_grid);

    //Package everything up into a vector of rules
    let mut rules_vec: Vec<Rule> = Vec::new();

    let mut base_rule = Rule{
        pattern: grids_vec[0].clone(),
        results: Vec::new(),
        rows: rule_rows,
        columns: rule_columns,
    };

    //Just in case
    if grids_vec.len() < 2 {
        println!("Base rule does not have any results in {}",display);
        return None;
    }

    for i in 1..grids_vec.len(){
        base_rule.results.push(grids_vec[i].clone());
    }

    rules_vec.push(base_rule);

    //Handle rotation and flip rules as needed
    if rotate {
        for i in 1..=3{
            let mut rot_rule = rules_vec[0].clone();
            for _j in 0..i{
                //println!("Being called {} of {}",j,i);
                rot_rule.rotate_cw();
            }
            rules_vec.push(rot_rule);
        }
    }

    if fliphorizontal {
        let mut flipped_rules: Vec<Rule> = Vec::new();
        for rule in &rules_vec{
            let mut flip_rule = rule.clone();
            flip_rule.flip_h();
            flipped_rules.push(flip_rule);
        }
        rules_vec.append(&mut flipped_rules);
    }

    if flipvertical {
        let mut flipped_rules: Vec<Rule> = Vec::new();
        for rule in &rules_vec{
            let mut flip_rule = rule.clone();
            flip_rule.flip_v();
            flipped_rules.push(flip_rule);
        }
        rules_vec.append(&mut flipped_rules);
    }

    Some(rules_vec)
}

pub fn import_rules_folder()->Option<Vec<Rule>>{

    let paths = fs::read_dir("./rules/").unwrap();
    let mut rules: Vec<Rule> = Vec::new();

    for path in paths{
        println!("Loading {}",path.as_ref().unwrap().path().display());
        let mut new_rules: Vec<Rule> = match import_rule_file(&path.unwrap().path()){
            None => {break;},
            Some(x) => x,
        };
        rules.append(&mut new_rules);
    }

    if rules.len() == 0{
        return None
    }

    return Some(rules)

}
