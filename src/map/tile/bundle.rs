use bevy::prelude::*;

use crate::map::Position;

#[derive(Bundle, Default)]
pub struct TileBundle {
    pub sprite: SpriteSheetBundle,
    pub position: Position,
}
