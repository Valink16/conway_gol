use gol::grid;
use gol::render;

fn main() {
    let mut r = render::Renderer::new(
        "Renderer test", 600, 400,
        grid::Grid::new(10, 10)
    );

    r.wait();
}
