use std::ops::Index;

use crate::engine::vector2::Vector2;

#[derive(Debug, Clone, Copy)]
pub enum ColliderIndex {
    TileCollider = 0,
    TroopCollider = 1,
    BuildingCollider = 2,
}
impl Index<ColliderIndex> for [Collider] {
    type Output = Collider;

    fn index(&self, index: ColliderIndex) -> &Self::Output {
        &self[index as usize]
    }
}

//IMPORTANT!
//Colliders' z value is their height, but they x and y values are the halves of a collider's width and height.
//This is to prevent dividing it by 2 when making calculations with it.
const TILE_COLLIDER: Collider = Collider::new(0.5, 0.5, 1.0);
const TROOP_COLLIDER: Collider = Collider::new(0.25, 0.25, 1.0);
const BUILDING_COLLIDER: Collider = Collider::new(12. / 32., 12. / 32., 24. / 32.);

pub const COLLIDER_ARRAY: [Collider; 3] = [TILE_COLLIDER, TROOP_COLLIDER, BUILDING_COLLIDER];

pub struct Collider {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Collider {
    const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn coordinates_inside(
        &self,
        collider_coordinates: Vector2<f32>,
        input_coordinates: Vector2<f32>,
    ) -> bool {
        //Coordinates here are already calculated so it's height is the same as the bottom of the collider.
        let bottom_difference_from_center = (collider_coordinates - input_coordinates).abs();
        //Bottom of collider
        if bottom_difference_from_center.x < self.x && bottom_difference_from_center.y < self.y {
            return true;
        }

        let top_difference_from_center =
            (collider_coordinates - input_coordinates - Vector2::uniform(self.z)).abs();
        if top_difference_from_center.x < self.x && top_difference_from_center.y < self.y {
            return true;
        }

        let top_left_coordinates =
            collider_coordinates + Vector2::new(-self.x, self.y) - Vector2::uniform(self.z);
        let bottom_right_coordinates = collider_coordinates + Vector2::new(self.x, -self.y);
        let top_left = Vector2::new(
            top_left_coordinates.x - top_left_coordinates.y,
            top_left_coordinates.x + top_left_coordinates.y,
        );
        let bottom_right = Vector2::new(
            bottom_right_coordinates.x - bottom_right_coordinates.y,
            bottom_right_coordinates.x + bottom_right_coordinates.y,
        );
        let input_position = Vector2::new(
            input_coordinates.x - input_coordinates.y,
            input_coordinates.x + input_coordinates.y,
        );
        if input_position.x > top_left.x
            && input_position.x < bottom_right.x
            && input_position.y > top_left.y
            && input_position.y < bottom_right.y
        {
            return true;
        }
        false
    }
}

pub trait HasCollider {
    fn get_collider(&self) -> &'static Collider;
}
