use bevy::prelude::*;

pub struct Data {
    pub index: usize,
    pub rect: Rect,
}

pub trait Cursor: Component {
    fn new() -> Self;

    fn data(&self) -> &Data;
}

#[derive(Component)]
pub struct HoverCursor(pub Data);

#[derive(Component)]
pub struct SelectCursor(pub Data);

impl Cursor for HoverCursor {
    fn new() -> Self {
        Self(Data {
            index: 0,
            rect: Rect::new(0., 16., 16., 25.),
        })
    }

    fn data(&self) -> &Data {
        &self.0
    }
}

impl Cursor for SelectCursor {
    fn new() -> Self {
        Self(Data {
            index: 1,
            rect: Rect::new(16., 16., 32., 25.),
        })
    }

    fn data(&self) -> &Data {
        &self.0
    }
}
