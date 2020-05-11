use std::iter::Iterator;
use std::fmt;
use std::error::Error;

// Simple error for NeighborIter
#[derive(Debug)]
pub struct NeighborError;
impl fmt::Display for NeighborError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cells in the outer border of the grid are not supposed to be used")
    }
}

impl Error for NeighborError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

// Iterator over neighbors of a cell
// Returns the indexes for each neighbor
pub struct NeighborIter<'a> {
    ops: &'a[isize; 8],
    index: isize,
    cn: usize,
}

impl<'a> NeighborIter<'a> {
    pub fn new(ops: &'a[isize; 8], index: isize, w: isize, h: isize) -> Result<NeighborIter, NeighborError> {
        let x = index % w;
        let y = index / w;

        if x == 0 || x == w - 1 || y == 0 || y == h - 1 {
            return Err(NeighborError);
        }

        Ok(NeighborIter {
            ops,
            index,
            cn: 0
        })
    }
}

impl<'a> Iterator for NeighborIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        while self.cn < self.ops.len() {
            let r = Some((self.index + self.ops[self.cn]) as usize);
            self.cn += 1;
            return r;
        }
        None
    }
}