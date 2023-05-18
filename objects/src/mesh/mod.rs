use std::{fs::File, io::BufReader, mem::size_of};

use ash::vk;
use nalgebra::Vector3;
use obj::{load_obj, Obj};

use renderer::{self, resources::buffer::Buffer, utils::vertex::Vertex, Renderer};

// #[derive(Clone)]
pub struct Mesh {
    pub vertex_buffer: vk::Buffer,
    pub index_buffer: vk::Buffer,
    index_count: u32,
}

impl Mesh {
    fn new(renderer: &mut Renderer, vertices: Vec<Vertex>, indicies: Vec<u16>) -> Self {
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
            size_of::<u16>() as u64 * indicies.len() as u64,
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

    pub fn from_obj(renderer: &mut Renderer, path: &str) -> Mesh {
        let input = BufReader::new(File::open(path).unwrap());
        let obj: Obj<obj::Vertex, u16> = load_obj(input).unwrap();

        let mut vertex_buffer = Vec::new();
        for vertex in obj.vertices {
            let position = vertex.position.into();
            let normal = vertex.normal.into();
            let new_vertex = Vertex::new(position, Vector3::new(1., 1., 1.), normal);
            vertex_buffer.push(new_vertex);
        }

        let mut index_buffer = Vec::new();
        for index in obj.indices {
            index_buffer.push(index);
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
