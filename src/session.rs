use bevy::prelude::*;

use crate::fruits::fruit::FruitPlugin;
use crate::chef::ChefPlugin;
use crate::level::LevelPlugin;
use crate::info::InfoPlugin;


pub struct SessionPlugin;



impl Plugin for SessionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((FruitPlugin, ChefPlugin, LevelPlugin, InfoPlugin))
        ;
    }
}

