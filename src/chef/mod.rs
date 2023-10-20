use bevy::prelude::*;

mod sprite;
mod chef;


pub struct ChefPlugin;
impl Plugin for ChefPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, chef::setup)
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
