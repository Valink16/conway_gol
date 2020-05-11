#[cfg(test)]
mod tests {
    // Checks if the outer rim detection works and throws errors correctly
    use crate::grid;
    #[test]
    fn outer_rim_exclusion() {
        const w: usize = 10;
        const h: usize = 10;

        let g = grid::Grid::new(w, h);

        // Upper rim check
        for x in 0..w {
            assert_eq!(g.get_neighbors(g.get_index(x, 0)).is_err(), true);
        }

        // Down rim check
        for x in 0..w {
            assert_eq!(g.get_neighbors(g.get_index(x, h - 1)).is_err(), true);
        }

        // Left rim check
        for y in 0..h {
            assert_eq!(g.get_neighbors(g.get_index(0, y)).is_err(), true);
        }

        // Right rim check
        for y in 0..h {
            assert_eq!(g.get_neighbors(g.get_index(w - 1, y)).is_err(), true);
        }

        // Check inside the rim
        assert_eq!(g.get_neighbors(g.get_index(w / 2, h / 2)).is_ok(), true);
    }
}

pub mod grid;
pub mod neigh;

pub enum State {
    Alive,
    Dead
}

use std::fmt;
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            State::Alive => "alive",
            State::Dead => "dead"
        };
        write!(f, "{}", s)
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            State::Alive => "1",
            State::Dead => "0"
        };
        write!(f, "{}", s)
    }
}


