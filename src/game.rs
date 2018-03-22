use constants;
use graphics::Graphics;
use sdl2;
use sprite::{Drawable, Updatable, AnimatedSprite};
use std::time::{Duration, Instant};
use units::Milliseconds;


pub struct Game {
    context: sdl2::Sdl,
    event_pump: sdl2::EventPump,
    sprite: AnimatedSprite,
}

impl Game {
    pub fn new() -> Result<Game, String> {
        let context = sdl2::init()?;
        let mut event_pump = context.event_pump()?;
        event_pump.disable_event(sdl2::event::EventType::MouseMotion);
        event_pump.disable_event(sdl2::event::EventType::Window);

        Ok(Game {
            context: context,
            event_pump: event_pump,
            sprite: AnimatedSprite::new("content/MyChar.bmp", 0, 0, constants::TILE_SIZE, constants::TILE_SIZE, 10, 3),
        })
    }

    fn update(&mut self, elapsed_time: Milliseconds) {
        self.sprite.update(elapsed_time);
    }

    fn draw(&self, graphics: &mut Graphics) {
        graphics.clear();
        self.sprite.draw(graphics, 320, 240);
        graphics.present();
    }


    pub fn event_loop(&mut self) {
        match Graphics::new(&self.context) {
            Ok(mut graphics) => {

                // target duration for one frame
                // A bit lower than actually needed to provide some wriggle room for thread::sleep
                let target_duration = Duration::new(0, 1_000_000_000 / constants::FPS);

                let mut running = true;
                let mut start_time = ::std::time::Instant::now();
                let mut last_update_time = start_time;
                while running {

                    // handle input
                    for event in self.event_pump.poll_iter() {
                        use sdl2::event::Event;
                        match event {
                            Event::Quit {..} => { running = false; }
                            Event::KeyDown { keycode: Some(code), .. } => {
                                if code == sdl2::keyboard::Keycode::Escape {
                                    running = false;
                                }
                            }
                            _ => (),
                        }
                    }

                    // handle timer callbacks

                    // update. move player, projectiles, check collisions
                    let current_time = Instant::now();
                    self.update(Milliseconds::from_duration(current_time - last_update_time));
                    last_update_time = current_time;

                    // draw EVERYTHING
                    self.draw(&mut graphics);

                    let frame_end = sync_duration(start_time, target_duration);

                    // println!("fps: {:.4}", 1_000_000_000.0 / (frame_end - start_time).subsec_nanos() as f64);
                    start_time = frame_end;
                }
            }
            Err(error) => {
                println!("Could not initialize graphics: {}", error);
            }
        }

    }
}

fn sync_duration(frame_start: Instant, target_duration: Duration) -> Instant {
    let approximate_duration = target_duration - Duration::new(0, 1_200_000);
    let mut current_time = Instant::now();
    let elapsed_time = current_time - frame_start;
    if elapsed_time < approximate_duration {
        ::std::thread::sleep(approximate_duration - elapsed_time);
        // busy wait
        current_time = Instant::now();
        let target = frame_start + target_duration - Duration::new(0, 10_000);
        while current_time < target {
            // sleep minimum time
            ::std::thread::sleep(Duration::new(0, 10));
            current_time = Instant::now();
        }
    } else {
        println!("overshot frame target. Duration: {:?}", elapsed_time - target_duration);
    }
    current_time
}
