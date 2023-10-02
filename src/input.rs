use crate::level::all_levels;
use crate::{AppState, GameState, GlobalVolumeSettings};
use bevy::prelude::*;

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_input);
    }
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut app_state: ResMut<NextState<AppState>>,
    mut global_volume_settings: ResMut<GlobalVolumeSettings>,
) {
    if keys.just_pressed(KeyCode::Right) && game_state.current_level + 1 < all_levels().len() {
        game_state.current_level += 1;
        app_state.set(AppState::SwitchLevel);
    }
    if keys.just_pressed(KeyCode::Left) && game_state.current_level > 0 {
        game_state.current_level -= 1;
        app_state.set(AppState::SwitchLevel);
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
