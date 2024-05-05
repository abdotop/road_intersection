use rand::Rng;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::collections::HashMap;
use std::time::Duration;
// use rodio::Source;

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
        new_car.new_direction = self.random_direction(adjacent);
        match direction {
            "north-start" => {
                new_car.position.x = road_adjacent.end.x + 55;
                new_car.position.y = road_adjacent.end.y - 50;
            }
            "south-start" => {
                new_car.position.x = road_adjacent.start.x + 5;
                new_car.position.y = road_adjacent.start.y - 50;
            }
            "west-start" => {
                new_car.position.x = road_adjacent.start.x;
                new_car.position.y = road_adjacent.start.y + 5;
            }
            "east-start" => {
                new_car.position.x = road_adjacent.end.x;
                new_car.position.y = road_adjacent.end.y;
            }
            _ => {
                return;
            }
        }
        if !self.will_collide(new_car.position, &mut new_car, &self.cars) {
            self.cars.push(new_car.clone());
        }
        println!("Car added {:?} {:?}", new_car.direction, new_car.position);
    }

    fn move_cars(&mut self) {
        let mut cars_clone = self.cars.clone();
        for car in cars_clone.iter_mut() {
            let mut new_position = car.position.clone();
            match car.direction.as_str() {
                "south-end" => {
                    if new_position.y < 0 {
                        self.cars.retain(|c| c.id != car.id);
                    }
                    // println!("south-end {:?}", car.position);
                    new_position.y -= car.speed as i32;
                }
                "south-start" => {
                    new_position.y += car.speed as i32;
                }
                "west-end" => {
                    // println!("west-end {:?}", car.position);
                    new_position.x -= car.speed as i32;
                }
                "east-start" => {
                    // println!("east-end {:?}", car.position);
                    new_position.x += car.speed as i32;
                }
                _ => {}
            }
            if !self.will_collide(new_position, car, &self.cars) {
                car.move_car(new_position);
            }
            if !car.direction_change {
                self.change_direction(car);
            }
        }

        self.cars = cars_clone;
    }

    fn change_direction(&mut self, car: &mut Car) {
        match car.direction.as_str() {
            "south-end" => {
                if car.new_direction == "west-end" && car.position.y < 280 {
                    car.direction_change = true;
                    car.direction = car.new_direction.clone();
                    car.position.y = 280;
                } else if car.new_direction == "east-start" && car.position.y < 330 {
                    car.direction_change = true;
                    car.direction = car.new_direction.clone();
                    car.position.y = 330;
                    println!("east-start {:?}={}", car.position, car.new_direction);
                } else if car.new_direction == "south-end" {
                    println!("South-start {:?}={}", car.position, car.new_direction);
                    
                    car.direction_change = true;
                }
            }
            _ => {}
        }
    }

    pub fn random_direction(&self, current_direction: &str) -> String {
        let mut rng = rand::thread_rng();
        let new_direction = match current_direction {
            "south-start" => {
                let directions = ["west-start", "east-start", "south-end"];
                directions[rng.gen_range(0..3)].to_string()
            }
            "south-end" => {
                let directions = ["west-end", "east-start", "east-start",];
                directions[rng.gen_range(0..3)].to_string()
            }
            "west-start" => {
                let directions = ["south-start", "north-start", "west-end"];
                directions[rng.gen_range(0..3)].to_string()
            }
            "west-end" => {
                let directions = ["south-end", "north-end", "west-start"];
                directions[rng.gen_range(0..3)].to_string()
            }
            "east-start" => {
                let directions = ["south-start", "north-start", "east-end"];
                directions[rng.gen_range(0..3)].to_string()
            }
            "east-end" => {
                let directions = ["south-end", "north-end", "east-start"];
                directions[rng.gen_range(0..3)].to_string()
            }
            _ => current_direction.to_string(),
        };
        new_direction
    }

    fn will_collide(&self, new_position: Point, car: &mut Car, cars: &Vec<Car>) -> bool {
        for other_car in cars {
            if other_car.id != car.id
                && new_position.x < other_car.position.x + other_car.width as i32
                && new_position.x + car.width as i32 > other_car.position.x
                && new_position.y < other_car.position.y + other_car.heith as i32
                && new_position.y + car.heith as i32 > other_car.position.y
            {
                car.speed = other_car.speed;
                return true;
            }
        }
        false
    }

    // random new end direction for the car when it reaches the end of the road

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
