use bevy::text::TextStyle;
use bevy::utils::default;

use bevy::prelude::*;
use bevy::ui::{Style, UiRect, Val};

use crate::level::{all_levels, GameLevel};
use crate::{AppState, GameState};

pub struct SelectLevelScreenPlugin<S: States + Copy>(pub S);

impl<S: States + Copy> Plugin for SelectLevelScreenPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.0), create_select_level_screen)
            .add_systems(Update, (handle_button_click).run_if(in_state(self.0)))
            .add_systems(OnExit(self.0), destroy_select_level_screen);
    }
}

const BUTTON_COLOR: Color = Color::rgb(239.0 / 256.0, 167.0 / 256.0, 115.0 / 256.0);

#[derive(Resource)]
pub struct SelectLevelScreenRoot {
    root: Entity,
}

pub fn create_select_level_screen(mut commands: Commands, server: Res<AssetServer>) {
    let id = commands
        .spawn(NodeBundle {
            style: Style {
                /// Use the CSS Grid algorithm for laying out this node
                display: Display::Grid,
                /// Make node fill the entirety it's parent (in this case the window)
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                /// Set the grid to have 2 columns with sizes [min-content, minmax(0, 1fr)]
                ///   - The first column will size to the size of it's contents
                ///   - The second column will take up the remaining available space
                grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
                /// Set the grid to have 3 rows with sizes [auto, minmax(0, 1fr), 20px]
                ///  - The first row will size to the size of it's contents
                ///  - The second row take up remaining available space (after rows 1 and 3 have both been sized)
                ///  - The third row will be exactly 20px high
                grid_template_rows: vec![
                    GridTrack::auto(),
                    GridTrack::flex(1.0),
                    GridTrack::px(20.),
                ],
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        })
        .with_children(|builder| {
            // Main content grid (auto placed in row 2, column 1)
            builder
                .spawn(NodeBundle {
                    style: Style {
                        /// Make the height of the node fill its parent
                        height: Val::Percent(100.0),
                        /// Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
                        /// As the height is set explicitly, this means the width will adjust to match the height
                        aspect_ratio: Some(1.0),
                        /// Use grid layout for this node
                        display: Display::Grid,
                        // Add 24px of padding around the grid
                        padding: UiRect::all(Val::Px(24.0)),
                        /// Set the grid to have 4 columns all with sizes minmax(0, 1fr)
                        /// This creates 4 exactly evenly sized columns
                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                        /// Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                        /// This creates 4 exactly evenly sized rows
                        grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                        /// Set a 12px gap/gutter between rows and columns
                        row_gap: Val::Px(12.0),
                        column_gap: Val::Px(12.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::GRAY),
                    ..default()
                })
                .with_children(|builder| {
                    for (index, level) in all_levels().iter().enumerate() {
                        item_level(builder, index, level, server.load(crate::TEXT_FONT_NAME));
                    }
                });
        })
        .id();

    commands.insert_resource(SelectLevelScreenRoot { root: id });
}

#[derive(Component, Debug)]
pub struct LevelIndex {
    index: usize,
}

fn item_level(builder: &mut ChildBuilder, index: usize, level: &GameLevel, font: Handle<Font>) {
    builder
        .spawn((
            ButtonBundle {
                style: Style {
                    display: Display::Grid,
                    align_items: AlignItems::Center,
                    justify_items: JustifyItems::Center,
                    padding: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                background_color: BackgroundColor(BUTTON_COLOR),
                border_color: BorderColor(Color::BLACK),
                ..default()
            },
            LevelIndex { index },
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                level.name.clone(),
                TextStyle {
                    font,
                    font_size: 48.0,
                    color: Color::WHITE,
                },
            ));
        });
}

pub fn handle_button_click(
    mut interaction_query: Query<(&Interaction, &LevelIndex), Changed<Interaction>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, level_index) in &mut interaction_query {
        // let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                game_state.current_level = level_index.index;
                app_state.set(AppState::SwitchLevel);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn destroy_select_level_screen(mut commands: Commands, root: Res<SelectLevelScreenRoot>) {
    let mut root_entity = commands.entity(root.root);
    root_entity.despawn();
}
