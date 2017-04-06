mod game;
use game::Game;
mod grid;
use grid::Grid;
mod cell;
use cell::Cell;

fn main() {
    let mut b = Grid::with_dimension(10);
    println!("{}", b);
    println!("Top left corner is {:?}", b.at(0,0).unwrap_or(&Cell::Alive));
    b.randomize();
    println!("{}", b);
    println!("Top left corner is {:?}", b.at(0,0).unwrap_or(&Cell::Dead));
    let mut g = Game::new_with_board(&mut b); 
    g.tick();
}

