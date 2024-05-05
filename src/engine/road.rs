use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
// use std::collections::HashMap;

// use crate::view::Board;

#[derive(Clone)]
pub struct Road {
    pub start: Point,
    pub end: Point,
    size: u8,
    is_x: bool,
    pub color: Color,
}

impl Road {
    pub fn new(start: Point, end: Point, size: u8, is_x: bool, coleur: Color) -> Self {
        Self {
            start,
            end,
            size,
            is_x,
            color: coleur,
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas
            .draw_line(self.start, self.end)
            .ok()
            .unwrap_or_default();
        let (s2, e2) = if !self.is_x {
            (
                Point::new(self.start.x, self.start.y + self.size as i32),
                Point::new(
                    self.start.x + (self.end.x - self.start.x),
                    self.start.y + self.size as i32,
                ),
            )
        } else {
            (
                Point::new(self.start.x + self.size as i32, self.start.y),
                Point::new(
                    self.start.x + self.size as i32,
                    self.start.y + (self.end.y - self.start.y),
                ),
            )
        };
        canvas.draw_line(s2, e2).ok().unwrap_or_default();
    }
}

mod car {
    // use super::Road;
    use sdl2::libc::rand;
    use sdl2::pixels::Color;
    use sdl2::rect::Point;
    use sdl2::rect::Rect;
    use sdl2::render::Canvas;
    use sdl2::video::Window;

    #[derive(Clone)]
    pub struct Car {
        pub position: Point,
        pub heith: u8,
        pub width: u8,
        color: Color,
        pub speed: u8,
        // pub cars: Vec<car::Car>,
        pub direction: String,
        // pub road_start_or_end: String,
    }

    impl Car {
        pub fn new(position: Point, color: Color, direction: &str) -> Self {
            // random speed between 1 and 5 for each car
            let speed = unsafe { rand() % 5 + 1 };

            Self {
                position,
                heith: 40,
                width: 40,
                color,
                direction: direction.to_string(),
                speed: speed as u8,
            }
        }

        // pub fn can_move(&self, new_position: Point, cars: &mut Vec<Car>) -> bool {
        //     // if roat.is_x {
        //     //     if new_position.x < roat.start.x || new_position.x > roat.end.x {
        //     //         return false;
        //     //     }
        //     // } else {
        //     //     if new_position.y < roat.start.y || new_position.y > roat.end.y {
        //     //         return false;
        //     //     }
        //     // }
        //     // check if the vehicles to avoid crashing into each other.
        //     for car in cars.iter() {
        //         if roat.is_x {
        //             if new_position.x < car.position.x
        //                 || new_position.x > car.position.x + car.width as i32
        //             {
        //                 return false;
        //             }
        //         } else {
        //             if new_position.y < car.position.y
        //                 || new_position.y > car.position.y + car.heith as i32
        //             {
        //                 return false;
        //             }
        //         }
        //     }

        //     true
        // }

        pub fn move_car(&mut self, new_position: Point) {
            self.position = new_position;
        }

        pub fn draw(&self, canvas: &mut Canvas<Window>) {
            canvas.set_draw_color(self.color);
            canvas
                .fill_rect(Rect::new(
                    self.position.x,
                    self.position.y,
                    self.width as u32,
                    self.heith as u32,
                ))
                .ok()
                .unwrap_or_default();
        }
    }
}

pub use car::Car;
