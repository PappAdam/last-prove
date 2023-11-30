use bevy::prelude::*;

use crate::map::MAP_SIZE;

const CIRCLE_COLOR: Color = Color::Rgba {
    red: 229. / 255.,
    green: 170. / 255.,
    blue: 141. / 255.,
    alpha: 1.,
};
const CUBE_COLOR: Color = Color::Rgba {
    red: 141. / 255.,
    green: 200. / 255.,
    blue: 229. / 255.,
    alpha: 1.,
};

pub struct TestScenePlugin;

impl Plugin for TestScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_circle)
            .add_systems(Startup, spawn_cube)
            .add_systems(Startup, spawn_sphere)
            .add_systems(Startup, spawn_directional_light);
    }
}

fn spawn_circle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0., -5., 0.)),
        mesh: meshes.add(
            shape::Cylinder {
                radius: 4.,
                height: 10.,
                resolution: 32,
                segments: 1,
            }
            .into(),
        ),
        material: materials.add(CIRCLE_COLOR.into()),
        ..default()
    });
}

fn spawn_cube(
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

fn spawn_sphere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(-2.0, 0.5, -1.),
        mesh: meshes.add(
            shape::UVSphere {
                radius: 0.5,
                sectors: 32,
                stacks: 64,
            }
            .into(),
        ),
        material: materials.add(CUBE_COLOR.into()),
        ..default()
    });
}

fn spawn_directional_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::new(1.8, 1., 1.4))
            .looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            color: Color::hex("FFFCE9").unwrap(),
            illuminance: 32000.,
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });
}
