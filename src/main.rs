use gol::grid;
use gol::render;

fn main() {
    let mut g = grid::Grid::new(60, 40);
    g.randomize();

    let mut r = render::Renderer::new(
        "Renderer test", 600, 400,
        g
    );

    r.render_loop();
}
