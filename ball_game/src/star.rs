use crate::player::Player;
use crate::resources::Score;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<StarSpawnTimer>()
            .add_systems(Startup, spawn_stars)
            .add_systems(Update, player_hit_star)
            .add_systems(Update, respawn_stars)
            .add_systems(Update, tick_spawn_timer)
        ;
    }
}

pub const NUMBER_OF_STARS: u32 = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_SPAWN_TIME: f32 = 5.0;

#[derive(Component)]
pub struct Star {}

#[derive(Resource)]
pub struct StarSpawnTimer {
    timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating)
        }
    }
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let x = random::<f32>() * window.width();
        let y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/star.png"),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            Star {}
        ));
    }
}

// tick tock
pub fn tick_spawn_timer(
    mut timer: ResMut<StarSpawnTimer>,
    time: Res<Time>,
) {
    timer.timer.tick(time.delta());
}

// Respawns stars if the number of present stars less than 
// NUMBER_OF_STARS and timer is finished
pub fn respawn_stars(
    mut commands: Commands,
    star_query: Query<Entity, With<Star>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    timer: Res<StarSpawnTimer>,
) {
    if star_query.iter().count() < NUMBER_OF_STARS as usize {
        if timer.timer.finished() {
            let window = window_query.get_single().unwrap();

            let x = random::<f32>() * window.width();
            let y = random::<f32>() * window.height();

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("sprites/star.png"),
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                Star {}
            ));
        }
    }
}

// Despawn stars and update player score
fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            if time.elapsed_seconds() > 3f32 {
                let distance = player_transform.translation.distance(star_transform.translation);

                if distance < STAR_SIZE {
                    let audio: Handle<AudioSource> = asset_server.load("audio/laserLarge_000.ogg");

                    commands.spawn(AudioBundle {
                        source: audio,
                        ..default()
                    });

                    commands.entity(star_entity).despawn();

                    score.value += 1;
                }
            }
        }
    }
}