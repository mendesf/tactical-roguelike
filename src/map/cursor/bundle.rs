use bevy::prelude::*;

use crate::map::{Cursor, Position};

#[derive(Bundle, Default)]
pub struct CursorBundle<T>
where
    T: Component + Cursor,
{
    pub cursor: T,
    pub sprite: SpriteSheetBundle,
    pub position: Position,
}
