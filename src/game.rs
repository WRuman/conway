use grid::{Grid, Cell};

/// Represents a game in progress, that takes ownership of a Grid for its entire lifetime
pub struct Game<'a> {
    board: &'a mut Grid,
    buff_a: Vec<Cell>,
    buff_b: Vec<Cell>,
    tick: u64
}

impl<'a> Game<'a> {
    pub fn new_with_board(board: &'a mut Grid) -> Game<'a> {
        let ba = Vec::<Cell>::with_capacity(board.dimension());
        let bb = Vec::<Cell>::with_capacity(board.dimension());
        Game {board: board, tick: 0, buff_a: ba, buff_b: bb}
    }

    /// Run the game one tick forward
    pub fn tick(&mut self) {
        self.tick += 1;
    }

}
