use nalgebra::Vector3;
use objects::{mesh::Mesh, transformations::Transformations, GameObjectCreateInfo};

use super::App;

impl<'a> App<'a> {
    #[inline]
    pub fn main_loop(&mut self) {
        self.gameobjects[0].render(&self.renderer);

        //gameobjects[1] is a ball tracking the camera
        // self.gameobjects[1].transform.set_position(Vector3::new(
        //     -self.camera.get_position().x,
        //     0.,
        //     -self.camera.get_position().y,
        // ));
        // self.gameobjects[1].render(&self.renderer);

        // self.game_controller
        //     .add_time_elapsed(self.delta_time.as_secs_f32(), &mut self.renderer);
    }

    pub fn setup(&mut self, meshes: &'a mut Vec<Mesh>) {
        meshes.push(self.map.convert_to_mesh(&mut self.renderer));
        self.load_mesh("resources/models/Basic_house", meshes);
        // self.load_mesh("resources/models/az", meshes);
        // self.load_mesh("resources/models/az", meshes);
        self.create_obj(
            &meshes[0],
            &GameObjectCreateInfo::position(Vector3::new(0., 0., 0.)),
        );
        self.create_obj(
            &meshes[1],
            &GameObjectCreateInfo::position(Vector3::new(0., 0., 0.)),
        );
    }
}
