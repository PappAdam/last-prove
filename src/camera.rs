use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::map::MAP_SIZE;

const CAMERA_SPEED: f32 = 1000.;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, (move_camera, rotate_camera, zoom_camera));
    }
}

fn move_camera(
    mut query: Query<(&mut Transform, &Projection), With<Camera>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut camera_transform, camera_projection) = query.single_mut();
    let camera_projection = projection_to_ortographic_projection(camera_projection);

    let mut movement_direction = Vec3::ZERO;

    if keys.pressed(KeyCode::A) {
        movement_direction += camera_transform.left();
    }
    if keys.pressed(KeyCode::D) {
        movement_direction += camera_transform.right();
    }
    if keys.pressed(KeyCode::W) {
        movement_direction += camera_transform.left().cross(Vec3::Y);
    }
    if keys.pressed(KeyCode::S) {
        movement_direction += camera_transform.right().cross(Vec3::Y);
    }
    camera_transform.translation += movement_direction.normalize_or_zero() * camera_projection.scale * time.delta_seconds() * CAMERA_SPEED;
}

fn rotate_camera(
    mut query: Query<&mut Transform, With<Camera>>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    if !mouse_buttons.pressed(MouseButton::Middle) {
        return;
    }
    let mut camera_transform = query.single_mut();
    let camera_position = camera_transform.translation;

    for motion_event in mouse_motion.read() {
        let camera_local_x = camera_transform.local_x();
        camera_transform.rotate_around(
            camera_position,
            Quat::from_axis_angle(camera_local_x, -motion_event.delta.y / 1000.),
        );
        camera_transform.rotate_around(
            camera_position,
            Quat::from_axis_angle(Vec3::Y, -motion_event.delta.x / 1000.),
        )
    }
}
fn zoom_camera(
    mut query: Query<&mut Projection, With<Camera>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    let projection = mut_projection_to_mut_ortographic_projection(query.single_mut().into_inner());

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

fn projection_to_ortographic_projection<'a>(
    projection: &'a Projection,
) -> &'a OrthographicProjection {
    if let Projection::Orthographic(ortographic_projection) = projection {
        ortographic_projection
    } else {
        panic!("Persepective camera zoom isn't supported!")
    }
}

fn mut_projection_to_mut_ortographic_projection<'a>(
    projection: &'a mut Projection,
) -> &'a mut OrthographicProjection {
    if let Projection::Orthographic(ortographic_projection) = projection {
        ortographic_projection
    } else {
        panic!("Persepective camera zoom isn't supported!")
    }
}
