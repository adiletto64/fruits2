use bevy::prelude::*;

use crate::components::Clock;
use crate::global::AppState;

use super::fruit::FruitType;
use super::sprite::{create_splash, SplashColor};



pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (spawn_splash, animate_splash).run_if(in_state(AppState::InGame)))
            .add_event::<SplashEvent>()
        ;
    }
}



const SPLASH_ANIMATION_SPEED: u64 = 80;

#[derive(Component)]
pub struct Splash;

#[derive(Event)]
pub struct SplashEvent {
    pub x: f32, 
    pub y: f32, 
    pub fruit_type: FruitType
}


pub fn spawn_splash(
    mut commands: Commands,
    mut event_reader: EventReader<SplashEvent>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>, 
){
    for event in event_reader.iter() {
        let splash_color = match event.fruit_type {
            FruitType::APPLE | FruitType::BANANA | FruitType::PINEAPPLE => SplashColor::Yellow,
            FruitType::ORANGE => SplashColor::Orange,
            FruitType::STRAWBERRY | FruitType::WATERMELON => SplashColor::Red,
            FruitType::POME => SplashColor::Orange
        };

        let sprite = create_splash(
            &asset_server, 
            &mut texture_atlases, 
            event.x, 
            event.y,
            splash_color
        );
        
        commands.spawn((Splash, Clock::millis(SPLASH_ANIMATION_SPEED), sprite));        
    }
}


pub fn animate_splash(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut Transform, &mut Clock, Entity), With<Splash>>,
    mut commands: Commands
) {
    for (mut sprite, mut transform, mut clock, entity) in query.iter_mut() {
        clock.tick(time.delta());
        if clock.just_finished() && sprite.index < 5 { 
            sprite.index += 1;
        }        

        transform.translation.y -= 300. * time.delta_seconds();
        
        let alpha = sprite.color.a();
        sprite.color.set_a(alpha - 0.01);

        if transform.translation.y < -480. {
            commands.entity(entity).despawn();
        }
    }
}
