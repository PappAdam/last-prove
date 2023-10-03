use objects::{hitbox::Hitbox, mesh::Mesh};
use renderer::utils::vertex::Vertex;

use super::App;

pub const _EMPTY_MESH_INDEX: usize = 0;
pub const _MAP_MESH_INDEX: usize = 1;
pub const _HOUSE_MESH_INDEX: usize = 2;
pub const _PLANE_MESH_INDEX: usize = 3;
pub const _SPHERE_MESH_INDEX: usize = 4;

impl<'a> App<'a> {
    pub fn load_meshes(&mut self, meshes: &mut Vec<Mesh>, hitboxes: &mut Vec<Hitbox>) {
        meshes.push(Mesh::new(
            &mut self.renderer,
            vec![Vertex::default()],
            vec![0, 0, 0],
        ));
        hitboxes.push(Hitbox::new(vec![], vec![]));
        let (map_mesh, map_hitbox) = self.map.convert_to_mesh(&mut self.renderer);
        hitboxes.push(map_hitbox);
        meshes.push(map_mesh);
        //Indexing starts from 1 because of map.
        // self.load_object("resources/models/Structures/House", meshes, hitboxes);
        self.load_object("resources/models/Structures/House", meshes, hitboxes);
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
