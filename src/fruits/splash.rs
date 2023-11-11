use std::time::Duration;

use bevy::prelude::*;

use super::fruit::FruitType;
use super::sprite::{create_splash, SplashColor};



pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (spawn_splash, animate_splash))
            .add_event::<SplashEvent>()
        ;
    }
}



const SPLASH_ANIMATION_SPEED: u64 = 80;

#[derive(Component)]
pub struct Splash {timer: Timer}

#[derive(Event)]
pub struct SplashEvent {
    pub x: f32, 
    pub y: f32, 
    pub fruit_type: FruitType
}

impl Splash {
    fn new() -> Self {
        Self { timer: Timer::new(Duration::from_millis(SPLASH_ANIMATION_SPEED), TimerMode::Repeating) }
    }
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
            FruitType::STRAWBERRY | FruitType::WATERMELON => SplashColor::Red
        };

        let sprite = create_splash(
            &asset_server, 
            &mut texture_atlases, 
            event.x, 
            event.y,
            splash_color
        );
        
        commands.spawn((Splash::new(), sprite));        
    }

}


pub fn animate_splash(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut Transform, &mut Splash, Entity)>,
    mut commands: Commands
) {
    for (mut sprite, mut transform, mut splash, entity) in query.iter_mut() {
        splash.timer.tick(time.delta());
        if splash.timer.just_finished() {
            if sprite.index < 5 { 
                sprite.index += 1;
            }
        }        

        transform.translation.y -= 300. * time.delta_seconds();
        
        let alpha = sprite.color.a();
        sprite.color.set_a(alpha - 0.01);

        if transform.translation.y < -480. {
            commands.entity(entity).despawn();
        }
    }
}
