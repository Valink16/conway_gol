use crate::neigh;
use crate::State;

// A grid representing the plane where the simulation takes place
// The data is stored as a 1D array of booleans, since each cell can only be either dead or alive 
// The outer rim of the grid should be never evaluated to reduce program complexity, calling get_neighbors on one of the cells in the outer rim will result in an error
pub struct Grid {
    pub cells: Vec::<(bool, bool)>,
    pub n_ops: [isize; 8], // This array stores the operations to apply to a given index to get it's 8 neighbors
    pub w: isize,
    pub h: isize,
}

impl Grid {
    // Initiates the grid
    pub fn new(w: usize, h: usize) -> Grid {
        let cells = vec![(false, false); w * h];
        let w = w as isize;
        let h = h as isize;
        let n_ops = [
            -1,               // left
            -w - 1,// up-left
            -w,    // up
            -w + 1,// up-right
            1,                // right
            w + 1,    // down-right
            w,       // down
            w - 1   // down-left
        ];

        Grid {
            cells,
            n_ops,
            w,
            h
        }
    }

    pub fn update_cells(&mut self) {
        let mut n = [0 as usize; 8];
        for i in 0..self.cells.len() as isize {
            match self.get_neighbors(i, &mut n) {
                Some(_) => {
                    for _n in n.iter() {
                        self.cells[*_n].0 = self.cells[*_n].1;
                    }
                },
                None => ()
            };
        }
    }

    // Returns the index of the cell at (x, y)
    pub fn get_index(&self, x: isize, y: isize) -> isize {
        y * self.w + x
    }

    // Returns an array over the indexes of all the neighbors at (x, y)
    pub fn get_neighbors(&self, index: isize, n: &mut [usize; 8]) -> Option<()> {
        let x = index % self.w;
        let y = index / self.w;

        if x == 0 || x == self.w - 1 || y == 0 || y == self.h - 1 {
            return None;
        }

        for i in 0..self.n_ops.len() {
            n[i] = (index + self.n_ops[i]) as usize;
        }

        Some(())
    }
}