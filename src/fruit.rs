use std::time::Duration;

use rand::prelude::*;
use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::chef::FruitHit;


pub struct FruitPlugin;


const SLICE_ANIMATION_SPEED: u64 = 80;


impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (gen_fruit, fall, hit, animate_slice))
        ;
    }
}


#[derive(Resource)]
struct FruitAssets {
    images: Vec<Handle<TextureAtlas>>
}

impl FruitAssets {
    fn get_random_image(&self) -> Handle<TextureAtlas> {
        let mut rng = rand::thread_rng();
        let random_index: usize = rng.gen_range(0..self.images.len());
        return self.images[random_index].clone();
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
struct FruitGenerationTimer(Timer);


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image_names = ["apple-frames.png", "strawberry.png", "orange.png"];
    let mut images: Vec<Handle<TextureAtlas>> = Vec::new();

    for name in image_names {
        let texture = TextureAtlas::from_grid(
            asset_server.load(name),
            Vec2::new(40.0, 40.0),
            4,
            1,
            None,
            None
        );

        let handle = texture_atlases.add(texture);

        images.push(handle);
    }

    let fruit_assets = FruitAssets {
        images: images
    };

    commands.insert_resource(fruit_assets);
    commands.insert_resource(
        FruitGenerationTimer(Timer::new(Duration::from_secs(1), TimerMode::Repeating))
    );
}


fn gen_fruit(
    time: Res<Time>,
    mut timer: ResMut<FruitGenerationTimer>,
    fruit_assets: Res<FruitAssets>,
    mut commands: Commands 
){
    timer.0.tick(time.delta());

    if timer.0.finished() {
        let combo = randint(1, 4);
        let x_axis = randint(-400, 400) as f32;

        for i in 0..combo {
            let sprite = SpriteSheetBundle {
                texture_atlas: fruit_assets.get_random_image(),
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_xyz(
                    x_axis, 350. + 40. * i as f32, 1.)
                    .with_scale(Vec3::splat(3.5)),
                ..default()
            };

            commands.spawn((sprite, Fruit::new()));            
        }

        timer.0.set_duration(Duration::from_millis(randint(500, 1500) as u64));

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
