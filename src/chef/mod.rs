use bevy::prelude::*;

mod sprite;

use self::sprite::AnimationSlice;
use crate::level::LevelUpdate;
use crate::sound::{SoundEvent, SoundType};

const SPEED: f32 = 900.;
const SPEED_UPDATE: f32 = 10.;


pub struct ChefPlugin;
impl Plugin for ChefPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (
                hit, 
                walk, 
                collect_rotten_fruits, 
                animate, 
                update_level
            ))
            .add_event::<ChefHitEvent>();
    }
}

#[derive(Component)]
struct Player {
    speed: f32
}

#[derive(Event)]
pub struct ChefHitEvent {
    pub translation: Vec3
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    let sprite = sprite::get_sprite(&asset_server, &mut texture_atlases);
    let animation = sprite::AnimationSlice::new();

    commands.spawn(
        (
            sprite, 
            animation, 
            Player { speed: SPEED }, 
        )
    );
}


fn animate(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut AnimationSlice), With<Player>>,
) {
    for (mut sprite, mut animation) in query.iter_mut() {
        animation.timer.tick(time.delta());

        if animation.timer.finished() {
            animation.update();
            sprite.index = animation.index;
        }

    }
}


fn walk(
    keys: Res<Input<KeyCode>>, 
    time: Res<Time>, 
    mut query: Query<(&mut Transform, &mut TextureAtlasSprite, &Player)>
) {
    for (
        mut transform, 
        mut sprite, 
        player
    ) in &mut query {
        if keys.pressed(KeyCode::Left) {
            if transform.translation.x > -500. {
                sprite.flip_x = true;
                transform.translation.x -= player.speed * time.delta_seconds();                
            }
        }
        else if keys.pressed(KeyCode::Right){
            if transform.translation.x < 500. {
                sprite.flip_x = false;
                transform.translation.x += player.speed * time.delta_seconds();                
            }
        }
    }
}


fn hit(
    keys: Res<Input<KeyCode>>, 
    mut event: EventWriter<ChefHitEvent>,
    mut sound_event: EventWriter<SoundEvent>,
    mut query: Query<(&Transform, &mut AnimationSlice), With<Player>>,
) { 
    if keys.just_pressed(KeyCode::Space) {
        for (transform, mut animation) in &mut query {
            event.send(ChefHitEvent {
                translation: transform.translation
            });
            animation.trigger_slice();
            sound_event.send(SoundEvent { 
                sound_type: SoundType::SLICE 
            });
        }
    }
}


fn collect_rotten_fruits(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut AnimationSlice, With<Player>>
) {

    if keys.pressed(KeyCode::S) || keys.just_pressed(KeyCode::S) {
        for mut animation in &mut query {
            animation.pullout_trash_bag();
        }
    }
    else if keys.just_released(KeyCode::S) {
        for mut animation in &mut query {
            animation.normal();
        }
    }
}


fn update_level(events: EventReader<LevelUpdate>, mut query: Query<&mut Player>) {
    if events.len() > 0 {
        for mut player in &mut query {
            player.speed += SPEED_UPDATE;
        }        
    }
}
