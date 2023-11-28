use bevy::prelude::*;

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
            .add_systems(Startup, spawn_point_light);
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
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(1.0, 0.5, 0.3),
        mesh: meshes.add(shape::Cube::new(1.).into()),
        material: materials.add(CUBE_COLOR.into()),
        ..default()
    });
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

fn spawn_point_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
