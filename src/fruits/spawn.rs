use std::time::Duration;

use bevy::prelude::*;

use crate::random::randint;
use super::sprite::{FruitAssets, get_sprite, get_pineapple};
use super::fruit::{Fruit, FruitType};


const MAX_COMBO_FRUITS: i32 = 4;
const FRUITS_SPAWN_SPAN: (i32, i32) = (-350, 350);


#[derive(Resource)]
pub struct FruitSpawnTimer(pub Timer);


impl FruitSpawnTimer {
    pub fn new() -> Self {
        return Self(Timer::new(Duration::from_secs(1), TimerMode::Repeating))
    }
}


fn random_fruit_type() -> FruitType {
    let n = randint(1, 10);

    if n == 5 {
        FruitType::PINEAPPLE
    } else {
        FruitType::RIPE
    }
}


pub fn spawn_fruits(
    time: Res<Time>,
    mut timer: ResMut<FruitSpawnTimer>,
    fruit_assets: Res<FruitAssets>,
    mut commands: Commands 
){
    timer.0.tick(time.delta());

    if timer.0.finished() {
        let x_axis = randint(FRUITS_SPAWN_SPAN.0, FRUITS_SPAWN_SPAN.1) as f32;

        let fruit_type = random_fruit_type();

        match fruit_type {
            FruitType::RIPE => {
                let combo = randint(1, MAX_COMBO_FRUITS);
                for i in 0..combo {
                    let sprite = get_sprite(&fruit_assets, x_axis, i as f32 * 30. + 330.);
                    commands.spawn((sprite, Fruit::new()));            
                }                
            },
            FruitType::PINEAPPLE => {
                let sprite = get_pineapple(&fruit_assets, x_axis, 350.);
                let mut fruit = Fruit::new();
                fruit.fruit_type = FruitType::PINEAPPLE;
                fruit.rotation_velocity = 0.0;
                commands.spawn((sprite, fruit));
            }
        }
    }
}