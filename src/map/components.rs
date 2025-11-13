use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub};

use bevy::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub enum Side {
    Left,
    #[default]
    Center,
    Right,
}

#[derive(Component, Copy, Clone, Default, Debug)]
pub struct Coordinates(pub i32, pub i32, pub Side);

impl Hash for Coordinates {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

impl PartialEq for Coordinates {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl Eq for Coordinates {}

impl Sub for Coordinates {
    type Output = IVec2;
    fn sub(self, rhs: Self) -> Self::Output {
        IVec2::new(self.0, self.1) - IVec2::new(rhs.0, rhs.1)
    }
}

impl Sub<Floor> for Coordinates {
    type Output = Self;
    fn sub(self, rhs: Floor) -> Self::Output {
        let x = self.0 - rhs.0;
        let y = self.1 - rhs.0;
        Self(x, y, self.2)
    }
}

impl Add<Floor> for Coordinates {
    type Output = Self;
    fn add(self, rhs: Floor) -> Self::Output {
        let x = self.0 + rhs.0;
        let y = self.1 + rhs.0;
        Self(x, y, self.2)
    }
}

#[derive(Component, Clone, Ord, PartialOrd, Eq, PartialEq, Copy, Default, Debug)]
pub struct Floor(pub i32);

#[derive(Component, Clone, Copy, Default, Debug)]
pub struct Order(pub f32);

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct Position {
    pub coordinates: Coordinates,
    pub floor: Floor,
    pub order: Order,
}
