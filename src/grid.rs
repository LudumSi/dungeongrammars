extern crate rand;

use crate::tile::*;

#[derive(Clone)]
pub struct Grid{
    pub array: Vec<Vec<Tile>>,
    pub rows: usize,
    pub columns: usize,
}

impl Grid{
    pub fn new(row:usize,column:usize,fill:Tile)->Grid{
        Grid{
            rows: row,
            columns: column,
            array: vec![vec![fill;column];row],
        }
    }

    pub fn print(&self){
        for i in 0..self.rows{
            for j in 0..self.columns{
                print!("{}",print_tile(&self.array[i][j]));
            }
            print!("{}",'\n');
        }
    }

    //Rotates the whole grid clockwise
    //Works for non-square grids as well as square ones
    pub fn rotate_cw(&mut self){
        let mut newgrid = vec![vec![Tile::Blank;self.rows];self.columns];
        for i in 0..self.rows{
            for j in 0..self.columns{
                newgrid[j][self.rows-i-1] = rotate_tile_cw(self.array[i][j]);
            }
        }

        self.array = newgrid;

        self.columns = self.array[0].len();
        self.rows = self.array.len();
    }

    pub fn flip_h(&mut self){
        let mut newgrid = vec![vec![Tile::Blank;self.columns];self.rows];
        for i in 0..self.rows{
            for j in 0..self.columns{
                newgrid[i][self.columns-j-1] = flip_tile_h(self.array[i][j])
            }
        }

        self.array = newgrid;

        self.columns = self.array[0].len();
        self.rows = self.array.len();
    }

    pub fn flip_v(&mut self){
        let mut newgrid = vec![vec![Tile::Blank;self.columns];self.rows];
        for i in 0..self.rows{
            for j in 0..self.columns{
                newgrid[self.rows-i-1][j] = flip_tile_v(self.array[i][j])
            }
        }

        self.array = newgrid;

        self.columns = self.array[0].len();
        self.rows = self.array.len();
    }
}
