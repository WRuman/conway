mod game;
use game::Game;
mod grid;
use grid::{Grid, Cell};

fn main() {
    let mut other_board = Grid::with_dimension(5);
    let tgt = (0, 0);
    other_board.randomize();
    println!("{}", other_board);
    println!("{} living neighbors for cell at 1,1", other_board.living_neighbor_count(tgt));
}

