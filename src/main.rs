use bevy::prelude::*;
mod chef;
mod fruit;


fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Fruits game!".into(),
                        resolution: (1200., 600.).into(),
                        ..default()
                    }),
                    ..default()
                })
        )
        .add_plugins((chef::ChefPlugin, fruit::FruitPlugin))
        .add_systems(Startup, setup)
        .run();
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
