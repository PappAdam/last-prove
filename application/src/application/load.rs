use objects::{hitbox::Hitbox, mesh::Mesh};
use renderer::utils::vertex::Vertex;

use super::App;

pub const MAX_VERTEX_DISTANCE: f32 = 1.1;
//True value is 1, add .1 to account for any floating point inaccuracies

pub const _EMPTY_MESH_INDEX: usize = 0;
pub const _MAP_MESH_INDEX: usize = 1;
pub const _HOUSE_MESH_INDEX: usize = 2;
pub const _MAPSELECTION_MESH_INDEX: usize = 3;
pub const _PLANE_MESH_INDEX: usize = 4;
pub const _SPHERE_MESH_INDEX: usize = 5;
//WHEN ADDING MORE OBJECTS, BE CAREFUL TO UPDATE MAX_VERTEX_DISTANCE IF NEEDED

impl<'a> App<'a> {
    pub fn load_meshes(&mut self, meshes: &mut Vec<Mesh>, hitboxes: &mut Vec<Hitbox>) {
        //Empty
        meshes.push(Mesh::new(
            &mut self.renderer,
            vec![Vertex::default()],
            vec![0, 0, 0],
        ));
        hitboxes.push(Hitbox::new(vec![], vec![]));
        //Map
        let (map_mesh, map_hitbox) = self.map.convert_to_mesh(&mut self.renderer);
        hitboxes.push(map_hitbox);
        meshes.push(map_mesh);
        //Indexing starts from 1 because of map.
        // self.load_object("resources/models/Structures/House", meshes, hitboxes);
        self.load_object("resources/models/Structures/House", meshes, hitboxes);
        self.load_object("resources/models/Indicators/MapSelection", meshes, hitboxes);
        self.load_object("resources/models/Debug/Plane", meshes, hitboxes);
        self.load_object("resources/models/Debug/Sphere", meshes, hitboxes);
    }
    #[inline]
    fn load_object(&mut self, path: &str, meshes: &mut Vec<Mesh>, hitboxes: &mut Vec<Hitbox>) {
        meshes.push(Mesh::from_file(&mut self.renderer, path));
        hitboxes.push(Hitbox::from_file(path));
    }
    // #[inline]
    // fn load_hitbox_file(path: &str, hitboxes: &mut Vec<Hitbox>) {

    // }
}
