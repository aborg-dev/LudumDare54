use bevy::prelude::{BuildChildren, Color, Commands, Component, Entity, Query, Res, Transform};
use bevy::sprite::{Sprite, SpriteBundle};
use std::default::Default;
use bevy::math::Vec2;
use crate::GameState;
use crate::level::CellType;

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

pub fn render_level(mut commands: Commands, game_state: Res<GameState>, level_render_query: Query<(Entity, &LevelRender)>) {
    let (level_render_entity, level_render) = level_render_query.single();
    let level = &game_state.level;

    commands.entity(level_render_entity).clear_children();

    for r in 0..level.rows() {
        for c in 0..level.columns() {
            let color = if level.field[r][c] == CellType::Grass { Color::GREEN } else { Color::NONE };
            let id = commands.spawn(SpriteBundle {
                sprite: Sprite { color, custom_size: Some(Vec2::new(100.0, 100.0)), ..Default::default() },
                transform: Transform::from_xyz(c as f32 * 100.0, r as f32 * 100.0, 0.0),
                ..Default::default()
            }).id();
            commands.entity(level_render_entity).add_child(id);
        }
    }
}
