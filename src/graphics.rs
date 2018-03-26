use constants;
use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::surface::Surface;
use std::collections::BTreeMap;
use std::error::Error;
use std::rc::Rc;

pub struct Graphics<'a> {
    canvas: &'a mut sdl2::render::WindowCanvas,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    texture_cache: BTreeMap<String, Rc<sdl2::render::Texture<'a>>>,
}

impl<'a> Graphics<'a> {
    pub fn load_canvas(context: &sdl2::Sdl) -> Result<sdl2::render::WindowCanvas, String> {
        let window = context
            .video()?
            .window(
                "Cave Story",
                constants::SCREEN_WIDTH,
                constants::SCREEN_HEIGHT,
            )
            //.fullscreen()
            .build()
            .map_err(|e| e.description().to_owned())?;
        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.description().to_owned())?;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        context.mouse().show_cursor(false);

        Ok(canvas)
    }

    pub fn new(
        canvas: &'a mut sdl2::render::WindowCanvas,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) -> Graphics<'a> {
        Graphics {
            canvas: canvas,
            texture_creator: texture_creator,
            texture_cache: BTreeMap::new(),
        }
    }

    pub fn load_image(&mut self, filename: &str) -> Rc<Texture<'a>> {
        let texture_creator = &self.texture_creator;
        Rc::clone(
            self.texture_cache
                .entry(filename.to_string())
                .or_insert_with(|| {
                    let surface = Surface::load_bmp(filename).expect("Failed to load bitmap");
                    let texture = texture_creator
                        .create_texture_from_surface(surface)
                        .expect("Failed to create texture from surface");
                    Rc::new(texture)
                }),
        )
    }

    pub fn blit_surface(&mut self, texture: &Texture, source: Rect, destination: Rect) {
        if let Err(error) = self.canvas.copy(texture, source, destination) {
            println!("error copying texture: {}", error);
        }
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
