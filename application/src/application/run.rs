use nalgebra::Vector3;
use objects::{hitbox::Hitbox, mesh::Mesh, transformations::Transformations, GameObjectCreateInfo};

use super::{
    App, load::{_MAP_MESH_INDEX, _HOUSE_MESH_INDEX, _SPHERE_MESH_INDEX, _EMPTY_MESH_INDEX},
};

impl<'a> App<'a> {
    #[inline]
    pub fn main_loop(&mut self) {
        
        if let Some(click_position) = self.click_detection() {
            self.gameobjects[1].transform.set_position(click_position);
        }
        
        // self.game_controller
        //     .add_time_elapsed(self.delta_time.as_secs_f32(), &mut self.renderer);
        for gameobject in &self.gameobjects {
            gameobject.render(&self.renderer);
        }
    }

    pub fn setup(&mut self, meshes: &'a mut Vec<Mesh>, hitboxes: &'a mut Vec<Hitbox>) {
        self.load_meshes(meshes, hitboxes);

        self.create_obj(
            &meshes[_MAP_MESH_INDEX],
            &hitboxes[_MAP_MESH_INDEX],
            &GameObjectCreateInfo::position(Vector3::new(0., 0., 0.)),
        );
        self.create_obj(
            &meshes[_SPHERE_MESH_INDEX],
            &hitboxes[_EMPTY_MESH_INDEX],
            &GameObjectCreateInfo::position(Vector3::new(0., 0., 0.)),
        );
        self.gameobjects[1].transform.scale_object(0.05);
    }
}
