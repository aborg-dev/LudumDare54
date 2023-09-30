use bevy::prelude::*;
use bevy::window::{close_on_esc, WindowMode};

use crate::level::validate_solution;

mod level;
mod render;

#[derive(Resource)]
struct GameState {
    level: level::Level,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(render::LevelRender::default());
    commands.insert_resource(GameState{level: level::first_level().0});
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
                resolution: (300.0, 300.0).into(),
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
        .add_systems(Update, render::render_level)
        .run();
}
