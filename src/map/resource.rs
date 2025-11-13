use std::collections::HashMap;

use bevy::prelude::*;

use super::{components::Coordinates, Position, Side, MAP_SIZE, SCALE_FACTOR, TILE_SIZE};

const ISOMETRIC_I: Vec2 = Vec2::new(1f32, -0.5f32);
const ISOMETRIC_J: Vec2 = Vec2::new(-1f32, -0.5f32);

#[derive(Resource)]
pub struct Map {
    pub size: Vec2,
    pub tile_size: Vec2,
    pub tiles: HashMap<Coordinates, Vec<Entity>>,
    half_size: Vec2,
    half_tile_size: Vec2,
}

impl Default for Map {
    fn default() -> Map {
        Map::new(Vec2::splat(MAP_SIZE), TILE_SIZE, SCALE_FACTOR)
    }
}

impl Map {
    pub fn new(size: Vec2, tile_size: Vec2, scale_factor: f32) -> Self {
        let tiles = HashMap::new();
        let tile_size = tile_size * scale_factor;
        let half_tile_size = tile_size / 2.0;
        let half_size = size / 2.0;

        Self {
            size,
            half_size,
            tiles,
            tile_size,
            half_tile_size,
        }
    }

    pub fn index_to_coordinates(&self, index: usize) -> Coordinates {
        let x = (index as f32 % self.size.x) as i32;
        let y = (index as f32 / self.size.x).floor() as i32;
        Coordinates(x, y, Side::Center)
    }

    pub fn point_to_coordinates(&self, point: Vec2) -> Coordinates {
        let x = point.x;
        let y = point.y;

        let a = ISOMETRIC_I.x * self.half_tile_size.x;
        let b = ISOMETRIC_J.x * self.half_tile_size.x;
        let c = ISOMETRIC_I.y * self.half_tile_size.y;
        let d = ISOMETRIC_J.y * self.half_tile_size.y;

        let det = 1. / (a * d - b * c);

        let inv_a = det * d;
        let inv_b = det * -b;
        let inv_c = det * -c;
        let inv_d = det * a;

        let coordinate_x = (x * inv_a + y * inv_b) + self.half_size.x;
        let coordinate_y = (x * inv_c + y * inv_d) + self.half_size.y;
        let coordinate_x_i32 = coordinate_x.ceil() as i32;
        let coordinate_y_i32 = coordinate_y.ceil() as i32;
        info!("x: {coordinate_x} | x_i32: ${coordinate_x_i32} | y: {coordinate_y} y_i32: {coordinate_y_i32}");

        let diff = coordinate_x_i32 as f32 - coordinate_x;

        let side = match diff {
            0.0..=0.5 => Side::Right,
            _ => Side::Left
        };

        Coordinates(coordinate_x_i32, coordinate_y_i32, side)
    }

    pub fn coordinates_to_point(&self, coordinates: Coordinates) -> Vec2 {
        let x = coordinates.0 as f32;
        let y = coordinates.1 as f32;

        let a = ISOMETRIC_I.x * x * self.half_tile_size.x;
        let b = ISOMETRIC_J.x * y * self.half_tile_size.x;
        let c = ISOMETRIC_I.y * x * self.half_tile_size.y;
        let d = ISOMETRIC_J.y * y * self.half_tile_size.y;

        let y_offset = self.half_size.y * self.half_tile_size.y;

        Vec2::new(a + b, c + d + y_offset)
    }

    pub fn position_to_translation(&self, position: &Position) -> Vec3 {
        let coordinates = position.coordinates - position.floor;
        let point = self.coordinates_to_point(coordinates);
        let z = position.order.0 / 5.
            + (coordinates.1 + coordinates.0) as f32 / 20.
            + position.floor.0 as f32 / 2.;
        Vec3::from((point, z))
    }

    pub fn position_to_translation_cursor(&self, position: &Position) -> Vec3 {
        let coordinates = position.coordinates - position.floor;
        let point = self.coordinates_to_point(coordinates);
        let bla = self.half_tile_size.x * position.floor.0 as f32;
        let point = Vec2::new(point.x + bla, point.y);
        let z = position.order.0 / 5.
            + (coordinates.1 + coordinates.0) as f32 / 20.
            + position.floor.0 as f32 / 2.;
        Vec3::from((point, z))
    }

    pub fn in_bounds(&self, coordinates: Coordinates) -> bool {
        let x = coordinates.0 as f32;
        let y = coordinates.1 as f32;

        x >= 0. && x < self.size.x && y >= 0. && y < self.size.y
    }
}
