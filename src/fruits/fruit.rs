use std::time::Duration;

use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::random::randint;
use crate::chef::FruitHit;
use crate::level::LevelUpdate;
use crate::session::Session;
use crate::state::AppState;

use super::sprite::get_fruit_assets;
use super::spawn::{spawn_fruits, FruitSpawnTimer};


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
                    hit, 
                    animate_slice, 
                    update_level
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
    sliced: bool,
    pub fruit_type: FruitType
}

impl Fruit {
    pub fn new() -> Self { 
        Self { 
            rotation_velocity: randint(-15, 20) as f32 * 0.1, 
            sliced: false,
            fruit_type: FruitType::RIPE
        } 
    }
}


#[derive(Component)]
struct Hit {
    timer: Timer,
}
impl Hit {
    fn start() -> Self {
        return Self {
            timer: Timer::new(Duration::from_millis(SLICE_ANIMATION_SPEED), TimerMode::Repeating),
        }
    }
}



fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    query: Query<Entity, With<Fruit>>
) {
    commands.insert_resource(get_fruit_assets(asset_server, &mut texture_atlases));
    commands.insert_resource(FruitSpawnTimer::new());

    for entity in &query {
        commands.entity(entity).despawn();
    }
}


fn update_level(
    events: EventReader<LevelUpdate>,
    mut spawn_timer: ResMut<FruitSpawnTimer>
) {
    if events.len() > 0 {
        let duration = spawn_timer.0.duration();
        spawn_timer.0.set_duration(duration / 100 * LEVEL_UPDATE_SPAWN_INTENSITY_PERCENT);        
    }
}



fn hit(
    mut events: EventReader<FruitHit>, 
    mut query: Query<(&Transform, Entity, &mut Fruit), With<Fruit>>,
    mut commands: Commands,
    mut session: ResMut<Session>
) {
    for event in events.iter() {
        let mut pinapple_hit = false;

        for (transform, entity, mut fruit) in &mut query {
            if collide(
                transform.translation, 
                Vec2::new(140., 260.), 
                event.translation, 
                Vec2::new(40., 40.), 
            ).is_some() {
                if !fruit.sliced {
                    commands.entity(entity).insert(Hit::start());
                    session.score += 1;
                }
                fruit.sliced = true;
                if fruit.fruit_type == FruitType::PINEAPPLE {
                    pinapple_hit = true;
                }

            }
        }

        if pinapple_hit {
            for (_, entity, mut fruit) in &mut query {
                if !fruit.sliced {
                    fruit.sliced = true;
                    commands.entity(entity).insert(Hit::start());
                    session.score += 1;
                }
            }
        }
    }
}


fn fall(
    time: Res<Time>, 
    mut query: Query<(&mut Transform, &Fruit, Entity)>, 
    mut commands: Commands,
    mut session: ResMut<Session>
) {
    for (mut transform, fruit, entity) in &mut query {
        transform.translation.y -= 400.0 * time.delta_seconds();
        transform.rotate_z(fruit.rotation_velocity.to_radians());

        if transform.translation.y <= -400.0 {
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
            if sprite.index == 5 { 
                commands.entity(entity).remove::<Hit>();
            }
            else {
                sprite.index += 1;
            }
        }        
    }
}
