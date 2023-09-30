use bevy::prelude::*;
use bevy::window::{close_on_esc, WindowMode};

use crate::level::validate_solution;

use self::input::GameInputPlugin;

mod input;
mod level;
mod render;

#[derive(Resource)]
pub struct GameState {
    level: level::Level,
    solution: level::Solution,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
enum AppState {
    InGame,
    SwitchLevel,
}

impl Default for AppState {
    fn default() -> Self {
        AppState::SwitchLevel
    }
}

fn setup(mut commands: Commands, mut app_state: ResMut<NextState<AppState>>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((render::LevelRender::default(), SpatialBundle::default()));
    let (level, solution) = level::third_level();
    commands.insert_resource(GameState { level, solution });
    app_state.set(AppState::InGame);
}

fn main() {
    let levels = vec![level::first_level(), level::second_level()];
    for (level, solution) in levels {
        println!("{}: {:?}", level, validate_solution(&solution, &level));
    }

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
        .add_systems(
            Update,
            (render::update_level_render).run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (render::update_placements_render).run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (render::update_available_buildings_text).run_if(in_state(AppState::InGame)),
        )
        .add_plugins(GameInputPlugin)
        .run();
}
