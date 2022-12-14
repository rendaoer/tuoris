use bevy::prelude::*;

use crate::{
    componets::ui::menu::{Main, Title},
    resources::GameFont,
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, setup_main_ui)
            .add_system(button_system);
    }
}

fn setup_main_ui(mut commands: Commands, game_font: Res<GameFont>) {
    // 主要层
    let main_node = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };

    // 标题层
    let title_node = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(40.0)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };
    // 标题文本
    let title_text = TextBundle::from_section(
        "Tuoris",
        TextStyle {
            font: game_font.smiley_sans_oblique.clone(),
            font_size: 180.0,
            color: Color::WHITE,
        },
    );

    // 菜单按钮层
    let button_node = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(60.0)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };

    // 开始按钮
    let start_button = ButtonBundle {
        style: Style {
            margin: UiRect {
                top: Val::Px(20.0),
                bottom: Val::Px(20.0),
                ..default()
            },
            size: Size::new(Val::Px(200.0), Val::Px(50.0)),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::RED.into(),
        ..default()
    };
    // 开始按钮文本
    let start_button_text = TextBundle::from_section(
        "Start",
        TextStyle {
            font: game_font.smiley_sans_oblique.clone(),
            font_size: 40.0,
            color: Color::WHITE,
            ..default()
        },
    );

    // 退出按钮
    let exit_button = ButtonBundle {
        style: Style {
            margin: UiRect {
                top: Val::Px(20.0),
                bottom: Val::Px(20.0),
                ..default()
            },
            size: Size::new(Val::Px(200.0), Val::Px(50.0)),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::RED.into(),
        ..default()
    };
    // 退出按钮文本
    let exit_button_text = TextBundle::from_section(
        "Exit",
        TextStyle {
            font: game_font.smiley_sans_oblique.clone(),
            font_size: 40.0,
            color: Color::WHITE,
            ..default()
        },
    );

    commands
        .spawn((main_node, Main))
        .with_children(|parent| {
            // 游戏标题
            parent.spawn((title_node, Title)).with_children(|parent| {
                parent.spawn(title_text);
            });
        })
        .with_children(|parent| {
            // 菜单 按钮[]
            parent
                .spawn(button_node)
                .with_children(|parent| {
                    // 开始游戏按钮
                    parent.spawn(start_button).with_children(|parent| {
                        parent.spawn(start_button_text);
                    });
                })
                .with_children(|parent| {
                    // 退出游戏按钮
                    parent.spawn(exit_button).with_children(|parent| {
                        parent.spawn(exit_button_text);
                    });
                });
        });
}

const NORMAL_BUTTON: Color = Color::RED;
const HOVERED_BUTTON: Color = Color::WHITE;
const PRESSED_BUTTON: Color = Color::GREEN;

fn button_system(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut button_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].style.color = Color::BLACK;
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].style.color = Color::BLACK;
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].style.color = Color::WHITE;
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
