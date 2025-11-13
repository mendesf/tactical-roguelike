use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::map::resource::Map;
use crate::map::{Coordinates, Cursor, Floor, HoverCursor, Order, Position, SelectCursor, SCALE_FACTOR, Side};

use super::bundle::CursorBundle;

pub fn setup(
    asset_server: Res<AssetServer>,
    map: Res<Map>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    let texture_handle = asset_server.load("textures/TRPGIsometricAssetPack_MapIndicators.png");
    let mut texture_atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(32., 25.));

    let mut hover_bundle = cursor_bundle::<HoverCursor>(&map);
    texture_atlas.add_texture(hover_bundle.cursor.0.rect);

    let mut select_bundle = cursor_bundle::<SelectCursor>(&map);
    texture_atlas.add_texture(select_bundle.cursor.0.rect);

    let texture_atlas_handle = &texture_atlases.add(texture_atlas);

    hover_bundle.sprite.texture_atlas = texture_atlas_handle.clone();
    select_bundle.sprite.texture_atlas = texture_atlas_handle.clone();

    commands.spawn(hover_bundle);
    commands.spawn(select_bundle);
}

pub fn cursor_bundle<T>(map: &Res<Map>) -> CursorBundle<T>
where
    T: Cursor,
{
    let cursor = T::new();

    let mut sprite = TextureAtlasSprite::new(cursor.data().index);
    sprite.anchor = Anchor::BottomCenter;

    let position = Position {
        coordinates: Coordinates(0, 0, Side::Center),
        floor: Floor(0),
        order: Order(1.),
    };
    let translation = map.position_to_translation(&position);

    CursorBundle {
        sprite: SpriteSheetBundle {
            sprite,
            transform: Transform {
                translation,
                scale: Vec3::splat(SCALE_FACTOR),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        position,
        cursor,
    }
}

pub fn hovering(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    map: Res<Map>,
    position_query: Query<&Position>,
    mut cursor_query: Query<(&mut Transform, &mut Visibility, &Position), With<HoverCursor>>,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    info!("point: {point:?}");

    let (mut transform, mut visibility, cursor_position) = cursor_query.single_mut();

    let mut mouse_coordinates = map.point_to_coordinates(point);

    info!("mouse_coordinates: {mouse_coordinates:?}");

    *visibility = Visibility::Hidden;

    // if let Some(entities) = map.tiles.get(&mouse_coordinates) {
    //     let positions = entities
    //         .iter()
    //         .map(|entity| position_query.get(*entity).unwrap())
    //         .collect::<Vec<&Position>>();
    //
    //     let position = positions.iter().max_by_key(|position| position.floor);
    //
    //     if let Some(position) = position {
    //         transform.translation = map.position_to_translation(&Position {
    //             coordinates: position.coordinates,
    //             floor: position.floor,
    //             order: cursor_position.order,
    //         });
    //         transform.translation.y -= 5.5;
    //         *visibility = Visibility::Visible;
    //     }
    // }

    // deu ruim, montar map com todas a posições resolvidas

    fn bla(coordinates: Coordinates, map: &Map, position_query: &Query<&Position>) -> Option<(Coordinates, Floor)> {
    // fazer função recursiva
    for i in (0..=2).rev() {
        let mut floor = Floor(i);
        // info!("coordinates {:?}", coordinates);
        // info!("floor {:?}", floor);
        let mut coordinates = coordinates + floor;

            if !map.in_bounds(coordinates) {
                continue;
            }

            if let Some(entities) = map.tiles.get(&coordinates) {
                let positions = entities.iter().map(|entity| {
                    position_query.get(*entity).unwrap()
                }).collect::<Vec<&Position>>();

                let position = positions.iter().max_by_key(|position| { position.floor });

                if let Some(position) = position {
                    info!("floor {:?}", floor);
                    info!("position {:?}", position);

                    if position.floor < floor {
                        continue;
                    }

                    if floor < position.floor {
                        // info!("coordinates before {:?}", coordinates);
                        match coordinates.2 {
                            Side::Left => coordinates.1 += position.floor.0,
                            Side::Right => coordinates.0 += position.floor.0,
                            Side::Center => ()
                        };
                        // info!("coordinates after {:?}", coordinates);
                        // coordinates.1 += 1;
                        // coordinates.0 += 1;
                        // return bla(coordinates, map, position_query);
                        return Some((coordinates, floor));
                    }

                    if map.in_bounds(coordinates) {
                        return Some((coordinates, floor));
                    }
                }
            }
        }

        None
    }


    let Some((coordinates, floor)) = bla(mouse_coordinates, &map, &position_query) else {
        return;
    };

    if map.in_bounds(coordinates) {
        transform.translation = map.position_to_translation(&Position {
            coordinates,
            floor,
            order: cursor_position.order,
        });
        transform.translation.y -= 5.5;
        *visibility = Visibility::Visible;
    }

    // let new_point = map.coordinates_to_point(mouse_coordinates);
    //
    // transform.translation.x = new_point.x;
    // transform.translation.y = new_point.y - 1.5;
    //
    // *visibility = Visibility::Visible
}
