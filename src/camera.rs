use std::f32::consts::PI;

use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::{lerp::lerp, map::MAP_SIZE};

const CAMERA_SPEED: f32 = 1000.;
const CAMERA_START_SCALE: f32 = 0.1;
const CAMERA_LERP_SPEED: f32 = 13.;
const MIN_TILT: f32 = PI / 3.;
const MAX_TILT: f32 = 0.;

#[derive(Component)]
struct CameraTarget {
    scale: f32,
    pos: Vec3,
    tilt: f32,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, update_camera)
            .add_systems(
                Update,
                (move_camera, rotate_camera, zoom_camera).before(update_camera),
            );
    }
}

fn move_camera(
    mut query: Query<(&mut Transform, &Projection, &mut CameraTarget), With<Camera>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (camera_transform, camera_projection, mut camera_target) = query.single_mut();
    let camera_projection = projection_to_ortographic_projection(camera_projection);

    let mut movement_dir = Vec3::ZERO;

    if keys.pressed(KeyCode::A) {
        movement_dir += camera_transform.left();
    }

    if keys.pressed(KeyCode::D) {
        movement_dir += camera_transform.right();
    }

    if keys.pressed(KeyCode::W) {
        movement_dir += camera_transform.left().cross(Vec3::Y);
    }

    if keys.pressed(KeyCode::S) {
        movement_dir += camera_transform.right().cross(Vec3::Y);
    }

    let new_target_pos = camera_target.pos
        + movement_dir.normalize_or_zero()
            * CAMERA_SPEED
            * time.delta_seconds()
            * camera_projection.scale;

    if new_target_pos.x > 0. && new_target_pos.x < MAP_SIZE as f32 {
        camera_target.pos.x = new_target_pos.x;
    }
    if new_target_pos.z > 0. && new_target_pos.z < MAP_SIZE as f32 {
        camera_target.pos.z = new_target_pos.z;
    }
}

fn rotate_camera(
    mut query: Query<(&mut Transform, &mut CameraTarget), With<Camera>>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    if !mouse_buttons.pressed(MouseButton::Middle) {
        mouse_motion.clear();
        return;
    }
    let (mut camera_transform, mut target) = query.single_mut();
    let camera_position = camera_transform.translation;

    for motion_event in mouse_motion.read() {
        let camera_local_x = camera_transform.local_x();
        let mut tilt = -motion_event.delta.y / 1000.;
        target.tilt += tilt;

        if target.tilt <= MAX_TILT || target.tilt >= MIN_TILT {
            target.tilt -= tilt;
            tilt = 0.;
        }

        camera_transform
            .rotate_around(camera_position, Quat::from_axis_angle(camera_local_x, tilt));
        camera_transform.rotate_around(
            camera_position,
            Quat::from_axis_angle(Vec3::Y, -motion_event.delta.x / 1000.),
        )
    }
}

fn zoom_camera(
    mut q: Query<&mut CameraTarget, With<Camera>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    let target: &mut CameraTarget = q.single_mut().into_inner();
    for ev in scroll_evr.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                target.scale *= 1.2_f32.powi(-ev.y as i32);
            }
            MouseScrollUnit::Pixel => {
                target.scale *= 1.2_f32.powi(ev.y as i32);
            }
        }
    }

    target.scale = target.scale.clamp(0.007, 0.1);
}

fn update_camera(
    mut query: Query<(&mut Projection, &CameraTarget, &mut Transform), With<Camera>>,
    time: Res<Time>,
) {
    let (mut _proj, target, mut transform) = query.single_mut();
    let projection = mut_projection_to_mut_ortographic_projection(_proj.into_inner());

    let zoom = lerp(
        projection.scale.sqrt(),
        target.scale.sqrt(),
        CAMERA_LERP_SPEED * time.delta_seconds(),
    );

    projection.scale += zoom;

    transform.translation = transform
        .translation
        .lerp(target.pos, CAMERA_LERP_SPEED * time.delta_seconds() * 2.5);
}

fn spawn_camera(mut commands: Commands) {
    let map_center = (MAP_SIZE / 2) as f32;
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(map_center - 1., 1., map_center - 1.))
                .looking_at(Vec3::new(map_center, 0., map_center), Vec3::Y),
            projection: ortographinc_projection(CAMERA_START_SCALE),
            ..Default::default()
        },
        CameraTarget {
            scale: CAMERA_START_SCALE,
            pos: Vec3::new(map_center - 1., 1., map_center - 1.),
            tilt: PI / 4.,
        },
    ));
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
