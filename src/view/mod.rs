use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

#[derive(Clone)]
pub struct Board {
    title: String,
    pub height: u32,
    pub width: u32,
}

impl Board {
    pub fn new(title: &str, height: u32, width: u32) -> Self {
        Self {
            title: title.to_string(),
            height,
            width,
        }
    }

    pub fn init(&mut self, sdl_context: &Sdl) -> Result<Canvas<Window>, String> {
        let video_subsystem = Some(sdl_context.video()?);
        let window = video_subsystem
            .as_ref()
            .unwrap()
            .window(&self.title, self.width, self.height)
            .position_centered()
            .build()
            .unwrap();
        Ok(window.into_canvas().build().unwrap())
    }
}
