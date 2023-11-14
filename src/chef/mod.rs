use bevy::prelude::*;

use crate::states::session;
use crate::global::AppState;

mod sprite;
mod chef;


const FROM_FINISH_ENTER: OnTransition<AppState> = OnTransition::<AppState> {
    from: AppState::Finish,
    to: AppState::InGame
};


pub struct ChefPlugin;
impl Plugin for ChefPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(FROM_FINISH_ENTER, chef::reset_speed)
            .add_systems(Startup, chef::setup.after(session::setup))
            .add_systems(Update, (
                chef::hit, 
                chef::walk, 
                chef::collect_rotten_fruits, 
                chef::animate, 
                chef::update_level
            ))
            .add_event::<ChefHitEvent>();
    }
}


#[derive(Event)]
pub struct ChefHitEvent {
    pub translation: Vec3
}
