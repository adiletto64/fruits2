use std::time::Duration;

use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::utils::random::randint;
use crate::chef::ChefHitEvent;
use crate::sound::{SoundEvent, SoundType};
use crate::states::session::Session;

use super::sprite::{FruitTextures, create_splash, SplashColor};
use super::spawn::FruitSpawnTimer;


const FALL_SPEED: f32 = 400.;
const SLICE_ANIMATION_SPEED: u64 = 80;


#[derive(PartialEq, Eq, Debug, Clone)]
pub enum FruitType {
    APPLE,
    STRAWBERRY,
    ORANGE,
    WATERMELON,
    PINEAPPLE,
    BANANA
}


#[derive(Component, Clone, Debug)]
pub struct Fruit {
    pub rotation_speed: f32,
    pub spread_speed: f32,
    pub fall_speed: f32,
    pub fruit_type: FruitType,
    sliced: bool,
}


impl Fruit {
    pub fn new() -> Self { 
        Self { 
            rotation_speed: randint(-15, 20) as f32 * 0.1, 
            spread_speed: 0.,
            fall_speed: FALL_SPEED,
            sliced: false,
            fruit_type: FruitType::APPLE,
        } 
    }
}


#[derive(Component)]
pub struct Splash {timer: Timer}

impl Splash {
    fn new() -> Self {
        Self { timer: Timer::new(Duration::from_millis(SLICE_ANIMATION_SPEED), TimerMode::Repeating) }
    }
}

#[derive(Component)]
pub struct HitAnimation {timer: Timer}


#[derive(Component)]
pub struct Boost {
    count: usize
}


pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    query: Query<Entity, With<Fruit>>
) {
    commands.insert_resource(FruitTextures::new(&asset_server, &mut texture_atlases));
    commands.insert_resource(FruitSpawnTimer::new());

    // cleanup fruits on restart
    for entity in &query {
        commands.entity(entity).despawn();
    }
}


pub fn hit(
    mut events: EventReader<ChefHitEvent>, 
    mut sound: EventWriter<SoundEvent>,
    mut query: Query<(&Transform, Entity, &mut Fruit)>,
    mut commands: Commands,
    mut session: ResMut<Session>,
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>, 
) {
    for event in events.iter() {
        let mut hitted_fruits = Vec::<Fruit>::new();

        for (transform, entity, mut fruit) in &mut query {

            let hit_the_fruit = collide(
                transform.translation, Vec2::new(140., 220.), 
                event.translation, Vec2::new(40., 40.), 
            ).is_some();

            if hit_the_fruit {
                if !fruit.sliced {
                    start_slice_animation(&mut commands, &entity);
                    session.score += 1;
                    
                    fruit.fall_speed += 100.;
                    fruit.spread_speed = (randint(-5, 5) as f32) * 100.;
                    fruit.sliced = true;

                    hitted_fruits.push(fruit.clone());
                }
                
                if fruit.fruit_type == FruitType::PINEAPPLE {
                    session.boosts += 1;
                }

                // create splash effect
                let splash_color = match fruit.fruit_type {
                    FruitType::APPLE | FruitType::BANANA | FruitType::PINEAPPLE => SplashColor::Yellow,
                    FruitType::ORANGE => SplashColor::Orange,
                    FruitType::STRAWBERRY | FruitType::WATERMELON => SplashColor::Red
                };

                let sprite = create_splash(
                    &asset_server, 
                    &mut texture_atlases, 
                    transform.translation.x, 
                    transform.translation.y,
                    splash_color
                );
                commands.spawn((Splash::new(), sprite));

            }
        };

        sound.send(SoundEvent::sound(SoundType::SLASH));

        if !hitted_fruits.is_empty() {
            sound.send(SoundEvent::sound(SoundType::HIT));
            
        }

        // send sound
        for fruit in hitted_fruits {
            match fruit.fruit_type {
                FruitType::APPLE =>      sound.send(SoundEvent::sound(SoundType::APPLE_SLICE)),
                FruitType::ORANGE =>     sound.send(SoundEvent::sound(SoundType::ORANGE_SLICE)),
                FruitType::STRAWBERRY => sound.send(SoundEvent::sound(SoundType::STRAWBERRY_SLICE)),
                FruitType::WATERMELON => sound.send(SoundEvent::sound(SoundType::WATERMELON_SLICE)),  // TODO set own sounds
                FruitType::PINEAPPLE =>  sound.send(SoundEvent::sound(SoundType::PINEAPPLE_SLICE)), 
                FruitType::BANANA =>     sound.send(SoundEvent::sound(SoundType::BANANA_SLICE)),
            }
        }
    }
}


pub fn fall(
    time: Res<Time>, 
    mut query: Query<(&mut Transform, &mut Fruit)>, 
) {
    for (mut transform, mut fruit) in &mut query {
        transform.translation.y -= fruit.fall_speed * time.delta_seconds();
        transform.rotate_z(fruit.rotation_speed.to_radians());

        if fruit.spread_speed != 0. {
            fruit.spread_speed *= 0.95;
        }

        transform.translation.x += fruit.spread_speed * time.delta_seconds();
    }
}


pub fn despawn_fallen_fruits(
    query: Query<(&Transform, &Fruit, Entity)>, 
    mut commands: Commands,
    mut session: ResMut<Session>
) {
    for (transform, fruit, entity) in &query {
        if transform.translation.y <= -480.0 {
            if !fruit.sliced {
                session.lives_left -= 1;
            }
            commands.entity(entity).despawn();
        }
    }
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


pub fn start_slice_animation(commands: &mut Commands, entity: &Entity) {
    commands.entity(*entity).insert(
        HitAnimation { 
            timer: Timer::new(Duration::from_millis(SLICE_ANIMATION_SPEED), TimerMode::Repeating)
        }
    );    
}


pub fn animate_slice(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut HitAnimation, Entity), With<Fruit>>,
    mut commands: Commands
) {

    for (mut sprite, mut hit, entity) in query.iter_mut() {
        hit.timer.tick(time.delta());
        if hit.timer.just_finished() {
            if sprite.index == 7 { 
                commands.entity(entity).remove::<HitAnimation>();
            }
            else {
                sprite.index += 1;
            }
        }        
    }
}


pub fn animate_splash(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut Transform, &mut Splash, Entity)>,
    mut commands: Commands
) {
    for (mut sprite, mut transform, mut splash, entity) in query.iter_mut() {
        splash.timer.tick(time.delta());
        if splash.timer.just_finished() {
            if sprite.index < 5 { 
                sprite.index += 1;
                
            }
        }        

        transform.translation.y -= 300. * time.delta_seconds();
        
        let alpha = sprite.color.a();
        sprite.color.set_a(alpha - 0.01);

        if transform.translation.y < -480. {
            commands.entity(entity).despawn();
        }
    }
}
