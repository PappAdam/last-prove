use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Default, Clone, Copy, Pod, Zeroable, Debug)]
pub struct GpuStoredGameObject {
    pub coordinates: [f32; 3],
    pub texture_layer: u32,
}
vulkano::impl_vertex!(GpuStoredGameObject, coordinates, texture_layer);

impl GpuStoredGameObject {
    pub fn zero() -> Self {
        Self {
            coordinates: [0.0, 0.0, 0.0],
            texture_layer: 0,
        }
    }
}

#[repr(C)]
#[derive(Default, Clone, Copy, Pod, Zeroable, Debug)]
pub struct GpuStoredHUDObject {
    pub screen_position: [f32; 3], //Represents top left corner
    pub object_size: [f32; 2],     //Bottom right corner relative to top left corner
    pub texture_layer: u32,
}
vulkano::impl_vertex!(
    GpuStoredHUDObject,
    screen_position,
    object_size,
    texture_layer
);

impl GpuStoredHUDObject {
    pub fn zero() -> Self {
        Self {
            screen_position: [0.0, 0.0, 0.0],
            object_size: [0.0, 0.0],
            texture_layer: 0,
        }
    }
}
