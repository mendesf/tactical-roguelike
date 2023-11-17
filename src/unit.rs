use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::map::{Coordinates, Map, TileSelected};

const SPEED: f32 = 200.0;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Turn::default())
            .add_systems(Startup, spawn)
            .add_systems(
                Update,
                (
                    movement,
                    click_to_move.run_if(not(moving())),
                    highlight_selected,
                ),
            );
    }
}

#[derive(Debug)]
pub struct Movement {
    pub coordinates: Coordinates,
    pub total_time: f32,
    pub time_passed: f32,
}

#[derive(Debug)]
pub struct SelectedUnit {
    pub entity: Entity,
    pub movement: Option<Movement>,
}

#[derive(Resource, Default, Debug)]
pub struct Turn {
    pub selected_unit: Option<SelectedUnit>,
}

#[derive(Component, Copy, Clone, Default)]
pub struct Unit;

#[derive(Bundle, Default)]
pub struct UnitBundle {
    pub sprite: SpriteSheetBundle,
    pub coordinates: Coordinates,
    pub unit: Unit,
}

pub fn print_turn(turn: Res<Turn>) {
    info!("turn: {:?}", turn);
}

pub fn spawn(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
) {
    let texture_handle = asset_server.load("textures/IsometricTRPGAssetPack_OutlinedEntities.png");
    let mut texture_atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(32., 25.));
    texture_atlas.add_texture(Rect::new(4., 37., 14., 50.));
    texture_atlas.add_texture(Rect::new(20., 36., 30., 50.));
    let texture_atlas_handle = &texture_atlases.add(texture_atlas);

    let coordinates = Coordinates(Vec2::new(2., 2.));
    let point = map.coordinates_to_point(coordinates);

    let mut sprite = TextureAtlasSprite::new(0);
    sprite.anchor = Anchor::BottomCenter;

    commands.spawn(UnitBundle {
        sprite: SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite,
            transform: Transform::from_translation(Vec3::from((point, 2.))),
            ..default()
        },
        coordinates,
        unit: Unit,
    });
}

pub fn movement(
    map: Res<Map>,
    time: Res<Time>,
    mut turn: ResMut<Turn>,
    mut unit_query: Query<(&mut Transform, &mut Coordinates), With<Unit>>,
) {
    let Some(selected_unit) = &mut turn.selected_unit else {
        return;
    };

    let Ok((mut transform, mut unit_coordinates)) = unit_query.get_mut(selected_unit.entity) else {
        return;
    };

    let Some(movement) = &mut selected_unit.movement else {
        return;
    };

    let start_point = transform.translation.xy();
    let end_point = map.coordinates_to_point(movement.coordinates);

    let direction = (end_point - start_point).normalize_or_zero();
    // info!("delta: {:?}", time.delta());
    // info!("direction: {direction}");
    if direction.length() > 0. {
        movement.time_passed += time.delta_seconds();
        let percentage = movement.time_passed / movement.total_time;
        let translation = start_point.lerp(end_point, percentage);

        transform.translation.x = translation.x;
        transform.translation.y = translation.y;
    } else {
        *unit_coordinates = movement.coordinates;
        selected_unit.movement = None;
    }
}

pub fn moving() -> impl Fn(Res<Turn>) -> bool {
    move |turn: Res<Turn>| match &turn.selected_unit {
        Some(selected_unit) => {
            let Some(movement) = &selected_unit.movement else {
                return false;
            };
            movement.time_passed < movement.total_time
        }
        None => false,
    }
}

pub fn click_to_move(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows_query: Query<&Window>,
    map: Res<Map>,
    mut turn: ResMut<Turn>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    unit_query: Query<(Entity, &Coordinates), With<Unit>>,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows_query.single().cursor_position() else {
        return;
    };

    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    for event in mouse_button_input_events.read() {
        if event.button == MouseButton::Left && event.state != ButtonState::Released {
            let mouse_coordinates = map.point_to_coordinates(point);
            if !map.in_bounds(mouse_coordinates) {
                return;
            }

            let unit_found = unit_query
                .iter()
                .find(|(_, unit_coordinates)| mouse_coordinates.eq(unit_coordinates));

            if let Some((entity, _)) = unit_found {
                if let Some(selected_unit) = &turn.selected_unit {
                    if selected_unit.entity.eq(&entity) {
                        turn.selected_unit = None;
                        return;
                    }
                }

                turn.selected_unit = Some(SelectedUnit {
                    entity,
                    movement: None,
                });
                return;
            }

            if let Some(selected_unit) = &mut turn.selected_unit {
                let (_, unit_coordinates) = unit_query.get(selected_unit.entity).unwrap();
                let displacement = mouse_coordinates.0 - unit_coordinates.0;

                selected_unit.movement = Some(Movement {
                    coordinates: mouse_coordinates,
                    total_time: displacement.length() * SPEED / 1000.,
                    time_passed: 0.,
                });
            }
        }
    }
}

pub fn highlight_selected(
    map: Res<Map>,
    turn: Res<Turn>,
    unit_query: Query<&Transform, (With<Unit>, Without<TileSelected>)>,
    mut cursor_query: Query<(&mut Transform, &mut Visibility), With<TileSelected>>,
) {
    let (mut tile_transform, mut visibility) = cursor_query.single_mut();

    *visibility = Visibility::Hidden;

    let Some(selected_unit) = &turn.selected_unit else {
        return;
    };

    if selected_unit.movement.is_some() {
        return;
    }

    let Ok(unit_transform) = unit_query.get(selected_unit.entity) else {
        return;
    };

    tile_transform.translation.x = unit_transform.translation.x;
    tile_transform.translation.y = unit_transform.translation.y - 1.5;
    *visibility = Visibility::Visible;
}
