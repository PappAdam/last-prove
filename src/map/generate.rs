use super::building::Building;
use super::tile::NeighborLocation;
use super::{automata, perlin};
use super::{tile::Tile, Map};
use crate::engine::vector2::Vector2;
use std::fs;

impl Map {
    #[allow(unused)]
    pub fn generate(&mut self, seed: Option<u16>) {
        let perlin_noise = perlin::Perlin2D::new(match seed {
            None => rand::Rng::gen::<u16>(&mut rand::thread_rng()),
            Some(i) => i,
        });

        let center_on_row = (self.size / 2) as f32;
        let map_center = Vector2::new(center_on_row, center_on_row);

        //The perlin noise value will be divided by this number
        //The result will be the height
        //The higher the self.height is, the lower this number gets, resulting in higher maps.
        let z_difference_for_height = 1.0 / self.height as f32;

        for y in 0..self.size as usize {
            for x in 0..self.size as usize {
                let tile_position = Vector2::new_usize(x, y);

                //Treshold gets higher when further from the center
                let treshold: f32 = Vector2::distance(map_center, tile_position) / center_on_row;

                let perlin_value = perlin_noise.perlin2d(x as f32, y as f32, 0.1, 2);

                if perlin_value > treshold {
                    let tile = Tile::new(
                        tile_position.into(),
                        ((perlin_value - treshold) / z_difference_for_height) as u8,
                    );
                    self.tile_matr[y][x] = Some(tile);
                    //self.building_vector.push(Building { coordinates: tile_position.into(), texture_layer: 0 })
                }
            }
        }
        //Calculating minimum Z values for optimized render, than returning the result.
        self.set_tile_types();
        self.calculate_min_z();
        self.calculate_vulkan_instances();
        self.building_vector.push(Building { coordinates: [self.size as u16 / 2, self.size as u16 / 2], texture_layer: 0 })
    }

    pub fn generate_automata(&mut self, density: f32) {
        let automata_matr = automata::generate(self.size, density);
        for y in 0..self.size as usize {
            for x in 0..self.size as usize {
                if automata_matr[y][x] == 0 {
                    self.tile_matr[y][x] = None;
                } else {
                    self.tile_matr[y][x] = Some(Tile::new([x as u16, y as u16], 0));
                }
            }
        }
        self.set_tile_types();
        self.calculate_min_z();
        self.calculate_vulkan_instances();
        self.building_vector.push(Building { coordinates: [4, 4], texture_layer: 0 });
    }

    #[allow(unused)]
    pub fn from_file(mut self, path: &str) -> Self {
        let read = fs::read_to_string(&path).unwrap();
        let rows = read.split("\r\n").collect::<Vec<&str>>();
        for (rowindex, row) in rows.iter().enumerate() {
            for (columnindex, column_value) in row.chars().enumerate() {
                match column_value {
                    '_' => self.tile_matr[rowindex][columnindex] = None,
                    _ => {
                        println!("{}", column_value);
                        let tile = Some(Tile::new(
                            [columnindex as u16, rowindex as u16],
                            column_value.to_digit(10).unwrap() as u8,
                        ));
                        self.tile_matr[rowindex][columnindex] = tile;
                    }
                }
            }
        }
        self
    }

    fn calculate_min_z(&mut self) {
        for y in 0..self.size as usize {
            for x in 0..self.size as usize {
                if let Some(mut tile) = self.copy_tile_from_matr(Vector2::new_usize(x, y)) {
                    //Calculate min_z for tiles behid this one
                    //If tile.max_z = 0 there are no tiles behind this one.
                    if tile.max_z > 0 {
                        //z_down is used for decreasing the height by one every iteration
                        //a tile is blocking vision until ( height - n ) at n tile behind itself.
                        let mut z_down = tile.max_z;

                        //z_up is used for going one index behind on every iteration
                        let mut z_up = 0;

                        //Going 1 tile behind every iteration
                        while z_down > 0 {
                            z_up += 1;
                            if let Some(mut other_tile) =
                                self.get_mut_tile_from_matr(Vector2::new_usize(x - z_up, y - z_up))
                            {
                                //Multiple other tiles can set this tile's min_z
                                //Only setting min_z if z_down is higher than min_z so we get the highest value of all.
                                if z_down > other_tile.min_z {
                                    //We start rendering from the first z that is higher on screen than this one.
                                    other_tile.min_z = z_down;
                                }
                            }
                            z_down -= 1;
                        }
                    }

                    //Calculate min_z for tiles that are blocked by neighbors
                    let left_neighbor: u8 = if let Some(other_tile) =
                        self.get_tile_from_matr(Vector2::new_usize(x, y + 1))
                    {
                        other_tile.max_z
                    } else {
                        0
                    };

                    let right_neighbor: u8 = if let Some(other_tile) =
                        self.get_tile_from_matr(Vector2::new_usize(x + 1, y))
                    {
                        other_tile.max_z
                    } else {
                        0
                    };

                    let neighbor_lowest_max_z = if left_neighbor < right_neighbor {
                        left_neighbor
                    } else {
                        right_neighbor
                    };
                    if neighbor_lowest_max_z > 0 {
                        //If a neighbor is not a tile, the neighbors will not block any z layers.

                        //Getting the min of the two max_z values

                        //If the height of neighbors is higher or equal to the height of the tile,
                        //The tile will be rendered from the height of the neighbors.
                        if neighbor_lowest_max_z >= tile.max_z {
                            tile.min_z = neighbor_lowest_max_z;
                        }

                        //If the neighbor's height is lower than than the height of the tile,
                        //The tile should be rendered from the .
                        if neighbor_lowest_max_z < tile.max_z {
                            tile.min_z = neighbor_lowest_max_z + 1;
                        }
                    }

                    //If only neighbors are blocking vision to a tile, and the tile is not directly behind to neighbors
                    //then the tile is still rendered. Very rare case but can happen
                    //(so yes, they are not neighbors but whatever)
                    self.tile_matr[y][x] = Some(tile);
                }
            }
        }
    }

    fn calculate_vulkan_instances(&mut self) {
        self.num_of_vulkan_instances = 0;
        for y in &self.tile_matr {
            for x in y {
                if let Some(tile) = x {
                    self.num_of_vulkan_instances += if tile.max_z + 1 > tile.min_z {
                        (tile.max_z - tile.min_z) as u32 + 1
                    } else {
                        0u32
                    };
                }
            }
        }
    }

    pub fn set_tile_types(&mut self) {
        for y in 0..self.size as usize {
            for x in 0..self.size as usize {
                if let Some(mut tile) = self.copy_tile_from_matr(Vector2::new_usize(x, y)) {
                    if let Some(other_tile) =
                        self.get_tile_from_matr(Vector2::new(x as f32, y as f32 - 1f32))
                    //Using floats to prevent subtracing with overflow
                    {
                        if other_tile.max_z == tile.max_z {
                            tile.neighbors |= NeighborLocation::Top as u8;
                        }
                    }
                    if let Some(other_tile) = self.get_tile_from_matr(Vector2::new_usize(x, y + 1))
                    {
                        if other_tile.max_z == tile.max_z {
                            tile.neighbors |= NeighborLocation::Bottom as u8;
                        }
                    }

                    if let Some(other_tile) =
                        self.get_tile_from_matr(Vector2::new(x as f32 - 1f32, y as f32))
                    //Using floats to prevent subtracing with overflow
                    {
                        if other_tile.max_z == tile.max_z {
                            tile.neighbors |= NeighborLocation::Left as u8;
                        }
                    }

                    if let Some(other_tile) = self.get_tile_from_matr(Vector2::new_usize(x + 1, y))
                    {
                        if other_tile.max_z == tile.max_z {
                            tile.neighbors |= NeighborLocation::Right as u8;
                        }
                    }
                    //println!("{:?}", tile);
                    self.tile_matr[y][x] = Some(tile);
                }
            }
        }
    }
}
