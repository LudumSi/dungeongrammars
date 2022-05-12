
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::str::Lines;

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

//Adds rotations, flips, etc.
fn add_rule_variants(rules_vec: &mut Vec<Rule>, options:RuleOptions){

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
        for i in 0..rules_vec.len(){
            let mut flip_rule = rules_vec[i].clone();
            flip_rule.flip_h();
            flipped_rules.push(flip_rule);
        }
        rules_vec.append(&mut flipped_rules);
    }

    if options.flip_v {
        let mut flipped_rules: Vec<Rule> = Vec::new();
        for i in 0..rules_vec.len(){
            let mut flip_rule = rules_vec[i].clone();
            flip_rule.flip_v();
            flipped_rules.push(flip_rule);
        }
        rules_vec.append(&mut flipped_rules);
    }
}

//Reads in a grid given the dimensions
pub fn read_grid(dimensions: &(usize,usize),lines: &mut Lines)->Option<Grid>{

    let mut counter = 0;
    let mut grid = Grid::new(dimensions.0,dimensions.1,Tile::Unknown);

    while counter < dimensions.0 {

        match lines.next(){
            Some(x) => {
                let chars_vec = x.chars().collect::<Vec<char>>();
                for j in 0..chars_vec.len(){
                    if j == dimensions.1{
                        println!("Warning: Line has too many columns");
                        break;
                    }

                    match chars_vec[j]{
                        //Indicates we were not given enough rows
                        '='|'\\' => return None,
                        //Literally anything else
                        _ => {
                            grid.array[counter][j] = import_tile(chars_vec[j]);
                        },
                    }
                }
            }

            //Indicates the grid has too many rows
            None => return None,
        }

        counter += 1;
    }

    return Some(grid)
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

    //The vector we will need to put the grids into
    let mut grids_vec: Vec<Grid> = Vec::new();

    let options: RuleOptions;
    //If there is an equals sign, we know there are no options
    if second_line.chars().next() != Some('='){

        //Handle the options
        options = match read_options(second_line){
            None => RuleOptions::new(),
            Some(x) => x,
        };

    }else{
        options = RuleOptions::new();

        //Handle the first grid, as we have consumed the equals
        match read_grid(&dimensions,&mut lines){
            None => (),
            Some(x) => grids_vec.push(x),
        }
    }

    //Handle the rest of the lines
    //These will either be lines full of data for grids or spacer lines with weights

    let mut getting_grids = true;
    while getting_grids{
        //Check the next line, ensuring there is an equals sign
        match lines.next(){
            None => {
                if grids_vec.len() == 0 {
                    println!("Missing content in {}",display);
                    return None
                }else{
                    getting_grids = false;
                }
            }
            //Ensure we have our spacer
            Some(x) => {
                //println!("{}",x);
                //This is where I will want to check for weights later
                match x.chars().next(){
                    Some('=') => {
                        match read_grid(&dimensions,&mut lines){
                            None => (),
                            Some(x) => grids_vec.push(x),
                        }
                    }
                    Some(_) => {
                        println!("Missing spacer in {}",display);
                        return None
                    }
                    None => (),
                }
            }
        }
    }

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
        println!("Base rule does not have any results in {}: {}",display,grids_vec.len());
        return None;
    }

    for i in 1..grids_vec.len(){
        base_rule.results.push(grids_vec[i].clone());
    }

    rules_vec.push(base_rule);

    //Adds rotations, mirrors, etc.
    add_rule_variants(&mut rules_vec,options);

    Some(rules_vec)
}

pub fn import_base_grid(str: &str)->Option<Grid>{

    let path = Path::new(str);
    let display = path.display();

    //Get the data
    let s = match string_from_file(&path){
        None => return None,
        Some(x) => x,
    };

    //Parse the string into lines
    let mut lines = s.lines();

    //Handle the first line, which should have the dimensions
    let dimensions = match read_dimensions(lines.next()){
        None => {
            println!("No dimensions in {}",display);
            return None
        }
        Some(x) => x,
    };

    //On the assumtion that there is a equation mark, consume it
    //Not strictly necessary in this case, but useful to reinforce it in the users minds
    match lines.next(){
        None => {
            println!("Missing content in {}",display);
            return None
        }
        Some(str) => {
            match str.chars().next(){
                Some('=') => (),
                _ => {
                    println!("Missing spacer in {}",display);
                    return None
                }
            }
        }
    }

    //From there, just get the grid
    return read_grid(&dimensions,&mut lines);
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
