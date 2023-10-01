use crate::level::{all_levels, CellType, Placement, Position};
use crate::{render, AppState, GameState, GlobalVolumeSettings, VolumeSettings};
use bevy::audio::*;
use bevy::math::*;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_input)
            .add_systems(Update, (mouse_input).run_if(in_state(AppState::InGame)));
    }
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut app_state: ResMut<NextState<AppState>>,
    mut global_volume_settings: ResMut<GlobalVolumeSettings>,
) {
    if keys.just_pressed(KeyCode::Right) {
        if game_state.current_level + 1 < all_levels().len() {
            game_state.current_level += 1;
            app_state.set(AppState::SwitchLevel);
            println!("Going to level {}", game_state.current_level);
        }
    }
    if keys.just_pressed(KeyCode::Left) {
        if game_state.current_level > 0 {
            game_state.current_level -= 1;
            app_state.set(AppState::SwitchLevel);
            println!("Going to level {}", game_state.current_level);
        }
    }
    if keys.just_pressed(KeyCode::L) {
        app_state.set(AppState::SelectLevelScreen);
    }

    if keys.just_pressed(KeyCode::M) {
        if global_volume_settings.volume == 0.0 {
            global_volume_settings.volume = 1.0;
        } else {
            global_volume_settings.volume = 0.0;
        }
    }
}

fn mouse_input(
    mouse: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    level_render_query: Query<&Transform, With<render::LevelRender>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    let level_transform = level_render_query.single();
    let (camera, camera_global_transform) = camera_query.single();
    let window = window_query.single();
    let (rows, columns) = (game_state.puzzle.rows(), game_state.puzzle.columns());

    let left_just_pressed = mouse.just_pressed(MouseButton::Left);
    let right_just_pressed = mouse.just_pressed(MouseButton::Right);

    let isometric_to_orthographic = |pi: Vec2| {
        let pi = pi - level_transform.translation.xy();
        let po = Vec2::new(pi.x + 2.0 * pi.y, pi.x - 2.0 * pi.y);
        po / render::CELL_SIZE
    };

    if let Some(p) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_global_transform, cursor))
        .map(isometric_to_orthographic)
    {
        let lower_bound = Vec2::new(0.0, 0.0);
        let upper_bound = Vec2::new(columns as f32, rows as f32);
        if p.cmpge(lower_bound).all() && p.cmplt(upper_bound).all() {
            let position = Position {
                row: p.y as usize,
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
                game_state.hints[r][c] = false;

                commands.spawn((
                    AudioBundle {
                        source: server.load("place.wav"),
                        settings: PlaybackSettings {
                            volume: Volume::new_absolute(0.0),
                            speed: 1.2,
                            ..default()
                        },
                        ..default()
                    },
                    VolumeSettings { volume: 0.6 },
                ));
            }

            // Remove placements at this position.
            if right_just_pressed {
                if let Some(index) = game_state
                    .solution
                    .placements
                    .iter()
                    .position(|x| x.position == position)
                {
                    game_state.solution.placements.remove(index);
                    commands.spawn((
                        AudioBundle {
                            source: server.load("remove.wav"),
                            settings: PlaybackSettings {
                                volume: Volume::new_absolute(0.0),
                                speed: 1.2,
                                ..default()
                            },
                            ..default()
                        },
                        VolumeSettings { volume: 0.5 },
                    ));
                    game_state.hints[r][c] = false;
                } else if game_state.puzzle.field[r][c] == CellType::Grass {
                    commands.spawn((
                        AudioBundle {
                            source: server.load("hint.wav"),
                            settings: PlaybackSettings {
                                volume: Volume::new_absolute(0.0),
                                speed: 1.2,
                                ..default()
                            },
                            ..default()
                        },
                        VolumeSettings { volume: 0.5 },
                    ));
                    game_state.hints[r][c] ^= true;
                }
            }
        }
    }
}
