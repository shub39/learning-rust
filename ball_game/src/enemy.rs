use bevy::audio::{AudioLoader, AudioPlugin, PlaybackMode};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use crate::player::Player;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_enemies)
            .add_systems(Update, update_enemies)
            .add_systems(Update, contain_enemies)
        ;
    }
}

#[derive(Component)]
pub struct Enemy {
    direction: Vec2
}

pub const NUMBER_OF_ENEMIES: usize = 20;
pub const ENEMY_SPEED: f32 = 500.0;
pub const ENEMY_SIZE: f32 = 64.0;

fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    for _ in 1..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                },
                Enemy {
                    direction: Vec2::new(random(), random()),
                }
            )
        );
    }
}

fn update_enemies(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

fn contain_enemies(
    mut enemy_query: Query<(&mut Enemy, &mut Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // audio: Res<>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();

    let half_enemy = ENEMY_SIZE / 2.;
    let x_min = 0. + half_enemy;
    let x_max = window.width() - half_enemy;
    let y_min = 0. + half_enemy;
    let y_max = window.height() - half_enemy;

    for (mut enemy, mut transform) in enemy_query.iter_mut() {
        let translation = transform.translation;
        let mut direction_changes = false;

        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changes = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changes = true;
        }

        if direction_changes {
            let sound_1: Handle<AudioSource> = asset_server.load("audio/pluck_001.ogg");
            let sound_2: Handle<AudioSource> = asset_server.load("audio/pluck_002.ogg");

            let audio = if random::<bool>() {
                sound_1
            } else {
                sound_2
            };


        }

        if translation.x > x_max {
            transform.translation.x = x_max;
        }
        if translation.x < x_min {
            transform.translation.x = x_min;
        }
        if translation.y > y_max {
            transform.translation.y = y_max;
        }
        if translation.y < y_min {
            transform.translation.y = y_min;
        }
    }
}