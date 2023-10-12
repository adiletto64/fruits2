use bevy::prelude::*;
mod chef;
mod fruits;
mod level;


fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Fruits game!".into(),
                        resolution: (1140., 660.).into(),
                        ..default()
                    }),
                    ..default()
                })
        )
        .add_plugins(level::LevelPlugin)
        .add_plugins((chef::ChefPlugin, fruits::fruit::FruitPlugin))
        .add_systems(Startup, setup)
        .run();
}


fn setup(mut commands: Commands, assert_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: assert_server.load("bg.png"),
        transform: Transform::from_xyz(0., 0., -1.).with_scale(Vec3::splat(15.)),
        ..default()
    });
}
