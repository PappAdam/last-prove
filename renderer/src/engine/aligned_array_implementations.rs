use nalgebra::Matrix4;

use super::aligned_array::AlignedValue;

impl AlignedValue for Matrix4<f32> {
    fn is_none(&self) -> bool {
        *self == Matrix4::<f32>::zeros()
    }
}
