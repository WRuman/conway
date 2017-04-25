use grid::{Grid, Cell};

/// Represents a game in progress, that takes ownership of a Grid for its entire lifetime
pub struct Game {
    board_a: Grid,
    board_b: Grid,
    tick: u64
}

impl Game {
    pub fn new_with_size(sz: usize) -> Game {
        let mut ga = Grid::with_dimension(sz);
        let mut gb = Grid::with_dimension(sz);
        ga.set_cell((2, 2), Cell::Alive);
        ga.set_cell((2, 3), Cell::Alive);
        ga.set_cell((2, 4), Cell::Alive);
        gb.set_cell((2, 2), Cell::Alive);
        gb.set_cell((2, 3), Cell::Alive);
        gb.set_cell((2, 4), Cell::Alive);
        Game {board_a: ga, board_b: gb, tick: 0}
    }

    pub fn show_board(&self) {
        // Reset cursor position 
        print!("\x1B[{}A\r", self.board_a.dimension());
        match self.tick % 2 {
            0 => print!("{}", self.board_b),
            _ => print!("{}", self.board_a)
        }
    }

    /// Run the game one tick forward
    pub fn tick(&mut self) {
        self.tick += 1;
        let (source_board, target_board) = match self.tick % 2 {
            0 => (&self.board_a, &mut self.board_b),
            _ => (&self.board_b, &mut self.board_a)
        };
        let mut griderator = source_board.into_iter();
        loop {
            let (current_row, current_col, current_cell) = match griderator.next() {
                Some(c) => c,
                None => break
            };
            let living_neighbors = source_board.living_neighbor_count((current_row, current_col));
            let c_next = match living_neighbors {
                n if n < 2 => Cell::Dead,
                2 | 3 => Cell::Alive,
                n if n > 3 => {
                    match *current_cell {
                        Cell::Alive => Cell::Dead,
                        Cell::Dead => Cell::Alive
                    }
                },
                _ => Cell::Dead
            };
            target_board.set_cell((current_row, current_col), c_next);
        }
    }

}
