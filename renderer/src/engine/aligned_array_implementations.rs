use nalgebra::Matrix4;

use super::aligned_array::NoneValue;

impl NoneValue for Matrix4<f32> {
    fn is_none(&self) -> bool {
        *self == Matrix4::<f32>::zeros()
    }

    fn set_to_none(&mut self) {
        *self = Matrix4::zeros();
    }
}
