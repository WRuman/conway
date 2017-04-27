extern crate rand;
use self::rand::Rng;
use std::cmp::Ordering;
use std::fmt;

const NEIGHBOR_LOCS: [(i8, i8); 8] = 
    [(-1, -1), (-1, 0), (-1, 1),
     ( 0, -1),          ( 0, 1),
     ( 1, -1), ( 1, 0), ( 1, 1)];

#[derive(Copy, Clone)]
pub enum Cell {
    Alive,
    Dead,
}

impl Cell {
    fn is_alive(&self) -> bool {
        match *self {
            Cell::Alive => true,
            Cell::Dead => false
        }
    }
}

pub struct Grid {
    cells: Vec<Cell>,
    dim: usize,
}

pub struct GridIterator<'a> {
    row_length: usize,
    index: usize,
    cells: &'a Vec<Cell>
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = (usize, usize, &'a Cell);
    fn next(&mut self) -> Option<(usize, usize, &'a Cell)> {
        let x = self.index % self.row_length;
        let y = (self.index - x) / self.row_length;
        let next_cell = self.cells.get(self.index);
        match next_cell {
            Some(c) => {
                self.index += 1; 
                Some((y, x, c))
            },
            None => None
        }
    }
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
    fn at(&self, y: usize, x : usize) -> Option<&Cell> {
        let idx = self.dim * y + x;
        self.cells.get(idx)
    }

    pub fn set_cell(&mut self, loc: (usize, usize), c: Cell) {
        let idx = self.dim * loc.0 + loc.1;
        if idx < self.cells.len() {
            self.cells[idx] = c;
        }
    }

    /// Returns the number of living neighbors at a given grid location
    /// Assume that the number of total neighbors (t) - number of living neighbors (l)
    /// == the number of dead neighbors (d) such that d + l = t
    pub fn living_neighbor_count(&self, loc: (usize, usize)) -> usize {
        let y_overflow = loc.0 >= self.dim;
        let x_overflow = loc.1 >= self.dim;
        let y_may_underflow = loc.0 < 1;
        let x_may_underflow = loc.1 < 1;

        NEIGHBOR_LOCS.iter().filter(|&neighbor| {
            let (ny, nx) = *neighbor;
            !((ny < 0 && y_may_underflow) ||
            (nx < 0 && x_may_underflow) ||
            (ny > 0 && y_overflow) ||
            (nx > 0 && x_overflow))
        }).fold(0, |acc, &neighbor| {
            let (cy, cx) = loc;
            let (ny, nx) = neighbor;
            
            let y = match ny.cmp(&0) {
                Ordering::Less => {
                    cy - ny.abs() as usize
                },
                Ordering::Equal => cy,
                Ordering::Greater => {
                    cy + ny as usize
                }
            };

            let x = match nx.cmp(&0) {
                Ordering::Less => {
                    cx - nx.abs() as usize
                },
                Ordering::Equal => cx,
                Ordering::Greater => {
                    cx + nx as usize
                }
            };

            acc + match self.at(y, x) {
                Some(cell) => if cell.is_alive() { 1 } else { 0 },
                None => 0
            }
        })
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
                Cell::Dead => '-'
            });
        }
        f.write_str(out.as_str())
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = (usize, usize, &'a Cell);
    type IntoIter = GridIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GridIterator {index: 0, row_length: self.dim, cells: &self.cells}
    }
}
