use std::f32::consts::PI;

use nalgebra::Vector3;
use objects::{mesh::Mesh, GameObjectCreateInfo, transformations::Transformations};

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
        self.load_mesh("resources/models/pine_tree", meshes);
        // self.load_mesh("resources/models/az", meshes);
        self.create_obj(&meshes[0], GameObjectCreateInfo::position(Vector3::new(-5., 5., 0.)));
        // self.gameobjects[0].transform.rotate(PI/2., 0., 0.);
        // self.gameobjects[0].transform.scale(0.1, 0.1, 0.1);
        self.create_obj(
            &meshes[1],
            GameObjectCreateInfo::position(Vector3::new(200., 0., 2.)),
        );
    }
}
