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

        for y in 0..self.size as usize {
            for x in 0..self.size as usize {
                let tile_position = Vec3::new(x as f32, 0., y as f32) / 32. - Vec3::new(6., 0., 6.);

                let perlin_value = perlin_noise.perlin2d(x as f32, y as f32, 0.03, 2);

                if perlin_value > 0.5 {
                    self.vertecies.append(&mut create_cube(
                        Side::CUBE,
                        tile_position,
                        1. / 32.,
                        perlin_value * self.size as f32 / 10. - self.size as f32 / 50. - 2.,
                        Vec3::new(
                            0.09 - perlin_value / 5.,
                            0.3 - perlin_value / 4. + 0.1,
                            0.04 - perlin_value / 5.,
                        ),
                    ));
                } else {
                    self.vertecies.append(&mut create_cube(
                        Side::CUBE,
                        tile_position,
                        1. / 32.,
                        perlin_value * self.size as f32 / 80. + self.size as f32 / 42. - 2.,
                        Vec3::new(0.01, 0.1, 0.4),
                    ));
                }
            }
        }
        //Calculating minimum Z values for optimized render, than returning the result.
    }
}
