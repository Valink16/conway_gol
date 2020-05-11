use crate::neigh;
use crate::State;

// A grid representing the plane where the simulation takes place
// The data is stored as a 1D array of booleans, since each cell can only be either dead or alive 
pub struct Grid {
    cells: Vec::<bool>,
    n_ops: [isize; 8], // This array stores the operations to apply to a given index to get it's 8 neighbors
    pub w: isize,
    pub h: isize,
}

impl Grid {
    // Initiates the grid
    pub fn new(w: usize, h: usize) -> Grid {
        let cells = vec![false; w * h]; 
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

    pub fn get_cell(&self, i: usize) -> State {
        match self.cells[i] {
            true => State::Alive,
            false => State::Dead
        }
    }

    // Returns the index of the cell at (x, y)
    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.w as usize + x
    }

    // Get an iterator over all the neighbors at (x, y)
    pub fn get_neighbors(&self, i: usize) -> Result<neigh::NeighborIter, neigh::NeighborError> {
        neigh::NeighborIter::new(&self.n_ops, i as isize, self.w, self.h)
    }
}