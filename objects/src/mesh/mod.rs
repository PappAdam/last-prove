pub mod vertex;

use std::ops::{Add, AddAssign};

use byteorder::{ByteOrder, LittleEndian};
use nalgebra::Vector3;

use self::vertex::Vertex;

#[derive(Default)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    indicies: Vec<u16>,
    vertices_count: u16,
    triangles_count: u16,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indicies: Vec<u16>) -> Self {
        assert_eq!(indicies.len() % 3, 0);
        Self {
            vertices_count: vertices.len() as u16,
            triangles_count: (indicies.len() / 3) as u16,
            vertices,
            indicies,
        }
    }
    pub fn from_gltf() -> Mesh {
        let (document, buffers, _) =
            gltf::import("resources/models/puss_in_boots_perrito_fan_art.glb").unwrap();
        for mesh in document.meshes() {
            for primitive in mesh.primitives() {
                let position_accessor = primitive.get(&gltf::Semantic::Positions).unwrap();
                let position_view = position_accessor.view().unwrap();
                let position_buffer = buffers.get(position_view.buffer().index()).unwrap();
                let position_data = position_buffer
                    .0
                    .as_slice()
                    .get(position_view.offset()..(position_view.offset() + position_view.length()))
                    .unwrap();
                let mut vertices = Vec::new();
                for i in 0..position_data.len() / 12 {
                    let index = i * 12;
                    let x = LittleEndian::read_f32(&position_data[index..(index + 4)]);
                    let y = LittleEndian::read_f32(&position_data[(index + 4)..(index + 8)]);
                    let z = LittleEndian::read_f32(&position_data[(index + 8)..(index + 12)]);
                    let vertex = Vertex::from_pos(Vector3::new(x, -y + 1., z));
                    vertices.push(vertex);
                }

                let indicies_accessor = primitive.indices().unwrap();
                let indicies_view = indicies_accessor.view().unwrap();
                let indicies_buffer = buffers.get(indicies_view.buffer().index()).unwrap();
                let indicies_data = indicies_buffer
                    .0
                    .as_slice()
                    .get(indicies_view.offset()..(indicies_view.offset() + indicies_view.length()))
                    .unwrap();
                let mut index_buffer = Vec::new();
                let data_size = match indicies_accessor.data_type() {
                    gltf::accessor::DataType::U8 => 1,
                    gltf::accessor::DataType::U16 => 2,
                    gltf::accessor::DataType::U32 => 4,
                    _ => panic!("Not supported data type when loading indicies!"),
                };
                for i in 0..indicies_data.len() / data_size {
                    let index = i * data_size;
                    let vertex_index =
                        LittleEndian::read_u16(&indicies_data[index..(index + data_size)]);
                    index_buffer.push(vertex_index);
                }
                return Mesh::new(vertices, index_buffer);
            }
        }
        return Mesh::default();
    }
    pub fn get_indicies(&self) -> &Vec<u16> {
        return &self.indicies;
    }
    pub fn add_vertex(&mut self, mut vertex: Vec<Vertex>) {
        assert_eq!(vertex.len() % 3, 0);
        self.vertices.append(&mut vertex);
        self.vertices_count += vertex.len() as u16;
        self.triangles_count += (vertex.len() / 3) as u16;
    }
}

impl AddAssign for Mesh {
    fn add_assign(&mut self, mut rhs: Self) {
        self.vertices.append(&mut rhs.vertices);
        self.vertices_count += rhs.vertices_count;
        self.triangles_count += rhs.triangles_count;
    }
}
impl Add for Mesh {
    type Output = Mesh;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}
