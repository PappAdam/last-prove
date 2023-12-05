use bevy::prelude::*;

use crate::{lerp::lerp, map::MAP_SIZE};

const CUBE_COLOR: Color = Color::Rgba {
    red: 141. / 255.,
    green: 200. / 255.,
    blue: 229. / 255.,
    alpha: 1.,
};

pub struct TestScenePlugin;

impl Plugin for TestScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cubes);
    }
}

fn spawn_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for x in 0..MAP_SIZE {
        commands.spawn(PbrBundle {
            transform: Transform::from_xyz(x as f32, 0.25, 50.),
            mesh: meshes.add(shape::Cube::new(0.5).into()),
            material: materials.add(CUBE_COLOR.into()),
            ..default()
        });
    }
}
