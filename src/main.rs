use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::map::MapPlugin;
use crate::unit::UnitPlugin;
use crate::window::DisplayPlugin;

mod map;
mod unit;
mod window;

fn main() {
    App::new()
        .add_plugins(DisplayPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins((MapPlugin, UnitPlugin))
        .run();
}
