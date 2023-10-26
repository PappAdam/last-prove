use nalgebra::Vector2;
use renderer::engine::aligned_array::NoneValue;

use super::Map;

pub enum StructureType {
    // Tree
    House,
    // Mansion,
    // Barracks,
    // Mine,
    // Forestry,
    // 
}

pub enum StructureFlag {
    None = 0b00000001,
}

pub struct Structure {
    coordinates: Vector2<usize>,
    ty: StructureType,
    flags: u8,
}

impl NoneValue for Structure {
    fn is_none(&self) -> bool {
        todo!()
    }

    fn set_to_none(&mut self) {
        todo!()
    }
}

impl Map {
    pub fn build_structure() {
        
    }
}