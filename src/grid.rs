extern crate rand;
use self::rand::Rng;
use std::fmt;

#[derive(Debug)]
pub enum Cell {
    Alive,
    Dead,
}

pub struct Grid {
    cells: Vec<Cell>,
    dim: usize,
}

impl Grid {
    /// Returns a square grid with dimensions (dim x dim). All cells default to Dead
    pub fn with_dimension(dim: usize) -> Grid {
        let cell_count = dim * dim;
        let mut cells = Vec::<Cell>::with_capacity(cell_count);
        for _ in 0..cell_count {
            cells.push(Cell::Dead);
        }
        Grid {dim: dim, cells: cells}
    }
    /// Returns reference to cell at a given place in the grid, if possible.
    /// Cells are 0-indexed, increasing from the upper left corner to the 
    /// lower right corner. 
    pub fn at(&self, y: usize, x : usize) -> Option<&Cell> {
        let idx = self.dim * y + x;
        self.cells.get(idx)
    }

    /// Sets each cell in the grid to a random Dead or Alive state
    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        for cell in self.cells.iter_mut() {
            *cell = match rng.gen::<bool>() {
                true => Cell::Alive,
                false => Cell::Dead
            };
        }
    }
    /// Returns length of any side of the grid
    pub fn dimension(&self) -> usize {
        self.dim
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        for (idx, cell) in self.cells.iter().enumerate() {
            if idx % self.dim == 0 {
                out.push('\n');
            }
            out.push(match *cell {
                Cell::Alive => 'X',
                Cell::Dead => 'O'
            });
        }
        f.write_str(out.as_str())
    }
}

