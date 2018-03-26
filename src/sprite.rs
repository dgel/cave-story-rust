use sdl2::rect::Rect;
use sdl2::render::Texture;
use graphics::Graphics;
use units::Milliseconds;
use constants;

use std::rc::Rc;

pub trait Drawable {
    fn draw(&self, graphics: &mut Graphics, x: i32, y: i32);
}

pub trait Updatable {
    fn update(&mut self, elapsed_time: Milliseconds);
}

pub trait UpdateAndDrawable: Updatable + Drawable {}
impl<T> UpdateAndDrawable for T
where
    T: Updatable + Drawable,
{
}

pub struct Sprite<'a> {
    sprite_sheet: Rc<Texture<'a>>,
    source_rect: Rect,
}

impl<'a> Sprite<'a> {
    pub fn new(graphics: &mut Graphics<'a>, filename: &str, x: i32, y: i32, width: u32, height: u32) -> Sprite<'a> {
        Sprite {
            sprite_sheet: graphics.load_image(filename),
            source_rect: Rect::new(x, y, width, height),
        }
    }
}

impl<'a> Drawable for Sprite<'a> {
    fn draw(&self, graphics: &mut Graphics, x: i32, y: i32) {
        graphics.blit_surface(
            &self.sprite_sheet,
            self.source_rect,
            Rect::new(x, y, self.source_rect.width(), self.source_rect.height()),
        );
    }
}

impl<'a> Updatable for Sprite<'a> {
    fn update(&mut self, _: Milliseconds) {}
}

pub struct AnimatedSprite<'a> {
    sprite_sheet: Rc<Texture<'a>>,
    source_rect: Rect,
    frame_time: Milliseconds,
    elapsed_time: Milliseconds,
    num_frames: u32,
    current_frame: u32,
}

impl<'a> AnimatedSprite<'a> {
    pub fn new(
        graphics: &mut Graphics<'a>,
        filename: &str,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fps: u64,
        num_frames: u32,
    ) -> AnimatedSprite<'a> {
        let frame_time = 1_000 / fps as u32;
        AnimatedSprite {
            sprite_sheet: graphics.load_image(filename),
            source_rect: Rect::new(x, y, width, height),
            frame_time: Milliseconds::new(frame_time),
            elapsed_time: Milliseconds::new(0),
            num_frames: num_frames,
            current_frame: 0,
        }
    }
}

impl<'a> Drawable for AnimatedSprite<'a> {
    fn draw(&self, graphics: &mut Graphics, x: i32, y: i32) {
        graphics.blit_surface(
            &self.sprite_sheet,
            self.source_rect,
            Rect::new(x, y, self.source_rect.width(), self.source_rect.height()),
        );
    }
}

impl<'a> Updatable for AnimatedSprite<'a> {
    fn update(&mut self, elapsed_time: Milliseconds) {
        self.elapsed_time += elapsed_time;

        if self.elapsed_time > self.frame_time {
            self.current_frame += 1;
            self.elapsed_time -= self.frame_time;
            if self.current_frame < self.num_frames {
                let new_val = self.source_rect.x() + constants::TILE_SIZE as i32;
                self.source_rect.set_x(new_val);
            } else {
                let new_val =
                    self.source_rect.x() - (constants::TILE_SIZE * (self.num_frames - 1)) as i32;
                self.source_rect.set_x(new_val);
                self.current_frame = 0;
            }
        }
    }
}
