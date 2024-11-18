use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::window::PrimaryWindow;
use crate::events::GameOver;
use crate::resources::Score;

mod player;
mod star;
mod events;
mod resources;
mod enemy;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // Resources
        .init_resource::<Score>()

        // Events
        .add_event::<GameOver>()

        // Plugins
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(star::StarPlugin)

        // Other systems that are not significant enough to be plugins :(
        .add_systems(Update, resources::update_score)
        .add_systems(Update, app_exit)
        .add_systems(Startup, spawn_camera)
        .run();
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            ..default()
        }
    );
}

pub fn app_exit(
    mut event_writer: EventWriter<AppExit>,
    input: Res<ButtonInput<KeyCode>>
) {
    if input.just_pressed(KeyCode::Escape) {
        event_writer.send(AppExit::Success);
    }
}
