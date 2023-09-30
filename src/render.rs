use bevy::prelude::{BuildChildren, Color, Commands, Component, Entity, Query, Res, Transform};
use bevy::sprite::{Sprite, SpriteBundle};
use std::default::Default;
use bevy::math::Vec2;
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

pub fn render_level_and_solution(mut commands: Commands, game_state: Res<GameState>, mut level_render_query: Query<(Entity, &mut LevelRender)>) {
    let (level_render_entity, mut level_render) = level_render_query.single_mut();
    let level = &game_state.level;
    let solution = &game_state.solution;

    commands.entity(level_render_entity).clear_children();

    let rows = level.rows();
    let columns = level.columns();
    let cell_size = 100.0;
    level_render.field.resize(rows, vec![]);
    for r in 0..rows {
        for c in 0..columns {
            let color = if level.field[r][c] == CellType::Grass { Color::GREEN } else { Color::NONE };
            let id = commands.spawn(SpriteBundle {
                sprite: Sprite { color, custom_size: Some(Vec2::new(cell_size, cell_size)), ..Default::default() },
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
                sprite: Sprite { color, custom_size: Some(Vec2::new(cell_size, cell_size)), ..Default::default() },
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
