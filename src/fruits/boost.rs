use bevy::prelude::*;

use crate::sound::{SoundEvent, SoundType};
use crate::states::session::Session;

use super::text::TextEvent;
use super::splash::SplashEvent;
use super::fruit::{Fruit, FruitType, start_slice_animation, DESPAWN_FLOOR};
use super::sprite::create_boost_shot;


pub struct BoostPlugin;

impl Plugin for BoostPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (spawn_boost, process_boost, spawn_boost_shot, animate_boost_shot))
            .add_event::<BoostEvent>()
        ;
    }
}


#[derive(Component)]
pub struct Boost {
    count: usize,
    timer: Timer
}

#[derive(Component)]
struct BoostShot(Timer);


#[derive(Event)]
pub struct BoostEvent {
    point1: Vec3,
    point2: Vec3
}


pub fn spawn_boost(
    mut commands: Commands, 
    mut session: ResMut<Session>,
    keys: Res<Input<KeyCode>>,
    query: Query<(&Transform, &Fruit)>,
    mut sound_event_writer: EventWriter<SoundEvent>,
) {
    if keys.just_pressed(KeyCode::A) && session.boosts > 0 {
        let count = query.iter().filter(|(t, _)| t.translation.y < 300.).count();

        sound_event_writer.send(SoundEvent::sound(SoundType::BOOST));

        commands.spawn(Boost {
            count,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating)
        });
        session.boosts -= 1;
    }
}


pub fn process_boost(
    time: Res<Time>,
    mut commands: Commands,
    mut boosts: Query<(&mut Boost, Entity)>,
    mut query: Query<(&mut Fruit, &Transform, Entity)>,
    mut session: ResMut<Session>,

    mut sound: EventWriter<SoundEvent>,
    mut text: EventWriter<TextEvent>,
    mut splash: EventWriter<SplashEvent>,
    mut boost_shot: EventWriter<BoostEvent>
) {
    for (mut boost, boost_entity) in &mut boosts {
        boost.timer.tick(time.delta());

        if !boost.timer.finished() {
            continue;
        }

        // get fruits with the lowest y coords
        let result = query
            .iter_mut()
            .filter(|(f, t, _)| !f.sliced && t.translation.y > DESPAWN_FLOOR - 20.)
            .min_by(|(_, t1, _), (_, t2, _)| t1.translation.y.partial_cmp(&t2.translation.y).unwrap());

        if let Some((mut fruit, transform, entity)) = result {
            session.score += 1;
            boost.count -= 1;
            
            boost_shot.send(BoostEvent { 
                point1: transform.translation,
                point2: Vec3::new(0.0, -380., 10.) 
            });
            
            start_slice_animation(&mut commands, &entity);
            fruit.slice();

            if fruit.fruit_type == FruitType::PINEAPPLE {
                session.boosts += 1;
                text.send(TextEvent{
                    text: "+1 boost!".to_string(), 
                    y: transform.translation.y,
                    x: transform.translation.x
                });
            } 

            else if fruit.fruit_type == FruitType::POME && session.lives_left < 5 {
                session.lives_left += 1;

                text.send(TextEvent{
                    text: "+1 live!".to_string(), 
                    y: transform.translation.y,
                    x: transform.translation.x
                });
            }
            
            splash.send(SplashEvent{
                x: transform.translation.x,
                y: transform.translation.y,
                fruit_type: fruit.fruit_type.clone()
            });

            sound.send(SoundEvent::fruit_sound(fruit.fruit_type.clone()));
            sound.send(SoundEvent::sound(SoundType::BOOST_HIT));
        }

        if boost.count == 0 {
            commands.entity(boost_entity).despawn();
        }
    }
}


pub fn spawn_boost_shot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<BoostEvent>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>, 
){
    for event in event_reader.iter() {
        let x1 = event.point1.x;
        let x2 = event.point2.x;
        let y1 = event.point1.y;
        let y2 = event.point2.y;

        let x = (x1 - x2) / 2.;
        let y = (y1 - y2) / 2.;

        let angle = (y2 - y1).atan2(x2 - x1);

        let sprite = create_boost_shot(
            &asset_server, 
            &mut texture_atlases, 
            x, 
            y - 400.,
            angle
        );
        
        commands.spawn((BoostShot(Timer::from_seconds(0.08, TimerMode::Repeating)), sprite));        
    }
}


fn animate_boost_shot(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut BoostShot, Entity)>,
    mut commands: Commands
) {
    for (mut sprite, mut boost, entity) in query.iter_mut() {
        boost.0.tick(time.delta());

        if !boost.0.finished() {
            continue;
        }

        if sprite.index == 4 {
            commands.entity(entity).despawn();
            continue;
        }

        sprite.index += 1;
    }
}
