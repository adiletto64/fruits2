use std::time::Duration;

use bevy::prelude::*;

use super::sprite::create_wave;


pub struct PenaltyPlugin;


impl Plugin for PenaltyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (spawn_wave, animate_wave))
            .add_event::<WaveEvent>()
        ;
    }
}


#[derive(Component)]
pub struct Wave;

#[derive(Component)]
pub struct WaveTimer(pub Timer);

#[derive(Event)]
pub struct WaveEvent(pub f32);


fn spawn_wave(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
    mut events: EventReader<WaveEvent>,
) {
    for event in events.iter() {
        commands.spawn((
            Wave, 
            create_wave(&asset_server, &mut texture_atlases, event.0),
            WaveTimer(Timer::new(Duration::from_millis(70), TimerMode::Repeating))
        ));
    }
}


fn animate_wave(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut WaveTimer, Entity), With<Wave>>,
    mut commands: Commands
) {
    for (mut sprite, mut timer, entity) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            if sprite.index == 5 {
                commands.entity(entity).despawn();
            }else {
                sprite.index += 1;
            }
        }

    }
}
