use std::time::Duration;

use rand::prelude::*;
use bevy::prelude::*;


pub struct FruitPlugin;


impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (gen_fruit, fall))
        ;
    }
}


#[derive(Resource)]
struct FruitAssets {
    images: Vec<Handle<Image>>
}

impl FruitAssets {
    fn get_random_image(&self) -> Handle<Image> {
        let mut rng = rand::thread_rng();
    
        let random_index: usize = rng.gen_range(0..self.images.len());
        return self.images[random_index].clone();
    }
    
}


#[derive(Component)]
struct Fruit;

#[derive(Resource)]
struct FruitGenerationTimer(Timer);



fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let apple_handle: Handle<Image> = asset_server.load("apple.png");
    let banana_handle: Handle<Image> = asset_server.load("banana.png");

    let fruit_assets = FruitAssets {
        images: vec![apple_handle, banana_handle]
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
        let sprite = SpriteBundle {
            texture: fruit_assets.get_random_image(),
            transform: Transform::from_xyz(
                gen_random_number(-600, 600) as f32, 200., 0.)
                .with_scale(Vec3::splat(0.2)),
            ..default()
        };

        commands.spawn((sprite, Fruit));
    }
}


fn fall(time: Res<Time>, mut query: Query<&mut Transform, With<Fruit>>) {
    for mut fruit in &mut query {
        fruit.translation.y -= 300.0 * time.delta_seconds();
    }
}



fn gen_random_number(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..max);
}
