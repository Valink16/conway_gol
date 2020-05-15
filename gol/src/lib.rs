#[cfg(test)]
mod tests {
    // Checks if the outer rim detection works and throws errors correctly
    use crate::grid;
    #[test]
    fn outer_rim_exclusion() {
        const w: isize = 10;
        const h: isize = 10;

        let g = grid::Grid::new(w as usize, h as usize);

        let mut n_buffer = [0 as usize; 8];

        // Upper rim check
        for x in 0..w as isize {
            assert_eq!(g.get_neighbors(g.get_index(x, 0), &mut n_buffer).is_none(), true);
        }

        // Down rim check
        for x in 0..w as isize {
            assert_eq!(g.get_neighbors(g.get_index(x, h - 1), &mut n_buffer).is_none(), true);
        }

        // Left rim check
        for y in 0..h as isize {
            assert_eq!(g.get_neighbors(g.get_index(0, y), &mut n_buffer).is_none(), true);
        }

        // Right rim check
        for y in 0..h as isize {
            assert_eq!(g.get_neighbors(g.get_index(w - 1, y), &mut n_buffer).is_none(), true);
        }

        // Check inside the rim
        assert_eq!(g.get_neighbors(g.get_index(w / 2, h / 2), &mut n_buffer).is_some(), true);
    }
}

pub mod grid;
pub mod neigh;