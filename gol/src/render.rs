use crate::grid;

use sfml::graphics::{RenderWindow, VertexArray, Color, RenderTarget, Rect};

use sfml::system::Vector2f;

use sfml::window::{Event, Key};
use sfml::window::mouse::Button;

pub struct Renderer {
    pub g: grid::Grid,
    pub window: RenderWindow,
    cells_vbuffer: VertexArray, // Vertex buffer containing all the quads representing the cells
    grid_vbuffer: VertexArray, // Outline of the grid
    cell_size: Rect<f32>
}

impl Renderer {
    // Initiates SFML and sets up a new window to render the grid on
    pub fn new(name: &str, w: u32, h: u32, g: grid::Grid) -> Self {
        // SFML setup
        let mut window = RenderWindow::new(
            sfml::window::VideoMode::from((w, h)), 
            name,
            sfml::window::Style::CLOSE,
            &Default::default()
        );

        window.set_framerate_limit(60);

        let cells_vbuffer = create_vbuffer(&g.cells, (w, h), (g.w as u32, g.h as u32));
        let grid_vbuffer = create_grid_vbuffer((w, h), (g.w as u32, g.h as u32));

        let cell_size = Rect::<f32> {
            left: 0.0,
            top: 0.0,
            width: w as f32 / g.w as f32,
            height: h as f32 / g.h as f32
        };

        Self {
            g,
            window,
            cells_vbuffer,
            cell_size,
            grid_vbuffer
        }
    }

    pub fn render_loop(&mut self) {
        let mut frame = 0;
        let mut pause = false;
        'running: loop {
            frame += 1;
            while let Some(e) = self.window.poll_event() {
                match e {
                    Event::Closed => break 'running,
                    Event::MouseButtonReleased { button, x, y } => {
                        match button {
                            Button::Left => {
                                // Convert screen coordinates to grid coordinates
                                let (gx, gy) = (
                                    (x as f32 / self.cell_size.width) as i32,
                                    (y as f32 / self.cell_size.height) as i32
                                );
                                let i = self.g.get_index(gx, gy);
                                // println!("{}, {}, {}, {}, {}, {}, {}", x, y, self.cell_size.width, self.cell_size.height, gx, gy, i);
                                self.g.cells[i].1 = !self.g.cells[i].0;
                            }
                            _ => ()
                        }
                    },
                    Event::KeyReleased { code, .. } => {
                        match code {
                            Key::Space => pause ^= true,
                            _ => ()
                        }
                    },
                    _ => ()
                }
            }

            if frame % 60 == 0 && !pause {
                self.g.step();
            }

            self.update_vbuffer();
            self.g.update_cells();
    
            self.window.clear(Color::WHITE);
            self.window.draw(&self.grid_vbuffer);
            self.window.draw(&self.cells_vbuffer);
            self.window.display();
        }
    }

    pub fn update_vbuffer(&mut self) { // Updates self.cells_vbuffer using self.g.cells, you MUST call self.g.update_cells AFTER calling this one
        for i in 0..self.g.cells.len() {
            let vi = i * 4;
            if self.g.cells[i].0 != self.g.cells[i].1 {
                if self.g.cells[i].1 {
                    self.cells_vbuffer[vi].color = Color::BLACK;
                    self.cells_vbuffer[vi + 1].color = Color::BLACK;
                    self.cells_vbuffer[vi + 2].color = Color::BLACK;
                    self.cells_vbuffer[vi + 3].color = Color::BLACK;
                } else {
                    self.cells_vbuffer[vi].color = Color::TRANSPARENT;
                    self.cells_vbuffer[vi + 2].color = Color::TRANSPARENT;
                    self.cells_vbuffer[vi + 1].color = Color::TRANSPARENT;
                    self.cells_vbuffer[vi + 3].color = Color::TRANSPARENT;
                }
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
    let c_w = screen_size.0 as f32 / grid_size.0 as f32;
    let c_h = screen_size.1 as f32 / grid_size.1 as f32;

    for i in 0..data.len() as u32 {
        let gx = (i % grid_size.0) as f32;
        let gy = (i / grid_size.0) as f32;

        let x = gx * c_w;
        let y = gy * c_h;
        let _i = (i * 4) as usize;

        // Setting up the 4 points of the quad representing cell at data[i]
        arr[_i].position = Vector2f::new(x, y);
        arr[_i + 1].position = Vector2f::new(x + c_w, y);
        arr[_i + 2].position = Vector2f::new(x + c_w, y + c_h);
        arr[_i + 3].position = Vector2f::new(x, y + c_h);
        
        let c = if data[i as usize].0 { // The cell is black if true
            Color::BLACK
        } else { // or white
            Color::TRANSPARENT
        };

        arr[_i].color = c;
        arr[_i + 1].color = c;
        arr[_i + 2].color = c;
        arr[_i + 3].color = c;

         // println!("{:?}, {:?}, {:?}, {:?}", arr[_i], arr[_i + 1], arr[_i + 2], arr[_i + 3]);
    }

    arr
}

// Creates and returns line vertex buffer representing a grid
fn create_grid_vbuffer(screen_size: (u32, u32), grid_size: (u32, u32)) -> VertexArray {
    let mut arr = VertexArray::new(sfml::graphics::PrimitiveType::Lines, ((grid_size.0 + 1) * (grid_size.1 + 1)) as usize);
    let c_w = screen_size.0 as f32 / grid_size.0 as f32;
    let c_h = screen_size.1 as f32 / grid_size.1 as f32;

    for y in 0..grid_size.1 + 1 {
        let _i = (y * 2) as usize;
        arr[_i].position = Vector2f::new(0.0, y as f32 * c_h);
        arr[_i + 1].position = Vector2f::new(screen_size.0 as f32, y as f32 * c_h);
    }

    for x in 0..grid_size.0 + 1 {
        let _i = ((grid_size.1) * 2 + x * 2) as usize;
        arr[_i].position = Vector2f::new(x as f32 * c_w, 0.0);
        arr[_i + 1].position = Vector2f::new(x as f32 * c_w, screen_size.1 as f32);
    }

    for i in 0..arr.vertex_count() {
        arr[i].color = Color::rgb(127, 127, 127);
    }

    arr
}