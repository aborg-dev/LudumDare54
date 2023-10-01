use crate::level::*;
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

#[derive(Component)]
pub struct RowBuildingsRequired {
    row: usize,
}

#[derive(Component)]
pub struct ColBuildingsRequired {
    col: usize,
}

#[derive(Component)]
pub struct IncorrectPlacement {
    row: usize,
    col: usize,
}

pub fn get_cell_color(cell_type: CellType) -> Color {
    match cell_type {
        CellType::Grass => Color::Rgba {
            alpha: 1.0,
            red: 173.0 / 256.0,
            green: 242.0 / 256.0,
            blue: 133.0 / 256.0,
        },
        CellType::Tree => Color::Rgba {
            alpha: 1.0,
            red: 19.0 / 256.0,
            green: 82.0 / 256.0,
            blue: 41.0 / 256.0,
        },
        CellType::Lake => Color::Rgba {
            alpha: 1.0,
            red: 105.0 / 256.0,
            green: 214.0 / 256.0,
            blue: 229.0 / 256.0,
        },
        CellType::Mountain => Color::Rgba {
            alpha: 1.0,
            red: 101.0 / 256.0,
            green: 119.0 / 256.0,
            blue: 121.0 / 256.0,
        },
    }
}

pub fn create_level_render(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut level_render_query: Query<(Entity, &mut LevelRender)>,
    server: Res<AssetServer>,
) {
    let (level_render_entity, mut level_render) = level_render_query.single_mut();
    let puzzle = &game_state.puzzle;

    let (rows, columns) = (puzzle.rows(), puzzle.columns());
    let puzzle_height = rows as f32 * CELL_SIZE;

    level_render.field.resize(rows, vec![]);
    for r in 0..rows {
        for c in 0..columns {
            let color = get_cell_color(puzzle.field[r][c]);
            let tx = c as f32 * CELL_SIZE;
            let ty = puzzle_height - CELL_SIZE - r as f32 * CELL_SIZE;

            let id = commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                        anchor: Anchor::BottomLeft,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(tx, ty, 0.0),
                    ..Default::default()
                })
                .id();
            commands.entity(level_render_entity).add_child(id);
            level_render.field[r].push(id);

            let id = commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                            anchor: Anchor::BottomLeft,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(tx, ty, 0.1),
                        texture: server.load("cross.png"),
                        ..Default::default()
                    },
                    IncorrectPlacement { row: r, col: c },
                ))
                .id();
            commands.entity(level_render_entity).add_child(id);
        }
    }

    for _ in 0..100 {
        let id = commands
            .spawn(SpriteBundle {
                texture: server.load("house.png"),
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

    let text_style = TextStyle {
        font: server.load("NotoSerif-SemiBold.ttf"),
        font_size: 40.0,
        color: Color::WHITE,
        ..default()
    };

    for r in 0..rows {
        let text_bundle = Text2dBundle {
            text: Text::from_section(
                game_state.puzzle.row_count[r].to_string(),
                text_style.clone(),
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(
                -0.2 * CELL_SIZE,
                puzzle_height - (r as f32 + 0.5) * CELL_SIZE,
                0.0,
            ),
            ..default()
        };
        let id = commands
            .spawn((text_bundle, RowBuildingsRequired { row: r }))
            .id();
        commands.entity(level_render_entity).add_child(id);
    }

    for c in 0..columns {
        let text_bundle = Text2dBundle {
            text: Text::from_section(
                game_state.puzzle.col_count[c].to_string(),
                text_style.clone(),
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(
                (c as f32 + 0.5) * CELL_SIZE,
                puzzle.rows() as f32 * CELL_SIZE + 0.2 * CELL_SIZE,
                0.0,
            ),
            ..default()
        };
        let id = commands
            .spawn((text_bundle, ColBuildingsRequired { col: c }))
            .id();
        commands.entity(level_render_entity).add_child(id);
    }
}

pub fn destroy_level_render(
    mut commands: Commands,
    mut level_render_query: Query<(Entity, &mut LevelRender)>,
) {
    let (level_render_entity, mut level_render) = level_render_query.single_mut();
    commands.entity(level_render_entity).despawn_descendants();
    commands.entity(level_render_entity).clear_children();

    level_render.field.clear();
    level_render.placements.clear();
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
    let puzzle_height = game_state.puzzle.rows() as f32 * CELL_SIZE;

    for i in 0..100 {
        let id = level_render.placements[i];
        if let Ok((mut transform, mut visibility)) = sprites_query.get_mut(id) {
            if i < game_state.solution.placements.len() {
                let placement = &game_state.solution.placements[i];
                *visibility = Visibility::Inherited;
                *transform = Transform::from_xyz(
                    placement.position.column as f32 * CELL_SIZE,
                    puzzle_height - CELL_SIZE - placement.position.row as f32 * CELL_SIZE,
                    0.0,
                );
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

pub fn update_buildings_required(
    game_state: Res<GameState>,
    mut row_buildings_required_text_query: Query<
        (&mut Text, &RowBuildingsRequired),
        Without<ColBuildingsRequired>,
    >,
    mut col_buildings_required_text_query: Query<
        (&mut Text, &ColBuildingsRequired),
        Without<RowBuildingsRequired>,
    >,
) {
    let validation_result = validate_solution(&game_state.solution, &game_state.puzzle);
    let (rows, cols) = (game_state.puzzle.rows(), game_state.puzzle.columns());

    let underflow_color = Color::WHITE;
    let match_color = Color::rgb(0.4, 1.0, 0.3);
    let overflow_color = Color::rgb(1.0, 0.3, 0.2);

    for r in 0..rows {
        let color = match validation_result.row_status[r] {
            LineStatus::Underflow => underflow_color,
            LineStatus::Match => match_color,
            LineStatus::Overflow => overflow_color,
        };
        if let Some((mut text, _)) = row_buildings_required_text_query
            .iter_mut()
            .find(|(_, x)| x.row == r)
        {
            text.sections[0].style.color = color;
        }
    }

    for c in 0..cols {
        let color = match validation_result.col_status[c] {
            LineStatus::Underflow => underflow_color,
            LineStatus::Match => match_color,
            LineStatus::Overflow => overflow_color,
        };
        if let Some((mut text, _)) = col_buildings_required_text_query
            .iter_mut()
            .find(|(_, x)| x.col == c)
        {
            text.sections[0].style.color = color;
        }
    }
}

pub fn update_incorrect_placements(
    game_state: Res<GameState>,
    mut incorrect_placements_query: Query<(&mut Visibility, &IncorrectPlacement)>,
) {
    let validation_result = validate_solution(&game_state.solution, &game_state.puzzle);
    let (rows, cols) = (game_state.puzzle.rows(), game_state.puzzle.columns());

    for r in 0..rows {
        for c in 0..cols {
            let (mut visibility, _) = incorrect_placements_query
                .iter_mut()
                .find(|(_, x)| x.row == r && x.col == c)
                .unwrap();
            *visibility = Visibility::Hidden;

            let matches_position = |x: &&PlacementViolation| {
                let placement = &game_state.solution.placements[x.house_index];
                placement.position == Position { row: r, column: c }
            };
            if let Some(x) = validation_result
                .placement_violations
                .iter()
                .find(matches_position)
            {
                *visibility = Visibility::Inherited;
            };
        }
    }
}
