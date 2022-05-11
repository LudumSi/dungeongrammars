
use crate::tile::*;
use crate::rule::*;
use crate::grid::*;

impl Grid{
    //Copies a random result from a rule to the given location
    pub fn replace_with_rule(&mut self, row: usize, column: usize, rule:&Rule){
        //Choose replacement
        let r = rand::random::<usize>() % rule.results.len();

        for k in 0..rule.rows{
            for l in 0..rule.columns{
                if rule.results[r].array[k][l] != Tile::Unknown {
                    self.array[row+k][column+l] = rule.results[r].array[k][l];
                }
            }
        }
    }

    //Checks a location to see if the target of a rule matches
    pub fn check_location(&mut self, row: usize, column: usize, rule:&Rule)->bool{
        //Safety checks
        if row+rule.rows >= self.rows{
            return false;
        }

        if column+rule.columns >= self.columns {
            return false;
        }

        for k in 0..rule.rows{
            for l in 0..rule.columns{
                if rule.pattern.array[k][l] != Tile::Unknown && self.array[row+k][column+l] != rule.pattern.array[k][l]{
                    return false;
                }
            }
        }

        return true;
    }

    //Tries to apply the rule to the first place it finds a match
    pub fn apply_rule_lazy(&mut self, rule: &Rule){

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

    //Applies the rule at a random location a match was found
    pub fn apply_rule(&mut self, rule: &Rule)->bool{

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

    //Will attempts to apply a random rule. If the rule it selected fails, it tries another.
    pub fn apply_random_rule(&mut self, rules:&Vec<Rule>){

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
