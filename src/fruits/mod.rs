use bevy::prelude::*;

pub mod fruit;
pub mod sprite;
pub mod spawn;
pub mod splash;
pub mod boost;
pub mod penalty;


pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                spawn::SpawnPlugin, 
                splash::SplashPlugin,
                boost::BoostPlugin,
                fruit::FruitPlugin,
                penalty::PenaltyPlugin
            ))
        ;
    }
}
