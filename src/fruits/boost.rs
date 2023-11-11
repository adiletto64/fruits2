use bevy::prelude::*;

use crate::sound::{SoundEvent, SoundType};
use crate::states::session::Session;

use super::fruit::{Fruit, start_slice_animation};


pub struct BoostPlugin;

impl Plugin for BoostPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (spawn_boost, process_boost))
        ;
    }
}



#[derive(Component)]
pub struct Boost {
    count: usize
}


pub fn spawn_boost(
    mut commands: Commands, 
    mut session: ResMut<Session>,
    keys: Res<Input<KeyCode>>,
    query: Query<(&Transform, &Fruit)>
) {
    if keys.just_pressed(KeyCode::A) && session.boosts > 0 {
        let count = query.iter().filter(|(t, _)| t.translation.y < 300.).count();

        commands.spawn(Boost {count});
        session.boosts -= 1;
    }
}


pub fn process_boost(
    mut commands: Commands,
    mut boosts: Query<(&mut Boost, Entity)>,
    mut query: Query<(&mut Fruit, Entity)>,
    mut session: ResMut<Session>,
    mut sound_event_writer: EventWriter<SoundEvent>,
) {
    for (mut boost, boost_entity) in &mut boosts {
        for (mut fruit, entity) in &mut query {
            if boost.count > 0 {
                start_slice_animation(&mut commands, &entity);

                session.score += 1;
                fruit.fall_speed += 100.;
                fruit.sliced = true;

                sound_event_writer.send(SoundEvent::sound(SoundType::BOOST));
                boost.count -= 1;
            }
        }

        if boost.count == 0 {
            commands.entity(boost_entity).despawn();
        }
    }
}
