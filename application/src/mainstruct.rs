use nalgebra::Matrix4;
use objects::GameObject;
use renderer::engine::aligned_array::AlignedArray;

use crate::input::Input;

struct MainStruct<'a> {
    input: Input,
    camera: Matrix4<f32>,
    gameobjects: AlignedArray<GameObject<'a>>,
}