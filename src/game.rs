use grid::{Grid, Cell};

/// Represents a game in progress, that takes ownership of a Grid for its entire lifetime
pub struct Game {
    board_a: Grid,
    board_b: Grid,
    yin_yang: bool
}

impl Game {
    pub fn new_with_size(sz: usize) -> Game {
        let ga = Grid::with_dimension(sz);
        let gb = Grid::with_dimension(sz);
        Game {board_a: ga, board_b: gb, yin_yang: true}
    }

    pub fn add_glider(&mut self) {
        self.board_a.set_cell((8, 2), Cell::Alive);
        self.board_a.set_cell((8, 3), Cell::Alive);
        self.board_a.set_cell((8, 4), Cell::Alive);
        self.board_a.set_cell((7, 4), Cell::Alive);
        self.board_a.set_cell((6, 3), Cell::Alive);
        self.yin_yang = true;
    }
    pub fn add_acorn(&mut self) {
        self.board_a.set_cell((8, 2), Cell::Alive);
        self.board_a.set_cell((8, 3), Cell::Alive);
        self.board_a.set_cell((8, 6), Cell::Alive);
        self.board_a.set_cell((8, 7), Cell::Alive);
        self.board_a.set_cell((8, 8), Cell::Alive);
        self.board_a.set_cell((7, 5), Cell::Alive);
        self.board_a.set_cell((6, 3), Cell::Alive);
        self.yin_yang = true;
    }

    pub fn show_board(&self) {
        // Reset cursor position 
        print!("\x1B[{}A\r", self.board_a.dimension());
        match self.yin_yang {
            false => print!("{}", self.board_b),
            true => print!("{}", self.board_a)
        }
    }

    /// Run the game one tick forward
    pub fn tick(&mut self) {
        let (source_board, target_board) = match self.yin_yang {
            true => (&self.board_a, &mut self.board_b),
            false => (&self.board_b, &mut self.board_a)
        };
        let mut griderator = source_board.into_iter();
        loop {
            let (current_row, current_col, current_cell) = match griderator.next() {
                Some(c) => c,
                None => break
            };
            let living_neighbors = source_board.living_neighbor_count((current_row, current_col));
            let c_next = match *current_cell {
                Cell::Alive => match living_neighbors {
                    n if n < 2 => Cell::Dead,
                    2 | 3 => Cell::Alive,
                    n if n > 3 => Cell::Dead,
                    _ => Cell::Dead
                },
                Cell::Dead => match living_neighbors {
                    3 => Cell::Alive,
                    _ => Cell::Dead
                }
            };
            target_board.set_cell((current_row, current_col), c_next);
        }
        self.yin_yang = !self.yin_yang;
    }

}
