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


#[derive(PartialEq)]
pub enum SoundType {
    APPLE_SLICE,
    ORANGE_SLICE,
    STRAWBERRY_SLICE,
    SLASH,
    HIT,
    BOOST,
}

impl SoundType {
    fn file(&self) -> &str {
        match self {
            Self::APPLE_SLICE => "audio/apple-slice.wav",
            Self::ORANGE_SLICE => "audio/orange-slice.wav",
            Self::STRAWBERRY_SLICE => "audio/strawberry-slice.wav",
            Self::BOOST => "audio/apple-slice.wav",
            Self::SLASH => "audio/slash.wav",
            Self::HIT => "audio/hit.wav"
        }
    }
}


#[derive(Event)]
pub struct SoundEvent {
    pub sound_type: SoundType,
}

impl SoundEvent {
    pub fn apple_slice() -> Self { Self { sound_type: SoundType::APPLE_SLICE } }
    pub fn orange_slice() -> Self { Self { sound_type: SoundType::ORANGE_SLICE } }
    pub fn strawberry_slice() -> Self { Self { sound_type: SoundType::STRAWBERRY_SLICE } }
    pub fn boost() -> Self { Self { sound_type: SoundType::BOOST } }
    pub fn slash() -> Self { Self { sound_type: SoundType::SLASH } }
    pub fn hit() -> Self { Self { sound_type: SoundType::HIT } }
}


#[derive(Component)]
struct Sound;


fn spawn_sound(
    asset_server: Res<AssetServer>, 
    audio: Res<Audio>,
    mut events: EventReader<SoundEvent>
) {
    for event in events.iter() {
        let handle = asset_server.load(event.sound_type.file());

        let mut volume = 0.0;

        match event.sound_type {
            SoundType::SLASH => volume = 0.6,
            SoundType::HIT   => volume = 0.7,
            _                => volume = 1.0
        }
        audio.play(handle).with_volume(volume);
    }
}
