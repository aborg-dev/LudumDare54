use bevy::prelude::*;
use bevy::window::{close_on_esc, WindowMode};

use self::input::GameInputPlugin;
use self::level::Solution;

mod input;
mod level;
mod render;

#[derive(Resource)]
pub struct GameState {
    puzzle: level::Puzzle,
    solution: level::Solution,
    current_level: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
enum AppState {
    InGame,
    #[default]
    SwitchLevel,
}

fn setup(mut commands: Commands, mut app_state: ResMut<NextState<AppState>>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((render::LevelRender::default(), SpatialBundle::default()));
    let game_level = level::all_levels().swap_remove(0);
    let solution = Solution::default();
    commands.insert_resource(GameState {
        puzzle: game_level.puzzle,
        solution,
        current_level: 0,
    });
    app_state.set(AppState::InGame);
}

fn switch_levels(mut game_state: ResMut<GameState>, mut app_state: ResMut<NextState<AppState>>) {
    let game_level = level::all_levels().swap_remove(game_state.current_level);
    game_state.puzzle = game_level.puzzle;
    game_state.solution = Solution::default();
    app_state.set(AppState::InGame);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_game".into(),
                resolution: (800.0, 800.0).into(),
                mode: WindowMode::Windowed,
                // Tells WASM to resize the window according to the available canvas.
                fit_canvas_to_parent: true,
                // Tells WASM not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .add_systems(OnEnter(AppState::InGame), render::create_level_render)
        .add_systems(OnExit(AppState::InGame), render::destroy_level_render)
        .add_systems(OnEnter(AppState::SwitchLevel), switch_levels)
        .add_systems(
            Update,
            (
                render::update_level_render,
                render::update_placements_render,
                render::update_buildings_required,
                render::update_incorrect_placements,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_plugins(GameInputPlugin)
        .run();
}
