pub mod templates;

use std::{fs::File, io::BufReader, mem::size_of};

use ash::vk;
use nalgebra::Vector3;
use obj::{load_obj, Obj};

use renderer::{self, resources::buffer::Buffer, utils::vertex::Vertex, Renderer};

// #[derive(Clone)]
pub struct Mesh {
    pub vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
    triangles_count: u16,
    transform_index: usize,
}

impl Mesh {
    pub fn new(
        renderer: &Renderer,
        vertices: Vec<Vertex>,
        indicies: Vec<u16>,
        transform_index: usize,
    ) -> Self {
        assert_eq!(indicies.len() % 3, 0);

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

        Self {
            index_buffer,
            vertex_buffer,
            index_count: indicies.len() as u32,
            triangles_count: (indicies.len() / 3) as u16,
            transform_index,
        }
    }

    pub fn into_tuple(&self) -> (&Buffer, &Buffer, u32, usize) {
        (
            &self.vertex_buffer,
            &self.index_buffer,
            self.index_count,
            self.transform_index,
        )
    }

    pub fn from_obj(renderer: &Renderer, path: &str, transform_index: usize) -> Mesh {
        let input = BufReader::new(File::open(path).unwrap());
        let obj: Obj<obj::Vertex, u16> = load_obj(input).unwrap();

        let mut vertex_buffer = Vec::new();
        for vertex in obj.vertices {
            let mut position = vertex.position.into();
            let mut normal = vertex.normal.into();
            // position *= -1.;
            // normal *= -1.;
            let new_vertex = Vertex::new(
                position,
                Vector3::new(1., 1., 1.),
                normal,
            );
            vertex_buffer.push(new_vertex);
        }
        let mut index_buffer = Vec::new();
        for index in obj.indices {
            index_buffer.push(index);
        }

        Mesh::new(renderer, vertex_buffer, index_buffer, transform_index)
    }

    #[inline]
    pub fn get_transform_index(&self) -> usize {
        self.transform_index
    }

    pub fn free(&self, device: &ash::Device) {
        unsafe {
            self.vertex_buffer.free(device);
            self.index_buffer.free(device);
        }
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
