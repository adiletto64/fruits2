use std::time::Duration;

use bevy::prelude::*;

use super::fruit::{Fruit, FruitType};
use super::sprite::{create_pineapple, create_sprite, FruitTextures};

use crate::utils::random::randint;
use crate::level::LevelUpdate;
use crate::global::AppState;

const MAX_COMBO_FRUITS: i32 = 3;
const FRUITS_SPAWN_BORDERS: (i32, i32) = (-350, 350);

const SPAWN_INTENSITY_UPDATE_PERCENT: u32 = 95;
const INITIAL_SPAWN_TIMER: Duration = Duration::from_millis(800);


pub struct FruitSpawnPlugin;


impl Plugin for FruitSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update, (spawn_fruits, increase_spawn_intensity).run_if(in_state(AppState::InGame))
        );
    }
}



#[derive(Resource)]
pub struct FruitSpawnTimer(pub Timer);

impl FruitSpawnTimer {
    pub fn new() -> Self {
        return Self(Timer::new(INITIAL_SPAWN_TIMER, TimerMode::Repeating));
    }
}


enum FoodType {
    FRUIT,
    PINEAPPLE
}


fn random_fruit_type() -> FoodType {
    let n = randint(1, 20);
    if n == 5 { FoodType::PINEAPPLE } else { FoodType::FRUIT } 
}


pub fn spawn_fruits(
    mut commands: Commands,
    mut spawn_timer: ResMut<FruitSpawnTimer>,
    time: Res<Time>,
    fruit_assets: Res<FruitTextures>,
) {
    spawn_timer.0.tick(time.delta());

    if spawn_timer.0.finished() {
        let x = randint(FRUITS_SPAWN_BORDERS.0, FRUITS_SPAWN_BORDERS.1) as f32;

        let fruit_type = random_fruit_type();

        match fruit_type {
            FoodType::PINEAPPLE => {
                let sprite = create_pineapple(&fruit_assets, x, 350.);
                
                let mut fruit = Fruit::new();
                fruit.fruit_type = FruitType::PINEAPPLE;
                fruit.rotation_speed = 0.0;
                
                commands.spawn((sprite, fruit));
            }

            FoodType::FRUIT => {
                let combo = randint(1, MAX_COMBO_FRUITS+1);
                for i in 0..combo {
                    
                    let (sprite, fruit_type) = create_sprite(
                        &fruit_assets, 
                        x, 
                        i as f32 * 30. + 330.,
                        5. + (MAX_COMBO_FRUITS - i) as f32
                    );

                    let mut fruit = Fruit::new();
                    fruit.fruit_type = fruit_type.clone();
                    commands.spawn((sprite, fruit));
                }
            }
        }
    }
}


pub fn increase_spawn_intensity(
    events: EventReader<LevelUpdate>,
    mut spawn_timer: ResMut<FruitSpawnTimer>
) {
    if !events.is_empty() {
        let duration = spawn_timer.0.duration();

        if duration.as_millis() > 450 {
            spawn_timer.0.set_duration(duration / 100 * SPAWN_INTENSITY_UPDATE_PERCENT);  
        }
    }
}

