use bevy::prelude::*;



pub struct TextPlugin;


impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (spawn_text, animate_text))
            .add_event::<TextEvent>()
        ;
    }
}



#[derive(Event)]
pub struct TextEvent {
    pub text: String,
    pub y: f32,
    pub x: f32
}

#[derive(Component)]
struct Message(Timer);


fn spawn_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<TextEvent>
) {
    for event in events.iter() {
        let text = event.text.clone();

        let text_style = TextStyle {
            font: asset_server.load("fonts/mn-regular.otf"),
            font_size: 32.,
            color: Color::WHITE
        };

        let handle = Text2dBundle {
            text: Text::from_section(text, text_style)
                .with_alignment(TextAlignment::Right),
            transform: Transform::from_xyz(event.x, event.y + 50., 7.),
            ..default()
        };
    
        commands.spawn((
            handle,
            Message(Timer::from_seconds(1.5, TimerMode::Once))
        ));        
    }
}


fn animate_text(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Message, Entity)>
) {
    for (mut transform, mut message, entity) in &mut query {
        transform.translation.y += 20. * time.delta_seconds();

        message.0.tick(time.delta());

        if message.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}
