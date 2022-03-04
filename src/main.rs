#![allow(unused_parens)]

use std::fs;
use std::env;

mod data;
mod snake;
mod grid;



fn main() {
    
    let     input  = env::args().collect::<Vec<String>>()[1].clone();
    let     script = read(input);
    let mut grid   = grid::Grid::from(script);
    loop {
        grid.update_snake();
        if (grid.check_snake_dead()) {
            break;
        }
    }
    
}

fn read(filename : String) -> String {
    return fs::read_to_string(filename).unwrap();
}
