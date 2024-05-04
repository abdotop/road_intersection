use engine::Road;
use sdl2::rect::Point;

// extern crate sdl2;
mod engine;
mod view;

// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
// use std::time::Duration;

fn main() {
    let mut roads = Vec::<Road>::new();

    // North -> South
    let n_s_1_road = Road::new(Point::new(350, 0), Point::new(350, 275), 50, false);
    let n_s_2_road = Road::new(Point::new(350, 375), Point::new(350, 650), 50, false);

    roads.push(n_s_1_road);
    roads.push(n_s_2_road);
    // South -> North
    let s_n_1_road = Road::new(Point::new(400, 0), Point::new(400, 275), 50, true);
    let s_n_2_road = Road::new(Point::new(400, 375), Point::new(400, 650), 50, true);

    roads.push(s_n_1_road);
    roads.push(s_n_2_road);

    // East -> West
    let e_w_1_road = Road::new(Point::new(0, 275), Point::new(350, 275), 50, false);
    let e_w_2_road = Road::new(Point::new(450, 275), Point::new(800, 275), 50, false); // Second road

    roads.push(e_w_1_road);
    roads.push(e_w_2_road);

    // West -> East
    let w_e_1_road = Road::new(Point::new(0, 325), Point::new(350, 325), 50, false);
    let w_e_2_road = Road::new(Point::new(450, 325), Point::new(800, 325), 50, false); // Second road

    roads.push(w_e_1_road);
    roads.push(w_e_2_road);

    let mut sumulation = engine::Engine::new(roads);
    sumulation.start()
    // let sdl_context = sdl2::init().unwrap();

    // /
}
