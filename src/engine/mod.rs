use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub use self::road::*;

// use view::*;

#[derive(Clone)]
pub struct Engine {
    sdl_context: Option<sdl2::Sdl>,
    running: bool,
    roads: Vec<Road>,
}

impl Engine {
    pub fn new(roads: Vec<Road>) -> Self {
        Self {
            sdl_context: Some(sdl2::init().unwrap()),
            running: false,
            roads,
        }
    }

    fn game_loop(&mut self) {
        // let sdl = self.sdl.
        let mut board = crate::view::Board::new("Road Intersection Simulation", 650, 800);

        let mut canvas: Canvas<Window> = board.init(&self.sdl_context.as_ref().unwrap()).unwrap();
        let mut event_pump = self.sdl_context.as_ref().unwrap().event_pump().unwrap();

        let mut x = 0;

        'running: loop {
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
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            self.roads
            .iter()
            .for_each(|road| road.draw(&mut canvas));
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            canvas.fill_rect(Rect::new(x, 280, 40, 40)).unwrap();
            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            // Déplacez le bloc à droite
            x += 1;
            // Si le bloc atteint la fin de la fenêtre, réinitialisez sa position
            if x > 800 {
                x = 0;
            }
        }
    }

    pub fn start(&mut self) {
        self.running = true;
        self.game_loop()
    }
}

mod road;
