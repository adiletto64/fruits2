use std::time::Duration;

use bevy::prelude::*;


pub struct ChefPlugin;


impl Plugin for ChefPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (hit, move_chef, animate_sprite))
            .add_event::<FruitHit>();
    }
}


#[derive(Component)]
struct Player {
    second_slice: bool,
    trigger_slice: bool
}

#[derive(Resource)]
struct PlayerSprites {
    sprites: TextureAtlasSprite,
}


#[derive(Component)]
struct Hit {
    timer: Timer,
}

impl Hit {
    fn start() -> Self {
        return Self {
            timer: Timer::new(Duration::from_millis(50), TimerMode::Repeating),
        }
    }
}


#[derive(Event)]
pub struct FruitHit {
    pub x: f32, 
    pub y: f32
}



fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    let texture_atlas =TextureAtlas::from_grid(
asset_server.load("chef2.png"), 
        Vec2::new(40.0, 40.0),
        8, 
        1, 
        None,
        None
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let sprite = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(0),
        transform: Transform::from_xyz(0.0, -250.0, 0.0).with_scale(Vec3::splat(5.0)),
        ..default()
    };

    commands.spawn((sprite, Player { second_slice: false, trigger_slice: false }));
}



fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut Hit, Entity, &mut Player), With<Player>>,
    mut commands: Commands
) {
    for (mut sprite, mut hit, entity, mut player) in query.iter_mut() {

        if player.trigger_slice {
            if player.second_slice { 
                sprite.index = 4 
            }
            else {
                sprite.index = 0;
            }            
        }

        player.trigger_slice = false;
        
        hit.timer.tick(time.delta());

        if hit.timer.just_finished() {
            if (sprite.index == 4 && !player.second_slice) || sprite.index == 7 { 
                sprite.index = 0;
                commands.entity(entity).remove::<Hit>();
                
            }
            else {
                sprite.index += 1;
            }
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
                sprite.flip_x = false;
                transform.translation.x -= 900. * time.delta_seconds();                
            }
        }
    }
    else if keys.pressed(KeyCode::Right){
        for (mut transform, mut sprite) in &mut query {
            if transform.translation.x < 500. {
                sprite.flip_x = true;
                transform.translation.x += 900. * time.delta_seconds();                
            }
        }
    }
}


fn hit(
    keys: Res<Input<KeyCode>>, 
    mut event: EventWriter<FruitHit>,
    mut query: Query<(&Transform, Entity, &mut Player), With<Player>>,
    mut commands: Commands
) { 
    if keys.just_pressed(KeyCode::Space) {

        for (transform, entity, mut player) in &mut query {
            commands.entity(entity)
                .insert(Hit::start());

            player.second_slice = !player.second_slice;
            player.trigger_slice = true;

            event.send(FruitHit {
                 x: transform.translation.x, 
                 y: transform.translation.y
            });
        }
    }
}
