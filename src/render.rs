use crate::input::SelectedBuilding;
use crate::level::{validate_solution, BuildingType, CellType, Level, Position, Solution};
use crate::GameState;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::ui::RelativeCursorPosition;
use std::default::Default;

pub const CELL_SIZE: f32 = 100.0;

#[derive(Component)]
pub struct LevelRender {
    need_update: bool,
    field: Vec<Vec<Entity>>,
    placements: Vec<Entity>,
}

impl Default for LevelRender {
    fn default() -> Self {
        Self {
            need_update: false,
            field: vec![],
            placements: vec![],
        }
    }
}

#[derive(Component)]
pub struct SolutionStatusText;

#[derive(Component)]
pub struct AvailableBuildingsText {
    building_index: usize,
}

pub fn create_level_render(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut level_render_query: Query<(Entity, &mut LevelRender)>,
    window_query: Query<&Window>,
    server: Res<AssetServer>,
) {
    let (level_render_entity, mut level_render) = level_render_query.single_mut();
    let level = &game_state.level;
    let solution = &game_state.solution;
    let window = window_query.single();

    // let window_width = window.resolution.width();
    // let window_height = window.resolution.height();
    // let (center_x, center_y) = (window_width / 2.0, window_height / 2.0);

    let (rows, columns) = (level.rows(), level.columns());
    // let (level_width, level_height) = (columns as f32 * CELL_SIZE, rows as f32 * CELL_SIZE);

    level_render.field.resize(rows, vec![]);
    for r in 0..rows {
        for c in 0..columns {
            let color = if level.field[r][c] == CellType::Grass {
                Color::GREEN
            } else {
                Color::NONE
            };
            let id = commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                        anchor: Anchor::BottomLeft,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(c as f32 * CELL_SIZE, r as f32 * CELL_SIZE, 0.0),
                    ..Default::default()
                })
                .id();
            commands.entity(level_render_entity).add_child(id);
            level_render.field[r].push(id);
        }
    }

    for placement in solution.placements.iter() {
        let color = match placement.building {
            BuildingType::House => Color::BEIGE,
            BuildingType::Trash => Color::BLACK,
            BuildingType::Hermit => Color::CYAN,
        };
        let id = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                visibility: Visibility::Hidden,
                ..Default::default()
            })
            .id();
        commands.entity(level_render_entity).add_child(id);
        level_render.placements.push(id);
    }

    let messages = build_available_buildings_texts(&game_state.level, &game_state.solution);
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(20.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            for (index, (building, message)) in messages.iter().enumerate() {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(10.),
                            height: Val::Percent(100.),
                            margin: UiRect::right(Val::Px(50.)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn(ImageBundle {
                                image: UiImage {
                                    texture: server.load(building.get_asset_name()),
                                    flip_x: false,
                                    flip_y: false,
                                },
                                style: Style {
                                    width: Val::Px(100.),
                                    height: Val::Px(100.),
                                    margin: UiRect::top(Val::Px(20.)),
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(RelativeCursorPosition::default());

                        parent.spawn((
                            TextBundle::from_section(
                                message.clone(),
                                TextStyle {
                                    font_size: 24.0,
                                    color: Color::WHITE,
                                    ..Default::default()
                                },
                            ),
                            AvailableBuildingsText {
                                building_index: index,
                            },
                        ));
                    });
            }
        });

    commands.spawn((
        TextBundle::from_section(
            "Solution status:",
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..Default::default()
            },
        )
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            width: Val::Px(600.0),
            ..default()
        }),
        SolutionStatusText,
    ));
}

pub fn destroy_level_render(
    mut commands: Commands,
    level_render_query: Query<(Entity), (With<LevelRender>, With<Transform>)>,
) {
    let level_render_entity = level_render_query.single();
    commands.entity(level_render_entity).despawn_descendants();
    commands.entity(level_render_entity).clear_children();
}

pub fn update_level_render(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut level_render_query: Query<(Entity, &LevelRender, &mut Transform)>,

    window_query: Query<&Window>,
    mut solution_status_text_query: Query<&mut Text, With<SolutionStatusText>>,
) {
    let (level_render_entity, level_render, mut transform) = level_render_query.single_mut();
    let level = &game_state.level;
    // let window = window_query.single();

    // let window_width = window.resolution.width();
    // let window_height = window.resolution.height();
    // let (center_x, center_y) = (window_width / 2.0, window_height / 2.0);

    let (rows, columns) = (level.rows(), level.columns());
    let (level_width, level_height) = (columns as f32 * CELL_SIZE, rows as f32 * CELL_SIZE);

    transform.translation = Vec3::new(-level_width / 2.0, -level_height / 2.0, 0.0);

    // TODO: We can actually update this only if solution changes.
    let solution = &game_state.solution;
    let validation_result = validate_solution(solution, level);
    solution_status_text_query.single_mut().sections[0].value = format!("{}", validation_result);
}

pub fn update_placements_render(
    mut commands: Commands,
    game_state: Res<GameState>,
    level_render_query: Query<(&LevelRender)>,
    mut sprites_query: Query<(&mut Transform, &mut Visibility)>,
) {
    let level_render = level_render_query.single();

    for i in 0..game_state.solution.placements.len() {
        let placement = &game_state.solution.placements[i];
        let id = level_render.placements[i];
        if let Ok((mut transform, mut visibility)) = sprites_query.get_mut(id) {
            let position = placement.position.unwrap_or(Position{row: 0, column: 0});
            let visible = placement.position.is_some();
            *transform = Transform::from_xyz(
                position.column as f32 * CELL_SIZE,
                position.row as f32 * CELL_SIZE,
                0.0,
            );
            *visibility = if visible {Visibility::Inherited} else {Visibility::Hidden};
        }
    }
}

pub fn build_available_buildings_texts(
    level: &Level,
    solution: &Solution,
) -> Vec<(BuildingType, String)> {
    let placed_building_count = solution.building_count();
    let mut messages = Vec::new();
    for (index, (building, total_count)) in level.building_count.iter().enumerate() {
        let placed_count = placed_building_count
            .get(&building)
            .cloned()
            .unwrap_or_default();
        messages.push((
            *building,
            format!("{}: {building:?}: {placed_count}/{total_count}", index + 1),
        ));
    }
    messages
}

// TODO: We can actually update this only if solution changes.
pub fn update_available_buildings_text(
    game_state: Res<GameState>,
    selected_building: Res<SelectedBuilding>,
    mut available_buildings_text: Query<(&mut Text, &AvailableBuildingsText)>,
) {
    let messages = build_available_buildings_texts(&game_state.level, &game_state.solution);
    for (mut text, available_building_text_component) in available_buildings_text.iter_mut() {
        text.sections[0].value = messages[available_building_text_component.building_index]
            .1
            .clone();
        if selected_building.number == Some(available_building_text_component.building_index) {
            text.sections[0].style.color = Color::RED;
        } else {
            text.sections[0].style.color = Color::WHITE;
        }
    }
}
