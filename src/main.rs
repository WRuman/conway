mod game;
use game::Game;
mod grid;
use grid::{Grid, Cell};

fn disp(board: &Grid) {
    for cell in board {
        let (y, x, c) = cell;
        let state = match *c {
            Cell::Alive => "Alive",
            Cell::Dead => "Dead",
        };
        println!("{}, {} -> {}", y, x, state);
    }
}

fn main() {
    let mut other_board = Grid::with_dimension(5);
    other_board.randomize();
    println!("{}", other_board);
    disp(&other_board);
}

