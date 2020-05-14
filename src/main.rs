use gol::grid;
use gol::neigh;

fn main() {
    let g = grid::Grid::new(10, 10);

    // Outer rim ex
    let mut n_buffer: [usize; 8] = [0; 8];

    for y in 1..g.h-1 {
        for x in 1..g.w-1 {
            let i = g.get_index(x, y);
            g.get_neighbors(i, &mut n_buffer).unwrap();
            println!("{}: {:?}", i, n_buffer);
        }
    }
}
