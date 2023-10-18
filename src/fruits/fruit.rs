use std::time::Duration;

use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::random::randint;
use crate::chef::ChefHitEvent;
use crate::level::LevelUpdate;
use crate::session::Session;
use crate::state::AppState;
use crate::sound::{SoundEvent, SoundType};

use super::sprite::FruitTextures;
use super::spawn::{spawn_fruits, FruitSpawnTimer};


const FALL_SPEED: f32 = 400.;

const SLICE_ANIMATION_SPEED: u64 = 80;
const SPAWN_INTENSITY_UPDATE_PERCENT: u32 = 90;


pub struct FruitPlugin;
impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter::<AppState>(AppState::InGame), setup)
            .add_systems(
                Update, 
                (
                    spawn_fruits, 
                    fall,
                    despawn_fallen_fruits,
                    hit, 
                    animate_slice, 
                    increase_spawn_intensity,
                    spawn_boost,
                    process_bost
                ).run_if(in_state(AppState::InGame))
            )
        ;
    }
}


#[derive(PartialEq, Eq, Debug)]
pub enum FruitType {
    RIPE,
    PINEAPPLE
}


#[derive(Component)]
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
            fruit_type: FruitType::RIPE,
        } 
    }
}


#[derive(Component)]
struct HitAnimation {timer: Timer}

#[derive(Component)]
struct Boost {height: f32}


fn setup(
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


fn increase_spawn_intensity(
    events: EventReader<LevelUpdate>,
    mut spawn_timer: ResMut<FruitSpawnTimer>
) {
    if !events.is_empty() {
        let duration = spawn_timer.0.duration();
        spawn_timer.0.set_duration(duration / 100 * SPAWN_INTENSITY_UPDATE_PERCENT);        
    }
}



fn hit(
    mut events: EventReader<ChefHitEvent>, 
    mut sound_event_writer: EventWriter<SoundEvent>,
    mut query: Query<(&Transform, Entity, &mut Fruit)>,
    mut commands: Commands,
    mut session: ResMut<Session>
) {
    for event in events.iter() {
        let mut sound_created = false;

        for (transform, entity, mut fruit) in &mut query {

            let hit_the_fruit = collide(
                transform.translation, Vec2::new(140., 220.), 
                event.translation, Vec2::new(40., 40.), 
            ).is_some();

            if hit_the_fruit {
                let mut sliced_fruits = 0;

                if !fruit.sliced {
                    start_slice_animation(&mut commands, &entity);

                    session.score += 1;
                    sliced_fruits += 1;

                    fruit.fall_speed += 100.;
                    fruit.spread_speed = (randint(-5, 5) as f32) * 100.;
                    
                    fruit.sliced = true;
                }
                
                if fruit.fruit_type == FruitType::PINEAPPLE {
                    session.boosts += 1;
                }

                if !sound_created && sliced_fruits > 0 {
                    sound_event_writer.send(SoundEvent {
                        sound_type: SoundType::SPLAT
                    });
                    sound_created = true;
                }
            }
        }
    }
}


fn fall(
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


fn despawn_fallen_fruits(
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


fn spawn_boost(
    mut commands: Commands, 
    keys: Res<Input<KeyCode>>,
    mut session: ResMut<Session>
) {
    if keys.just_pressed(KeyCode::A) && session.boosts > 0 {
        commands.spawn(Boost {
            height: -450.0
        });
        session.boosts -= 1;
    }
}


fn process_bost(
    mut commands: Commands,
    mut boosts: Query<(&mut Boost, Entity)>,
    time: Res<Time>, 
    mut query: Query<(&Transform, &mut Fruit, Entity)>,
    mut session: ResMut<Session>,
    mut sound_event_writer: EventWriter<SoundEvent>,
) {
    for (mut boost, boost_entity) in &mut boosts {
        boost.height += 1000. * time.delta_seconds();

        for (transform, mut fruit, entity) in &mut query {
            let collides_with_fruit = collide(
                transform.translation, Vec2::new(10., 10.), 
                Vec3 {y: boost.height, x: 0., z: 0.}, Vec2::new(1000., 10.), 
            ).is_some();

            if collides_with_fruit {
                start_slice_animation(&mut commands, &entity);

                session.score += 1;
                fruit.fall_speed += 100.;
                fruit.sliced = true;

                sound_event_writer.send(SoundEvent {
                    sound_type: SoundType::SPLAT
                });
            }

        }

        if boost.height > 800. {
            commands.entity(boost_entity).despawn();
        }
    }
}


fn start_slice_animation(commands: &mut Commands, entity: &Entity) {
    commands.entity(*entity).insert(
        HitAnimation { 
            timer: Timer::new(Duration::from_millis(SLICE_ANIMATION_SPEED), TimerMode::Repeating)
        }
    );    
}


fn animate_slice(
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
