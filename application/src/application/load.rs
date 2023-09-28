use objects::{mesh::Mesh, hitbox::Hitbox};

use super::App;

impl<'a> App<'a> {
    pub fn load_meshes(&mut self, meshes: &mut Vec<Mesh>, hitboxes: &mut Vec<Hitbox>) {
        meshes.push(self.map.convert_to_mesh(&mut self.renderer));
        self.load_mesh_file("resources/models/Basic_house", meshes);

    }
    #[inline]
    fn load_mesh_file(&mut self, path: &str, meshes: &mut Vec<Mesh>) {
        meshes.push(Mesh::from_file(&mut self.renderer, path));
    }
    // #[inline]
    // fn load_hitbox_file(path: &str, hitboxes: &mut Vec<Hitbox>) {

    // }
}