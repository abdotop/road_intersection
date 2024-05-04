// use sdl2::video::Window;
mod engine;

fn main() {
    let mut game = engine::Engine::new();
    game.start()
}
