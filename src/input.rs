
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

//Read the dimensions from the given line
fn read_dimensions(in_line: Option<&str>) -> Option<(usize,usize)>{

    //Handle case where there is no line
    let line: &str;
    match in_line{
        None => return None,
        Some(x) => line = x,
    }

    let dimensions = line.split(',');
    let dimensions_vec = dimensions.collect::<Vec<&str>>();

    //Check for lack of dimensions
    if dimensions_vec.len() < 2 {
        return None;
    }

    //Could use some further error handling here
    let columns = dimensions_vec[0].parse::<usize>().unwrap();
    let rows = dimensions_vec[1].parse::<usize>().unwrap();

    return Some((columns,rows));
}

//For storing info on the rule options
//Research better data structure
struct RuleOptions{
    rotation: bool,
    flip_h: bool,
    flip_v: bool,
}

impl RuleOptions{
    fn new() -> RuleOptions{
        RuleOptions{rotation:false,flip_h:false,flip_v:false}
    }
}

//Read option flags from the given line
fn read_options(line: &str) -> Option<RuleOptions>{
    let mut new_options = RuleOptions::new();

    let options = line.split(',');
    let options_vec = options.collect::<Vec<&str>>();

    //Handle the rest of the options
    for option in options_vec{
        for char in option.chars(){
            match char{
                'R' => {new_options.rotation = true;},
                'H' => {new_options.flip_h = true;},
                'V' => {new_options.flip_v = true;},
                _ => (),
            }
        }
    }

    return Some(new_options);
}

pub fn import_rule_file(path: &Path)->Option<Vec<Rule>>{

    //Get the string
    let s = match string_from_file(&path){
        None => return None,
        Some(x) => x,
    };

    let display = path.display();

    //Parse the string
    
    let mut lines = s.lines();
    //let lines_vec = lines.collect::<Vec<&str>>();

    //Handle the first line, which should have the dimensions
    let dimensions = match read_dimensions(lines.next()){
        None => {
            println!("No dimensions in {}",display);
            return None
        }
        Some(x) => x,
    };

    //Handle the second line, which should either have the options or an equals sign
    let second_line = match lines.next(){
        None => {
            println!("Missing content in {}",display);
            return None
        },
        Some(x) => x,
    };

    let options: RuleOptions;
    //If there is an equals sign, we know there are no options
    if second_line.clone().chars().next() != Some('='){

        //Handle the options
        options = match read_options(second_line){
            None => RuleOptions::new(),
            Some(x) => x,
        };

        //Consume the next line, ensuring there is an equals sign
        match lines.next(){
            None => {
                println!("Missing content in {}",display);
                return None
            }
            Some(x) => {
                if x.chars().next() != Some('='){
                    println!("Missing target in {}",display);
                    return None
                }
            }
        }
    }else{
        options = RuleOptions::new();
    }

    //Handle the rest of it (unrefactored)
    let lines_vec = lines.collect::<Vec<&str>>();

    //Handle the rest of the lines
    //These will either be lines full of data for grids or spacer lines
    let mut grids_vec: Vec<Grid> = Vec::new();

    let mut current_row = 0;
    let mut reset_grid = false;

    let mut current_grid = Grid::new(dimensions.0,dimensions.1,Tile::Unknown);

    for i in 0..lines_vec.len(){

        let chars_vec = lines_vec[i].chars().collect::<Vec<char>>();
        for j in 0..chars_vec.len(){
            if j == dimensions.1{
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
                    if current_row == dimensions.0{
                        println!("Grid with too many rows in {} at line {}", display,i);
                        return None;
                    }
                    current_grid.array[current_row][j] = import_tile(chars_vec[j]);
                },
            }
        }

        //Resets for the next grid
        if reset_grid{
            if current_row < (dimensions.0)-1{
                println!("Grid with too few rows in {} at line {}", display,i);
                return None;
            }
            current_row = 0;
            reset_grid = false;
            grids_vec.push(current_grid);
            current_grid = Grid::new(dimensions.0,dimensions.1,Tile::Unknown);
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
        rows: dimensions.0,
        columns: dimensions.1,
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
    if options.rotation {
        for i in 1..=3{
            let mut rot_rule = rules_vec[0].clone();
            for _j in 0..i{
                //println!("Being called {} of {}",j,i);
                rot_rule.rotate_cw();
            }
            rules_vec.push(rot_rule);
        }
    }

    if options.flip_h {
        let mut flipped_rules: Vec<Rule> = Vec::new();
        for rule in &rules_vec{
            let mut flip_rule = rule.clone();
            flip_rule.flip_h();
            flipped_rules.push(flip_rule);
        }
        rules_vec.append(&mut flipped_rules);
    }

    if options.flip_v {
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
            None => Vec::new(),
            Some(x) => x,
        };
        rules.append(&mut new_rules);
    }

    if rules.len() == 0{
        return None
    }

    return Some(rules)
}
