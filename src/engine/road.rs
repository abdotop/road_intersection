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
    use rand::random;
    use sdl2::libc::rand;
    use sdl2::pixels::Color;
    use sdl2::rect::Point;
    use sdl2::rect::Rect;
    use sdl2::render::Canvas;
    use sdl2::video::Window;

    #[derive(Clone)]
    pub struct Car {
        pub position: Point,
        pub height: u8,
        pub width: u8,
        color: Color,
        pub speed: u8,
        pub id: u32,
        pub direction: String,
        pub direction_change: bool,
        pub new_direction: String,
    }

    impl Car {
        pub fn new(position: Point, color: Color, direction: &str) -> Self {
            // random speed between 1 and 5 for each car
            let speed = unsafe { rand() % 5 + 1 };
            let id: u32 = random();

            Self {
                position,
                height: 40,
                width: 40,
                color,
                direction: direction.to_string(),
                speed: speed as u8,
                id,
                direction_change: false,
                new_direction: "".to_string(),
            }
        }

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
                    self.height as u32,
                ))
                .ok()
                .unwrap_or_default();
        }
    }
}

mod traffic_lights {
    use sdl2::pixels::Color;
    use sdl2::rect::Point;
    use sdl2::rect::Rect;
    use sdl2::render::Canvas;
    use sdl2::video::Window;

    #[derive(Clone)]
    pub struct TrafficLight {
        pub position: Point,
        radius: u32,
        color: Color,
        pub is_on: bool,
        pub timer: u32,
    }

    impl TrafficLight {
        pub fn new(position: Point) -> Self {
            Self {
                position,
                radius: 5,
                color: Color::RGB(0, 255, 0),
                is_on: false,
                timer: 0,
            }
        }

        pub fn turn_on(&mut self) {
            self.is_on = true;
            self.color = Color::RGB(0, 255, 0);
        }

        pub fn turn_off(&mut self) {
            self.is_on = false;
            self.color = Color::RGB(255, 0, 0);
        }

        pub fn draw(&self, canvas: &mut Canvas<Window>) {
            canvas.set_draw_color(self.color);
            canvas
                .draw_rect(Rect::new(
                    self.position.x - (self.radius as i32 / 2),
                    self.position.y - (self.radius as i32 / 2),
                    self.radius,
                    self.radius,
                ))
                .unwrap();
            for angle in 0..360 {
                let radian = (angle as f64).to_radians();
                let x = self.position.x + (radian.cos() * self.radius as f64) as i32;
                let y = self.position.y + (radian.sin() * self.radius as f64) as i32;
                canvas.draw_point(Point::new(x, y)).unwrap();
            }
        }
    }
}

pub use car::Car;
pub use traffic_lights::TrafficLight;
