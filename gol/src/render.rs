use crate::grid;

use sfml::graphics::RenderWindow;
use sfml::window::Event;

pub struct Renderer {
    pub g: grid::Grid,
    pub window: RenderWindow,
}

impl Renderer {
    // Initiates SFML and sets up a new window to render the grid on
    pub fn new(name: &str, w: u32, h: u32, g: grid::Grid) -> Self {
        // SFML setup
        let window = RenderWindow::new(
            sfml::window::VideoMode::from((w, h)), 
            name,
            sfml::window::Style::CLOSE,
            &Default::default()
        );
        
        Self {
            g,
            window
        }
    }

    // Temporary test function, will just wait for a quit event and exit
    pub fn wait(&mut self) {
        'running: loop {
            while let Some(e) = self.window.poll_event() {
                match e {
                    Event::Closed => break 'running,
                    _ => ()
                }
            }
        }
    }
}