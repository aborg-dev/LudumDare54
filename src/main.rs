use bevy::audio::PlaybackMode;
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
    hints: Vec<Vec<bool>>,
}

impl GameState {
    pub fn new(puzzle: level::Puzzle, current_level: usize) -> Self {
        let rows = puzzle.rows();
        let cols = puzzle.columns();
        Self {
            puzzle,
            solution: Solution::default(),
            current_level,
            hints: vec![vec![false; cols]; rows],
        }
    }
}

#[derive(Resource)]
pub struct TextureHandles {
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

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
enum AppState {
    InGame,
    #[default]
    SwitchLevel,
}

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let game_level = level::all_levels().swap_remove(0);
    commands.insert_resource(GameState::new(game_level.puzzle, 0));
    commands.insert_resource(TextureHandles {
        textures: ["house.png", "cross.png", "dot.png"]
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

fn switch_levels(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    commands.spawn((render::LevelRender::default(), SpatialBundle::default()));
    let game_level = level::all_levels().swap_remove(game_state.current_level);
    game_state.puzzle = game_level.puzzle;
    game_state.solution = Solution::default();
    game_state.hints = vec![vec![false; game_state.puzzle.columns()]; game_state.puzzle.rows()];
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
                render::update_cell_hints,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_plugins(GameInputPlugin)
        .run();
}
