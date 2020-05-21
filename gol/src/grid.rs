use std::convert::TryInto;

use crate::neigh;

use rand;

// A grid representing the plane where the simulation takes place
// The data is stored as a 1D array of booleans, since each cell can only be either dead or alive 
// The outer rim of the grid should be never evaluated to reduce program complexity, calling get_neighbors on one of the cells in the outer rim will result in an error
pub struct Grid {
    pub cells: Vec::<(bool, bool)>,
    pub n_ops: [isize; 8], // This array stores the operations to apply to a given index to get it's 8 neighboring indexes
    pub w: usize,
    pub h: usize,
}

impl Grid {
    // Initiates the grid
    pub fn new(w: usize, h: usize) -> Grid {
        let cells = vec![(false, false); w * h];
        let w = w;
        let h = h;
        let n_ops = [
            -1,               // left
            -(w as isize) - 1,// up-left
            -(w as isize),    // up
            -(w as isize) + 1,// up-right
            1,                // right
            (w as isize) + 1,    // down-right
            (w as isize),       // down
            (w as isize) - 1   // down-left
        ];

        Grid {
            cells,
            n_ops,
            w,
            h
        }
    }

    pub fn update_cells(&mut self) {
        for c in &mut self.cells {
            c.0 = c.1;
        }
    }

    // Returns the index of the cell at (x, y)
    pub fn get_index(&self, x: i32, y: i32) -> usize {
        let r = y * self.w as i32 + x;
        r.try_into().expect("Cannot convert negative coordinates")
    }

    // Returns an array over the indexes of all the neighbors at index
    pub fn get_neighbors(&self, index: isize, n: &mut [usize; 8]) -> Option<()> {
        let x = index as usize % self.w;
        let y = index as usize / self.w;

        if x == 0 || x == self.w - 1 || y == 0 || y == self.h - 1 {
            return None;
        }

        for i in 0..self.n_ops.len() {
            n[i] = (index + self.n_ops[i]) as usize;
        }

        Some(())
    }

    // Returns the amount of living neighbours for cell at index
    pub fn count_live_neighbors(&self, index: usize) -> u32 {
        let x = index % self.w;
        let y = index / self.w;

        if x == 0 || x == self.w - 1 || y == 0 || y == self.h - 1 { // The cells at the outer rim must never become alive, therefore they always have 0 live neighbors
            return 0;
        }

        let mut c = 0;
        for i in 0..self.n_ops.len() {
            if self.cells[(index as isize + self.n_ops[i]) as usize].0 {
                c += 1;
            }
        }

        c
    }

    // Simulates one generation of the game of life
    pub fn step(&mut self) {
        for y in 1..(self.h - 1) as i32 {
            for x in 1..(self.w - 1) as i32 {
                let i = self.get_index(x, y);
                let n = self.count_live_neighbors(i);
                if self.cells[i].0 {
                    if n < 2 { // Underpopulation
                        self.cells[i].1 = false;
                    } else if n > 3 { // Overpopulation
                        self.cells[i].1 = false;
                    } else if n >= 2 { // Survival
                        ();
                    } 
                } else {
                    if n == 3 { // Reproduction
                        self.cells[i].1 = true;
                    }
                }
            }
        }
    }

    // Test function which randomizes the data
    pub fn randomize(&mut self) {
        for y in 1..self.h - 1 {
            for x in 1..self.w - 1 {
                let i = self.get_index(x as i32, y as i32);
                let n = rand::random::<usize>();
                self.cells[i].1 = n % 25 == 0;
            }
        }
    }
}