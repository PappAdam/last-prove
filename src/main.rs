mod camera;
mod testscene;
use bevy::prelude::*;
use camera::CameraPlugin;
use testscene::TestScenePlugin;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::rgb(1., 1., 1.),
            brightness: 0.1,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(TestScenePlugin)
        .add_plugins(CameraPlugin)
        .run()
}
