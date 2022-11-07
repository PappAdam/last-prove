use super::perlin;
use super::tile::NeighborLocation;
use super::{tile::Tile, Map};
use crate::engine::vector2::Vector2;
use crate::render::TileTextures;

impl<'a> Map<'a> {
    pub fn generate(mut self) -> Self {
        let perlin_noise = perlin::Perlin2D::new(self.seed as i32);

        let center_on_row = (self.size / 2) as f32;
        let map_center = Vector2::new(center_on_row, center_on_row);

        //The perlin noise value will be divided by this number
        //The result will be the height
        //The higher the self.height is, the lower this number gets, resulting in higher maps.
        let z_difference_for_height = 1.0 / self.height as f32;

        for y in 0..self.size as usize {
            for x in 0..self.size as usize {
                let tile_position = Vector2::new(x as f32, y as f32);

                //Treshold gets higher when further from the center
                let treshold: f32 = Vector2::distance(map_center, tile_position) / center_on_row;

                let perlin_value = perlin_noise.perlin2d(x as f32, y as f32, 0.1, 2);

                if perlin_value > treshold {
                    let tile = Some(Tile::new(
                        Vector2::new(x as f32, y as f32),
                        None,
                        0,
                        ((perlin_value - treshold) / z_difference_for_height) as u8,
                    ));

                    self.matr[y][x] = tile;
                }
            }
        }
        //Calculating minimum Z values for optimized render, than returning the result.
        self.calculate_min_z()
    }

    // pub fn flat(mut self, z: u8) -> Self {
    //     for y in 0..self.size as usize {
    //         for x in 0..self.size as usize {
    //             self.matr[y][x] = Some(Tile::new(Vector2::new(x as f32, y as f32), None, 0, z));
    //         }
    //     }
    //     self
    // }

    pub fn calculate_min_z(mut self) -> Self {
        for y in 0..self.size as usize {
            for x in 0..self.size as usize {
                if let Some(mut tile) = self.matr[y][x] {
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
                            if let Some(mut other_tile) = self.matr[y - z_up][x - z_up] {
                                //Multiple other tiles can set this tile's min_z
                                //Only setting min_z if z_down is higher than min_z so we get the highest value of all.
                                if z_down > other_tile.min_z {
                                    //We start rendering from the first z that is higher on screen than this one.
                                    other_tile.min_z = z_down;
                                }

                                //After setting the min_z value the new tile gets passed back into the matrix
                                self.matr[y - z_up][x - z_up] = Some(other_tile);
                            }
                            z_down -= 1;
                        }
                    }

                    //Calculate min_z for tiles that are blocked by neighbors
                    let left_neighbor: Option<u8> = match self.matr[y + 1][x] {
                        Some(other_tile) => {
                            let z = if other_tile.max_z + 1 > other_tile.min_z { other_tile.max_z } else { other_tile.min_z };
                            Some(z)
                        },
                        None => None,
                    };
                    let right_neighbor: Option<u8> = match self.matr[y][x + 1] {
                        Some(other_tile) => {
                            let z = if other_tile.max_z + 1 > other_tile.min_z { other_tile.max_z } else { other_tile.min_z };
                            Some(z)
                        },
                        None => None,
                    };

                    if let Some(left_neighbor) = left_neighbor {
                        if let Some(right_neighbor) = right_neighbor {
                            //If a neighbor is not a tile, the neighbors will not block any z layers.

                            //Getting the min of the two max_z values
                            let neighbor_lowest_max_z = if left_neighbor < right_neighbor {
                                left_neighbor
                            } else {
                                right_neighbor
                            };

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
                    }

                    //If only neighbors are blocking vision to a tile, and the tile is not directly behind to neighbors
                    //then the tile is still rendered. Very rare case but can happen
                    //(so yes, they are not neighbors but whatever)
                    self.matr[y][x] = Some(tile);
                }
            }
        }
        self
    }

    pub fn set_tile_types(&mut self, textures: &'a TileTextures<'a>) {
        unsafe {
            for y in 1..self.size as usize - 1 {
                for x in 1..self.size as usize - 1 {
                    let current_tile_reference: *mut Tile;
                    let mut neighbors: u8 = 0b0000;
                    if let Some(tile) = &mut self.matr[y][x] {
                        current_tile_reference = tile;

                        if let Some(other_tile) = self.matr[y - 1][x] {
                            if other_tile.max_z == (*current_tile_reference).max_z {
                                neighbors |= NeighborLocation::Top as u8;
                            }
                        }

                        if let Some(other_tile) = self.matr[y + 1][x] {
                            if other_tile.max_z == (*current_tile_reference).max_z {
                                neighbors |= NeighborLocation::Bottom as u8;
                            }
                        }

                        if let Some(other_tile) = self.matr[y][x - 1] {
                            if other_tile.max_z == (*current_tile_reference).max_z {
                                neighbors |= NeighborLocation::Left as u8;
                            }
                        }

                        if let Some(other_tile) = self.matr[y][x + 1] {
                            if other_tile.max_z == (*current_tile_reference).max_z {
                                neighbors |= NeighborLocation::Right as u8;
                            }
                        }

                        match neighbors {
                            0b0000 => (*current_tile_reference).tile_type = Some(&textures.t0),

                            0b1000 => (*current_tile_reference).tile_type = Some(&textures.t1_tr),
                            0b0100 => (*current_tile_reference).tile_type = Some(&textures.t1_tl),
                            0b0010 => (*current_tile_reference).tile_type = Some(&textures.t1_bl),
                            0b0001 => (*current_tile_reference).tile_type = Some(&textures.t1_br),

                            0b1100 => {
                                (*current_tile_reference).tile_type = Some(&textures.t2_tl_tr)
                            }
                            0b1010 => {
                                (*current_tile_reference).tile_type = Some(&textures.t2_bl_tr)
                            }
                            0b1001 => {
                                (*current_tile_reference).tile_type = Some(&textures.t2_br_tr)
                            }

                            0b0110 => {
                                (*current_tile_reference).tile_type = Some(&textures.t2_tl_bl)
                            }
                            0b0101 => {
                                (*current_tile_reference).tile_type = Some(&textures.t2_tl_br)
                            }

                            0b0011 => {
                                (*current_tile_reference).tile_type = Some(&textures.t2_bl_br)
                            }

                            0b1110 => {
                                (*current_tile_reference).tile_type = Some(&textures.t3_tl_bl_tr)
                            }
                            0b1101 => {
                                (*current_tile_reference).tile_type = Some(&textures.t3_tl_br_tr)
                            }
                            0b1011 => {
                                (*current_tile_reference).tile_type = Some(&textures.t3_bl_br_tr)
                            }
                            0b0111 => {
                                (*current_tile_reference).tile_type = Some(&textures.t3_tl_bl_br)
                            }

                            0b1111 => (*current_tile_reference).tile_type = Some(&textures.t4),
                            _ => (),
                        }
                    }
                }
            }
        }
    }
}
