use std::f32::consts::{E, PI};

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, (move_camera, zoom_camera));
    }
}

fn move_camera(
    mut query: Query<&mut Transform, With<Camera>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut camera_transform = query.single_mut();
    let camera_position = camera_transform.translation;
    if keys.pressed(KeyCode::A) {
        camera_transform.rotate_around(
            camera_position,
            Quat::from_rotation_y(-PI * time.delta_seconds()),
        )
    }
    if keys.pressed(KeyCode::D) {
        camera_transform.rotate_around(
            camera_position,
            Quat::from_rotation_y(PI * time.delta_seconds()),
        )
    }
    if keys.pressed(KeyCode::W) {
        let camera_local_x = camera_transform.local_x();
        camera_transform.rotate_around(
            camera_position,
            Quat::from_axis_angle(camera_local_x, -PI * time.delta_seconds()),
        )
    }
    if keys.pressed(KeyCode::S) {
        let camera_local_x = camera_transform.local_x();
        camera_transform.rotate_around(
            camera_position,
            Quat::from_axis_angle(camera_local_x, PI * time.delta_seconds()),
        )
    }
}

fn zoom_camera(
    mut query: Query<&mut Transform, With<Camera>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    let mut camera = query.single_mut();
    for ev in scroll_evr.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                camera.scale *= 1.2_f32.powi(-ev.y as i32);
            }
            MouseScrollUnit::Pixel => {
                camera.scale *= 1.2_f32.powi(-ev.y as i32);
            }
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(50., 4., 50.))
            .looking_at(Vec3::new(50., 0., 50.), Vec3::Y),
        projection: Projection::Orthographic(OrthographicProjection {
            scale: 1.,
            near: -100000000.,
            far: 100000000.,
            ..Default::default()
        }),
        ..Default::default()
    });
}
