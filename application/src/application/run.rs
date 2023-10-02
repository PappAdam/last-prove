use nalgebra::Vector3;
use objects::{hitbox::Hitbox, mesh::Mesh, transformations::Transformations, GameObjectCreateInfo};

use super::{
    load::{HOUSE_MESH_INDEX, MAP_MESH_INDEX, SPHERE_MESH_INDEX},
    App,
};

impl<'a> App<'a> {
    #[inline]
    pub fn main_loop(&mut self) {
        self.gameobjects[0].render(&self.renderer);
        self.gameobjects[1].render(&self.renderer);
        self.gameobjects[2].render(&self.renderer);

        //gameobjects[1] is a ball tracking the camera
        self.gameobjects[1].transform.set_position(Vector3::new(
            -self.camera.get_position().x,
            0.,
            -self.camera.get_position().y,
        ));
        if let Some(click_position) = self.click_detection() {
            self.gameobjects[2].transform.set_position(click_position);
            dbg!(click_position);
        }

        // self.game_controller
        //     .add_time_elapsed(self.delta_time.as_secs_f32(), &mut self.renderer);
    }

    pub fn setup(&mut self, meshes: &'a mut Vec<Mesh>, hitboxes: &'a mut Vec<Hitbox>) {
        self.load_meshes(meshes, hitboxes);

        self.create_obj(
            &meshes[MAP_MESH_INDEX],
            &hitboxes[MAP_MESH_INDEX],
            &GameObjectCreateInfo::position(Vector3::new(0., 0., 0.)),
        );
        self.create_obj(
            &meshes[HOUSE_MESH_INDEX],
            &hitboxes[HOUSE_MESH_INDEX],
            &GameObjectCreateInfo::position(Vector3::new(0., 0., 0.)),
        );
        self.create_obj(
            &meshes[SPHERE_MESH_INDEX],
            &hitboxes[SPHERE_MESH_INDEX],
            &GameObjectCreateInfo::position(Vector3::new(0., 0., 0.)),
        );
    }
}
