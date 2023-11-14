use std::time::Duration;

use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::global::AppState;
use crate::utils::random::randint;
use crate::chef::ChefHitEvent;
use crate::sound::{SoundEvent, SoundType};
use crate::states::session::Session;

use super::penalty::WaveEvent;
use super::sprite::FruitTextures;
use super::spawn::SpawnTimer;
use super::splash::SplashEvent;
use super::text::TextEvent;


pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter::<AppState>(AppState::InGame),setup)
            .add_systems(
                Update, 
                (
                    fall,
                    despawn_fallen_fruits,
                    hit, 
                    animate_slice, 
                ).run_if(in_state(AppState::InGame))
            )
        ;
    }
}



const FALL_SPEED: f32 = 400.;
const SLICE_ANIMATION_SPEED: u64 = 80;
pub const DESPAWN_FLOOR: f32 = -480.;


#[derive(PartialEq, Eq, Debug, Clone)]
pub enum FruitType {
    APPLE,
    STRAWBERRY,
    ORANGE,
    WATERMELON,
    PINEAPPLE,
    BANANA,
    POME
}


#[derive(Component, Clone, Debug)]
pub struct Fruit {
    pub rotation_speed: f32,
    pub spread_speed: f32,
    pub fall_speed: f32,
    pub fruit_type: FruitType,
    pub sliced: bool,
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

    pub fn slice(&mut self) {
        if self.fruit_type != FruitType::POME {
            self.fall_speed += 100.;
        }
        self.spread_speed = (randint(-5, 5) as f32) * 100.;
        self.sliced = true;
    }
}


#[derive(Component)]
pub struct HitAnimation {timer: Timer}


pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    query: Query<Entity, With<Fruit>>
) {
    commands.insert_resource(FruitTextures::new(&asset_server, &mut texture_atlases));
    commands.insert_resource(SpawnTimer::new());

    // cleanup fruits on restart
    for entity in &query {
        commands.entity(entity).despawn();
    }
}


pub fn hit(
    mut commands: Commands,
    
    mut events: EventReader<ChefHitEvent>, 
    mut sound: EventWriter<SoundEvent>,
    mut splash: EventWriter<SplashEvent>,
    mut text: EventWriter<TextEvent>,
    
    mut query: Query<(&Transform, Entity, &mut Fruit)>,
    mut session: ResMut<Session>,
) {
    for event in events.iter() {
        let mut hitted_fruits = Vec::<Fruit>::new();

        for (transform, entity, mut fruit) in &mut query {

            let successfull_hit = collide(
                transform.translation, Vec2::new(140., 220.), 
                event.translation, Vec2::new(40., 40.), 
            ).is_some();

            if successfull_hit {
                if !fruit.sliced {
                    session.score += 1;
                    start_slice_animation(&mut commands, &entity);
                    fruit.slice();
                    hitted_fruits.push(fruit.clone());


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
                }
                

                splash.send(SplashEvent{
                    x: transform.translation.x,
                    y: transform.translation.y,
                    fruit_type: fruit.fruit_type.clone()
                });
            }
        };

        sound.send(SoundEvent::sound(SoundType::SLASH));

        if !hitted_fruits.is_empty() {
            sound.send(SoundEvent::sound(SoundType::HIT));
            
        }

        // send sound
        for fruit in hitted_fruits {
            sound.send(SoundEvent::fruit_sound(fruit.fruit_type))
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
    mut session: ResMut<Session>,
    mut wave: EventWriter<WaveEvent>,
    mut sound: EventWriter<SoundEvent>
) {
    for (transform, fruit, entity) in &query {
        if transform.translation.y <= DESPAWN_FLOOR {
            if !fruit.sliced {
                session.lives_left -= 1;
                wave.send(WaveEvent(transform.translation.x));
                sound.send(SoundEvent::sound(SoundType::PENALTY));
            }
            commands.entity(entity).despawn();
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
