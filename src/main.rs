use gol::grid;

fn main() {
    let g = grid::Grid::new(10, 10);

    // Outer rim ex
    for _n in g.get_neighbors(1).unwrap() {
        println!("{:?}", _n);
    }
}
