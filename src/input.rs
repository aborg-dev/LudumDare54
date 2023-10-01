use crate::level::{CellType, Placement, Position};
use crate::{render, GameState};
use bevy::math::*;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};

pub struct GameInputPlugin;

#[derive(Resource, Default)]
pub struct SelectedLevel {
    pub number: Option<usize>,
}

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedLevel>()
            .add_systems(Update, keyboard_input)
            .add_systems(Update, mouse_input);
    }
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut selected_level: ResMut<SelectedLevel>) {
    if keys.just_pressed(KeyCode::Key1) {
        selected_level.number = Some(0);
    }
    if keys.just_pressed(KeyCode::Key2) {
        selected_level.number = Some(1);
    }
    if keys.just_pressed(KeyCode::Key3) {
        selected_level.number = Some(2);
    }
    if keys.just_pressed(KeyCode::Key4) {
        selected_level.number = Some(3);
    }
}

fn mouse_input(
    mouse: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    level_render_query: Query<&Transform, With<render::LevelRender>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut game_state: ResMut<GameState>,
) {
    let level_transform = level_render_query.single();
    let (camera, camera_global_transform) = camera_query.single();
    let window = window_query.single();
    let (rows, columns) = (game_state.puzzle.rows(), game_state.puzzle.columns());

    let left_just_pressed = mouse.just_pressed(MouseButton::Left);
    let right_just_pressed = mouse.just_pressed(MouseButton::Right);

    if let Some(p) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_global_transform, cursor))
        .map(|cursor| (cursor - level_transform.translation.xy()) / render::CELL_SIZE)
    {
        let lower_bound = Vec2::new(0.0, 0.0);
        let upper_bound = Vec2::new(columns as f32, rows as f32);
        if p.cmpge(lower_bound).all() && p.cmplt(upper_bound).all() {
            let position = Position {
                row: rows - 1 - p.y as usize,
                column: p.x as usize,
            };
            let r = position.row;
            let c = position.column;

            if left_just_pressed
                && game_state.puzzle.field[r][c] == CellType::Grass
                && game_state
                    .solution
                    .placements
                    .iter()
                    .all(|x| !(x.position == position))
            {
                game_state.solution.placements.push(Placement { position });
            }

            // Remove placements at this position.
            if right_just_pressed {
                game_state
                    .solution
                    .placements
                    .retain(|p| p.position != position);
            }
        }
    }
}
