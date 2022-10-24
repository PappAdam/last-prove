pub mod camera;

use crate::game::Game;
use sdl2::{
    image::LoadTexture,
    render::{self, Texture},
    video::WindowContext,
};

macro_rules! loadtexture {
    ($texture_creator:expr, $path:expr ) => {
        $texture_creator.load_texture($path).unwrap()
    };
}

pub struct TileTextures<'a> {
    pub t0: Texture<'a>,
    pub t1_bl: Texture<'a>,
    pub t1_br: Texture<'a>,
    pub t1_tl: Texture<'a>,
    pub t1_tr: Texture<'a>,
    pub t2_bl_br: Texture<'a>,
    pub t2_bl_tr: Texture<'a>,
    pub t2_br_tr: Texture<'a>,
    pub t2_tl_bl: Texture<'a>,
    pub t2_tl_br: Texture<'a>,
    pub t2_tl_tr: Texture<'a>,
    pub t3_bl_br_tr: Texture<'a>,
    pub t3_tl_bl_br: Texture<'a>,
    pub t3_tl_bl_tr: Texture<'a>,
    pub t3_tl_br_tr: Texture<'a>,
    pub t4: Texture<'a>,
}


impl<'a> TileTextures<'a> {
    pub fn init(texture_creator: &'a render::TextureCreator<WindowContext>) -> Self {
        Self {
            t0: loadtexture!(texture_creator, "Assets/debug_tiles/0.png"),
            t1_bl: loadtexture!(texture_creator, "Assets/debug_tiles/1_bl.png"),
            t1_br: loadtexture!(texture_creator, "Assets/debug_tiles/1_br.png"),
            t1_tl: loadtexture!(texture_creator, "Assets/debug_tiles/1_tl.png"),
            t1_tr: loadtexture!(texture_creator, "Assets/debug_tiles/1_tr.png"),
            t2_bl_br: loadtexture!(texture_creator, "Assets/debug_tiles/2_bl_br.png"),
            t2_bl_tr: loadtexture!(texture_creator, "Assets/debug_tiles/2_bl_tr.png"),
            t2_br_tr: loadtexture!(texture_creator, "Assets/debug_tiles/2_br_tr.png"),
            t2_tl_bl: loadtexture!(texture_creator, "Assets/debug_tiles/2_tl_bl.png"),
            t2_tl_br: loadtexture!(texture_creator, "Assets/debug_tiles/2_tl_br.png"),
            t2_tl_tr: loadtexture!(texture_creator, "Assets/debug_tiles/2_tl_tr.png"),
            t3_bl_br_tr: loadtexture!(texture_creator, "Assets/debug_tiles/3_bl_br_tr.png"),
            t3_tl_bl_br: loadtexture!(texture_creator, "Assets/debug_tiles/3_tl_bl_br.png"),
            t3_tl_bl_tr: loadtexture!(texture_creator, "Assets/debug_tiles/3_tl_bl_tr.png"),
            t3_tl_br_tr: loadtexture!(texture_creator, "Assets/debug_tiles/3_tl_br_tr.png"),
            t4: loadtexture!(texture_creator, "Assets/debug_tiles/4.png"),
        }
    }
}

pub trait Render {
    fn render_objects(&mut self, textures: &TileTextures) -> Result<(), String>;
}

impl<'a> Render for Game<'a> {
    fn render_objects(&mut self, textures: &TileTextures) -> Result<(), String> {
        let mut dst_rect = sdl2::rect::Rect::new(
            0,
            0,
            (64 as f32 + self.camera.zoom) as u32,
            (64 as f32 + self.camera.zoom) as u32,
        );

        let max_size = (
            self.window_size.0 as i32 / dst_rect.w + 5,
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

        for i in 1..max_size.1 as i32 {
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
                            dst_rect.y = y * dst_rect.h / 4 + x * dst_rect.w / 4 - dst_rect.h
                                + tile_offset.1
                                - z as i32 * dst_rect.h / 2;

                            if let Some(texture) = tile.tile_type {
                                self.canvas
                                    .copy(texture, None, Some(dst_rect))?;
                            }
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
