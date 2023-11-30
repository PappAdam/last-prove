use std::f32::consts::{E, PI};

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::map::MAP_SIZE;

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
        camera_transform.translation.
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
    mut query: Query<&mut Projection, With<Camera>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    let projection = {
        if let Projection::Orthographic(ortographic_projection) = (query.single_mut()).into_inner()
        {
            ortographic_projection
        } else {
            panic!("Persepective camera zoom isn't supported!")
        }
    };
    for ev in scroll_evr.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                projection.scale *= 1.2_f32.powi(-ev.y as i32);
            }
            MouseScrollUnit::Pixel => {
                projection.scale *= 1.2_f32.powi(ev.y as i32);
            }
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    let map_center = (MAP_SIZE / 2) as f32;
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(map_center - 1., 1., map_center - 1.))
            .looking_at(Vec3::new(map_center, 0., map_center), Vec3::Y),
        projection: ortographinc_projection(0.1),
        ..Default::default()
    });
}

fn ortographinc_projection(camera_scale: f32) -> Projection {
    Projection::Orthographic(OrthographicProjection {
        scale: camera_scale,
        near: -1000.,
        far: 1000.,
        ..Default::default()
    })
}
