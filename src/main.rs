extern crate sdl2;

mod constants;
mod game;
use game::Game;
mod graphics;
mod sprite;
mod units;
mod input;
mod player;
mod entities;

fn main() {
    match Game::new() {
        Ok(mut game) => game.event_loop(),
        Err(error) => {
            println!("Could not initialize game: {}", error);
        }
    }
}
