use sdl2;
use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::rect::Rect;
use std::error::Error;
use constants;

pub struct Graphics {
    canvas: sdl2::render::WindowCanvas,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>
}

impl Graphics {
    pub fn new(context: &sdl2::Sdl) -> Result<Self, String> {
        let window = context.video()?.window("Cave Story", constants::SCREEN_WIDTH, constants::SCREEN_HEIGHT).fullscreen().build().map_err(|e| e.description().to_owned())?;
        let mut canvas = window.into_canvas().build().map_err(|e| e.description().to_owned())?;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        context.mouse().show_cursor(false);

        let texture_creator = canvas.texture_creator();

        Ok(Graphics{
            canvas: canvas,
            texture_creator: texture_creator
        })
    }

    pub fn blit_surface<'a>(&mut self, surface: &Surface<'a>, source: Rect, destination: Rect) {
        if let Ok(texture) = self.texture_creator.create_texture_from_surface(surface) {
            if let Err(error) = self.canvas.copy(&texture, source, destination) {
                println!("error copying texture: {}", error);
            }
        } else {
            println!("couldn't load texture!");
        }

    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
