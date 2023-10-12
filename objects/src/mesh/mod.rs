pub mod primitives;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    mem::size_of,
};

use ash::vk;

use renderer::{self, resources::buffer::Buffer, utils::vertex::Vertex, Renderer};

use crate::hitbox::Hitbox;

// #[derive(Clone)]
pub struct Mesh {
    pub vertex_buffer: vk::Buffer,
    pub index_buffer: vk::Buffer,
    pub hitbox: Hitbox,
    pub index_count: u32,
}

impl Mesh {
    pub fn new(renderer: &mut Renderer, vertices: Vec<Vertex>, indicies: Vec<u32>, hitbox: Hitbox) -> Self {
        let vertex_buffer = Buffer::device_local(
            &renderer.base.device,
            vertices.as_ptr() as _,
            size_of::<Vertex>() as u64 * vertices.len() as u64,
            renderer.base.physical_device_memory_properties,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            renderer.base.queue,
            renderer.data.command_pool,
        )
        .unwrap();

        let index_buffer = Buffer::device_local(
            &renderer.base.device,
            indicies.as_ptr() as _,
            size_of::<u32>() as u64 * indicies.len() as u64,
            renderer.base.physical_device_memory_properties,
            vk::BufferUsageFlags::INDEX_BUFFER,
            renderer.base.queue,
            renderer.data.command_pool,
        )
        .unwrap();

        let ib = index_buffer.buf;
        let vb = vertex_buffer.buf;

        renderer.load_mesh([index_buffer, vertex_buffer]);

        Self {
            index_buffer: ib,
            vertex_buffer: vb,
            index_count: indicies.len() as u32,
            hitbox
        }
    }

    #[inline]
    pub fn into_tuple(&self, transform_index: usize) -> (vk::Buffer, vk::Buffer, u32, usize) {
        (
            self.vertex_buffer,
            self.index_buffer,
            self.index_count,
            transform_index,
        )
    }

    pub fn from_file(renderer: &mut Renderer, path: &str) -> Mesh {
        let obj_file = BufReader::new(File::open(path.to_owned() + "/Object.obj").unwrap());
        let mtl_file = BufReader::new(File::open(path.to_owned() + "/Object.mtl").unwrap());

        //Loading materials
        let mut materials: HashMap<String, [f32; 3]> = HashMap::new();
        let mut current_material_name = String::from("");
        for line in mtl_file.lines() {
            let line = line.unwrap();
            //Getting material name
            if line.contains("newmtl") {
                current_material_name = line[7..].to_owned();
            }
            //Getting material color (Color value is written after the Kd keyword in mtl files)
            if line.contains("Kd") {
                materials.insert(
                    current_material_name.to_owned(),
                    [
                        line[3..11].parse::<f32>().unwrap(),
                        line[12..20].parse::<f32>().unwrap(),
                        line[21..29].parse::<f32>().unwrap(),
                    ],
                );
            }
        }
        //A default material, if no materials are present.
        if materials.len() == 0 {
            materials.insert("default".to_owned(), [1.0, 0.0, 1.0]);
        }
        //Finished loading materials

        //The buffers get filled up when reading face data
        let mut vertex_buffer = vec![];
        let mut index_buffer = vec![];

        //These vectors get filled up with v, vn, and vt values.
        let mut vertices = vec![];
        let mut normals = vec![];
        let mut textures = vec![];

        //Keeping track of what material the face uses.
        let mut current_material = String::from("default");

        for line in obj_file.lines() {
            let line = line.unwrap();
            let splitted_line = line.split(' ').collect::<Vec<_>>();
            match splitted_line[0] {
                //Vertex xample: v 0.0000000 1.0000000 0.5000000
                "v" => vertices.push([
                    splitted_line[1].parse::<f32>().unwrap(),
                    -splitted_line[2].parse::<f32>().unwrap(),
                    splitted_line[3].parse::<f32>().unwrap(),
                ]),
                //Normal example: vn 0.0000000 1.0000000 0.0000000
                "vn" => normals.push([
                    splitted_line[1].parse::<f32>().unwrap(),
                    -splitted_line[2].parse::<f32>().unwrap(),
                    splitted_line[3].parse::<f32>().unwrap(),
                ]),
                //Texture example: vt 0.5000000 1.0000000 (texture is always 2D)
                "vt" => textures.push([
                    splitted_line[1].parse::<f32>().unwrap(),
                    splitted_line[2].parse::<f32>().unwrap(),
                ]),
                //Face example 1/1/1 2/1/1 3/4/1
                //Format is following: positionindex1/colorindex1/normalindex1 positionindex2/...
                "f" => {
                    for segment in &splitted_line[1..] {
                        let splitted_segment = segment.split('/').collect::<Vec<_>>();
                        vertex_buffer.push(Vertex::new(
                            vertices[splitted_segment[0].parse::<usize>().unwrap() - 1].into(),
                            materials[&current_material].into(),
                            normals[splitted_segment[2].parse::<usize>().unwrap() - 1].into(),
                        ));

                        //We are currently loading all vertices in order,
                        //so the index buffer is just a vector of incrementing numbers.
                        index_buffer.push(index_buffer.len() as u32);
                    }
                }

                //Material example: usemtl Brick
                "usemtl" => {
                    current_material = splitted_line[1].to_owned();
                }
                _ => {}
            }
        }
        Mesh::new(renderer, vertex_buffer, index_buffer, Hitbox::from_file(path))
    }
}

// impl AddAssign for Mesh {
//     fn add_assign(&mut self, mut rhs: Self) {
//         self.vertices.append(&mut rhs.vertices);
//         self.vertices_count += rhs.vertices_count;
//         self.triangles_count += rhs.triangles_count;
//     }
// }
// impl Add for Mesh {
//     type Output = Mesh;

//     fn add(mut self, rhs: Self) -> Self::Output {
//         self += rhs;
//         self
//     }
// }
