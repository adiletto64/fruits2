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
    let image_names = ["apple.png", "banana.png", "strawberry.png", "peach.png", "orange.png"];
    let mut images: Vec<Handle<Image>> = Vec::new();

    for name in image_names {
        images.push(asset_server.load(name))
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
        let combo = gen_random_number(1, 4);
        let x_axis = gen_random_number(-400, 400) as f32;

        for i in 0..combo {
            let sprite = SpriteBundle {
                texture: fruit_assets.get_random_image(),
                transform: Transform::from_xyz(
                    x_axis, 350. + 40. * i as f32, 0.)
                    .with_scale(Vec3::splat(3.5))
                    .with_rotation(Quat::from_rotation_z((gen_random_number(0, 240) as f32).to_radians())),
                ..default()
            };

            commands.spawn((sprite, Fruit));            
        }

        timer.0.set_duration(Duration::from_millis(gen_random_number(500, 1500) as u64));

    }
}


fn fall(time: Res<Time>, mut query: Query<&mut Transform, With<Fruit>>) {
    for mut fruit in &mut query {
        fruit.translation.y -= 300.0 * time.delta_seconds();
        fruit.rotate_z(2.0_f32.to_radians());
    }
}



fn gen_random_number(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..max);
}
