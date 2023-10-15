use bevy::prelude::*;

use crate::session::Session;


pub struct InfoPlugin;


impl Plugin for InfoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, update);
    }
}


#[derive(Component)]
struct TextInfo;


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


fn update(mut query: Query<&mut Text, With<TextInfo>>, info: Res<Session>) {
    for mut text in &mut query {
        text.sections[0].value = info.text();
    }
}
