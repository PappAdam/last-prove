use std::ops::Range;

use bevy::{prelude::*, render::render_resource::PrimitiveTopology};

use super::{
    debug_map,
    materials::{GRASS_MATERIAL, WATER_MATERIAL},
    Map, MAP_SIZE,
};

pub struct MapMeshPlugin;

impl Plugin for MapMeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_mesh_to_map.after(debug_map));
    }
}

fn add_mesh_to_map(
    mut commands: Commands,
    mut query: Query<(Entity, &Map)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (map_entity, map) = query.single_mut();
    let mut map_entity = commands.entity(map_entity);
    map_entity.insert(PbrBundle {
        mesh: meshes.add(map_to_mesh(map)),
        material: materials.add(GRASS_MATERIAL),
        ..default()
    });

    map_entity.with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: meshes.add(water_mesh()),
            material: materials.add(WATER_MATERIAL),
            ..default()
        });
    });
}

fn map_to_mesh(map: &Map) -> Mesh {
    let mut tile_quads: Vec<Vec<Range<usize>>> = vec![];

    let mut vertices = vec![];
    let mut indicies = vec![];
    let mut tile_index = 0;
    //Iterating over rows
    for (y, _) in map.matrix.iter().enumerate() {
        //Iterating over columns, using while so I can modify x.
        tile_quads.push(vec![]);
        let mut x = 0;
        while x < MAP_SIZE {
            //If a tile is solid, we search for the next water tile in that column.
            if map.matrix[y][x].solid {
                for offset in x..MAP_SIZE {
                    if map.matrix[y][offset].solid {
                        //Searching for the next water tile on the column, increasing offset.
                        continue;
                    }
                    tile_quads[y].push(x..offset);
                    x = offset;
                    break;
                }
            }
            x += 1;
        }
    }

    //We can skip all previously checked tiles.
    let mut y = 0;
    while y < MAP_SIZE {
        let row = tile_quads[y].clone();
        for section in row {
            let mut y_offset = 1;
            let mut index = 0;
            while tile_quads[y + y_offset]
                .iter()
                .enumerate()
                .find(|(i, foundsection)| {
                    index = *i;
                    &&section == foundsection
                })
                .is_some()
            {
                tile_quads[y + y_offset].remove(index);
                y_offset += 1;
            }
            let (mut square_vertices, mut tile_rounded_quad) = rounded_quad(
                [
                    Vec3::new(section.start as f32, 0., y as f32),
                    Vec3::new(section.start as f32, 0., y as f32 + y_offset as f32),
                    Vec3::new(section.end as f32, 0., y as f32 + y_offset as f32),
                    Vec3::new(section.end as f32, 0., y as f32),
                ],
                tile_index * 30,
            );
            vertices.append(&mut square_vertices);
            indicies.append(&mut tile_rounded_quad);
            tile_index += 1;
        }
        y += 1;
    }
    Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_computed_flat_normals()
}

fn water_mesh() -> Mesh {
    let (water_vertices, _) = quad(
        [
            Vec3::new(0., -0.2, 0.),
            Vec3::new(0., -0.2, MAP_SIZE as f32),
            Vec3::new(MAP_SIZE as f32, -0.2, MAP_SIZE as f32),
            Vec3::new(MAP_SIZE as f32, -0.2, 0.),
        ],
        0,
    );
    Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, water_vertices)
        .with_computed_flat_normals()
}

fn rounded_quad(
    positions: [Vec3; 4],
    start_index: u32, //We return a Vec<Vertex> instead of a [Vertex; 4], so we can append the return value to Vecs without converting types.
) -> (Vec<Vec3>, Vec<u32>) {
    //Initializing all needed values.
    let top_corner_0 = positions[0];
    let top_corner_1 = positions[1];
    let top_corner_2 = positions[2];
    let top_corner_3 = positions[3];
    let bottom_corner_0 = top_corner_0 - Vec3::new(0.1, 0.2, 0.1);
    let bottom_corner_1 = top_corner_1 - Vec3::new(0.1, 0.2, -0.1);
    let bottom_corner_2 = top_corner_2 - Vec3::new(-0.1, 0.2, -0.1);
    let bottom_corner_3 = top_corner_3 - Vec3::new(-0.1, 0.2, 0.1);

    //Creating each quad (5 in total)
    //Each quad will have it's own vertices in order to have edges in render. (4 * 5 = 20 vertices in total)
    //Start index is increasing by 4 after each quad.
    let (mut top_quad_vertices, top_quad) = quad(
        [top_corner_0, top_corner_1, top_corner_2, top_corner_3],
        start_index,
    );
    let (mut side_quad_0_vertices, side_quad_0) = quad(
        [top_corner_0, bottom_corner_0, bottom_corner_1, top_corner_1],
        start_index + 6,
    );
    let (mut side_quad_1_vertices, side_quad_1) = quad(
        [top_corner_1, bottom_corner_1, bottom_corner_2, top_corner_2],
        start_index + 12,
    );
    let (mut side_quad_2_vertices, side_quad_2) = quad(
        [top_corner_2, bottom_corner_2, bottom_corner_3, top_corner_3],
        start_index + 18,
    );
    let (mut side_quad_3_vertices, side_quad_3) = quad(
        [top_corner_3, bottom_corner_3, bottom_corner_0, top_corner_0],
        start_index + 24,
    );
    //End of quad creation

    //We chain all 20 vertices of the 5 quads together
    let mut vertices = Vec::with_capacity(20);
    vertices.append(&mut top_quad_vertices);
    vertices.append(&mut side_quad_0_vertices);
    vertices.append(&mut side_quad_1_vertices);
    vertices.append(&mut side_quad_2_vertices);
    vertices.append(&mut side_quad_3_vertices);

    //We chain all 5 quads together.
    let quads = vec![top_quad, side_quad_0, side_quad_1, side_quad_2, side_quad_3].concat();
    //We return the chained quads.
    return (vertices, quads);
}

fn quad(
    corners: [Vec3; 4],
    start_index: u32, //We return a Vec<Vertex> instead of a [Vertex; 4], so we can append the return value to Vecs without converting types.
) -> (Vec<Vec3>, Vec<u32>) {
    let indicies = vec![
        start_index + 0,
        start_index + 1,
        start_index + 2,
        start_index + 3,
        start_index + 4,
        start_index + 5,
    ];
    let corners = vec![
        corners[0], corners[1], corners[2], corners[0], corners[2], corners[3],
    ];
    //Returning the created values.
    return (corners.to_vec(), indicies);
}
