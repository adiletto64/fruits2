use bevy::prelude::*;

use crate::states::session::Session;
use crate::global::AppState;


pub struct InfoPlugin;


impl Plugin for InfoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(OnEnter::<AppState>(AppState::InGame), respawn_hearts)
            .add_systems(Update, (update, update_live));
    }
}


#[derive(Component)]
struct TextInfo;

#[derive(Component)]
struct Live;


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let session = Session::default();

    commands.insert_resource(session.clone());
    let text = session.text();

    let text_style = TextStyle {
        font: asset_server.load("fonts/mn-regular.otf"),
        font_size: 32.,
        color: Color::WHITE
    };

    commands.spawn((Text2dBundle {
        text: Text::from_section(text, text_style)
            .with_alignment(TextAlignment::Right),
        transform: Transform::from_xyz(400., 250., 10.),
        ..default()
    }, TextInfo));
}


fn respawn_hearts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let session = Session::default();

    for i in 0..session.lives_left {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("images/heart.png"),
                transform: Transform::from_xyz(530. - 40. * i as f32, 180., 10.).with_scale(Vec3::splat(3.)),
                ..default()
            },
            TextInfo,
            Live
        ));
    }
}


fn update_live(
    mut commands: Commands, 
    query: Query<(Entity, &Live)>, 
    session: Res<Session>,
    asset_server: Res<AssetServer>
) {
    if session.is_changed() {
        // clear all hearts
        for (entity, _) in &query {
            commands.entity(entity).despawn();
        }

        for i in 0..(5 - session.lives_left) {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("images/heart-empty.png"),
                    transform: Transform::from_xyz(370. + 40. * i as f32, 180., 9.).with_scale(Vec3::splat(3.)),
                    ..default()
                },
                TextInfo,
                Live
            ));
        }

        // respawn all hearts for given lives number
        for i in 0..session.lives_left {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("images/heart.png"),
                    transform: Transform::from_xyz(530. - 40. * i as f32, 180., 9.).with_scale(Vec3::splat(3.)),
                    ..default()
                },
                TextInfo,
                Live
            ));
        }

    }
}


fn update(mut query: Query<&mut Text, With<TextInfo>>, info: Res<Session>) {
    for mut text in &mut query {
        text.sections[0].value = info.text();
    }
}
