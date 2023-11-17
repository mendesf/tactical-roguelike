use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::prelude::*;
use crate::unit;

const I_HAT: Vec2 = Vec2::new(1f32, -0.5f32);
const J_HAT: Vec2 = Vec2::new(-1f32, -0.5f32);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::default())
            .add_systems(Startup, (spawn_tiles, spawn_cursor))
            .add_systems(Update, show_cursor.run_if(not(unit::moving())));
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Black,
    White,
}

#[derive(Component, Copy, Clone, PartialEq, Default, Debug)]
pub struct Coordinates(pub Vec2);

#[derive(Resource)]
pub struct Map {
    pub size: Vec2,
    pub tile_size: Vec2,
    tiles: Vec<TileType>,
    half_size: Vec2,
    half_tile_size: Vec2,
}

impl Map {
    pub fn new(size: Vec2, tile_size: Vec2) -> Self {
        let tiles = vec![TileType::Black; (size.x * size.y) as usize];
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
        let x = index as f32 % self.size.x;
        let y = (index as f32 / self.size.x).floor();
        Coordinates(Vec2::new(x, y))
    }

    pub fn point_to_coordinates(&self, point: Vec2) -> Coordinates {
        let x = point.x;
        let y = point.y;

        let a = I_HAT.x * self.half_tile_size.x;
        let b = J_HAT.x * self.half_tile_size.x;
        let c = I_HAT.y * self.half_tile_size.y;
        let d = J_HAT.y * self.half_tile_size.y;

        let det = 1. / (a * d - b * c);

        let inv_a = det * d;
        let inv_b = det * -b;
        let inv_c = det * -c;
        let inv_d = det * a;

        Coordinates(Vec2::new(
            ((x * inv_a + y * inv_b) + self.half_size.x).ceil(),
            ((x * inv_c + y * inv_d) + self.half_size.y).ceil(),
        ))
    }

    pub fn coordinates_to_point(&self, coordinates: Coordinates) -> Vec2 {
        let Coordinates(coordinates) = coordinates;

        let x = coordinates.x;
        let y = coordinates.y;

        let a = I_HAT.x * x * self.half_tile_size.x;
        let b = J_HAT.x * y * self.half_tile_size.x;
        let c = I_HAT.y * x * self.half_tile_size.y;
        let d = J_HAT.y * y * self.half_tile_size.y;

        let y_offset = self.half_size.y * self.half_tile_size.y;

        Vec2::new(a + b, c + d + y_offset)
    }

    pub fn in_bounds(&self, coordinates: Coordinates) -> bool {
        let Coordinates(coordinates) = coordinates;
        coordinates.x >= 0.
            && coordinates.x < self.size.x
            && coordinates.y >= 0.
            && coordinates.y < self.size.y
    }
}

impl Default for Map {
    fn default() -> Map {
        Map::new(Vec2::splat(MAP_SIZE), TILE_SIZE)
    }
}

pub fn spawn_tiles(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
) {
    let columns = 11;
    let rows = 10;
    let texture_handle = asset_server.load("textures/Isometric_MedievalFantasy_Tiles.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle.clone(), TILE_SIZE, columns, rows, None, None);
    let texture_atlas_handle = &texture_atlases.add(texture_atlas);

    map.tiles.iter().enumerate().for_each(|(i, tile)| {
        let coordinates = map.index_to_coordinates(i);
        let point = map.coordinates_to_point(coordinates);

        commands.spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(92),
            transform: Transform::from_translation(Vec3::new(point.x, point.y, 0.)),
            ..default()
        });
    });
}

#[derive(Component)]
pub struct TileCursor;

#[derive(Component)]
pub struct TileSelected;

pub fn spawn_cursor(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("textures/TRPGIsometricAssetPack_MapIndicators.png");
    let mut texture_atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(32., 25.));
    texture_atlas.add_texture(Rect::new(0., 16., 16., 25.));
    texture_atlas.add_texture(Rect::new(16., 16., 32., 25.));
    let texture_atlas_handle = &texture_atlases.add(texture_atlas);

    let mut cursor_sprite = TextureAtlasSprite::new(0);
    cursor_sprite.anchor = Anchor::BottomCenter;

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: cursor_sprite,
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            visibility: Visibility::Hidden,
            ..default()
        },
        TileCursor,
    ));

    let mut selected_sprite = TextureAtlasSprite::new(1);
    selected_sprite.anchor = Anchor::BottomCenter;

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: selected_sprite,
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            visibility: Visibility::Hidden,
            ..default()
        },
        TileSelected,
    ));
}

pub fn show_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    map: Res<Map>,
    mut cursor_query: Query<(&mut Transform, &mut Visibility), With<TileCursor>>,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    info!("point: {point:?}");

    let mouse_coordinates = map.point_to_coordinates(point);
    info!("mouse_coordinates: {mouse_coordinates:?}");

    let (mut transform, mut visibility) = cursor_query.single_mut();
    let new_point = map.coordinates_to_point(mouse_coordinates);

    transform.translation.x = new_point.x;
    transform.translation.y = new_point.y - 1.5;

    *visibility = if map.in_bounds(mouse_coordinates) {
        Visibility::Visible
    } else {
        Visibility::Hidden
    }
}
