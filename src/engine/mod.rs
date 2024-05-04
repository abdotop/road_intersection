use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
mod view;
// use view::*;

#[derive(Clone)]
pub struct Engine {
    sdl: Option<sdl2::Sdl>,
    running: bool,
    // board:view::Board,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            sdl: Some(sdl2::init().unwrap()),
            running: false,
        }
    }

    fn game_loop(&self) {
        // let sdl = self.sdl.
        let video_subsystem = self.sdl.as_ref().unwrap().video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
        let mut event_pump = self.sdl.as_ref().unwrap().event_pump().unwrap();
        let mut i = 0;
        'running: loop {
            i = (i + 1) % 255;
            canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            // The rest of the game loop goes here...

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    pub fn start(&mut self) {
        self.running = true;
        self.game_loop()
    }
}
