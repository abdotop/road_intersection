use std::collections::HashMap;

use engine::{Car, Road};
use sdl2::{pixels::Color, rect::Point};

// extern crate sdl2;
mod engine;
mod view;

// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
// use std::time::Duration;

fn main() {
    let mut roads: HashMap<String, Road> = HashMap::<String, Road>::new();

    // North -> South
    let n_s_1_road = Road::new(
        Point::new(350, 0),
        Point::new(350, 275),
        50,
        false,
        Color::RGB(255, 0, 0), // "start",
    );
    let n_s_2_road = Road::new(
        Point::new(350, 375),
        Point::new(350, 650),
        50,
        false,
        Color::RGB(0, 0, 255),
        // "end",
    );

    roads.insert("south-start".to_string(), n_s_1_road);
    roads.insert("south-end".to_string(), n_s_2_road);
    // South -> North
    let s_n_1_road = Road::new(
        Point::new(400, 0),
        Point::new(400, 275),
        50,
        true,
        Color::RGB(0, 0, 255), // "start",
    );
    let s_n_2_road = Road::new(
        Point::new(400, 375),
        Point::new(400, 650),
        50,
        true,
        Color::RGB(255, 0, 0),
        // "end",
    );

    roads.insert("north-start".to_string(), s_n_1_road);
    roads.insert("north-end".to_string(), s_n_2_road);

    // East -> West
    let e_w_1_road = Road::new(
        Point::new(0, 275),
        Point::new(350, 275),
        50,
        false,
        Color::RGB(0, 255, 0), // "start",
    );
    let e_w_2_road = Road::new(
        Point::new(450, 275),
        Point::new(800, 275),
        50,
        false,
        Color::RGB(0, 255, 0),
        // "end",
    );

    roads.insert("west-start".to_string(), e_w_1_road);
    roads.insert("west-end".to_string(), e_w_2_road);

    // West -> East
    let w_e_1_road = Road::new(
        Point::new(0, 325),
        Point::new(350, 325),
        50,
        false,
        Color::RGB(255, 255, 0), // "start",
    );
    let w_e_2_road = Road::new(
        Point::new(450, 325),
        Point::new(800, 325),
        50,
        false,
        Color::RGB(255, 255, 0), // "end",
    );

    roads.insert("east-start".to_string(), w_e_1_road);
    roads.insert("east-end".to_string(), w_e_2_road);

    let mut sumulation = engine::Engine::new(roads);
    sumulation.start()
    // let sdl_context = sdl2::init().unwrap();

    // /
}
