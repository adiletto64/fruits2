use std::time::Duration;
use bevy::prelude::*;

use crate::states::session::Session;
use crate::global::AppState;

const LEVEL_UPDATE_TIME: u64 = 6;


pub struct LevelPlugin;


impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
            .add_systems(Update, update_level.run_if(in_state(AppState::InGame)))
            .add_event::<LevelUpdate>();
    }
}



#[derive(Event)]
pub struct LevelUpdate {
    #[allow(dead_code)]
    number: u32
}


#[derive(Resource)]
struct Level { 
    timer: Timer, 
    number: u32
}


fn startup(mut commands: Commands) {
    commands.insert_resource(
        Level {
            timer: Timer::new(Duration::from_secs(LEVEL_UPDATE_TIME), TimerMode::Repeating),
            number: 1
        }
    );
}


fn update_level(
    time: Res<Time>,
    mut level: ResMut<Level>,
    mut event: EventWriter<LevelUpdate>,
    mut session: ResMut<Session>
) {
    level.timer.tick(time.delta());

    if level.timer.finished() {
        level.number += 1;
        event.send(LevelUpdate { number: level.number });
        session.level = level.number;
    }
}
