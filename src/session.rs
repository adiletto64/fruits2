use bevy::prelude::*;

use crate::fruits::fruit::FruitPlugin;
use crate::chef::ChefPlugin;
use crate::level::LevelPlugin;
use crate::info::InfoPlugin;

use crate::state::AppState;


pub struct SessionPlugin;



impl Plugin for SessionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((FruitPlugin, ChefPlugin, LevelPlugin, InfoPlugin))
            .add_systems(Startup, setup)
            .add_systems(Update, (pause, check_lives).run_if(in_state(AppState::InGame)))
        ;
    }
}


#[derive(Resource, Clone)]
pub struct Session {
    pub level: u32,
    pub lives_left: u32,
    pub score: u32,
    pub boosts: u32
}

impl Session {
    pub fn default() -> Self {
        Self { level: 1, lives_left: 3, score: 0, boosts: 0 }
    }
    pub fn text(&self) -> String {
        format!("Current level: {}\nLive left: {}\nscore: {}", self.level, self.lives_left, self.score)
    }
}


fn setup(mut commands: Commands) {
    commands.insert_resource(Session::default());
}


fn check_lives(session: Res<Session>, mut app_state: ResMut<NextState<AppState>>) {
    if session.lives_left == 0 {
        app_state.set(AppState::Finish);
    }
}


fn pause(keys: Res<Input<KeyCode>>, mut app_state: ResMut<NextState<AppState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::Paused);
    }
}
