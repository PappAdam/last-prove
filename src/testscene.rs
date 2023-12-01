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
        app.add_systems(Startup, spawn_directional_light);
    }
}

fn spawn_directional_light(mut commands: Commands) {
    // commands.spawn(DirectionalLightBundle {
    //     transform: Transform::from_translation(Vec3::new(1.8, 1., 1.4))
    //         .looking_at(Vec3::ZERO, Vec3::Y),
    //     directional_light: DirectionalLight {
    //         color: Color::hex("FFFCE9").unwrap(),
    //         illuminance: 32000.,
    //         shadows_enabled: true,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });
}
