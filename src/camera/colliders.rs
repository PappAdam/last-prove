const TROOP_COLLIDER: Collider = Collider::new(0.5, 0.5, 1.0);
const BUILDING_COLLIDER: Collider = Collider::new(0.9, 0.9, 0.9);

struct Collider {
    x: f32,
    y: f32,
    z: f32,
}

impl Collider {
    const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}