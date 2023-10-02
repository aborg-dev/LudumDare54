use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use bevy::window::{close_on_esc, WindowMode};

use self::game_screen::GameScreenPlugin;
use self::input::GameInputPlugin;
use self::level::Solution;
use self::main_menu_screen::MainMenuScreenPlugin;
use self::select_level_screen::SelectLevelScreenPlugin;

mod game_screen;
mod input;
mod level;
mod main_menu_screen;
mod select_level_screen;

#[derive(Resource)]
pub struct GameState {
    puzzle: level::Puzzle,
    solution: level::Solution,
    name: String,
    current_level: usize,
    hints: Vec<Vec<bool>>,
}

impl GameState {
    pub fn new(game_level: level::GameLevel, current_level: usize) -> Self {
        let puzzle = game_level.puzzle;
        let (rows, cols) = puzzle.dims();
        Self {
            puzzle,
            solution: Solution::default(),
            name: game_level.name,
            current_level,
            hints: vec![vec![false; cols]; rows],
        }
    }
}

#[derive(Resource)]
pub struct TextureHandles {
    #[allow(dead_code)]
    textures: Vec<Handle<u32>>,
}

#[derive(Resource)]
pub struct GlobalVolumeSettings {
    pub volume: f32,
}

#[derive(Component)]
pub struct VolumeSettings {
    pub volume: f32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default, Copy)]
pub enum AppState {
    InGame,
    #[default]
    SwitchLevel,
    SelectLevelScreen,
    MainMenuScreen,
}

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let game_level = level::all_levels().swap_remove(0);
    commands.insert_resource(GameState::new(game_level, 0));
    commands.insert_resource(TextureHandles {
        textures: [
            "cross_iso.png",
            "button_play.png",
            "button_levels.png",
            "button_back.png",
            "continue.png",
            "empty.png",
            "forest_iso.png",
            "house_iso.png",
            "lake_iso.png",
            "mountain_iso.png",
            "grass_iso_dark_1.png",
            "grass_iso_dark_2.png",
            "grass_iso_dark_3.png",
            "grass_iso_light_1.png",
            "grass_iso_light_2.png",
            "grass_iso_light_3.png",
            "marker_iso_1.png",
            "marker_iso_2.png",
            "marker_iso_3.png",
            "full.png",
        ]
        .map(|name| server.load(name))
        .to_vec(),
    });

    commands.spawn((
        AudioBundle {
            source: server.load("ambient.mp3"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
            ..default()
        },
        VolumeSettings { volume: 0.1 },
    ));

    commands.insert_resource(GlobalVolumeSettings { volume: 0.0 });
}

fn update_sounds(
    mut audio_query: Query<(&mut AudioSink, &VolumeSettings)>,
    global_volume_settings: Res<GlobalVolumeSettings>,
) {
    for (sink, volume_settings) in &mut audio_query.iter_mut() {
        sink.set_volume(global_volume_settings.volume * volume_settings.volume);
    }
}

fn switch_levels(mut game_state: ResMut<GameState>, mut app_state: ResMut<NextState<AppState>>) {
    let game_level = level::all_levels().swap_remove(game_state.current_level);
    game_state.puzzle = game_level.puzzle;
    game_state.solution = Solution::default();
    game_state.name = game_level.name;
    game_state.hints = vec![vec![false; game_state.puzzle.cols()]; game_state.puzzle.rows()];
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
        .add_systems(Update, update_sounds)
        .add_systems(OnEnter(AppState::SwitchLevel), switch_levels)
        .add_plugins(MainMenuScreenPlugin(AppState::MainMenuScreen))
        .add_plugins(SelectLevelScreenPlugin(AppState::SelectLevelScreen))
        .add_plugins(GameScreenPlugin(AppState::InGame))
        .add_plugins(GameInputPlugin)
        .run();
}
