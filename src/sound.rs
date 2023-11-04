#![allow(non_camel_case_types)]

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;


pub struct SoundPlugin;


impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_plugins(AudioPlugin)
            .add_systems(Update, spawn_sound)
            .add_event::<SoundEvent>()
        ;
    }
}


#[derive(PartialEq, Eq, Debug)]
pub enum SoundType {
    APPLE_SLICE,
    ORANGE_SLICE,
    STRAWBERRY_SLICE,
    PINEAPPLE_SLICE,
    WATERMELON_SLICE,
    BANANA_SLICE,
    SLASH,
    BOOST,
    HIT,
}

impl SoundType {
    const fn file(&self) -> &str {
        match self {
            Self::APPLE_SLICE => "audio/apple-slice.wav",
            Self::ORANGE_SLICE => "audio/orange-slice.wav",
            Self::STRAWBERRY_SLICE => "audio/strawberry-slice.wav",
            Self::PINEAPPLE_SLICE  => "audio/pineapple.wav",
            Self::WATERMELON_SLICE => "audio/watermelon.wav",
            Self::BANANA_SLICE => "audio/banana.wav",
            Self::SLASH => "audio/slash.wav",
            Self::BOOST => "audio/boost.wav",
            Self::HIT => "audio/hit.wav",
        }
    }
}


#[derive(Event, Debug)]
pub struct SoundEvent {
    pub sound_type: SoundType,
}

impl SoundEvent {
    pub const fn sound(sound_type: SoundType) -> Self {
        Self { sound_type }
    } 
}


#[derive(Component)]
struct Sound;


fn spawn_sound(
    asset_server: Res<AssetServer>, 
    audio: Res<Audio>,
    mut events: EventReader<SoundEvent>
) {
    for event in &mut events {

        let handle = asset_server.load(event.sound_type.file());

        let volume =match event.sound_type {
            SoundType::SLASH => 0.5,
            SoundType::HIT   => 0.7,
            _                => 1.0
        };

        audio.play(handle).with_volume(volume);
    }
}
