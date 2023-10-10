use bevy::prelude::*;
use self::sprite::AnimationSlice;
mod sprite;


pub struct ChefPlugin;

impl Plugin for ChefPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (hit, move_chef, animate))
            .add_event::<FruitHit>();
    }
}


#[derive(Component)]
struct Player;


#[derive(Event)]
pub struct FruitHit {
    pub translation: Vec3
}


const SPEED: f32 = 900.;



fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    let sprite = sprite::get_sprite(asset_server, &mut texture_atlases);
    let animation = sprite::AnimationSlice::new();
    commands.spawn((sprite, animation, Player));
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


fn move_chef(
    keys: Res<Input<KeyCode>>, 
    time: Res<Time>, 
    mut query: Query<(&mut Transform, &mut TextureAtlasSprite), With<Player>>
) {

    if keys.pressed(KeyCode::Left) {
        for (mut transform, mut sprite) in &mut query {
            if transform.translation.x > -500. {
                sprite.flip_x = true;
                transform.translation.x -= SPEED * time.delta_seconds();                
            }
        }
    }
    else if keys.pressed(KeyCode::Right){
        for (mut transform, mut sprite) in &mut query {
            if transform.translation.x < 500. {
                sprite.flip_x = false;
                transform.translation.x += SPEED * time.delta_seconds();                
            }
        }
    }
}


fn hit(
    keys: Res<Input<KeyCode>>, 
    mut event: EventWriter<FruitHit>,
    mut query: Query<(&Transform, &mut AnimationSlice), With<Player>>,
) { 
    if keys.just_pressed(KeyCode::Space) {

        for (transform, mut animation) in &mut query {
            event.send(FruitHit {
                translation: transform.translation
            });

            animation.trigger_slice();
        }
    }
}
