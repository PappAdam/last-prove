use nalgebra_glm::{TVec2, TVec3, Vec2, Vec3};
use renderer::utils;

use super::{
    perlin,
    utils::{create_cube, Side},
    Map,
};

impl Map {
    pub fn generate(&mut self, seed: Option<u16>) {
        let perlin_noise = perlin::Perlin2D::new(match seed {
            None => rand::Rng::gen::<u16>(&mut rand::thread_rng()),
            Some(i) => i,
        });

        //The perlin noise value will be divided by this number
        //The result will be the height
        //The higher the self.height is, the lower this number gets, resulting in higher maps.
        let z_difference_for_height = 1.0 / self.height as f32;

        for y in 0..self.size as usize {
            for x in 0..self.size as usize {
                let tile_position =
                    Vec3::new(x as f32, 0., y as f32) / 16. - Vec3::new(1., 0.5, 1.);

                let perlin_value = perlin_noise.perlin2d(x as f32, y as f32, 0.05, 1);

                if perlin_value > 0.6 {
                    self.vertecies
                        .append(&mut create_cube(Side::CUBE, tile_position, 1. / 16.));
                }
            }
        }
        //Calculating minimum Z values for optimized render, than returning the result.
    }
}
