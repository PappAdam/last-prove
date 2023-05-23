use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    mem::size_of,
};

use ash::vk;
use nalgebra::Vector3;
use obj::{load_obj, raw::parse_mtl, Obj};

use renderer::{self, resources::buffer::Buffer, utils::vertex::Vertex, Renderer};

// #[derive(Clone)]
pub struct Mesh {
    pub vertex_buffer: vk::Buffer,
    pub index_buffer: vk::Buffer,
    pub index_count: u32,
}

impl Mesh {
    pub fn new(renderer: &mut Renderer, vertices: Vec<Vertex>, indicies: Vec<u32>) -> Self {
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
        let obj_file = BufReader::new(File::open(path.to_owned() + ".obj").unwrap());
        let mtl_file = BufReader::new(File::open(path.to_owned() + ".mtl").unwrap());

        //Loading materials
        let mut materials: HashMap<String, [f32; 3]> = HashMap::new();
        let mut current_material_name = String::from("");
        for line in mtl_file.lines() {
            let line = line.unwrap();
            if line.contains("newmtl") {
                current_material_name = line[7..].to_owned();
            }
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
        //Finished loading materials
        let mut vertex_buffer = vec![];
        let mut index_buffer = vec![];

        let mut vertices = vec![];
        let mut normals = vec![];
        let mut textures = vec![];
        let mut current_material = String::from("");
        for line in obj_file.lines() {
            let line = line.unwrap();
            let splitted_line = line.split(' ').collect::<Vec<_>>();
            match splitted_line[0] {
                "v" => {
                    vertices.push([
                        splitted_line[1].parse::<f32>().unwrap(),
                        splitted_line[2].parse::<f32>().unwrap(),
                        splitted_line[3].parse::<f32>().unwrap(),
                    ])
                },
                "vn" => {
                    normals.push([
                        splitted_line[1].parse::<f32>().unwrap(),
                        splitted_line[2].parse::<f32>().unwrap(),
                        splitted_line[3].parse::<f32>().unwrap(),
                    ])
                },
                "vt" => {
                    textures.push([
                        splitted_line[1].parse::<f32>().unwrap(),
                        splitted_line[2].parse::<f32>().unwrap(),
                    ])
                },
                "f" => {
                    for segment in &splitted_line[1..] {
                        let splitted_segment = segment.split('/').collect::<Vec<_>>();
                        vertex_buffer.push(Vertex::new(
                            vertices[splitted_segment[0].parse::<usize>().unwrap() - 1].into(),
                            materials[&current_material].into(),
                            normals[splitted_segment[2].parse::<usize>().unwrap() - 1].into(),
                        ));
                        index_buffer.push(index_buffer.len() as u32);
                    }
                },
                "usemtl" => {
                    current_material = splitted_line[1].to_owned();
                }
                _ => {}
            } 
        }

        Mesh::new(renderer, vertex_buffer, index_buffer)
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
