use std::f32::consts::PI;

use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, move_camera);
    }
}

fn move_camera(mut query: Query<&mut Transform, With<Camera>>, keys: Res<Input<KeyCode>>, time: Res<Time>) {
    let mut camera_transform = query.single_mut();
    if keys.pressed(KeyCode::A) {
        camera_transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(-PI * time.delta_seconds()))
    }
    if keys.pressed(KeyCode::D) {
        camera_transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(PI * time.delta_seconds()))
    }
    if keys.pressed(KeyCode::W) {
        let camera_local_x = camera_transform.local_x();
        camera_transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(camera_local_x, -PI * time.delta_seconds()))
    }
    if keys.pressed(KeyCode::S) {
        let camera_local_x = camera_transform.local_x();
        camera_transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(camera_local_x, PI * time.delta_seconds()))
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(5., 4., 5.)).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
