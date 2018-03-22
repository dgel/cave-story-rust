
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use graphics::Graphics;
use units::Milliseconds;
use constants;

pub trait Drawable {
    fn draw(&self, graphics: &mut Graphics, x: i32, y: i32);
}

pub trait Updatable {
    fn update(&mut self, elapsed_time: Milliseconds);
}

pub struct Sprite {
    sprite_sheet: Surface<'static>,
    source_rect: Rect,
}

impl Sprite {
    pub fn new(filename: &str, x: i32, y: i32, width: u32, height: u32) -> Sprite {
        let surface = Surface::load_bmp(filename).expect("Failed to load bitmap");
        let rectangle = Rect::new(x, y, width, height);
        Sprite {
            sprite_sheet: surface,
            source_rect: rectangle
        }
    }
}

impl Drawable for Sprite {
    fn draw(&self, graphics: &mut Graphics, x: i32, y: i32) {
        graphics.blit_surface(&self.sprite_sheet, self.source_rect, Rect::new(x, y, self.source_rect.width(), self.source_rect.height()));
    }
}

impl Updatable for Sprite {
    fn update(&mut self, _: Milliseconds) {
    }
}

pub struct AnimatedSprite {
    sprite_sheet: Surface<'static>,
    source_rect: Rect,
    frame_time: Milliseconds,
    elapsed_time: Milliseconds,
    num_frames: u32,
    current_frame: u32,
}

impl AnimatedSprite {
    pub fn new(filename: &str, x: i32, y: i32, width: u32, height: u32, fps: u64, num_frames: u32) -> AnimatedSprite {
        let surface = Surface::load_bmp(filename).expect("Failed to load bitmap");
        let rectangle = Rect::new(x, y, width, height);
        let frame_time = 1_000 / fps as u32;
        AnimatedSprite {
            sprite_sheet: surface,
            source_rect: rectangle,
            frame_time: Milliseconds::new(frame_time),
            elapsed_time: Milliseconds::new(0),
            num_frames: num_frames,
            current_frame: 0
        }
    }
}

impl Drawable for AnimatedSprite {
    fn draw(&self, graphics: &mut Graphics, x: i32, y: i32) {
        graphics.blit_surface(&self.sprite_sheet, self.source_rect, Rect::new(x, y, self.source_rect.width(), self.source_rect.height()));
    }
}

impl Updatable for AnimatedSprite {
    fn update(&mut self, elapsed_time: Milliseconds) {
        self.elapsed_time += elapsed_time;

        if self.elapsed_time > self.frame_time {
            self.current_frame += 1;
            self.elapsed_time -= self.frame_time;
            if self.current_frame < self.num_frames {
                let new_val =  self.source_rect.x() + constants::TILE_SIZE as i32;
                self.source_rect.set_x(new_val);
            } else {
                let new_val =  self.source_rect.x() - (constants::TILE_SIZE * (self.num_frames - 1) ) as i32;
                self.source_rect.set_x(new_val);
                self.current_frame = 0;
            }
        }
    }
}
