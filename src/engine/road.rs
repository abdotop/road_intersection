use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

// use crate::view::Board;

#[derive(Clone)]
pub struct Road {
    start: Point,
    end: Point,
    size: u8,
    is_x: bool,
}

impl Road {
    pub fn new(start: Point, end: Point, size: u8, is_x: bool) -> Self {
        Self {
            start,
            end,
            size,
            is_x,
        }
    }

    // pub fn add_car(&mut self, car: Car) {
    //     self.cars.push(car);
    // }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
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
