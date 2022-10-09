use rand::Rng;

const SIZE_X: i32 = 50;
const SIZE_Y: i32 = 50;

#[derive(Copy, Clone)]
struct Tile;

pub struct Map {
    matr: [[Option<Tile>; SIZE_X as usize]; SIZE_Y as usize],
}

impl Map {
    pub fn new() -> Self {
        Self {
            matr: [[None; SIZE_X as usize]; SIZE_Y as usize],
        }
    }

    pub fn with_random_seed(&mut self) -> Self {
        let rand = rand::thread_rng().gen::<i32>();

        Self { matr: self.matr }
    }
}
