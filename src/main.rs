use bevy::prelude::*;
use tuoris::{config::SMILEY_SANS_OBLIQUE_FONT, resources::GameFont, ui::UIPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Tuoris".to_string(),
                width: 1280.,
                height: 720.,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup_system)
        .add_plugin(UIPlugin)
        .run();
}

fn setup_system(mut commands: Commands, assets_server: Res<AssetServer>) {
    // 添加2D摄像头
    commands.spawn(Camera2dBundle::default());
    // 添加字体资源
    commands.insert_resource(GameFont {
        smiley_sans_oblique: assets_server.load(SMILEY_SANS_OBLIQUE_FONT),
    });
}
