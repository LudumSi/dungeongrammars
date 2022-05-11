
use crate::grid::*;

#[derive(Clone)]
pub struct Rule{
    pub pattern: Grid,
    pub results: Vec<Grid>,
    pub rows: usize,
    pub columns: usize,
}

impl Rule{

    //Prints a rule. Was mostly used for debug
    pub fn print(&self){
        self.pattern.print();
        println!("\nbecomes\n");
        for i in 0..self.results.len(){
            if i != 0{
                println!("\nor\n")
            }
            self.results[i].print();
        }
    }

    //Rotates the target and the results of a rule clockwise
    pub fn rotate_cw(&mut self){
        //println!("Rotate CW being called from the rule!");
        self.pattern.rotate_cw();
        for i in 0..self.results.len(){
            self.results[i].rotate_cw();
        }

        self.rows = self.pattern.array.len();
        self.columns = self.pattern.array[0].len();
    }

    //Fips the target and results of a rule veritcally
    pub fn flip_v(&mut self){
        self.pattern.flip_v();
        for i in 0..self.results.len(){
            self.results[i].flip_v();
        }

        self.rows = self.pattern.array.len();
        self.columns = self.pattern.array[0].len();
    }

    //Flips the target and results of a rule horizontally
    pub fn flip_h(&mut self){
        self.pattern.flip_h();
        for i in 0..self.results.len(){
            self.results[i].flip_h();
        }

        self.rows = self.pattern.array.len();
        self.columns = self.pattern.array[0].len();
    }

}
