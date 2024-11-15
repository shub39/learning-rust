use crate::resolution;
use bevy::prelude::*;

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_aliens)
            .add_systems(Update, (update_aliens, manage_alien_logic));
    }
}

#[derive(Component)]
pub struct Alien;

#[derive(Resource)]
pub struct AlienManager {
    pub direction: f32,
    pub shift_aliens_down: bool,
    pub dist_from_boundary: f32,
}

const WIDTH: i32 = 10;
const HEIGHT: i32 = 5;
const SPACING: f32 = 24.;
const SPEED: f32 = 100.;
const ALIEN_SHIFT_AMOUNT: f32 = 32.;

fn setup_aliens(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resolution: Res<resolution::Resolution>,
) {
    commands.insert_resource(AlienManager {
        shift_aliens_down: false,
        direction: 1.,
        dist_from_boundary: 0.,
    });

    let alien_texture = asset_server.load("alien.png");

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let position = Vec3::new(x as f32 * SPACING, y as f32 * SPACING, 0.)
                - (Vec3::X * WIDTH as f32 * SPACING * 0.5)
                - (Vec3::Y * HEIGHT as f32 * SPACING * 1.0)
                + (Vec3::Y * resolution.screen_dimensions.y * 0.5);

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(position)
                        .with_scale(Vec3::splat(resolution.pixel_ratio)),
                    texture: alien_texture.clone(),
                    ..default()
                },

                Alien {},
            ));
        }
    }
}

fn update_aliens(
    mut alien_query: Query<(&Alien, &mut Transform)>,
    mut alien_manager: ResMut<AlienManager>,
    resolution: Res<resolution::Resolution>,
    time: Res<Time>,
) {
    for (_alien, mut transform) in alien_query.iter_mut() {
        transform.translation.x += time.delta_seconds() * alien_manager.direction * SPEED;
        if transform.translation.x.abs() > resolution.screen_dimensions.x * 0.5 {
            alien_manager.shift_aliens_down = true;
            alien_manager.dist_from_boundary =
                resolution.screen_dimensions.x * alien_manager.direction * 0.5
                    - transform.translation.x;
        }
    }
}

fn manage_alien_logic(
    mut alien_query: Query<(&mut Alien, &mut Transform)>,
    mut alien_manager: ResMut<AlienManager>,
) {
    if alien_manager.shift_aliens_down {
        alien_manager.shift_aliens_down = false;
        alien_manager.direction *= -1.;

        for (_alien, mut transform) in alien_query.iter_mut() {
            transform.translation.x += alien_manager.dist_from_boundary;
            transform.translation.y -= ALIEN_SHIFT_AMOUNT;
        }
    }
}
