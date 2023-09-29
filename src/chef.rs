use std::time::Duration;

use bevy::prelude::*;


pub struct ChefPlugin;


impl Plugin for ChefPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (hit, move_chef, animate_sprite));
    }
}


#[derive(Component)]
struct Player;


#[derive(Resource)]
struct Hit {
    finished: bool,
    timer: Timer,
}

impl Hit {
    fn start() -> Self {
        return Self {
            finished: false,
            timer: Timer::new(Duration::from_millis(30), TimerMode::Repeating),
        }
    }
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    let texture_atlas =TextureAtlas::from_grid(
        asset_server.load("chef-frames.png"), 
        Vec2::new(32.0, 40.0),
        4, 
        1, 
        None,
        None
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let sprite = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(0),
        transform: Transform::from_xyz(0.0, -250.0, 2.0).with_scale(Vec3::splat(5.0)),
        ..default()
    };

    commands.spawn((sprite, Player));
    let mut hit = Hit::start();
    hit.finished = true;
    commands.insert_resource(hit);
}



fn animate_sprite(
    time: Res<Time>,
    mut query: Query<&mut TextureAtlasSprite>,
    mut hit: ResMut<Hit>
) {
    if hit.finished {
        return;
    }

    hit.timer.tick(time.delta());
    if hit.timer.just_finished() {
        for mut sprite in &mut query {
            if sprite.index == 3 { 
                sprite.index = 0;
                hit.finished = true;
                return;
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
            sprite.flip_x = false;
            transform.translation.x -= 900. * time.delta_seconds();
        }
    }
    else if keys.pressed(KeyCode::Right){
        for (mut transform, mut sprite) in &mut query {
            sprite.flip_x = true;
            transform.translation.x += 900. * time.delta_seconds();
        }
    }
}


fn hit(keys: Res<Input<KeyCode>>, mut hit: ResMut<Hit>) {
    if keys.just_pressed(KeyCode::Space) {
        *hit = Hit::start();
    }
}
