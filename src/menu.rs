use bevy::prelude::*;

use crate::state::AppState;
use crate::ui::text;


pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter::<AppState>(AppState::MainMenu), setup)
            .add_systems(OnExit::<AppState>(AppState::MainMenu), exit)
            .add_systems(Update, enter_game.run_if(in_state(AppState::MainMenu)))
        ;
    }
}


#[derive(Component)]
struct MenuItem;


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // add background image
    commands.spawn((SpriteBundle {
        texture: asset_server.load("menu.png"),
        transform: Transform::from_xyz(0., 0., 10.).with_scale(Vec3::splat(7.5)),
        ..default()
    }, MenuItem));

    let title = text(&asset_server, "Fruits 2", 0., 180., 90.);
    commands.spawn((title, MenuItem));

    let enter_game_text = text(&asset_server, "Press --Enter-- to start!", 0., 10., 40.);
    commands.spawn((enter_game_text, MenuItem));
}


fn exit(mut commands: Commands, query: Query<Entity, With<MenuItem>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}


fn enter_game(keys: Res<Input<KeyCode>>, mut app_state: ResMut<NextState<AppState>>) {
    if keys.just_pressed(KeyCode::Return) {
        app_state.set(AppState::InGame);
    }
}
