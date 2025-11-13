use bevy::{prelude::*, sprite::Anchor};

use super::{components::{Floor, Order}, resource::Map, Coordinates, Position, TileBundle, SCALE_FACTOR, TILE_SIZE, Side};

pub fn setup(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut map: ResMut<Map>,
) {
    let columns = 11;
    let rows = 10;
    let texture_handle = asset_server.load("textures/Isometric_MedievalFantasy_Tiles.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle.clone(), TILE_SIZE, columns, rows, None, None);
    let texture_atlas_handle = &texture_atlases.add(texture_atlas);

    let tiles_count = (map.size.x * map.size.y) as usize;

    for i in 0..tiles_count {
        let coordinates = map.index_to_coordinates(i);
        let position = Position {
            coordinates,
            floor: Floor(0),
            order: Order(0.),
        };
        let translation = map.position_to_translation(&position);

        let mut sprite = TextureAtlasSprite::new(92);
        sprite.anchor = Anchor::Center;

        let entity = commands.spawn(TileBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite,
                transform: Transform {
                    translation,
                    scale: Vec3::splat(SCALE_FACTOR),
                    ..default()
                },
                ..default()
            },
            position,
        });

        match map.tiles.get_mut(&coordinates) {
            Some(entities) => entities.push(entity.id()),
            None => {
                map.tiles.insert(coordinates, vec![entity.id()]);
            }
        }
    }

    let coordinates = Coordinates(10, 10, Side::Center);

    for i in 1..=2 {
        let position = Position {
            coordinates,
            floor: Floor(i),
            order: Order(0.),
        };
        let translation = map.position_to_translation(&position);

        let entity = commands.spawn(TileBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(93),
                transform: Transform {
                    scale: Vec3::splat(SCALE_FACTOR),
                    translation,
                    ..default()
                },
                ..default()
            },
            position,
        });

        match map.tiles.get_mut(&coordinates) {
            Some(entities) => entities.push(entity.id()),
            None => {
                map.tiles.insert(coordinates, vec![entity.id()]);
            }
        }
    }

    let coordinates = Coordinates(9, 10, Side::Center);

    for i in 1..2 {
        let position = Position {
            coordinates,
            floor: Floor(i),
            order: Order(0.),
        };
        let translation = map.position_to_translation(&position);

        let entity = commands.spawn(TileBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(93),
                transform: Transform {
                    scale: Vec3::splat(SCALE_FACTOR),
                    translation,
                    ..default()
                },
                ..default()
            },
            position,
        });

        match map.tiles.get_mut(&coordinates) {
            Some(entities) => entities.push(entity.id()),
            None => {
                map.tiles.insert(coordinates, vec![entity.id()]);
            }
        }
    }

    for i in 0..10 {
        let position = Position {
            coordinates: Coordinates(i, 0, Side::Center),
            floor: Floor(1),
            order: Order(0.),
        };
        let translation = map.position_to_translation(&position);

        let entity = commands.spawn(TileBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(93),
                transform: Transform {
                    scale: Vec3::splat(SCALE_FACTOR),
                    translation,
                    ..default()
                },
                ..default()
            },
            position,
        });

        if let Some(entities) = map.tiles.get_mut(&position.coordinates) {
            info!("coordinates: {:?}", position.coordinates);
            info!("entities length: {:?}", entities.len());
            entities.push(entity.id())
        }
    }

    let entities = map.tiles.get(&Coordinates(0, 0, Side::Center)).unwrap();
    info!("entities length: {:?}", entities.len());
    entities.iter().for_each(|entity| {
        info!("entity id: {:?}", entity);
    });
}

pub fn update_z_index(map: Res<Map>, mut query: Query<(&mut Transform, &Position)>) {
    query.iter_mut().for_each(|(mut transform, position)| {
        transform.translation = map.position_to_translation(position);
    });
}

// pub fn update_position(
//     map: Res<Map>,
//     mut query: Query<(&mut Transform, &Floor, &Coordinates), With<Tile>>,
// ) {
//     query.iter_mut().for_each(|(mut transform, floor, coordinates)| {
//         let coordinates = Coordinates(coordinates.0 - floor.0);
//         let point = map.coordinates_to_point(coordinates);
//         transform.translation.x = point.x;
//         transform.translation.y = point.y;
//     });
// }
