use bevy::prelude::*;

use crate::map::MapPlugin;
use crate::unit::UnitPlugin;
use crate::window::DisplayPlugin;

mod prelude {
    use bevy::math::Vec2;

    pub const MAP_SIZE: f32 = 11.;
    pub const TILE_SIZE: Vec2 = Vec2::new(16.0, 17.0);
}

mod draft;
mod map;
mod unit;
mod window;

fn main() {
    App::new()
        .add_plugins((DisplayPlugin, MapPlugin, UnitPlugin))
        .run();
}
