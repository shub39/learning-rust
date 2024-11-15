use bevy::prelude::*;
use crate::alien::{Alien, Dead};
use crate::resolution;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_projectiles, update_alien_interactions));
    }
}

#[derive(Component)]
pub struct Projectile {
    pub speed: f32
}

fn update_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Projectile, &mut Transform)>,
    time: Res<Time>,
    resolution: Res<resolution::Resolution>
) {
    for (entity, projectile, mut transform) in projectile_query.iter_mut() {
        transform.translation.y += projectile.speed * time.delta_seconds();

        if transform.translation.y > resolution.screen_dimensions.y * 0.5 {
            commands.entity(entity).despawn();
        }
    }
}

const BULLET_RADIUS: f32 = 24.;

fn update_alien_interactions(
    mut alien_query: Query<(&mut Alien, &Transform), Without<Dead>>,
    mut projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    mut commands: Commands,
) {
    for (mut alien, alien_transform) in alien_query.iter_mut() {
        for (projectile, projectile_transform) in projectile_query.iter_mut() {
            let projectile_pos = Vec2::new(
                projectile_transform.translation.x,
                projectile_transform.translation.y
            );

            let alien_pos = Vec2::new(
                alien_transform.translation.x,
                alien_transform.translation.y
            );

            if Vec2::distance(alien_pos, projectile_pos) < BULLET_RADIUS {
                alien.dead = true;
                commands.entity(projectile).despawn();
            }
        }
    }
}