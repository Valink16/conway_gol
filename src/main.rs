use gol::grid;
use gol::neigh;

fn main() {
    let g = grid::Grid::new(10, 10);

    // Outer rim ex
    for y in 1..g.h-1 {
        for x in 1..g.w-1 {
            let i = g.get_index(x as usize, y as usize);
            let n: Vec<usize> = g.get_neighbors(i).unwrap().collect();
            println!("{}: {:?}", i, n);
        }
    }
}
