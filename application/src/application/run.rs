use std::f32::consts::PI;

use nalgebra::Vector3;
use objects::{mesh::Mesh, transformations::Transformations, ObjectType};

use super::App;

impl<'a> App<'a> {
    #[inline]
    pub fn main_loop(&mut self) {
        self.gameobjects[0].render(&self.renderer);
        self.gameobjects[1].render(&self.renderer);

        self.game_controller
            .add_time_elapsed(self.delta_time.as_secs_f32(), &mut self.renderer);
    }

    pub fn setup(&mut self, meshes: &'a mut Vec<Mesh>) {
        self.load_mesh("resources/models/Basic_house", meshes);
        // self.load_mesh("resources/models/az", meshes);
        self.create_obj(ObjectType::SomeObject, &meshes[0]);
        self.create_obj(ObjectType::SomeObject, &meshes[1]);
        self.gameobjects[0]
            .transform
            .translate(0., 0.5, 0.)
            .rotate(0., 0., PI)
            .scale(0.3, 0.3, 0.3);

        self.gameobjects[1]
            .transform
            .translate(1., 0., 0.)
            .rotate(0., 0., PI)
            .scale(0.3, 0.3, 0.3);
    }
}
