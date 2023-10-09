use nalgebra::Vector3;
use objects::{hitbox::Hitbox, mesh::Mesh, transformations::Transformations, GameObjectCreateInfo};

use super::{
    load::{
        _EMPTY_MESH_INDEX, _HOUSE_MESH_INDEX, _MAPSELECTION_MESH_INDEX, _MAP_MESH_INDEX,
        _SPHERE_MESH_INDEX,
    },
    App,
};

impl<'a> App<'a> {
    #[inline]
    pub fn main_loop(&mut self) {
        if let Some((clicked_object, click_position)) = self.click_detection() {
            if clicked_object.has_tag(&objects::tags::ObjectTag::Map) {
                let map_coordinates = self.map.world_coordinate_to_tile_center(&click_position);
                self.gameobjects[1].transform.set_position(map_coordinates);
            }
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
        self.gameobjects[0].add_tag(objects::tags::ObjectTag::Map);

        self.create_obj(
            &meshes[_MAPSELECTION_MESH_INDEX],
            &hitboxes[_EMPTY_MESH_INDEX],
            &GameObjectCreateInfo::position(Vector3::new(0., 0., 0.)),
        );
    }
}
