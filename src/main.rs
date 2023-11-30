mod camera;
mod testscene;
mod map;
use bevy::prelude::*;
use camera::CameraPlugin;
use map::MapPlugin;
use testscene::TestScenePlugin;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::rgb(1., 1., 1.),
            brightness: 0.1,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(MapPlugin)
        .add_plugins(TestScenePlugin)
        .add_plugins(CameraPlugin)
        .run()
}
