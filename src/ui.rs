use bevy::prelude::{Res, AssetServer, TextStyle, Text2dBundle, Text, TextAlignment, Transform, Color, default};


pub fn text(asset_server: &Res<AssetServer>, text: &str, x: f32, y: f32, font_size: f32) -> Text2dBundle {
    let text_style = TextStyle {
        font: asset_server.load("fonts/mn-regular.otf"),
        font_size: font_size,
        color: Color::WHITE
    };

    Text2dBundle {
        text: Text::from_section(text, text_style)
            .with_alignment(TextAlignment::Right),
        transform: Transform::from_xyz(x, y, 11.),    
        ..default()
    }
}
