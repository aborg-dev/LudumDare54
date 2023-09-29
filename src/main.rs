use bevy::prelude::*;
use bevy::window::WindowMode;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy".into(),
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
        .run();
}
