use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    sprite::Anchor,
};

use crate::map::{Coordinates, Floor, Map, Order, Position, SelectCursor, SCALE_FACTOR, Side};

const SPEED: f32 = 200.0;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Turn::default())
            .add_systems(Startup, setup)
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
    pub position: Position,
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
    pub unit: Unit,
    pub position: Position,
}

pub fn print_turn(turn: Res<Turn>) {
    info!("turn: {:?}", turn);
}

pub fn setup(
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

    let mut sprite = TextureAtlasSprite::new(0);
    sprite.anchor = Anchor::BottomCenter;

    let position = Position {
        coordinates: Coordinates(2, 2, Side::Center),
        floor: Floor(0),
        order: Order(2.),
    };
    let translation = map.position_to_translation(&position);

    commands.spawn(UnitBundle {
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
        unit: Unit,
        position,
    });
}

pub fn movement(
    map: Res<Map>,
    time: Res<Time>,
    mut turn: ResMut<Turn>,
    mut unit_query: Query<(&mut Transform, &mut Position), With<Unit>>,
) {
    let Some(selected_unit) = &mut turn.selected_unit else {
        return;
    };

    let Ok((mut transform, mut unit_position)) = unit_query.get_mut(selected_unit.entity) else {
        return;
    };

    let Some(movement) = &mut selected_unit.movement else {
        return;
    };

    // let start_point = transform.translation.xy();
    let start_point = transform.translation;
    let end_point = map.position_to_translation(&movement.position);
    // let end_point = map.coordinates_to_point(movement.position);

    movement.time_passed += time.delta_seconds();
    info!("movement.time_passed: {}", movement.time_passed);
    info!("movement.total_time: {}", movement.total_time);

    let percentage = movement.time_passed / movement.total_time;
    info!("percentage : {percentage}");

    let direction = ((end_point - start_point) * 100.).round() / 100.;

    // info!("delta: {:?}", time.delta());
    info!("direction: {direction}");
    if direction.length() != 0. {
        let translation = start_point.lerp(end_point, percentage);

        transform.translation = translation;
    } else {
        unit_position.coordinates = movement.position.coordinates;
        unit_position.floor = movement.position.floor;
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
    unit_query: Query<(Entity, &Position), With<Unit>>,
    position_query: Query<&Position>,
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
            // if !map.in_bounds(mouse_coordinates) {
            //     continue;
            // }

            let unit_found = unit_query
                .iter()
                .find(|(_, unit_position)| mouse_coordinates.eq(&unit_position.coordinates));

            if let Some((entity, _)) = unit_found {
                if let Some(selected_unit) = &turn.selected_unit {
                    if selected_unit.entity.eq(&entity) {
                        turn.selected_unit = None;
                        continue;
                    }
                }

                turn.selected_unit = Some(SelectedUnit {
                    entity,
                    movement: None,
                });
                continue;
            }

            if let Some(selected_unit) = &mut turn.selected_unit {
                let (_, unit_position) = unit_query.get(selected_unit.entity).unwrap();

                for i in (0..=1).rev() {
                    let bla = mouse_coordinates + Floor(i);
                    if let Some(entities) = map.tiles.get(&bla) {
                        let position = entities
                            .iter()
                            .filter_map(|entity| position_query.get(*entity).ok())
                            .max_by_key(|position| position.floor);

                        if let Some(position) = position {
                            let displacement_x =
                                position.coordinates.0.abs_diff(unit_position.coordinates.0);
                            let displacement_y =
                                position.coordinates.1.abs_diff(unit_position.coordinates.1);

                            selected_unit.movement = Some(Movement {
                                position: Position {
                                    coordinates: position.coordinates,
                                    floor: position.floor,
                                    order: unit_position.order,
                                },
                                total_time: (displacement_x + displacement_y) as f32 * SPEED
                                    / 1000.,
                                time_passed: 0.,
                            });
                        }
                    }
                }

                // selected_unit.movement = Some(Movement {
                //     coordinates: mouse_coordinates,
                //     total_time: displacement.length_squared() as f32 * SPEED / 1000.,
                //     time_passed: 0.,
                // });
            }
        }
    }
}

pub fn highlight_selected(
    map: Res<Map>,
    turn: Res<Turn>,
    unit_query: Query<&Position, (With<Unit>, Without<SelectCursor>)>,
    mut cursor_query: Query<(&mut Transform, &mut Visibility, &mut Position), With<SelectCursor>>,
) {
    let (mut tile_transform, mut visibility, mut position) = cursor_query.single_mut();

    *visibility = Visibility::Hidden;

    let Some(selected_unit) = &turn.selected_unit else {
        return;
    };

    if selected_unit.movement.is_some() {
        return;
    }

    let Ok(unit_position) = unit_query.get(selected_unit.entity) else {
        return;
    };

    position.coordinates = unit_position.coordinates;
    position.floor = unit_position.floor;

    tile_transform.translation = map.position_to_translation(&position);
    tile_transform.translation.y -= 5.5;
    *visibility = Visibility::Visible;
}
