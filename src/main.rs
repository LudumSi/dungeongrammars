extern crate rand;

use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Read;

#[derive(Debug,Clone,Copy,Eq,PartialEq)]
enum DungeonTile{
    Blank,
    CorridorNS,
    CorridorEW,
    CornerNW,
    CornerNE,
    CornerSW,
    CornerSE,
    ThreewayN,
    ThreewayS,
    ThreewayE,
    ThreewayW,
    Fourway,
    Room,
    Unknown,
}

fn rotate_tile_cw(tile:DungeonTile)->DungeonTile{
    match tile{
        DungeonTile::CorridorNS => DungeonTile::CorridorEW,
        DungeonTile::CorridorEW => DungeonTile::CorridorNS,
        DungeonTile::CornerNW => DungeonTile::CornerNE,
        DungeonTile::CornerNE => DungeonTile::CornerSE,
        DungeonTile::CornerSE => DungeonTile::CornerSW,
        DungeonTile::CornerSW => DungeonTile::CornerNW,
        DungeonTile::ThreewayN => DungeonTile::ThreewayE,
        DungeonTile::ThreewayE => DungeonTile::ThreewayS,
        DungeonTile::ThreewayS => DungeonTile::ThreewayW,
        DungeonTile::ThreewayW => DungeonTile::ThreewayN,
        _ => tile,
    }
}

fn print_tile(tile:&DungeonTile)->char{
    match tile{
        DungeonTile::Blank => '.',
        DungeonTile::CorridorNS => '║',
        DungeonTile::CorridorEW => '═',
        DungeonTile::CornerNW => '╝',
        DungeonTile::CornerNE => '╚',
        DungeonTile::CornerSE => '╔',
        DungeonTile::CornerSW => '╗',
        DungeonTile::ThreewayN => '╦',
        DungeonTile::ThreewayE => '╣',
        DungeonTile::ThreewayS => '╩',
        DungeonTile::ThreewayW => '╠',
        DungeonTile::Fourway => '╬',
        DungeonTile::Room => '█',
        DungeonTile::Unknown => '?',
    }
}

fn import_tile(ch:char)->DungeonTile{
    match ch{
        'c' => DungeonTile::CorridorEW,
        'C' => DungeonTile::CorridorNS,
        '7' => DungeonTile::CornerSE,
        '8' => DungeonTile::ThreewayN,
        '9' => DungeonTile::CornerSW,
        '4' => DungeonTile::ThreewayW,
        '5' => DungeonTile::Fourway,
        '6' => DungeonTile::ThreewayE,
        '1' => DungeonTile::CornerNE,
        '2' => DungeonTile::ThreewayS,
        '3' => DungeonTile::CornerNW,
        'r' => DungeonTile::Room,
        '.' => DungeonTile::Blank,
        '?' => DungeonTile::Unknown,
        _ => DungeonTile::Unknown,
    }
}

#[derive(Clone)]
struct DungeonGrid{
    array: Vec<Vec<DungeonTile>>,
    rows: usize,
    columns: usize,
}

impl DungeonGrid{
    fn new(row:usize,column:usize,fill:DungeonTile)->DungeonGrid{
        DungeonGrid{
            rows: row,
            columns: column,
            array: vec![vec![fill;column];row],
        }
    }

    fn print(&self){
        for i in 0..self.rows{
            for j in 0..self.columns{
                print!("{}",print_tile(&self.array[i][j]));
            }
            print!("{}",'\n');
        }
    }

    fn init_tjuction(&mut self){
        let x = rand::random::<usize>() % self.columns;
        let y = rand::random::<usize>() % self.rows;

        //The way across
        for i in 0..self.columns{
            self.array[y][i] = DungeonTile::CorridorEW;
        }

        //From above
        for i in 0..y{
            self.array[i][x] = DungeonTile::CorridorNS;
        }

        //Overwrite the intersection
        self.array[y][x] = DungeonTile::ThreewayS;
    }

    //Works for non-square grids as well as square ones
    fn rotate_cw(&mut self){
        let mut newgrid = vec![vec![DungeonTile::Blank;self.rows];self.columns];
        for i in 0..self.rows{
            for j in 0..self.columns{
                newgrid[j][self.rows-i-1] = rotate_tile_cw(self.array[i][j]);
            }
        }

        self.array = newgrid;

        self.columns = self.array[0].len();
        self.rows = self.array.len();
    }

    //Copies a random result from a rule at the given location
    fn replace_with_rule(&mut self, row: usize, column: usize, rule:&Rule){
        //Choose replacement
        let r = rand::random::<usize>() % rule.results.len();

        for k in 0..rule.rows{
            for l in 0..rule.columns{
                if rule.results[r].array[k][l] != DungeonTile::Unknown {
                    self.array[row+k][column+l] = rule.results[r].array[k][l];
                }
            }
        }
    }

    fn check_location(&mut self, row: usize, column: usize, rule:&Rule)->bool{
        //Safety checks
        if row+rule.rows >= self.rows{
            return false;
        }

        if column+rule.columns >= self.columns {
            return false;
        }

        for k in 0..rule.rows{
            for l in 0..rule.columns{
                if rule.pattern.array[k][l] != DungeonTile::Unknown && self.array[row+k][column+l] != rule.pattern.array[k][l]{
                    return false;
                }
            }
        }

        return true;
    }

    //Tries to apply the rule to the first place it finds a match
    fn apply_rule_lazy(&mut self, rule: &Rule){

        //Iterate over the whole dungeon grid the rule can reach
        for i in 0..(self.rows-rule.rows){
            for j in 0..(self.columns-rule.columns){

                //Scan and replace with a random replacement
                if self.check_location(i,j,rule) {
                    self.replace_with_rule(i,j,&rule);
                    return;
                }
            }
        }
    }

    //Applies the rule at a random location it finds a match
    fn apply_rule(&mut self, rule: &Rule)->bool{

        let mut candidates: Vec<(usize,usize)> = Vec::new();

        //Iterate over the whole dungeon grid the rule can reach
        for i in 0..(self.rows-rule.rows){
            for j in 0..(self.columns-rule.columns){

                if self.check_location(i,j,rule) {
                    candidates.push((i,j));
                }
            }
        }

        //Handle no candidates
        if candidates.len() == 0{
            return false;
        }

        //Pick a random candidate
        let r = rand::random::<usize>() % candidates.len();
        let candidate:(usize,usize) = candidates[r];
        self.replace_with_rule(candidate.0,candidate.1,rule);

        return true;
    }

    //Keep trying to apply a rule until you can't anymore...
    fn apply_random_rule(&mut self, rules:&Vec<Rule>){

        let mut viable: Vec<usize> = (0..rules.len()).collect();

        while viable.len() > 0 {

            let r = rand::random::<usize>() % viable.len();

            if self.apply_rule(&rules[r]){
                return;
            }else{
                viable.remove(r);
            }
        }
    }
}

#[derive(Clone)]
struct Rule{
    pattern: DungeonGrid,
    results: Vec<DungeonGrid>,
    rows: usize,
    columns: usize,
}

impl Rule{
    fn print(&self){
        self.pattern.print();
        println!("\nbecomes\n");
        for i in 0..self.results.len(){
            if i != 0{
                println!("\nor\n")
            }
            self.results[i].print();
        }
    }

    fn rotate_cw(&mut self){
        //println!("Rotate CW being called from the rule!");
        self.pattern.rotate_cw();
        for i in 0..self.results.len(){
            self.results[i].rotate_cw();
        }

        self.rows = self.pattern.array.len();
        self.columns = self.pattern.array[0].len();
    }
}

fn string_from_file(path: &Path)->Option<String>{
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

fn import_rule_file(path: &Path)->Option<Vec<Rule>>{

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
    let mut flipveritcal = false;

    //Handle the rest of the options
    for i in 2..options_vec.len(){
        for char in options_vec[i].chars(){
            match char{
                'R' => {rotate = true;},
                //'H' => {fliphorizontal = true;},
                //'V' => {flipvertical = true;},
                _ => (),
            }
        }
    }

    //Handle the rest of the lines
    //These will either be lines full of data for grids or spacer lines
    let mut grids_vec: Vec<DungeonGrid> = Vec::new();

    let mut current_row = 0;
    let mut reset_grid = false;

    let mut current_grid = DungeonGrid::new(rule_rows,rule_columns,DungeonTile::Unknown);

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
            current_grid = DungeonGrid::new(rule_rows,rule_columns,DungeonTile::Unknown);
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
            for j in 0..i{
                //println!("Being called {} of {}",j,i);
                rot_rule.rotate_cw();
            }
            rules_vec.push(rot_rule);
        }
    }

    Some(rules_vec)
}

fn import_rules_folder()->Option<Vec<Rule>>{

    let paths = fs::read_dir("./rules/").unwrap();
    let mut rules: Vec<Rule> = Vec::new();

    for path in paths{
        println!("{}",path.as_ref().unwrap().path().display());
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

fn main() {

    let mut test_grid = DungeonGrid::new(50,50,DungeonTile::Blank);
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
