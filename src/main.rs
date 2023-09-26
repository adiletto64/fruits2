use std::time::Duration;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (add_chef, add_apple, setup))
        .add_systems(Update, keyboard_input)
        .add_systems(Update, (hit, fall, spawn_apples, remove_apples))
        .run();
}


#[derive(Component)]
struct Player;


#[derive(Component)]
struct Apple;


#[derive(Resource)]
struct Chef {
    normal: Handle<Image>,
    hit: Handle<Image>
}

#[derive(Resource)]
struct AppleSpawnTimer {
    timer: Timer,
}


const SPEED: f32 = 800.;


fn add_chef(mut commands: Commands, asset_server: Res<AssetServer>) {
    let transform = Transform::from_scale(Vec3::new(0.3, 0.3, 0.3));

    let chef: Handle<Image> = asset_server.load("chef.png");
    let chef_hit: Handle<Image> = asset_server.load("chef-hit.png");

    let sprite = SpriteBundle {
        texture: chef.clone(),
        transform: transform,
        ..default()
    };        
    
    commands.spawn((sprite, Player));
    commands.insert_resource(Chef {normal: chef, hit: chef_hit});
}


fn add_apple(mut commands: Commands, asset_server: Res<AssetServer>) {
    let transform = Transform::from_xyz(100., 200., 0.)
        .with_scale(Vec3::new(0.2, 0.2, 0.2));

    let apple: Handle<Image> = asset_server.load("apple.png");

    let sprite = SpriteBundle {
        texture: apple.clone(),
        transform: transform,
        ..default()
    };        
    
    commands.spawn((sprite, Apple));
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(AppleSpawnTimer {
        timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating)
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("bg.png"),
        transform: Transform::from_xyz(0.0, 0.0, -1.0).with_scale(Vec3::splat(1.7)),
        ..default()
    });
}



fn keyboard_input(
    keys: Res<Input<KeyCode>>, 
    time: Res<Time>, 
    mut query: Query<&mut Transform, With<Player>>
) {

    if keys.pressed(KeyCode::Left) {
        for mut transform in &mut query {
            transform.translation.x -= SPEED * time.delta_seconds();
        }
    }
    else if keys.pressed(KeyCode::Right){
        for mut transform in &mut query {
            transform.translation.x += SPEED * time.delta_seconds();
        }
    }
}


fn hit(
    keys: Res<Input<KeyCode>>, 
    images: Res<Chef>,
    mut query: Query<&mut Handle<Image>, With<Player>>
) {
    if keys.pressed(KeyCode::Space) {
        for mut handle in &mut query {
            *handle = images.hit.clone();
        }
    } else {
        for mut handle in &mut query {
            *handle = images.normal.clone();
        }
    }
}


fn fall(time: Res<Time>, mut query: Query<&mut Transform, With<Apple>>) {
    for mut apple in &mut query {
        apple.translation.y -= 300.0 * time.delta_seconds();
    }
}


fn spawn_apples(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut config: ResMut<AppleSpawnTimer>,
) {
    // tick the timer
    config.timer.tick(time.delta());

    if config.timer.finished() {
        let transform = Transform::from_xyz(100., 200., 0.)
        .with_scale(Vec3::new(0.2, 0.2, 0.2));

        let apple: Handle<Image> = asset_server.load("apple.png");

        let sprite = SpriteBundle {
            texture: apple.clone(),
            transform: transform,
            ..default()
        };        
        
        commands.spawn((sprite, Apple));
    }
}


fn remove_apples(
    mut commands: Commands,
    query: Query<(&Transform, Entity), With<Apple>>
) {
    for (transform, entity) in &query {
        if transform.translation.y <= -300.0 {
            commands.entity(entity).despawn();
        }
    }
}
