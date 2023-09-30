use bevy::prelude::{BuildChildren, Commands, Component, Entity, Query, Res};
use bevy::sprite::Sprite;
use crate::GameState;

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
pub struct Test {
    pub x: bool,
}

pub fn render_level(mut commands: Commands, game_state: Res<GameState>, level_render_query: Query<(Entity, &LevelRender)>) {
    let (entity, level_render) = level_render_query.single();

    let level = &game_state.level;
    // commands.entity(entity).push_children(&[entity]);
    commands.entity(entity).clear_children();
    for r in 0..level.rows() {
        for c in 0..level.columns() {
            println!("{r}:{c}");
        }
    }
}