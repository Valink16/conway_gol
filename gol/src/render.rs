use crate::grid;

use sfml::graphics::RenderWindow;
use sfml::window::Event;
use sfml::graphics::VertexArray;
use sfml::graphics::Color;
use sfml::system::Vector2f;

use sfml::graphics::RenderTarget;

pub struct Renderer {
    pub g: grid::Grid,
    pub window: RenderWindow,
    cells_vbuffer: VertexArray // Vertex buffer containing all the quads representing the cells
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

        let cells_vbuffer = create_vbuffer(&g.cells, (w, h), (g.w as u32, g.h as u32));
        
        Self {
            g,
            window,
            cells_vbuffer
        }
    }

    pub fn render_loop(&mut self) {
        'running: loop {
            while let Some(e) = self.window.poll_event() {
                match e {
                    Event::Closed => break 'running,
                    _ => ()
                }
            }

            self.g.randomize();
            self.update_vbuffer();
    
            self.window.clear(Color::WHITE);
            self.window.draw(&self.cells_vbuffer);
            self.window.display();
        }
    }

    pub fn update_vbuffer(&mut self) { // Updates self.cells_vbuffer using self.g.cells
        for i in (0..self.cells_vbuffer.vertex_count()).step_by(4) {
            if self.g.cells[i / 4].0 {
                self.cells_vbuffer[i].color = Color::BLACK;
                self.cells_vbuffer[i + 1].color = Color::BLACK;
                self.cells_vbuffer[i + 2].color = Color::BLACK;
                self.cells_vbuffer[i + 3].color = Color::BLACK;
            } else {
                self.cells_vbuffer[i].color = Color::WHITE;
                self.cells_vbuffer[i + 1].color = Color::WHITE;
                self.cells_vbuffer[i + 2].color = Color::WHITE;
                self.cells_vbuffer[i + 3].color = Color::WHITE;
            }
        
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

// Creates and return a vertex buffer representing a 1D (bool, bool) vector as a grid of BLACK or WHITE rectangles
fn create_vbuffer(data: &Vec<(bool, bool)>, screen_size: (u32, u32), grid_size: (u32, u32)) -> VertexArray {
    let mut arr = VertexArray::new(sfml::graphics::PrimitiveType::Quads, data.len() * 4); // Multiply by 4 because these are quads
    let c_w = (screen_size.0 / grid_size.0) as f32;
    let c_h = (screen_size.1 / grid_size.1) as f32;

    for i in 0..data.len() as u32 {
        let x = (i % grid_size.0) as f32 * c_w;
        let y = (i / grid_size.1) as f32 * c_h;
        let _i = (i * 4) as usize;

        // Setting up the 4 points of the quad representing cell at data[i]
        arr[_i].position = Vector2f::new(x, y);
        arr[_i + 1].position = Vector2f::new(x + c_w, y);
        arr[_i + 2].position = Vector2f::new(x + c_w, y + c_h);
        arr[_i + 3].position = Vector2f::new(x, y + c_h);
        
        let c: Color;
        if data[i as usize].0 { // The cell is black if true
            c = Color::BLACK;
        } else { // or white
            c = Color::WHITE;
        }

        arr[_i].color = c;
        arr[_i + 1].color = c;
        arr[_i + 2].color = c;
        arr[_i + 3].color = c;

         // println!("{:?}, {:?}, {:?}, {:?}", arr[_i], arr[_i + 1], arr[_i + 2], arr[_i + 3]);
    }

    arr
}