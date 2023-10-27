use bevy::prelude::*;

pub mod fruit;
pub mod sprite;
pub mod spawn;

use crate::global::AppState;



pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(spawn::FruitSpawnPlugin)
            .add_systems(
                OnEnter::<AppState>(AppState::InGame),
                 fruit::setup
            )
            .add_systems(
                Update, 
                (
                    fruit::fall,
                    fruit::despawn_fallen_fruits,
                    fruit::hit, 
                    fruit::animate_slice, 
                    fruit::spawn_boost,
                    fruit::process_bost
                ).run_if(in_state(AppState::InGame))
            )
        ;
    }
}
