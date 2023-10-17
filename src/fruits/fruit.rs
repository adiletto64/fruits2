use std::time::Duration;

use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::random::randint;
use crate::chef::FruitHit;
use crate::level::LevelUpdate;
use crate::session::Session;
use crate::state::AppState;

use super::sprite::FruitTextures;
use super::spawn::{spawn_fruits, FruitSpawnTimer};


const FALL_SPEED: f32 = 400.;

const SLICE_ANIMATION_SPEED: u64 = 80;
const LEVEL_UPDATE_SPAWN_INTENSITY_PERCENT: u32 = 95;


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
    pub rotation_velocity: f32,
    pub fruit_type: FruitType,
    sliced: bool,
    spread_speed: f32,
    fall_speed: f32
}

impl Fruit {
    pub fn new() -> Self { 
        Self { 
            rotation_velocity: randint(-15, 20) as f32 * 0.1, 
            sliced: false,
            fruit_type: FruitType::RIPE,
            spread_speed: 0.,
            fall_speed: FALL_SPEED
        } 
    }
}

#[derive(Component)]
struct Hit {
    timer: Timer,
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    query: Query<Entity, With<Fruit>>
) {
    commands.insert_resource(FruitTextures::new(&asset_server, &mut texture_atlases));
    commands.insert_resource(FruitSpawnTimer::new());

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
        spawn_timer.0.set_duration(duration / 100 * LEVEL_UPDATE_SPAWN_INTENSITY_PERCENT);        
    }
}



fn hit(
    mut events: EventReader<FruitHit>, 
    mut query: Query<(&Transform, Entity, &mut Fruit), With<Fruit>>,
    mut commands: Commands,
    mut session: ResMut<Session>,
    asset_server: Res<AssetServer>
) {
    for event in events.iter() {
        let mut sound_created = false;

        for (transform, entity, mut fruit) in &mut query {
            let hit_the_fruit = collide(
                transform.translation, Vec2::new(140., 220.), 
                event.translation, Vec2::new(40., 40.), 
            ).is_some();

            if hit_the_fruit {
                if !fruit.sliced {
                    commands.entity(entity).insert(
                        Hit { 
                            timer: Timer::new(Duration::from_millis(SLICE_ANIMATION_SPEED), TimerMode::Repeating)
                        }
                    );

                    session.score += 1;
                    fruit.spread_speed = (randint(-5, 5) as f32) * 100.;
                    fruit.fall_speed += 100.;
                    fruit.sliced = true;
                }
                
                if fruit.fruit_type == FruitType::PINEAPPLE {
                    session.boosts += 1;
                }

                if !sound_created {
                    commands.spawn(get_splat_audio(&asset_server));
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
        transform.rotate_z(fruit.rotation_velocity.to_radians());

        if fruit.spread_speed != 0. {
            fruit.spread_speed *= 0.9;
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


fn animate_slice(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut Hit, Entity), With<Fruit>>,
    mut commands: Commands
) {

    for (mut sprite, mut hit, entity) in query.iter_mut() {
        hit.timer.tick(time.delta());
        if hit.timer.just_finished() {
            if sprite.index == 7 { 
                commands.entity(entity).remove::<Hit>();
            }
            else {
                sprite.index += 1;
            }
        }        
    }
}


fn get_splat_audio(asset_server: &Res<AssetServer>) -> AudioBundle {
    AudioBundle{
        source: asset_server.load("audio/splat1.ogg"), 
        settings: PlaybackSettings {
            speed: 1.5,
            ..default()
        },
        ..default()
    }
}
