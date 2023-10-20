use bevy::prelude::*;

use crate::global::AppState;
use crate::utils::ui::text;


pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter::<AppState>(AppState::Paused), setup)
            .add_systems(OnExit::<AppState>(AppState::Paused), exit)
            .add_systems(Update, continue_game.run_if(in_state(AppState::Paused)))
        ;
    }
}


#[derive(Component)]
struct PauseItem;


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // add background image
    commands.spawn((SpriteBundle {
        texture: asset_server.load("images/menu.png"),
        transform: Transform::from_xyz(0., 0., 10.).with_scale(Vec3::splat(7.5)),
        ..default()
    }, PauseItem));

    let title = text(&asset_server, "Paused", 0., 100., 70.);
    commands.spawn((title, PauseItem));

    let enter_game_text = text(&asset_server, "Press --Enter-- to continue!", 0., 10., 40.);
    commands.spawn((enter_game_text, PauseItem));
}


fn exit(mut commands: Commands, query: Query<Entity, With<PauseItem>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}


fn continue_game(keys: Res<Input<KeyCode>>, mut app_state: ResMut<NextState<AppState>>) {
    if keys.just_pressed(KeyCode::Return) {
        app_state.set(AppState::InGame);
    }
}
