pub mod coordinate;
mod debugmap;
mod heightmap;
mod mapmesh;
mod materials;
pub mod tile;

use std::vec;

use self::{heightmap::HeightMap, mapmesh::MapMeshPlugin, tile::Tile};
use bevy::prelude::*;

pub const MAP_SIZE: usize = 400;
const MAP_NOISE_SCALE: f64 = 30.;
const MAP_NOISE_PERSISTENCE: f64 = 0.55;
const MAP_NOISE_OCTAVES: usize = 4;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_map, apply_deferred, debug_map).chain());
        app.add_plugins(MapMeshPlugin);
    }
}

fn spawn_map(mut commands: Commands) {
    commands.spawn(Map::new());
}

fn debug_map(query: Query<&Map>) {
    let map = query.single();
    // dbg!(map);
}

#[derive(Component)]
pub struct Map {
    matrix: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new() -> Self {
        let mut map = Self { matrix: vec![] };
        map.generate();
        map
    }
    pub fn generate(&mut self) -> &mut Self {
        let heightmap = HeightMap::perlin_noise(
            MAP_SIZE,
            MAP_NOISE_SCALE,
            MAP_NOISE_PERSISTENCE,
            MAP_NOISE_OCTAVES,
        );
        let mut tile_matrix = vec::from_elem(vec::from_elem(Tile::new(false), MAP_SIZE), MAP_SIZE);
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                if heightmap[y][x] > 0.7 {
                    tile_matrix[y][x] = Tile::new(true);
                }
            }
        }
        self.matrix = tile_matrix;
        self
    }
}
