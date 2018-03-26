use graphics::Graphics;
use input::Input;
use player::Player;
use units::Milliseconds;

use sdl2::keyboard::Keycode;

pub struct Entities<'a> {
    player: Player<'a>,
}

impl<'a> Entities<'a> {
    pub fn new(graphics: &mut Graphics<'a>) -> Entities<'a> {
        Entities {
            player: Player::new(graphics, 320, 240),
        }
    }

    pub fn process_input(&mut self, input: &Input) {
        //  if both left and right pressed
        //    stop moving
        //  elif left
        //    move left
        //  elif right
        //    move right
        //  else
        //    stop moving
        match (
            input.key_held(Keycode::Left),
            input.key_held(Keycode::Right),
        ) {
            (true, true) => self.player.stop_moving(),
            (false, false) => self.player.stop_moving(),
            (true, false) => self.player.start_moving_left(),
            (false, true) => self.player.start_moving_right(),
        }

        match (
            input.key_held(Keycode::Up),
            input.key_held(Keycode::Down),
        ) {
            (true, true) => self.player.look_horizontal(),
            (false, false) => self.player.look_horizontal(),
            (true, false) => self.player.look_up(),
            (false, true) => self.player.look_down(),
        }

        if input.key_pressed(Keycode::Z) {
            self.player.start_jump();
        } else if input.key_released(Keycode::Z) {
            self.player.stop_jump();
        }
    }

    pub fn draw(&self, graphics: &mut Graphics) {
        graphics.clear();
        self.player.draw(graphics);
        graphics.present();
    }

    pub fn update(&mut self, elapsed_time: Milliseconds) {
        self.player.update(elapsed_time);
    }
}
