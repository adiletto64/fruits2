use bevy::prelude::*;

use crate::global::AppState;
use crate::utils::ui::text;
use crate::utils::record;

use crate::sound::{SoundEvent, SoundType};
use super::session::Session;


pub struct FinishPlugin;

impl Plugin for FinishPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter::<AppState>(AppState::Finish), setup)
            .add_systems(OnExit::<AppState>(AppState::Finish), exit)
            .add_systems(Update, (restart, animate_confetti).run_if(in_state(AppState::Finish)))
        ;
    }
}


#[derive(Component)]
struct FinishItem;

#[derive(Component)]
struct Confetti {
    timer: Timer
}


fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>, 
    session: Res<Session>,
    mut sound: EventWriter<SoundEvent>
) {
    if record::is_record(session.score) {
        // new record text
        {
            let text_style = TextStyle {
                font: asset_server.load("fonts/mn-regular.otf"),
                color: Color::WHITE,
                font_size: 40.,
            };
            let new_record = Text2dBundle {
                text: Text::from_section(format!("New record: {}!", session.score), text_style)
                    .with_alignment(TextAlignment::Right),
                transform: Transform::from_xyz(-290., 160., 11.).with_rotation(Quat::from_rotation_z(45.0_f32.to_radians())),    
                ..default()
            };
            commands.spawn((new_record, FinishItem));             
        }


        // record menu image
        commands.spawn((SpriteBundle {
            texture: asset_server.load("images/menu-record.png"),
            transform: Transform::from_xyz(0., 0., 10.).with_scale(Vec3::splat(7.5)),
            ..default()
        }, FinishItem));    

        sound.send(SoundEvent::sound(SoundType::RECORD));
    } else {
        // add background image
        commands.spawn((SpriteBundle {
            texture: asset_server.load("images/menu.png"),
            transform: Transform::from_xyz(0., 0., 10.).with_scale(Vec3::splat(7.5)),
            ..default()
        }, FinishItem));

    }
    sound.send(SoundEvent::sound(SoundType::GAME_OVER));

    // text
    {
        let title = text(&asset_server, "End Game!", 0., 100., 70.);
        commands.spawn((title, FinishItem));

        let score = text(&asset_server, format!("Your score {}", session.score).as_str(), 0., 20., 40.);
        commands.spawn((score, FinishItem));
        
        let enter_game_text = text(&asset_server, "Press --Enter-- to restart!", 0., -20., 40.);
        commands.spawn((enter_game_text, FinishItem));        
    }

    // create confetti
    if record::is_record(session.score) {
        let transform = Transform::from_xyz(0., 800., 11.).with_scale(Vec3::splat(7.5));

        let texture = TextureAtlas::from_grid(
            asset_server.load("images/confetti.png"),
            Vec2::new(152., 200.),
            2,
            1,
            None,
            None
        );

        let texture_atlas = texture_atlases.add(texture);

        let confetti = SpriteSheetBundle { 
            texture_atlas, 
            transform, 
            ..default()
        };

        commands.spawn((confetti, Confetti{timer: Timer::from_seconds(0.3, TimerMode::Repeating)}, FinishItem));
        record::write_record(session.score);
    }
}


fn exit(mut commands: Commands, query: Query<Entity, With<FinishItem>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}


fn restart(keys: Res<Input<KeyCode>>, mut app_state: ResMut<NextState<AppState>>, mut session: ResMut<Session>) {
    if keys.just_pressed(KeyCode::Return) {
        *session = Session::default();
        app_state.set(AppState::InGame);
    }
}

fn animate_confetti(
    time: Res<Time>,
    mut query: Query<(&mut Confetti, &mut TextureAtlasSprite, &mut Transform, Entity)>,
    mut commands: Commands
) {
    for (mut confetti, mut sprite, mut transform, entity) in &mut query {
        confetti.timer.tick(time.delta());
        
        if confetti.timer.finished() {
            sprite.index = if sprite.index == 0 { 1 } else { 0 };
        }

        transform.translation.y -= 300. * time.delta_seconds(); 

        if transform.translation.y < -1700. {
            commands.entity(entity).despawn();
        }
    }
}
