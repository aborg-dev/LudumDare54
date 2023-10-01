use crate::level::{validate_solution, CellType, Position, Puzzle, Solution};
use crate::GameState;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::default::Default;

pub const CELL_SIZE: f32 = 100.0;

#[derive(Component, Default)]
pub struct LevelRender {
    field: Vec<Vec<Entity>>,
    placements: Vec<Entity>,
}

#[derive(Component)]
pub struct SolutionStatusText;

pub fn create_level_render(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut level_render_query: Query<(Entity, &mut LevelRender)>,
    server: Res<AssetServer>,
) {
    let (level_render_entity, mut level_render) = level_render_query.single_mut();
    let puzzle = &game_state.puzzle;

    let (rows, columns) = (puzzle.rows(), puzzle.columns());
    level_render.field.resize(rows, vec![]);
    for r in 0..rows {
        for c in 0..columns {
            let color = if puzzle.field[r][c] == CellType::Grass {
                Color::Rgba {
                    alpha: 1.0,
                    blue: 133.0 / 256.0,
                    green: 242.0 / 256.0,
                    red: 173.0 / 256.0,
                }
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

    for placement in game_state.solution.placements.iter() {
        let id = commands
            .spawn(SpriteBundle {
                texture: server.load(placement.building.get_asset_name()),
                sprite: Sprite {
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
    level_render_query: Query<Entity, (With<LevelRender>, With<Transform>)>,
) {
    let level_render_entity = level_render_query.single();
    commands.entity(level_render_entity).despawn_descendants();
    commands.entity(level_render_entity).clear_children();
}

pub fn update_level_render(
    game_state: Res<GameState>,
    mut level_render_query: Query<(Entity, &LevelRender, &mut Transform)>,
) {
    let (_, _, mut transform) = level_render_query.single_mut();
    let puzzle = &game_state.puzzle;
    let (rows, columns) = (puzzle.rows(), puzzle.columns());
    let (puzzle_width, puzzle_height) = (columns as f32 * CELL_SIZE, rows as f32 * CELL_SIZE);
    transform.translation = Vec3::new(-puzzle_width / 2.0, -puzzle_height / 2.0, 0.0);
}

pub fn update_placements_render(
    game_state: Res<GameState>,
    level_render_query: Query<&LevelRender>,
    mut sprites_query: Query<(&mut Transform, &mut Visibility)>,
) {
    let level_render = level_render_query.single();

    for i in 0..game_state.solution.placements.len() {
        let placement = &game_state.solution.placements[i];
        let id = level_render.placements[i];
        if let Ok((mut transform, mut visibility)) = sprites_query.get_mut(id) {
            let position = placement.position;
            let visible = true;
            *transform = Transform::from_xyz(
                position.column as f32 * CELL_SIZE,
                position.row as f32 * CELL_SIZE,
                0.0,
            );
            *visibility = if visible {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
}

// TODO: We can actually update this only if solution changes.
pub fn update_solution_status(
    game_state: Res<GameState>,
    mut solution_status_text_query: Query<&mut Text, With<SolutionStatusText>>,
) {
    let validation_result = validate_solution(&game_state.solution, &game_state.puzzle);
    solution_status_text_query.single_mut().sections[0].value = format!("{}", validation_result);
}
