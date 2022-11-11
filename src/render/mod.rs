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
    //Just assigning every texture
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
    fn render_objects(&mut self) -> Result<(), String>;
}

impl<'a> Render for Game<'a> {
    fn render_objects(&mut self) -> Result<(), String> {
        let mut dst_rect = sdl2::rect::Rect::new(
            0,
            0,
            (64 as f32 + self.camera.zoom) as u32,
            (64 as f32 + self.camera.zoom) as u32,
        );

        //Max size = Tiles that can fit in the screen
        //We have to render beyond the screen a little.
        //We add extra tiles, so tiles on the edge of screen don't disappear.
        let max_size = (
            self.window_size.0 as i32 / dst_rect.w + 5,
            self.window_size.1 as i32 / dst_rect.h * 4 - 16,
        );

        //Cx and Cy = The render row and column that the camera is at (top left corner of screen)
        let cx = self.camera.position.y as i32 / dst_rect.h * 2;
        let cy = self.camera.position.x as i32 / dst_rect.w;

        //Cam offset = The tile coordinates of the tile that the camera is at (top left corner of screen)
        let cam_offset = (cx + cy, cx - cy);

        //Tile offset = The amount in pixels by what the render should be shifted by.
        //Camera offset shifts whole tiles, the tile offset shifts the render by amounts smaller than one tile.
        let tile_offset = (
            -self.camera.position.x as i32 % dst_rect.w,
            -self.camera.position.y as i32 % dst_rect.h,
        );

        //DEBUG
        //let mut rendered_tiles = 0;

        //Going trough every render row (i), and every render column(j)
        for i in 1..max_size.1 as i32 {
            for j in 0..max_size.0 as i32 {

                //Converts render coordinates into matrix cooerdinates
                let x = (i - 1) / 2 + 1 + j;
                let y = i / 2 - j;

                //Checks if tile is within map
                if x + cam_offset.0 > 0
                    && y + cam_offset.1 > 0
                    && x + cam_offset.0 < self.map.size as i32
                    && y + cam_offset.1 < self.map.size as i32
                {
                    if let Some(tile) =
                        self.map.matr[(y + cam_offset.1) as usize][(x + cam_offset.0) as usize]
                    {

                        //Going from min_z to max_z. +1 Because max_z equals the height of a tile, which we should include.
                        for z in tile.min_z..tile.max_z + 1 {

                            dst_rect.x = 
                                //Shifting the tile by one on X means we have to shift by half its width on screen.
                                x * dst_rect.w / 2

                                //Shifting the tile by one on Y means we have to shift by half its width on screen.
                                - y * dst_rect.w / 2 

                                //Adding the tile_offset to the screen coordinates (Line 58)
                                + tile_offset.0

                                //Shifting every tile by 2 to the left so everything we render is on screen.
                                //We have to do this so that tiles on the left of the screen don't disappear.
                                //On the right we still render beyond screen (Line 44)
                                - dst_rect.w * 2;


                            dst_rect.y =
                                //Shifting the tile by one on Y means we have to shift by half its height on screen.
                                //The top surface of a tile has n width and ( n / 2 ) height, so that is why we divide by 4.
                                y * dst_rect.h / 4

                                //Shifting the tile by one on X means we have to shift by half its height on screen.
                                + x * dst_rect.h / 4

                                //Shifting the tile by one up on Z means we have to shift by its height on screen.
                                - z as i32 * dst_rect.h / 2

                                //Adding the tile_offset to the screen coordinates (Line 58)
                                + tile_offset.1
                                
                                //Shifting every tile by 1 to the left so everything we render is on screen.
                                //We have to do this so that tiles on the top of the screen don't disappear.
                                //On the bottom we still render beyond screen (Line 44)
                                - dst_rect.h;
                            
                            //Rendering the tile on canvas
                            if let Some(texture) = tile.tile_type {
                                self.canvas
                                    .copy(texture, None, Some(dst_rect))?;
                            }

                            //DEBUG
                            //rendered_tiles += 1
                        }
                    }
                }
            }
        }

        //DEBUG
        //println!("Rendered tiles: {}", rendered_tiles);

        Ok(())
    }
}
