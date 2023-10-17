use bevy::prelude::*;

use crate::state::AppState;
use crate::ui::text;

use crate::session::Session;


pub struct FinishPlugin;

impl Plugin for FinishPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter::<AppState>(AppState::Finish), setup)
            .add_systems(OnExit::<AppState>(AppState::Finish), exit)
            .add_systems(Update, restart.run_if(in_state(AppState::Finish)))
        ;
    }
}


#[derive(Component)]
struct FinishItem;


fn setup(mut commands: Commands, asset_server: Res<AssetServer>, session: Res<Session>) {
    // add background image
    commands.spawn((SpriteBundle {
        texture: asset_server.load("images/menu.png"),
        transform: Transform::from_xyz(0., 0., 10.).with_scale(Vec3::splat(7.5)),
        ..default()
    }, FinishItem));

    let title = text(&asset_server, "End Game!", 0., 100., 70.);
    commands.spawn((title, FinishItem));

    let score = text(&asset_server, format!("Your score {}", session.score).as_str(), 0., 50., 40.);
    commands.spawn((score, FinishItem));

    let enter_game_text = text(&asset_server, "Press --Enter-- to restart!", 0., 10., 40.);
    commands.spawn((enter_game_text, FinishItem));
}


fn exit(mut commands: Commands, query: Query<Entity, With<FinishItem>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}


fn restart(keys: Res<Input<KeyCode>>, mut app_state: ResMut<NextState<AppState>>, mut session: ResMut<Session>) {
    if keys.just_pressed(KeyCode::Return) {
        *session = Session::default();
        app_state.set(AppState::InGame);
    }
}
