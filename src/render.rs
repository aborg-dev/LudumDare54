use bevy::prelude::*;
use bevy::math::Vec2;
use std::default::Default;
use bevy::sprite::Anchor;
use crate::GameState;
use crate::level::{BuildingType, CellType, validate_solution};

#[derive(Component)]
pub struct LevelRender {
    need_update: bool,
    field: Vec<Vec<Entity>>,
}

impl Default for LevelRender {
    fn default() -> Self {
        Self {
            need_update: false,
            field: vec![],
        }
    }
}

#[derive(Component)]
pub struct SolutionStatusText;

#[derive(Component)]
pub struct AvailableBuildingsText;

pub fn create_level_render(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut level_render_query: Query<(Entity, &mut LevelRender)>,
    window_query: Query<&Window>,
) {
    let (level_render_entity, mut level_render) = level_render_query.single_mut();
    let level = &game_state.level;
    let solution = &game_state.solution;
    let window = window_query.single();

    // let window_width = window.resolution.width();
    // let window_height = window.resolution.height();
    // let (center_x, center_y) = (window_width / 2.0, window_height / 2.0);

    let (rows, columns) = (level.rows(), level.columns());
    let cell_size = 100.0;
    // let (level_width, level_height) = (columns as f32 * cell_size, rows as f32 * cell_size);

    level_render.field.resize(rows, vec![]);
    for r in 0..rows {
        for c in 0..columns {
            let color = if level.field[r][c] == CellType::Grass { Color::GREEN } else { Color::NONE };
            let id = commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(cell_size, cell_size)),
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    c as f32 * cell_size,
                    r as f32 * cell_size,
                    0.0,
                ),
                ..Default::default()
            }).id();
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
        if let Some(p) = &placement.position {
            let (r, c) = (p.row, p.column);
            let id = commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(cell_size, cell_size)),
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    c as f32 * cell_size,
                    r as f32 * cell_size,
                    0.0,
                ),
                ..Default::default()
            }).id();
            commands.entity(level_render_entity).add_child(id);
        }
    }

    commands.spawn((
        TextBundle::from_section(
            "Available buildings:",
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..Default::default()
            },
        )
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            width: Val::Px(600.0),
            ..default()
        }),
        AvailableBuildingsText,
    ));

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

pub fn update_lever_render(
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
    let cell_size = 100.0;
    let (level_width, level_height) = (columns as f32 * cell_size, rows as f32 * cell_size);

    transform.translation = Vec3::new(-level_width / 2.0, -level_height / 2.0, 0.0);

    // TODO: We can actually update this only if solution changes.
    let solution = &game_state.solution;
    let validation_result = validate_solution(solution, level);
    solution_status_text_query.single_mut().sections[0].value = format!("{}", validation_result);
}

// TODO: We can actually update this only if solution changes.
pub fn update_available_buildings_text(
    game_state: Res<GameState>,
    mut available_buildings_text: Query<&mut Text, With<AvailableBuildingsText>>,
) {
    let level = &game_state.level;
    let placed_building_count = game_state.solution.building_count();
    let mut messages = Vec::new();
    for (index, (building, total_count)) in game_state.level.building_count.iter().enumerate() {
        let placed_count = placed_building_count.get(&building).cloned().unwrap_or_default();
        messages.push(format!("{}: {building:?}: {placed_count}/{total_count}", index + 1));
    }
    
    available_buildings_text.single_mut().sections[0].value = format!("{}", messages.join("\n"));
}
