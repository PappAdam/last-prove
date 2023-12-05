use bevy::{prelude::*, render::primitives::Frustum, window::PresentMode};

use crate::{map::MAP_SIZE, lerp::lerp};

const CUBE_COLOR: Color = Color::Rgba {
    red: 141. / 255.,
    green: 200. / 255.,
    blue: 229. / 255.,
    alpha: 1.,
};

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, modify_window_present_mode);
        // app.add_systems(Update, half_camera_frustrum_size);
    }
}

fn half_camera_frustrum_size(mut query: Query<(&mut Frustum, &Transform)>, mut gizmos: Gizmos) {
    let (mut frustrum, transform) = query.single_mut();
    let mut frustrum = frustrum.into_inner();
    let mut left_space = frustrum.half_spaces[0];
    let mut top_space = frustrum.half_spaces[1];
    let mut right_space = frustrum.half_spaces[2];
    let mut bottom_space = frustrum.half_spaces[3];

    let left_d = lerp(left_space.d(), -right_space.d(), 0.5);
    
    for half_space in frustrum.half_spaces {
        gizmos.ray(
            (-half_space.normal() * half_space.d()).into(),
            half_space.normal().cross(transform.local_z().into()).into(),
            Color::RED,
        );
        // gizmos.line((half_space.normal() * half_space.d()).into(), (half_space.normal() * half_space.d() * 0.9).into(), Color::BLACK);
        // dbg!(half_space.d());
    }
}

fn modify_window_present_mode(mut query: Query<&mut Window>) {
    let window = query.single_mut().into_inner();
    window.present_mode = PresentMode::Immediate;
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
