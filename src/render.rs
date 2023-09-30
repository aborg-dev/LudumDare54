use bevy::prelude::*;
use bevy::math::Vec2;
use std::default::Default;
use bevy::sprite::Anchor;
use crate::GameState;
use crate::level::{BuildingType, CellType};

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

pub fn render_level_and_solution(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut level_render_query: Query<(Entity, &mut LevelRender, &mut Transform)>,
    window_query: Query<&Window>,
) {
    let (level_render_entity, mut level_render, mut transform) = level_render_query.single_mut();
    let level = &game_state.level;
    let solution = &game_state.solution;
    let window = window_query.single();

    let window_width = window.resolution.width();
    let window_height = window.resolution.height();
    let (center_x, center_y) = (window_width / 2.0, window_height / 2.0);

    let (rows, columns) = (level.rows(), level.columns());
    let cell_size = 100.0;
    let (level_width, level_height) = (columns as f32 * cell_size, rows as f32 * cell_size);

    transform.translation = Vec3::new(-level_width / 2.0, -level_height / 2.0, 0.0);

    commands.entity(level_render_entity).despawn_descendants();
    commands.entity(level_render_entity).clear_children();

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
}
