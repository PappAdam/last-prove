mod camera;
mod lighting;
mod map;
mod testscene;
use bevy::prelude::*;
use camera::CameraPlugin;
use lighting::LightingPlugin;
use map::MapPlugin;
use testscene::TestScenePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MapPlugin)
        .add_plugins(TestScenePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(LightingPlugin)
        .run()
}
