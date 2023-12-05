mod camera;
pub mod lerp;
mod lighting;
mod map;
mod testplugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use lighting::LightingPlugin;
use map::MapPlugin;
use testplugin::TestPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MapPlugin)
        .add_plugins(TestPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(LightingPlugin)
        .run()
}
