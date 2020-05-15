use crate::grid;

use sdl2;
use sdl2::event::Event;

pub struct Renderer {
    pub g: grid::Grid,
    pub w: u32,
    pub h: u32,
    sdl: sdl2::Sdl,
    canvas: sdl2::render::WindowCanvas,
    events: sdl2::EventPump,
    c_size: sdl2::rect::Rect
}

impl Renderer {
    // Initiates a SDL and sets up a new window to render the grid on
    pub fn new(name: &str, w: u32, h: u32, g: grid::Grid) -> Self {
        // SDL setup
        let sdl = sdl2::init().expect("Could not init SDL2");
        let _video_subs = sdl.video().expect("Could not init Video");
        let canvas = _video_subs.window(name, w, h)
            .build().unwrap()
            .into_canvas().build().unwrap();
        let events = sdl.event_pump().unwrap();

        // calculate size of the cells on screen
        let c_size = sdl2::rect::Rect::new(0, 0, w / g.w as u32, h / g.h as u32);
        
        Self {
            g,
            w,
            h,
            sdl,
            canvas,
            events,
            c_size
        }
    }

    // Temporary test function, will just wait for a quit event and exit
    pub fn wait(&mut self) {
        'running: loop {
            for e in self.events.poll_iter() {
                match e {
                    Event::Quit {..} => break 'running,
                    _ => ()
                }
            }
        }
    }
}