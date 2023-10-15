use bevy::prelude::*;


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


#[derive(Resource, Clone)]
pub struct Info {
    pub level: u32,
    pub lives_left: u32,
    pub score: u32
}

impl Info {
    fn default() -> Self {
        Self { level: 1, lives_left: 3, score: 0 }
    }
    fn text(&self) -> String {
        format!("Current level: {}\nLive left: {}\nscore: {}", self.level, self.lives_left, self.score)
    }
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let info = Info::default();

    commands.insert_resource(info.clone());
    let text = info.text();

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


fn update(mut query: Query<&mut Text, With<TextInfo>>, info: Res<Info>) {
    for mut text in &mut query {
        text.sections[0].value = info.text();
    }
}
