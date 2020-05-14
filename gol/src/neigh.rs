use std::iter::Iterator;

// Iterator over neighbors of a cell
// Returns the indexes for each neighbor
pub struct NeighborIter<'a> {
    ops: &'a[isize; 8],
    index: isize,
    cn: usize,
}

impl<'a> NeighborIter<'a> {
    pub fn new(ops: &'a[isize; 8], index: isize, w: isize, h: isize) -> Option<NeighborIter> {
        let x = index % w;
        let y = index / w;

        if x == 0 || x == w - 1 || y == 0 || y == h - 1 {
            return None;
        }

        Some(NeighborIter {
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