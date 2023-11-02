// Hide console on Windows on release 
// https://bevy-cheatbook.github.io/platforms/windows.html#disabling-the-windows-console
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// complexity
// #![warn(clippy::nursery)]

// pedantic
// #![warn(clippy::pedantic, clippy::style)]

#![allow(clippy::upper_case_acronyms, clippy::suboptimal_flops, clippy::needless_pass_by_value, clippy::module_name_repetitions)]


use std::time::Duration;

use bevy::{prelude::*, asset::ChangeWatcher, audio::VolumeLevel};

mod chef;
mod fruits;
mod level;
mod info;
mod sound;

mod states;
mod utils;
mod global;



fn main() {
    std::env::set_var("RUST_LOG", "symphonia_core=error");

    App::new()
        .add_plugins(settings())
        .add_state::<global::AppState>()
        .add_plugins((
            states::menu   ::MenuPlugin, 
            states::session::SessionPlugin, 
            states::pause  ::PausePlugin, 
            states::finish ::FinishPlugin,
            sound  ::SoundPlugin, 
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
        }
    });
}
