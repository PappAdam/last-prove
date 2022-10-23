pub mod camera;

use crate::game::Game;
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
        let mut dst_rect = sdl2::rect::Rect::new(
            0,
            0,
            (64 as f32 + self.camera.zoom) as u32,
            (64 as f32 + self.camera.zoom) as u32,
        );

        let max_size = (
            self.window_size.0 as i32 / dst_rect.w + 4,
            self.window_size.1 as i32 / dst_rect.h * 4 + 16,
        );

        let (cam_offset, tile_offset) = {
            let (cx, cy) = (
                self.camera.position.y as i32 / dst_rect.h * 2,
                self.camera.position.x as i32 / dst_rect.w,
            );

            (
                (cx + cy, cx - cy),
                (
                    -self.camera.position.x as i32 % dst_rect.w,
                    -self.camera.position.y as i32 % dst_rect.h,
                ),
            )
        };

        //let mut rendered_tiles = 0;
        for i in 0..max_size.1 as i32 {
            for j in 0..max_size.0 as i32 {
                let x = (i - 1) / 2 + 1 + j;
                let y = i / 2 - j;

                if x + cam_offset.0 > 0
                    && y + cam_offset.1 > 0
                    && x + cam_offset.0 < self.map.size as i32
                    && y + cam_offset.1 < self.map.size as i32
                {
                    if let Some(tile) =
                        self.map.matr[(y + cam_offset.1) as usize][(x + cam_offset.0) as usize]
                    {
                        for z in tile.min_z..tile.max_z + 1 {
                            dst_rect.x = x * dst_rect.w / 2 - y * dst_rect.h / 2 - dst_rect.w * 2
                                + tile_offset.0;
                            dst_rect.y = y * dst_rect.h / 4 + x * dst_rect.w / 4 - dst_rect.h / 2
                                + tile_offset.1
                                - z as i32 * dst_rect.h / 2;
                            self.canvas
                                .copy(&textures.base_texture, None, Some(dst_rect))?;
                            //rendered_tiles += 1
                        }
                    }
                }
            }
        }
        //println!("Rendered tiles: {}", rendered_tiles);
        Ok(())
    }
}
