use nalgebra::Vector3;
use objects::{mesh::Mesh, transformations::Transformations, GameObjectCreateInfo};

use super::App;

impl<'a> App<'a> {
    #[inline]
    pub fn main_loop(&mut self) {
        let cam = self.camera;
        self.gameobjects[1]
            .transform
            .set_transform(&cam.try_inverse().unwrap());
        self.gameobjects[1].transform.scale(0.01);

        self.gameobjects[0].render(&self.renderer);
        
        //gameobjects[1] is a ball tracking the camera
        // self.gameobjects[1].render(&self.renderer);

        // self.game_controller
        //     .add_time_elapsed(self.delta_time.as_secs_f32(), &mut self.renderer);
    }

    pub fn setup(&mut self, meshes: &'a mut Vec<Mesh>) {
        meshes.push(self.map.convert_to_mesh(&mut self.renderer));
        self.load_mesh("resources/models/az", meshes);
        self.load_mesh("resources/models/az", meshes);
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
