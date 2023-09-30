use bevy::prelude::*;
use bevy::window::{close_on_esc, WindowMode};

use crate::level::validate_solution;

use self::input::GameInputPlugin;

mod level;
mod render;
mod input;

#[derive(Resource)]
pub struct GameState {
    level: level::Level,
    solution: level::Solution,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((render::LevelRender::default(), SpatialBundle::default()));
    let (level, solution) = level::third_level();
    commands.insert_resource(GameState{level, solution});
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
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .add_systems(Update, render::render_level_and_solution)
        .add_plugins(GameInputPlugin)
        .run();
}
