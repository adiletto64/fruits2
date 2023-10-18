// Hide console on Windows on release 
// https://bevy-cheatbook.github.io/platforms/windows.html#disabling-the-windows-console
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use bevy::{prelude::*, asset::ChangeWatcher, audio::VolumeLevel};

mod chef;
mod fruits;
mod level;
mod random;
mod info;
mod sound;

mod menu;
mod session;
mod pause;
mod finish;

mod state;
mod ui;



fn main() {
    App::new()
        .add_plugins(settings())
        .add_state::<state::AppState>()
        .add_plugins((
            menu   ::MenuPlugin, 
            sound  ::SoundPlugin, 
            session::SessionPlugin, 
            pause  ::PausePlugin, 
            finish ::FinishPlugin
        ))
        .add_systems(Startup, setup)
        .run();
}



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
        .set(AssetPlugin {
            watch_for_changes: ChangeWatcher::with_delay(Duration::from_secs(1)),
            ..default()
        })
}


fn setup(mut commands: Commands, assert_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    // add background image
    commands.spawn(SpriteBundle {
        texture: assert_server.load("images/bg.png"),
        transform: Transform::from_xyz(0., 0., -1.).with_scale(Vec3::splat(7.5)),
        ..default()
    });

    commands.spawn(AudioBundle {
        source: assert_server.load("audio/bg.ogg"),
        settings: PlaybackSettings { 
            mode: bevy::audio::PlaybackMode::Loop, 
            speed: 2.0, 
            paused: false,
            volume: bevy::audio::Volume::Absolute(VolumeLevel::new(0.3))
        },
        ..default()
    });
}
