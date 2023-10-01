use bevy::a11y::accesskit::{NodeBuilder, Role};
use bevy::a11y::AccessibilityNode;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::text::TextStyle;
use bevy::utils::default;

use bevy::prelude::*;
use bevy::ui::{
    AlignItems, AlignSelf, FlexDirection, JustifyContent, Overflow, Style, UiRect, Val,
};

use crate::level::all_levels;

#[derive(Resource)]
pub struct SelectLevelScreenRoot {
    root: Entity,
}

pub fn create_select_level_screen(mut commands: Commands, server: Res<AssetServer>) {
    let id = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::Stretch,
                        height: Val::Percent(50.),
                        overflow: Overflow::clip_y(),
                        ..default()
                    },
                    background_color: Color::rgb(0.10, 0.10, 0.10).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Moving panel
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            ScrollingList::default(),
                            AccessibilityNode(NodeBuilder::new(Role::List)),
                        ))
                        .with_children(|parent| {
                            // List items
                            for level in all_levels() {
                                parent.spawn((
                                    TextBundle::from_section(
                                        format!("{}", level.name),
                                        TextStyle {
                                            font: server.load("NotoSerif-SemiBold.ttf"),
                                            font_size: 24.,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    Label,
                                    AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                ));
                            }
                        });
                });

            // left vertical fill (border)
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(200.),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                ..default()
            });
        })
        .id();

    commands.insert_resource(SelectLevelScreenRoot { root: id });
}

pub fn destroy_select_level_screen(mut commands: Commands, root: Res<SelectLevelScreenRoot>) {
    let mut root_entity = commands.entity(root.root);
    root_entity.despawn();
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}
