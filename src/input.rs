use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::{
    App, Input, IntoSystemConfigs, KeyCode, MouseButton, Plugin, Query, Res, ResMut, Resource,
    Update, With,
};
use bevy::window::{PrimaryWindow, Window};

pub struct GameInputPlugin;

#[derive(Resource, Default)]
pub struct SelectedBuilding {
    pub number: Option<usize>,
}

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedBuilding>()
            .add_systems(Update, (keyboard_input,))
            .add_systems(
                Update,
                (handle_left_click).run_if(input_just_pressed(MouseButton::Left)),
            )
            .add_systems(
                Update,
                (handle_right_click).run_if(input_just_pressed(MouseButton::Right)),
            );
    }
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut selected_building: ResMut<SelectedBuilding>) {
    if keys.just_pressed(KeyCode::Key1) {
        selected_building.number = Some(0);
    }
    if keys.just_pressed(KeyCode::Key2) {
        selected_building.number = Some(1);
    }
    if keys.just_pressed(KeyCode::Key3) {
        selected_building.number = Some(2);
    }
    if keys.just_pressed(KeyCode::Key4) {
        selected_building.number = Some(3);
    }
}

fn handle_left_click(q_windows: Query<&Window, With<PrimaryWindow>>) {
    // Games typically only have one window (the primary window)
    if let Some(position) = q_windows.single().cursor_position() {
        println!("Cursor is inside the primary window, at {:?}", position);
    } else {
        println!("Cursor is not in the game window.");
    }
}

fn handle_right_click(q_windows: Query<&Window, With<PrimaryWindow>>) {
    // Games typically only have one window (the primary window)
    if let Some(position) = q_windows.single().cursor_position() {
        println!("Cursor is inside the primary window, at {:?}", position);
    } else {
        println!("Cursor is not in the game window.");
    }
}
