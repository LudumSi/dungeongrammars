
use crate::tile::*;
use crate::rule::*;
use crate::grid::*;

pub fn compute_cdf(weights: Vec<usize>) -> Vec<usize>{
    let mut current_total = 0;
    let mut cdf: Vec<usize> = Vec::new();

    for weight in weights{
        current_total += weight;
        cdf.push(current_total);
    }

    cdf
}

pub fn weighted_pick(size: usize, cdf: &Vec<usize>) -> usize{

    //Safety check to ensure usize is the size of the cdf
    if size != cdf.len(){
        panic!("Weighted pick was passed a size {} which did not match cdf {}!",size,cdf.len());
    }

    let last = match cdf.last(){
        Some(x) => x,
        None => panic!("Weighted pick was passed an empty cdf function!"),
    };

    //Random float limited to range of the cdf
    let r = rand::random::<usize>() % last+1;

    //Binary search by the random val to get the index to go with
    match cdf.binary_search(&r){
        Ok(y) => return y,
        Err(y) => return y,
    };

}

impl Grid{
    //Copies a random result from a rule to the given location
    pub fn replace_with_rule(&mut self, row: usize, column: usize, rule:&Rule){
        //Choose replacement
        let r = weighted_pick(rule.results.len(),&rule.result_weights);

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
    pub fn _apply_rule_lazy(&mut self, rule: &Rule){

        //Iterate over the whole dungeon grid the rule can reach
        for i in 0..=(self.rows-rule.rows){
            for j in 0..=(self.columns-rule.columns){

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
        for i in 0..=(self.rows-rule.rows){
            for j in 0..=(self.columns-rule.columns){

                //println!("Considering {} {}",i,j);

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
    pub fn apply_random_rule(&mut self, rset:&RuleSet){

        let mut viable: Vec<usize> = (0..rset.rules.len()).collect();

        while viable.len() > 0 {

            //println!("Running random rule");

            //Shitty way to do this, will have to think of a better way
            let r = weighted_pick(rset.rules.len(),&rset.weights);
            match viable.binary_search(&r){
                Ok(x) => {
                    if self.apply_rule(&rset.rules[r]){
                        return;
                    }else{
                        viable.remove(x);
                    }
                }
                Err(_) => (),
            }
        }
    }
}
