use crate::level::*;
use crate::AppState;
use crate::GameState;
use crate::GlobalVolumeSettings;
use crate::VolumeSettings;
use bevy::audio::PlaybackMode;
use bevy::audio::Volume;
use bevy::math::Vec2;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use std::default::Default;

pub struct GameScreenPlugin<S: States + Copy>(pub S);

impl<S: States + Copy> Plugin for GameScreenPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.0), create_game_screen)
            .add_systems(
                Update,
                (
                    update_game_screen,
                    update_placements_render,
                    update_buildings_required,
                    update_incorrect_placements,
                    update_cell_hints,
                    detect_complete_level,
                    handle_mouse_input,
                    button_system,
                )
                    .run_if(in_state(self.0)),
            )
            .add_systems(OnExit(self.0), destroy_game_screen);
    }
}

// Tag component used to tag entities added on the game screen.
#[derive(Component)]
pub struct OnGameScreen;

// All actions that can be triggered from a button click.
#[derive(Component)]
enum GameScreenButtonAction {
    Back,
    ToggleSound,
    Complete,
}

pub const CELL_SIZE: f32 = 150.0;

pub const GRASS_LAYER: f32 = 0.0;
pub const MARKER_LAYER: f32 = 100.0;
pub const CELL_LAYER: f32 = 200.0;
pub const CROSS_LAYER: f32 = 300.0;
pub const TEXT_LAYER: f32 = 400.0;
pub const AXIS_LAYER: f32 = 500.0;

// Update if the size of the field grows beyond 10x10.
pub const MAX_HOUSE_COUNT: usize = 100;

const NORMAL_BUTTON: Color = Color::WHITE;
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
// const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const PRESSED_BUTTON: Color = HOVERED_BUTTON;

#[derive(Component, Default)]
pub struct GameScreenRoot {
    random_number: Vec<Vec<u32>>,
}

#[derive(Component)]
pub struct SolutionStatusText;

#[derive(Component)]
pub struct RowBuildingsRequired {
    row: usize,
}

#[derive(Component)]
pub struct ColBuildingsRequired {
    col: usize,
}

#[derive(Component)]
pub struct IncorrectPlacement {
    row: usize,
    col: usize,
}

#[derive(Component)]
pub struct ConstraintViolationRender {
    row: usize,
    col: usize,
}

#[derive(Component)]
pub struct CellHint {
    row: usize,
    col: usize,
}

#[derive(Component)]
pub struct HouseIndex {
    index: usize,
}

pub fn get_cell_texture(server: &Res<AssetServer>, cell_type: CellType) -> Handle<Image> {
    match cell_type {
        CellType::Grass => server.load("grass_iso_1.png"),
        CellType::Tree => server.load("forest_iso.png"),
        CellType::Lake => server.load("lake_iso.png"),
        CellType::Mountain => server.load("mountain_iso.png"),
    }
}

pub fn item_cell(
    builder: &mut ChildBuilder,
    r: usize,
    c: usize,
    puzzle: &Puzzle,
    rid: u32,
    server: &Res<AssetServer>,
) {
    let (_rows, cols) = puzzle.dims();
    let cell_type = puzzle.field[r][c];

    let z = ((cols - c + 1) + r) as f32 * 0.1;

    let texture = get_cell_texture(&server, cell_type);

    let ix = (c as f32 + r as f32) * CELL_SIZE * 0.5;
    let iy = (c as f32 - r as f32) * CELL_SIZE * 0.25;

    let grass_texture = if (r + c) % 2 == 0 {
        server.load(format!("grass_iso_dark_{rid}.png"))
    } else {
        server.load(format!("grass_iso_light_{rid}.png"))
    };
    builder.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
            anchor: Anchor::CenterLeft,
            ..Default::default()
        },
        transform: Transform::from_xyz(ix, iy, z + GRASS_LAYER),
        texture: grass_texture,
        ..Default::default()
    });

    if cell_type != CellType::Grass {
        builder.spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                anchor: Anchor::CenterLeft,
                ..Default::default()
            },
            transform: Transform::from_xyz(ix, iy, z + CELL_LAYER),
            texture,
            ..Default::default()
        });
    }

    builder.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                anchor: Anchor::CenterLeft,
                ..Default::default()
            },
            transform: Transform::from_xyz(ix, iy, z + CROSS_LAYER),
            texture: server.load("cross_iso.png"),
            ..Default::default()
        },
        IncorrectPlacement { row: r, col: c },
    ));

    let constraint_text = match cell_type {
        CellType::Lake => "3",
        CellType::Mountain => "2",
        _ => "",
    };
    let text_bundle = Text2dBundle {
        text: Text::from_section(
            constraint_text,
            TextStyle {
                font: server.load("NotoSerif-SemiBold.ttf"),
                font_size: 32.0,
                color: Color::GRAY,
                ..default()
            },
        )
        .with_alignment(TextAlignment::Center),
        transform: Transform::from_xyz(ix + CELL_SIZE * 0.35, iy + CELL_SIZE * 0.3, z + TEXT_LAYER),
        ..default()
    };
    builder.spawn((text_bundle, ConstraintViolationRender { row: r, col: c }));

    builder.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                anchor: Anchor::CenterLeft,
                ..Default::default()
            },
            transform: Transform::from_xyz(ix, iy, z + MARKER_LAYER),
            texture: server.load(format!("marker_iso_{rid}.png")),
            visibility: Visibility::Hidden,
            ..Default::default()
        },
        CellHint { row: r, col: c },
    ));
}

pub fn item_number_constraints(
    builder: &mut ChildBuilder,
    puzzle: &Puzzle,
    server: &Res<AssetServer>,
) {
    let (rows, cols) = puzzle.dims();

    let text_style = TextStyle {
        font: server.load("NotoSerif-SemiBold.ttf"),
        font_size: 40.0,
        color: Color::WHITE,
        ..default()
    };

    for r in 0..rows {
        let c = cols;
        let ix = (c as f32 + r as f32) * CELL_SIZE * 0.5;
        let iy = (c as f32 - r as f32) * CELL_SIZE * 0.25;

        let text_bundle = Text2dBundle {
            text: Text::from_section(puzzle.row_count[r].to_string(), text_style.clone())
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(
                ix + 0.35 * CELL_SIZE,
                iy + 0.05 * CELL_SIZE,
                AXIS_LAYER,
            ),
            ..default()
        };
        builder.spawn((text_bundle, RowBuildingsRequired { row: r }));
    }

    for c in 0..cols {
        let r = 0;
        let ix = (c as f32 + r as f32) * CELL_SIZE * 0.5;
        let iy = (c as f32 - r as f32) * CELL_SIZE * 0.25;

        let text_bundle = Text2dBundle {
            text: Text::from_section(puzzle.col_count[c].to_string(), text_style.clone())
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(ix + 0.15 * CELL_SIZE, iy + 0.3 * CELL_SIZE, AXIS_LAYER),
            ..default()
        };
        builder.spawn((text_bundle, ColBuildingsRequired { col: c }));
    }
}

pub fn create_hud(commands: &mut Commands, name: &str, server: &Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    // height: Val::Percent(10.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                // background_color: BackgroundColor(Color::RED),
                ..default()
            },
            OnGameScreen,
        ))
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        ..Default::default()
                    },
                    // background_color: BackgroundColor(Color::BLUE),
                    ..Default::default()
                })
                .with_children(|builder| {
                    builder.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(50.0),
                                height: Val::Px(50.0),
                                margin: UiRect::all(Val::Px(20.0)),
                                align_self: AlignSelf::Center,
                                ..default()
                            },
                            background_color: NORMAL_BUTTON.into(),
                            image: UiImage::new(server.load("button_back.png")),
                            ..default()
                        },
                        GameScreenButtonAction::Back,
                    ));

                    builder.spawn(TextBundle::from_section(
                        name,
                        TextStyle {
                            font: server.load(crate::TEXT_FONT_NAME),
                            font_size: 80.0,
                            color: crate::CUSTOM_ORANGE,
                            ..default()
                        },
                    ));
                    builder.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(50.0),
                                height: Val::Px(50.0),
                                margin: UiRect::all(Val::Px(20.0)),
                                align_self: AlignSelf::Center,
                                ..default()
                            },
                            background_color: NORMAL_BUTTON.into(),
                            image: UiImage::new(server.load("button_snd_on.png")),
                            ..default()
                        },
                        GameScreenButtonAction::ToggleSound,
                    ));
                });

            builder.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(280.0),
                        height: Val::Px(90.0),
                        margin: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    image: UiImage::new(server.load("button_next_level.png")),
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
                CompleteBanner,
                GameScreenButtonAction::Complete,
            ));
        });
}

#[derive(Component)]
pub struct CompleteBanner;

pub fn create_game_screen(
    mut commands: Commands,
    game_state: Res<GameState>,
    server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let game_screen_entity = commands.spawn(SpatialBundle::default()).id();
    // This component is added to the entity in the end of this function.
    let mut game_screen_root = GameScreenRoot::default();
    let window = window_query.single();

    let puzzle = &game_state.puzzle;
    let (rows, cols) = puzzle.dims();
    let mut rng = StdRng::seed_from_u64(game_state.current_level as u64);
    game_screen_root.random_number = vec![vec![0; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            game_screen_root.random_number[r][c] = rng.gen();
        }
    }

    // commands
    //     .entity(game_screen_entity)
    //     .with_children(|builder| {
    //         builder.spawn((
    //             SpriteBundle {
    //                 texture: server.load("complete.png"),
    //                 sprite: Sprite {
    //                     custom_size: Some(Vec2::new(300.0, 100.0)),
    //                     anchor: Anchor::TopRight,
    //                     ..Default::default()
    //                 },
    //                 visibility: Visibility::Visible,
    //                 ..Default::default()
    //             },
    //             CompleteBanner,
    //         ));
    //     });

    // commands
    //     .entity(game_screen_entity)
    //     .with_children(|builder| {
    //         builder.spawn(SpriteBundle {
    //             // texture: server.load("full.png"),
    //             sprite: Sprite {
    //                 // custom_size: Some(Vec2::new(window.height(), window.width())),
    //                 anchor: Anchor::CenterLeft,
    //                 color: crate::SKY_COLOR,
    //                 ..Default::default()
    //             },
    //             ..Default::default()
    //         });
    //     });

    // assert_eq!(rows, cols);
    for r in 0..rows {
        for c in 0..cols {
            let rid = game_screen_root.random_number[r][c] % 3 + 1;
            commands
                .entity(game_screen_entity)
                .with_children(|builder| {
                    item_cell(builder, r, c, &puzzle, rid, &server);
                });
        }
    }

    commands
        .entity(game_screen_entity)
        .with_children(|builder| {
            for index in 0..MAX_HOUSE_COUNT {
                builder.spawn((
                    SpriteBundle {
                        texture: server.load("house_iso.png"),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                            anchor: Anchor::CenterLeft,
                            ..Default::default()
                        },
                        visibility: Visibility::Hidden,
                        ..Default::default()
                    },
                    HouseIndex { index },
                ));
            }
        });

    commands
        .entity(game_screen_entity)
        .with_children(|builder| {
            item_number_constraints(builder, &puzzle, &server);
        });

    create_hud(&mut commands, &game_state.name, &server);

    commands.entity(game_screen_entity).insert(game_screen_root);
}

pub fn destroy_game_screen(
    mut commands: Commands,
    mut game_screen_query: Query<(Entity, &mut GameScreenRoot)>,
    query: Query<Entity, With<OnGameScreen>>,
) {
    let (game_screen_entity, _) = game_screen_query.single_mut();
    let mut entity_commands = commands.entity(game_screen_entity);
    entity_commands.despawn_descendants();
    entity_commands.clear_children();
    entity_commands.despawn();

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_game_screen(
    game_state: Res<GameState>,
    mut game_screen_query: Query<(Entity, &GameScreenRoot, &mut Transform)>,
) {
    let (_, _, mut transform) = game_screen_query.single_mut();
    let puzzle = &game_state.puzzle;
    let (rows, cols) = puzzle.dims();
    let (puzzle_width, _puzzle_height) = (cols as f32 * CELL_SIZE, rows as f32 * CELL_SIZE);
    transform.translation = Vec3::new(-puzzle_width / 2.0, 0.0, 0.0);
}

pub fn update_placements_render(
    game_state: Res<GameState>,
    mut houses_query: Query<(&mut Transform, &mut Visibility, &HouseIndex)>,
) {
    let (_rows, cols) = game_state.puzzle.dims();
    for (mut transform, mut visibility, house_index) in houses_query.iter_mut() {
        if house_index.index < game_state.solution.placements.len() {
            let position = game_state.solution.placements[house_index.index].position;
            *visibility = Visibility::Inherited;

            let (c, r) = (position.col, position.row);
            let ix = (c as f32 + r as f32) * CELL_SIZE * 0.5;
            let iy = (c as f32 - r as f32) * CELL_SIZE * 0.25;

            let z = ((cols - c + 1) + r) as f32 * 0.1;

            *transform = Transform::from_xyz(ix, iy, z + CELL_LAYER);
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn update_buildings_required(
    game_state: Res<GameState>,
    mut row_buildings_required_text_query: Query<
        (&mut Text, &RowBuildingsRequired),
        Without<ColBuildingsRequired>,
    >,
    mut col_buildings_required_text_query: Query<
        (&mut Text, &ColBuildingsRequired),
        Without<RowBuildingsRequired>,
    >,
) {
    let validation_result = validate_solution(&game_state.solution, &game_state.puzzle);
    let (rows, cols) = game_state.puzzle.dims();

    let underflow_color = Color::WHITE;
    let match_color = Color::rgb(0.4, 1.0, 0.3);
    let overflow_color = Color::rgb(1.0, 0.3, 0.2);

    for r in 0..rows {
        let color = match validation_result.row_status[r] {
            LineStatus::Underflow => underflow_color,
            LineStatus::Match => match_color,
            LineStatus::Overflow => overflow_color,
        };
        if let Some((mut text, _)) = row_buildings_required_text_query
            .iter_mut()
            .find(|(_, x)| x.row == r)
        {
            text.sections[0].style.color = color;
        }
    }

    for c in 0..cols {
        let color = match validation_result.col_status[c] {
            LineStatus::Underflow => underflow_color,
            LineStatus::Match => match_color,
            LineStatus::Overflow => overflow_color,
        };
        if let Some((mut text, _)) = col_buildings_required_text_query
            .iter_mut()
            .find(|(_, x)| x.col == c)
        {
            text.sections[0].style.color = color;
        }
    }
}

pub fn update_incorrect_placements(
    game_state: Res<GameState>,
    mut incorrect_placements_query: Query<(&mut Visibility, &IncorrectPlacement)>,
    mut constraint_violations_query: Query<(&mut Text, &ConstraintViolationRender)>,
) {
    let validation_result = validate_solution(&game_state.solution, &game_state.puzzle);
    let (rows, cols) = game_state.puzzle.dims();

    let underflow_color = Color::GRAY;
    let match_color = Color::rgb(0.2, 0.8, 0.2);
    let overflow_color = Color::rgb(1.0, 0.3, 0.2);

    for r in 0..rows {
        for c in 0..cols {
            let (mut visibility, _) = incorrect_placements_query
                .iter_mut()
                .find(|(_, x)| x.row == r && x.col == c)
                .unwrap();
            *visibility = Visibility::Hidden;

            let matches_position = |x: &&PlacementViolation| {
                let placement = &game_state.solution.placements[x.house_index];
                placement.position == Position { row: r, col: c }
            };
            if let Some(_) = validation_result
                .placement_violations
                .iter()
                .find(matches_position)
            {
                *visibility = Visibility::Inherited;
            };

            let (mut text, _) = constraint_violations_query
                .iter_mut()
                .find(|(_, x)| x.row == r && x.col == c)
                .unwrap();
            let matches_position =
                |x: &&ConstraintViolation| x.position == Position { row: r, col: c };
            if let Some(result) = validation_result
                .constraint_violations
                .iter()
                .find(matches_position)
            {
                text.sections[0].style.color = match result.violation {
                    ConstraintViolationType::Underflow => underflow_color,
                    ConstraintViolationType::Match => match_color,
                    ConstraintViolationType::Overflow => overflow_color,
                };
            };
        }
    }
}

pub fn update_cell_hints(
    game_state: Res<GameState>,
    mut cell_hint_query: Query<(&mut Visibility, &CellHint)>,
) {
    let (rows, cols) = game_state.puzzle.dims();
    for r in 0..rows {
        for c in 0..cols {
            let (mut visibility, _) = cell_hint_query
                .iter_mut()
                .find(|(_, x)| x.row == r && x.col == c)
                .unwrap();
            *visibility = Visibility::Hidden;

            if game_state.hints[r][c] {
                *visibility = Visibility::Inherited;
            }
        }
    }
}

fn detect_complete_level(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut complete_banner: Query<&mut Visibility, With<CompleteBanner>>,
    server: Res<AssetServer>,
) {
    let validation_result = validate_solution(&game_state.solution, &game_state.puzzle);
    if validation_result.complete {
        let mut visibility = complete_banner.get_single_mut().unwrap();
        if matches!(*visibility, Visibility::Hidden) {
            *visibility = Visibility::Visible;
            commands.spawn((
                AudioBundle {
                    source: server.load("level_success.wav"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        volume: Volume::new_absolute(0.0),
                        speed: 1.2,
                        ..default()
                    },
                    ..default()
                },
                VolumeSettings { volume: 0.12 },
            ));
        }
    }
}

fn handle_mouse_input(
    mouse: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    game_screen_query: Query<&Transform, With<GameScreenRoot>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    let game_screen_transform = game_screen_query.single();
    let (camera, camera_global_transform) = camera_query.single();
    let window = window_query.single();
    let (rows, cols) = game_state.puzzle.dims();

    let left_just_pressed = mouse.just_pressed(MouseButton::Left);
    let right_just_pressed = mouse.just_pressed(MouseButton::Right);

    let isometric_to_orthographic = |pi: Vec2| {
        let pi = pi - game_screen_transform.translation.xy();
        let po = Vec2::new(pi.x + 2.0 * pi.y, pi.x - 2.0 * pi.y);
        po / CELL_SIZE
    };

    if let Some(p) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_global_transform, cursor))
        .map(isometric_to_orthographic)
    {
        let lower_bound = Vec2::new(0.0, 0.0);
        let upper_bound = Vec2::new(cols as f32, rows as f32);
        if p.cmpge(lower_bound).all() && p.cmplt(upper_bound).all() {
            let position = Position {
                row: p.y as usize,
                col: p.x as usize,
            };
            let r = position.row;
            let c = position.col;

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
                            mode: PlaybackMode::Despawn,
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
                                mode: PlaybackMode::Despawn,
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
                    let source = if game_state.hints[r][c] {
                        server.load("erase.wav")
                    } else {
                        server.load("draw.wav")
                    };

                    commands.spawn((
                        AudioBundle {
                            source,
                            settings: PlaybackSettings {
                                mode: PlaybackMode::Despawn,
                                volume: Volume::new_absolute(0.0),
                                speed: 0.9,
                                ..default()
                            },
                            ..default()
                        },
                        VolumeSettings { volume: 0.12 },
                    ));
                    game_state.hints[r][c] ^= true;
                }
            }
        }
    }
}

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &GameScreenButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<GameState>,
    mut app_state: ResMut<NextState<AppState>>,
    mut global_volume_settings: ResMut<GlobalVolumeSettings>,
) {
    for (interaction, mut color, action) in &mut interaction_query {
        *color = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
        };

        if *interaction == Interaction::Pressed {
            match *action {
                GameScreenButtonAction::Back => {
                    app_state.set(AppState::MainMenuScreen);
                }
                GameScreenButtonAction::ToggleSound => {
                    if global_volume_settings.volume == 0.0 {
                        global_volume_settings.volume = 1.0;
                    } else {
                        global_volume_settings.volume = 0.0;
                    }
                }
                GameScreenButtonAction::Complete => {
                    if game_state.current_level + 1 < all_levels().len() {
                        game_state.current_level += 1;
                        app_state.set(AppState::SwitchLevel);
                    }
                }
            };
        }
    }
}
