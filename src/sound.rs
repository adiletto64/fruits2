use bevy::prelude::*;


pub struct SoundPlugin;


impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_systems(Update, (spawn_sound, despawn_sound))
            .add_event::<SoundEvent>()
        ;
    }
}


pub enum SoundType {
    SLICE,
    SPLAT
}

impl SoundType {
    fn file(&self) -> &str {
        match self {
            Self::SLICE => "audio/slash.ogg",
            Self::SPLAT => "audio/splat1.ogg"
        }
    }
}


#[derive(Event)]
pub struct SoundEvent {
    pub sound_type: SoundType,
}


#[derive(Component)]
struct Sound {
    timer: Timer
}


fn spawn_sound(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut events: EventReader<SoundEvent>
) {
    for event in events.iter() {
        let handle = asset_server.load(event.sound_type.file());
        let audio_bundle = AudioBundle {
            source: handle,
            settings: PlaybackSettings {
                speed: 1.6,
                ..default()
            },
            ..default()
        };
        let sound = Sound { timer: Timer::from_seconds(2.0, TimerMode::Once) };

        commands.spawn((audio_bundle, sound));
    }
}


fn despawn_sound(
    mut commands: Commands, 
    time: Res<Time>, 
    mut query: Query<(&mut Sound, Entity)>
) {
    for (mut sound, entity) in &mut query {
        sound.timer.tick(time.delta());
        if sound.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
