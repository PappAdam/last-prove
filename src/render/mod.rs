pub mod camera;

use crate::{game::Game};
use sdl2::{
    image::LoadTexture,
    render::{self, Texture},
    video::WindowContext,
};

pub struct TileTextures<'a> {
    base_texture: Texture<'a>,
}

impl<'a> TileTextures<'a> {
    pub fn init(texture_creator: &'a render::TextureCreator<WindowContext>) -> Self {
        let texture = texture_creator.load_texture("Assets/base.png").unwrap();
        Self {
            base_texture: texture,
        }
    }
}

pub trait Render {
    fn render_objects(&mut self, textures: &TileTextures) -> Result<(), String>;
}

//                        dstR.x = x*dstR.w/2 - y*dstR.h/2 + cam->xoffset;
//                        dstR.y = y*dstR.h/4 + x*dstR.w/4 - z*dstR.h/2 + cam->yoffset;

impl Render for Game {
    fn render_objects(&mut self, textures: &TileTextures) -> Result<(), String> {
        let mut dst_rect = sdl2::rect::Rect::new(0, 0, (64 as f32 * self.camera.zoom) as u32, (64 as f32 * self.camera.zoom) as u32);
        for y in 0..self.map.size as i32 {
            for x in 0..self.map.size as i32 {
                if let Some(tile) = self.map.matr[y as usize][x as usize] {
                    for z in 0..(tile.height + 1) {
                        dst_rect.x = x * dst_rect.w / 2 - y * dst_rect.h / 2 - self.camera.position.x as i32;
                        dst_rect.y = y * dst_rect.h / 4 + x * dst_rect.w / 4 - z as i32 * dst_rect.h / 2 - self.camera.position.y as i32;
                        self.canvas
                            .copy(&textures.base_texture, None, Some(dst_rect))?;
                    }
                }
            }
        }
        Ok(())
    }
}
