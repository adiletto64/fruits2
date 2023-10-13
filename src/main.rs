// Hide console on Windows on release 
// https://bevy-cheatbook.github.io/platforms/windows.html#disabling-the-windows-console
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

mod chef;
mod fruits;
mod level;
mod random;
mod text;


fn settings() -> bevy::app::PluginGroupBuilder {
    DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fruits game!".into(),
                resolution: (1140., 660.).into(),
                ..default()
            }),
            ..default()
        })
}


fn main() {
    App::new()
        .add_plugins(settings())
        .add_plugins(level::LevelPlugin)
        .add_plugins((chef::ChefPlugin, fruits::fruit::FruitPlugin, text::InfoPlugin))
        .add_systems(Startup, setup)
        .run();
}


fn setup(mut commands: Commands, assert_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // add background image
    commands.spawn(SpriteBundle {
        texture: assert_server.load("bg.png"),
        transform: Transform::from_xyz(0., 0., -1.).with_scale(Vec3::splat(15.)),
        ..default()
    });
}
