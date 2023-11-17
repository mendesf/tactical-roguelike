use crate::map::Coordinates;
use bevy::prelude::*;

pub fn _point_to_coordinates(map_size: Vec2, tile_size: Vec2, point: Vec2) -> Coordinates {
    let half_map_size = map_size / 2.;

    let calc = |point: f32| {
        let coordinate = point / tile_size.x;
        if map_size.x % 2. == 0. {
            coordinate.floor()
        } else {
            coordinate.round()
        }
    };

    let x = (calc(point.x) + half_map_size.x).floor();
    info!("x {x}");
    let y = (calc(-point.y) + half_map_size.x).floor();
    info!("y {y}");

    Coordinates(Vec2::new(x, y))
}

pub fn _coordinates_to_point(map_size: Vec2, tile_size: Vec2, coordinates: Coordinates) -> Vec2 {
    let Coordinates(coordinates) = coordinates;
    let half_tile_size = tile_size / 2.;
    let x = coordinates.x * tile_size.x + half_tile_size.x - map_size.x * half_tile_size.x;
    let y = map_size.y * half_tile_size.y - (coordinates.y * tile_size.y + half_tile_size.y);

    Vec2::new(x, y)
}
