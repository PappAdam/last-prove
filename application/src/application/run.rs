use nalgebra::Vector3;
use objects::{hitbox::Hitbox, mesh::Mesh, transformations::Transformations, GameObjectCreateInfo};

use super::{load::HOUSE_MESH_INDEX, App};

impl<'a> App<'a> {
    #[inline]
    pub fn main_loop(&mut self) {
        self.gameobjects[0].render(&self.renderer);

        //gameobjects[1] is a ball tracking the camera
        self.gameobjects[1].transform.set_position(Vector3::new(
            -self.camera.get_position().x,
            0.,
            -self.camera.get_position().y,
        ));
        self.gameobjects[1].render(&self.renderer);
        // self.game_controller
        //     .add_time_elapsed(self.delta_time.as_secs_f32(), &mut self.renderer);
    }

    pub fn setup(&mut self, meshes: &'a mut Vec<Mesh>, hitboxes: &'a mut Vec<Hitbox>) {
        self.load_meshes(meshes, hitboxes);

        self.create_obj(
            &meshes[0],
            &hitboxes[0],
            &GameObjectCreateInfo::position(Vector3::new(0., 0., 0.)),
        );
        self.create_obj(
            &meshes[HOUSE_MESH_INDEX],
            &hitboxes[HOUSE_MESH_INDEX],
            &GameObjectCreateInfo::position(Vector3::new(0., 0., 0.)),
        );
    }
}
