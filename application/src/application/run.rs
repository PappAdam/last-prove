use nalgebra::Vector3;
use objects::{mesh::Mesh, transformations::Transformations, GameObjectCreateInfo};

use super::{
    load::{_HOUSE_MESH_INDEX, _MAPSELECTION_MESH_INDEX, _MAP_MESH_INDEX, _SPHERE_MESH_INDEX},
    App,
};

impl<'a> App<'a> {
    #[inline]
    pub fn main_loop(&mut self) {
        if let Some((clicked_object, click_position)) = self.world_mouse_intersection_point() {
            if clicked_object.has_tag(&objects::tags::ObjectTag::Map) {
                // let map_coordinates = self.map.world_coordinate_to_tile_center(&click_position);
            }
            self.gameobjects[2].transform.set_position(click_position);
        }

        // self.game_controller
        //     .add_time_elapsed(self.delta_time.as_secs_f32(), &mut self.renderer);
        for gameobject in &self.gameobjects {
            gameobject.render(&self.renderer);
        }
    }

    pub fn setup(&mut self, meshes: &'a mut Vec<Mesh>) {
        self.load_meshes(meshes);

        self.create_obj(
            &meshes[_MAP_MESH_INDEX],
            &GameObjectCreateInfo::position(Vector3::new(0., 0., 0.)),
        );
        self.gameobjects[0].add_tag(objects::tags::ObjectTag::Map);
        self.create_obj(
            &meshes[_HOUSE_MESH_INDEX],
            &GameObjectCreateInfo::position(Vector3::new(250.5, 0., 250.5)),
        );
        self.create_obj(
            &meshes[_SPHERE_MESH_INDEX],
            &GameObjectCreateInfo::position(Vector3::new(0., 0., 0.)),
        );
        self.gameobjects[2].set_flag(objects::GameObjectFlag::NotClickable);
        self.gameobjects[2].transform.scale_object(0.1);
    }
}
