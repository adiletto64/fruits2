#![allow(non_camel_case_types)]

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use crate::fruits::fruit::FruitType;


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
    SLASH,
    HIT,
    PENALTY,
    BOOST,
    BOOST_HIT,
}


#[derive(Event, Debug)]
pub struct SoundEvent {
    pub sound: &'static str,
}

impl SoundEvent {
    pub const fn sound(sound_type: SoundType) -> Self {
        let file = match sound_type {
            SoundType::SLASH => "audio/slash.wav",
            SoundType::HIT => "audio/hit.wav",
            SoundType::PENALTY => "audio/penalty.wav",
            SoundType::BOOST => "audio/boost.wav",
            SoundType::BOOST_HIT => "audio/critical.wav"
        };

        Self {sound: file}
    } 

    pub const fn fruit_sound(fruit: FruitType) -> Self {
        let file = match fruit {
            FruitType::APPLE => "audio/apple-slice.wav",
            FruitType::ORANGE => "audio/orange-slice.wav",
            FruitType::STRAWBERRY => "audio/strawberry-slice.wav",
            FruitType::PINEAPPLE  => "audio/pineapple.wav",
            FruitType::WATERMELON => "audio/watermelon.wav",
            FruitType::BANANA => "audio/banana.wav",
            FruitType::POME => "audio/pome.wav"
        };

        Self {sound: file}
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

        let handle = asset_server.load(event.sound);

        let volume = match event.sound {
            "audio/slash.wav" => 0.5,
            "audio/hit.wav"   => 0.7,
            _                 => 1.0
        };

        audio.play(handle).with_volume(volume);
    }
}
