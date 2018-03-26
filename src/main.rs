extern crate sdl2;

mod constants;
mod game;
use game::Game;
mod entities;
mod graphics;
mod input;
mod player;
mod sprite;
mod units;

fn main() {
    match Game::new() {
        Ok(mut game) => game.event_loop(),
        Err(error) => {
            println!("Could not initialize game: {}", error);
        }
    }
}
