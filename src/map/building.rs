use bytemuck::{Pod, Zeroable};
use vulkano::impl_vertex;
pub struct Building {
    pub coordinates: [u16; 2],
    pub texture_layer: u16, //First 16 bits represent sampler index, last 16 represent the texture layer
}

#[repr(C)]
#[derive(Default, Clone, Copy, Pod, Zeroable, Debug)]
pub struct GpuStoredBuilding {
    pub coordinates: [f32; 3],
    pub texture_layer: u32,
}
impl_vertex!(GpuStoredBuilding, coordinates, texture_layer);

impl GpuStoredBuilding {
    pub fn zero() -> Self {
        Self { coordinates: [0.0, 0.0, 0.0], texture_layer: 0 }
    }
}