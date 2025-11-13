use bevy::prelude::*;

use cursor::CursorPlugin;

mod components;
mod cursor;
mod resource;
mod systems;
mod tile;

pub use components::*;
pub use cursor::components::*;
pub use resource::*;
pub use tile::{bundle::*, components::*};

pub const MAP_SIZE: f32 = 11.;
pub const TILE_SIZE: Vec2 = Vec2::new(16.0, 17.0);
pub const SCALE_FACTOR: f32 = 4.;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::default())
            .add_plugins(CursorPlugin)
            .add_systems(Startup, systems::setup);
        // .add_systems(Update, (systems::update_z_index));
    }
}
