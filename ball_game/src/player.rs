use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

pub const PLAYER_SPEED: f32 = 500.0;

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                ..default()
            },
            Player {}
        )
    );
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {
    let mut transform = player_query.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction += Vec3::new(-1., 0., 0.);
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += Vec3::new(1., 0., 0.);
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction += Vec3::new(0., 1., 0.);
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction += Vec3::new(0., -1., 0.);
    }

    if direction.length() > 0. {
        direction = direction.normalize();
    }

    transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
}