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

    //Creates an initial t junction to fill the grid with
    pub fn init_tjuction(&mut self){
        let x = rand::random::<usize>() % self.columns;
        let y = rand::random::<usize>() % self.rows;

        //The way across
        for i in 0..self.columns{
            self.array[y][i] = Tile::CorridorEW;
        }

        //From above
        for i in 0..y{
            self.array[i][x] = Tile::CorridorNS;
        }

        //Overwrite the intersection
        self.array[y][x] = Tile::ThreewayS;
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
                newgrid[self.columns-i-1][j] = flip_tile_v(self.array[i][j])
            }
        }

        self.array = newgrid;

        self.columns = self.array[0].len();
        self.rows = self.array.len();
    }
}
