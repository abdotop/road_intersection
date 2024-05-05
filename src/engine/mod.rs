use rand::Rng;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::collections::HashMap;
use std::time::Duration;
// use sdl2::pixels::Color;

use sdl2::render::Canvas;
use sdl2::video::Window;

pub use self::road::*;

#[derive(Clone)]
pub struct Engine {
    sdl_context: Option<sdl2::Sdl>,
    running: bool,
    roads: HashMap<String, Road>,
    cars: Vec<Car>,
    traffic_lights: HashMap<String, TrafficLight>,
}

impl Engine {
    pub fn new(roads: HashMap<String, Road>) -> Self {
        let mut traffic_lights = HashMap::<String, TrafficLight>::new();
        traffic_lights.insert("south-start".to_string(), TrafficLight::new(Point::new(342, 260)));
        traffic_lights.insert("west-end".to_string(), TrafficLight::new(Point::new(465, 268)));
        traffic_lights.insert("south-end".to_string(), TrafficLight::new(Point::new(458, 390)));
        traffic_lights.insert("east-start".to_string(), TrafficLight::new(Point::new(335, 383)));
        // TrafficLight::new(Point::new(342, 260)).draw(&mut canvas);
        //    TrafficLight::new(Point::new(465, 268)).draw(&mut canvas);
        //    TrafficLight::new(Point::new(458, 390)).draw(&mut canvas);
        //    TrafficLight::new(Point::new(335, 383)).draw(&mut canvas);
        Self {
            sdl_context: Some(sdl2::init().unwrap()),
            running: false,
            roads,
            cars: Vec::<Car>::new(),
            traffic_lights,
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

            self.traffic_lcontrol();
            self.remove_cars();

            for traffic_light in &mut self.traffic_lights {
                traffic_light.1.draw(&mut canvas);
            }

            self.draw_cars(&mut canvas);

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            // Déplacez le bloc à droite
            self.move_cars();
        }
    }

    fn traffic_lcontrol(&mut self) {
        for (road_name, traffic_light) in &mut self.traffic_lights {
            // Increment the timer
            traffic_light.timer += 1;

            // Get the number of cars on this road
            let car_count = self
                .cars
                .iter()
                .filter(|car| car.direction == *road_name)
                .count();

            // If the timer has reached the limit...
            if traffic_light.timer
                >= if traffic_light.is_on {
                    // If the light is green, the limit is higher for roads with more cars
                    60 * (car_count as u32 + 1)
                } else {
                    // If the light is red, the limit is lower for roads with more cars
                    60 * (5u32.saturating_sub(car_count as u32)).max(1)
                }
            {
                // Reset the timer
                traffic_light.timer = 0;

                // Switch the traffic light color
                match traffic_light.is_on {
                    true => traffic_light.turn_off(),
                    false => traffic_light.turn_on(),
                };
            }
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
                new_car.position.y = road_adjacent.end.y + 5;
            }
            _ => {
                return;
            }
        }
        if !self.will_collide(new_car.position, &mut new_car, &self.cars) {
            self.cars.push(new_car.clone());
        }
        println!("{:?},new car", new_car.direction);
    }

    fn move_cars(&mut self) {
        let mut cars_clone = self.cars.clone();
        for car in cars_clone.iter_mut() {
            let mut new_position = car.position.clone();
            match car.direction.as_str() {
                "south-end" => {
                    if self.traffic_lights.get("south-end").unwrap().is_on || new_position.y < 380 || new_position.y > 390{
                        new_position.y -= car.speed as i32;
                    }
                }
                "south-start" => {
                    if self.traffic_lights.get("south-start").unwrap().is_on || new_position.y <260 || new_position.y > 270{
                        new_position.y += car.speed as i32;
                    }
                }
                "west-end" => {
                    if self.traffic_lights.get("west-end").unwrap().is_on || new_position.x < 455 || new_position.x > 465 {
                        new_position.x -= car.speed as i32;
                        
                    }
                }
                "east-start" => {
                    if self.traffic_lights.get("east-start").unwrap().is_on || new_position.x < 335 || new_position.x > 345 {
                        new_position.x += car.speed as i32;
                    }
                }
                _ => {}
            }
            if !self.will_collide(new_position, car, &self.cars) {
                car.move_car(new_position);
                // println!("{:?},car", car.position);
            }
            if !car.direction_change {
                self.change_direction(car);
            }
        }

        self.cars = cars_clone;
    }

    // fn remove_cars remove car over the road
    fn remove_cars(&mut self) {
        self.cars.retain(|car| {
            match car.direction.as_str() {
                "south-end" => car.position.y >= -50,
                "south-start" => car.position.y <= 700,
                "west-end" => car.position.x >= -50,
                "east-start" => car.position.x <= 850,
                _ => true,
            }
        });
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
                } else if car.new_direction == "south-end" {
                    car.direction_change = true;
                }
            }
            "south-start" => {
                if car.new_direction == "east-start" && car.position.y > 330 {
                    car.direction_change = true;
                    car.direction = car.new_direction.clone();
                    car.position.y = 330;
                } else if car.new_direction == "west-end" && car.position.y > 280 {
                    car.direction_change = true;
                    car.direction = car.new_direction.clone();
                    car.position.y = 280;
                } else if car.new_direction == "south-start" {
                    car.direction_change = true;
                }
            }
            "west-end" => {
                if car.new_direction == "south-start" && car.position.x < 355 {
                    car.direction_change = true;
                    car.direction = car.new_direction.clone();
                    car.position.x = 355;
                } else if car.new_direction == "south-end" && car.position.x < 405 {
                    car.direction = car.new_direction.clone();
                    car.direction_change = true;
                    car.position.x = 405;
                } else if car.new_direction == "west-end" {
                    car.direction_change = true;
                }
            }
            "east-start" => {
                if car.new_direction == "south-end" && car.position.x > 405 {
                    car.direction_change = true;
                    car.direction = car.new_direction.clone();
                    car.position.x = 405;
                } else if car.new_direction == "south-start" && car.position.x > 355 {
                    car.direction_change = true;
                    car.direction = car.new_direction.clone();
                    car.position.x = 355;
                } else if car.new_direction == "east-start" {
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
                let directions = ["east-start", "west-end", "south-start"];
                directions[rng.gen_range(0..3)].to_string()
            }
            "south-end" => {
                let directions = ["west-end", "south-end", "east-start"];
                directions[rng.gen_range(0..3)].to_string()
            }
            "west-end" => {
                let directions = ["south-start", "south-end", "west-end"];
                directions[rng.gen_range(0..3)].to_string()
            }
            "east-start" => {
                let directions = ["south-start", "south-end", "east-end"];
                directions[rng.gen_range(0..3)].to_string()
            }
            _ => current_direction.to_string(),
        };
        new_direction
    }

    fn will_collide(&self, new_position: Point, car: &mut Car, cars: &Vec<Car>) -> bool {
        let safety_distance: i32 = 5;
        for other_car in cars {
            if other_car.id != car.id
                && new_position.x < other_car.position.x + other_car.width as i32 + safety_distance
                && new_position.x + car.width as i32 + safety_distance > other_car.position.x
                && new_position.y < other_car.position.y + other_car.height as i32 + safety_distance
                && new_position.y + car.height as i32 + safety_distance > other_car.position.y
            {
                car.speed = other_car.speed;
                return true;
            }
        }
        false
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
