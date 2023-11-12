use bevy::prelude::{Res, AssetServer, TextStyle, Text2dBundle, Text, TextAlignment, Transform, Color, default};


struct Txt<'a> {
    texts: &'a str,
    asset_server: &'a Res<'a, AssetServer>,
    x: f32,
    y: f32,
    font_size: f32,
}

impl Txt<'_> {
        

    fn get_bundle(&self, text: &str, x: f32, y: f32) -> Text2dBundle {
        let text_style = TextStyle {
            font: self.asset_server.load("fonts/mn-regular.otf"),
            color: Color::WHITE,
            font_size: self.font_size,
        };

        Text2dBundle {
            text: Text::from_section(text, text_style)
                .with_alignment(TextAlignment::Right),
            transform: Transform::from_xyz(x, y, 11.),    
            ..default()
        }
    }
}

pub fn text(asset_server: &Res<AssetServer>, text: &str, x: f32, y: f32, font_size: f32) -> Text2dBundle {
    let text_style = TextStyle {
        font: asset_server.load("fonts/mn-regular.otf"),
        color: Color::WHITE,
        font_size,
    };

    Text2dBundle {
        text: Text::from_section(text, text_style)
            .with_alignment(TextAlignment::Right),
        transform: Transform::from_xyz(x, y, 11.),    
        ..default()
    }
}
