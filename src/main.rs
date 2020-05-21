use gol::grid;
use gol::render;

fn main() {
    let mut g = grid::Grid::new(450, 450);

    let mut r = render::Renderer::new(
        "Renderer test", 1000, 1000,
        g
    );

    //r.window.set_vertical_sync_enabled(true); // This is shit
    r.window.set_framerate_limit(85);
    r.render_loop();
}
