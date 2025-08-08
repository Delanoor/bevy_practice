use bevy::prelude::*;

use crate::bullet::Bullet;
use crate::constants::{BULLET_SIZE, ENEMY_SIZE};
use crate::enemy::Enemy;
use crate::score::Score;
use crate::state::GameState;

pub fn collide(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> bool {
    // AABB collision

    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;
    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;

    a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y
}

pub fn bullet_enemy_collision(
    mut commands: Commands,
    mut score: ResMut<Score>,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    let bullet_size = BULLET_SIZE;
    let enemy_size = ENEMY_SIZE;

    for (bullet_entity, bullet_tf) in &bullet_query {
        for (enemy_entity, enemy_tf) in &enemy_query {
            if collide(
                bullet_tf.translation,
                bullet_size,
                enemy_tf.translation,
                enemy_size,
            ) {
                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();
                score.0 += 1;
                break; // One bullet hits one enemy only
            }
        }
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            bullet_enemy_collision.run_if(in_state(GameState::Playing)),
        );
    }
}
