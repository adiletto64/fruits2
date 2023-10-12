use std::time::Duration;
use bevy::prelude::*;


const LEVEL_UPDATE_TIME: u64 = 8;


pub struct LevelPlugin;


impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
            .add_systems(Update, update_level)
            .add_event::<LevelUpdate>();
    }
}



#[derive(Event)]
pub struct LevelUpdate;


#[derive(Resource)]
struct LevelTimer { timer: Timer }


fn startup(mut commands: Commands) {
    commands.insert_resource(
        LevelTimer {
            timer: Timer::new(Duration::from_secs(LEVEL_UPDATE_TIME), TimerMode::Repeating),
        }
    );
}


fn update_level(
    time: Res<Time>,
    mut level: ResMut<LevelTimer>,
    mut event: EventWriter<LevelUpdate>
) {
    level.timer.tick(time.delta());

    if level.timer.finished() {
        event.send(LevelUpdate);
    }
}
