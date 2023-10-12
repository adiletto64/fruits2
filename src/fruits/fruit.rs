use std::time::Duration;

use rand::prelude::*;
use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::chef::FruitHit;
use crate::level::LevelUpdate;

use super::sprite::{FruitAssets, get_sprite, get_fruit_assets};


const SLICE_ANIMATION_SPEED: u64 = 80;
const MAX_COMBO_FRUITS: i32 = 4;
const FRUITS_SPAWN_SPAN: (i32, i32) = (-350, 350);
const LEVEL_UPDATE_SPAWN_INTENSITY_PERCENT: u32 = 95;


pub struct FruitPlugin;
impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (spawn_fruits, fall, hit, animate_slice, update_level))
        ;
    }
}


#[derive(Component)]
struct Fruit {
    rotation_velocity: f32,
}
impl Fruit {
    fn new() -> Self { Self { rotation_velocity: randint(-15, 20) as f32 * 0.1 } }
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


#[derive(Resource)]
struct FruitSpawnTimer(Timer);


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.insert_resource(get_fruit_assets(asset_server, &mut texture_atlases));
    commands.insert_resource(
        FruitSpawnTimer(Timer::new(Duration::from_secs(1), TimerMode::Repeating))
    );
}


fn spawn_fruits(
    time: Res<Time>,
    mut timer: ResMut<FruitSpawnTimer>,
    fruit_assets: Res<FruitAssets>,
    mut commands: Commands 
){
    timer.0.tick(time.delta());

    if timer.0.finished() {
        let combo = randint(1, MAX_COMBO_FRUITS);
        let x_axis = randint(FRUITS_SPAWN_SPAN.0, FRUITS_SPAWN_SPAN.1) as f32;

        for i in 0..combo {
            let sprite = get_sprite(&fruit_assets, x_axis, i as f32 * 30. + 330.);
            commands.spawn((sprite, Fruit::new()));            
        }
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
    mut query: Query<(&Transform, Entity), With<Fruit>>,
    mut commands: Commands,
) {
    for event in events.iter() {
        for (transform, entity) in &mut query {
            if collide(
                transform.translation, 
                Vec2::new(140., 240.), 
                event.translation, 
                Vec2::new(40., 40.), 
            ).is_some() {

                commands.entity(entity).insert(Hit::start());
            }
        }
    }
}

fn fall(time: Res<Time>, mut query: Query<(&mut Transform, &Fruit, Entity)>, mut commands: Commands) {
    for (mut transform, fruit, entity) in &mut query {
        transform.translation.y -= 400.0 * time.delta_seconds();
        transform.rotate_z(fruit.rotation_velocity.to_radians());

        if transform.translation.y <= -400.0 {
            commands.entity(entity).despawn();
        }
    }
}



fn randint(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..max);
}



fn animate_slice(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut Hit, Entity), With<Fruit>>,
    mut commands: Commands
) {

    for (mut sprite, mut hit, entity) in query.iter_mut() {
        hit.timer.tick(time.delta());
        if hit.timer.just_finished() {
            if sprite.index == 3 { 
                commands.entity(entity).remove::<Hit>();
            }
            else {
                sprite.index += 1;
            }
        }        
    }
}
