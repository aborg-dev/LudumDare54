use bevy::app::AppExit;
use bevy::prelude::*;

use crate::AppState;

pub struct MainMenuScreenPlugin<S: States + Copy>(pub S);

impl<S: States + Copy> Plugin for MainMenuScreenPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.0), create_main_menu_screen)
            .add_systems(
                Update,
                (update_main_menu_screen, button_system).run_if(in_state(self.0)),
            )
            .add_systems(OnExit(self.0), destroy_main_menu_screen);
    }
}

const NORMAL_BUTTON: Color = Color::WHITE;
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const TEXT_COLOR: Color = Color::rgb(239.0 / 256.0, 167.0 / 256.0, 115.0 / 256.0);

const RULES: &str = "
Welcome to Skyland -- a flying island with green-green grass, forests, lakes, mountains, and... houses! 

Your job is to place houses, following the following simple rules:
- Houses cannot be placed in adjacent cells. Diagonal cells are OK. 
- Each row/column should have a given number of houses (written next to it). 
- Lake: exactly 3 houses around it (in the 8 cells surrounding the lake).
- Mountain: exactly 2 houses on the diagonals crossing the mountain (in total). 

Once the houses are placed, the neighbors can live peacefully and enjoy the surroundings! 
";

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Levels,
    Quit,
}

fn create_main_menu_screen(mut commands: Commands, server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "Skyland",
                            TextStyle {
                                font: server.load(crate::TEXT_FONT_NAME),
                                font_size: 80.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );

                    // Display three buttons for each action available from the main menu:
                    // - Play
                    // - Levels
                    // - Quit
                    parent.spawn((
                        ButtonBundle {
                            style: button_style.clone(),
                            background_color: NORMAL_BUTTON.into(),
                            image: UiImage::new(server.load("UI/button_play.png")),
                            ..default()
                        },
                        MenuButtonAction::Play,
                    ));
                    parent.spawn((
                        ButtonBundle {
                            style: button_style.clone(),
                            background_color: NORMAL_BUTTON.into(),
                            image: UiImage::new(server.load("UI/button_levels.png")),
                            ..default()
                        },
                        MenuButtonAction::Levels,
                    ));
                    parent.spawn((
                        ButtonBundle {
                            style: button_style.clone(),
                            background_color: NORMAL_BUTTON.into(),
                            image: UiImage::new(server.load("UI/button_quit.png")),
                            ..default()
                        },
                        MenuButtonAction::Quit,
                    ));
                });
        });
}

fn destroy_main_menu_screen(mut commands: Commands, query: Query<Entity, With<OnMainMenuScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_main_menu_screen() {}

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut color, action) in &mut interaction_query {
        *color = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
        };

        if *interaction == Interaction::Pressed {
            match *action {
                MenuButtonAction::Play => {
                    app_state.set(AppState::SwitchLevel);
                }
                MenuButtonAction::Levels => {
                    app_state.set(AppState::SelectLevelScreen);
                }
                MenuButtonAction::Quit => {
                    exit.send(AppExit);
                }
            };
        }
    }
}
