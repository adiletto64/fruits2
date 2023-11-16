use bevy::prelude::*;

use crate::{global::AppState, components::Clock};

use super::sprite::create_wave;


pub struct PenaltyPlugin;


impl Plugin for PenaltyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (spawn_wave, animate_wave).run_if(in_state(AppState::InGame)))
            .add_event::<WaveEvent>()
        ;
    }
}


#[derive(Component)]
pub struct Wave;

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
            Clock::millis(70)
        ));
    }
}


fn animate_wave(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut Clock, Entity), With<Wave>>,
    mut commands: Commands
) {
    for (mut sprite, mut clock, entity) in query.iter_mut() {
        clock.tick(time.delta());

        if clock.finished() {
            if sprite.index == 5 {
                commands.entity(entity).despawn();
            }else {
                sprite.index += 1;
            }
        }

    }
}
