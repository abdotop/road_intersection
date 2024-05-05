use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::collections::HashMap;
use std::time::Duration;

// use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub use self::road::*;

// use view::*;

#[derive(Clone)]
pub struct Engine {
    sdl_context: Option<sdl2::Sdl>,
    running: bool,
    roads: HashMap<String, Road>,
    cars: Vec<Car>,
}

impl Engine {
    pub fn new(roads: HashMap<String, Road>) -> Self {
        Self {
            sdl_context: Some(sdl2::init().unwrap()),
            running: false,
            roads,
            cars: Vec::<Car>::new(),
        }
    }

    fn game_loop(&mut self) {
        let mut board = crate::view::Board::new("Road Intersection Simulation", 650, 800);
        let mut canvas: Canvas<Window> = board.init(&self.sdl_context.as_ref().unwrap()).unwrap();
        let mut event_pump = self.sdl_context.as_ref().unwrap().event_pump().unwrap();
        let mut x = 0;

        while self.running {
            for event in event_pump.poll_iter() {
                self.command(event, &mut canvas);
            }

            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            for road in &mut self.roads {
                road.1.draw(&mut canvas);
            }

            self.draw_cars(&mut canvas);

            // canvas.set_draw_color(Color::RGB(255, 0, 0));
            // canvas.fill_rect(Rect::new(x, 280, 40, 40)).unwrap();
            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            // Déplacez le bloc à droite
            self.move_cars();
        }
    }

    fn command(&mut self, event: Event, _: &mut Canvas<Window>) {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => self.running = false,
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                self.add_car("north-start");
            }
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                self.add_car("south-start");
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => self.add_car("east-start"),
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => self.add_car("west-start"),
            _ => {}
        }
    }

    pub fn add_car(&mut self, direction: &str) {
        // let road = self.roads.get(direction).unwrap();
        let adjacent = match direction {
            "north-start" => "south-end",
            "south-start" => "south-start",
            "west-start" => "east-start",
            "east-start" => "west-end",
            _ => "",
        };
        let road_adjacent = self.roads.get(adjacent).unwrap();
        let mut new_car = Car::new(road_adjacent.start, road_adjacent.color, adjacent);
        match direction {
            "north-start" => {
                new_car.position.x = road_adjacent.end.x + 55;
                new_car.position.y = road_adjacent.end.y - 50;
            }
            "south-start" => {
                new_car.position.x = road_adjacent.start.x + 5;
                new_car.position.y = road_adjacent.start.y;
            }
            "west-start" => {
                new_car.position.x = road_adjacent.start.x;
                new_car.position.y = road_adjacent.start.y + 5;
            }
            "east-start" => {
                new_car.position.x = road_adjacent.end.x - 50;
                new_car.position.y = road_adjacent.end.y;
            }
            _ => {
                return;
            }
        }
        self.cars.push(new_car);
    }

    fn move_cars(&mut self) {
        for car in &mut self.cars {
            let mut new_position = car.position.clone();
            match car.direction.as_str() {
                "south-end" => {
                    new_position.y -= car.speed as i32;
                }
                "south-start" => {
                    new_position.y += car.speed as i32;
                }
                "west-end" => {
                    new_position.x -= car.speed as i32;
                }
                "east-start" => {
                    new_position.x += car.speed as i32;
                }
                _ => {}
            }

            car.move_car(new_position);
        }
    }

    pub fn draw_cars(&mut self, canvas: &mut Canvas<Window>) {
        for car in &mut self.cars {
            car.draw(canvas);
        }
    }

    pub fn start(&mut self) {
        self.running = true;
        self.game_loop()
    }
}

mod road;
