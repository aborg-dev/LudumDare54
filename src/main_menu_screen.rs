use bevy::prelude::*;

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

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    SettingsDisplay,
    SettingsSound,
    BackToMainMenu,
    BackToSettings,
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
    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_style = TextStyle {
        font: server.load("NotoSerif-SemiBold.ttf"),
        font_size: 40.0,
        color: TEXT_COLOR,
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
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "Flying paradise",
                            TextStyle {
                                font: server.load("NotoSerif-SemiBold.ttf"),
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
                    // - new game
                    // - settings
                    // - quit
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            let icon = server.load("textures/Game Icons/right.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "New Game",
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Settings,
                        ))
                        .with_children(|parent| {
                            let icon = server.load("textures/Game Icons/wrench.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "Settings",
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            let icon = server.load("textures/Game Icons/exitRight.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style,
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section("Quit", button_text_style));
                        });
                });
        });
}

fn destroy_main_menu_screen() {}

fn update_main_menu_screen() {}

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}
