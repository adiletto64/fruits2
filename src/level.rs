use bevy::prelude::*;

use crate::components::Clock;
use crate::states::session::Session;
use crate::global::AppState;

const LEVEL_UPDATE_TIME: f32 = 6.;


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
    clock: Clock, 
    number: u32
}


fn startup(mut commands: Commands) {
    commands.insert_resource(
        Level {
            clock: Clock::seconds(LEVEL_UPDATE_TIME),
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
    level.clock.tick(time.delta());

    if level.clock.finished() {
        level.number += 1;
        event.send(LevelUpdate { number: level.number });
        session.level = level.number;
    }
}
