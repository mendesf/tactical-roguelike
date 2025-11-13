use bevy::prelude::*;

mod bundle;
pub mod components;
mod systems;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup)
            .add_systems(Update, systems::hovering);
    }
}
