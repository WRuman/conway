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
        let mut last_row = 0;
        for c in self.board {
            let (current_row, current_col, _) = c;
            let use_buff_a = match last_row % 2 { 0 => true, _ => false };
            let living_neighbors = self.board.living_neighbor_count((current_row, current_col));
            let c_next = match living_neighbors {
                n if n < 2 => Cell::Dead,
                2 | 3 => Cell::Alive,
                n if n > 3 => {
                    match *c.2 {
                        Cell::Alive => Cell::Dead,
                        Cell::Dead => Cell::Alive
                    }
                },
                _ => Cell::Dead
            };
            match use_buff_a {
                true => self.buff_a.push(c_next),
                false => self.buff_b.push(c_next)
            }
            if current_row != last_row {
                // We need to write the buffer to the grid
                if current_row > 1 {
                    match use_buff_a {
                        true => {
                            self.board.write_row(last_row - 1, &self.buff_a);
                            self.buff_a.clear();
                        }
                        false => {
                            self.board.write_row(last_row - 1, &self.buff_b);
                            self.buff_b.clear();
                        }
                    }
                }
                last_row = current_row;
            }
        }
    }

}
