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
    pub fn load_meshes(&mut self, meshes: &mut Vec<Mesh>) {
        //Empty
        meshes.push(Mesh::new(
            &mut self.renderer,
            vec![Vertex::default()],
            vec![0, 0, 0],
            Hitbox::new(vec![], vec![], vec![])
        ));
        //Map
        let map_mesh = self.map.convert_to_mesh(&mut self.renderer);
        meshes.push(map_mesh);
        self.load_object("resources/models/Structures/House", meshes);
        self.load_object("resources/models/Indicators/MapSelection", meshes);
        self.load_object("resources/models/Debug/Plane", meshes);
        self.load_object("resources/models/Debug/Sphere", meshes);
    }
    #[inline]
    fn load_object(&mut self, path: &str, meshes: &mut Vec<Mesh>) {
        meshes.push(Mesh::from_file(&mut self.renderer, path));
    }
    // #[inline]
    // fn load_hitbox_file(path: &str, hitboxes: &mut Vec<Hitbox>) {

    // }
}
