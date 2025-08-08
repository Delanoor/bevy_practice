use crate::{
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    player::Player,
    state::GameState,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct BulletVelocity(pub Vec2);

#[derive(Component)]
pub struct BulletLifeTime(pub Timer);

// pub fn move_bullet(mut query: Query<&mut Transform, With<Bullet>>, time: Res<Time>) {
//     const BULLET_VELOCITY: f32 = 400.0;
//     for mut transform in &mut query {
//         transform.translation.y += BULLET_VELOCITY * time.delta_secs();
//     }
// }

// pub fn spawn_bullet(pos: Vec3) -> (Sprite, Transform, Bullet) {
//     (
//         Sprite {
//             color: Color::linear_rgb(0.5, 0.5, 0.5),
//             custom_size: Some(Vec2::new(10.0, 10.0)),
//             ..default()
//         },
//         Transform::from_translation(Vec3::new(pos.x, pos.y + 50.0, pos.z)),
//         Bullet,
//     )
// }

pub fn move_bullets(
    mut commands: Commands,
    mut query: Query<(&mut Transform, Entity, &BulletVelocity, &mut BulletLifeTime), With<Bullet>>,
    time: Res<Time>,
) {
    for (mut transform, entity, velocity, mut bullet_life_time) in &mut query {
        // move bullets
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();

        // tick BulletLifeTime
        bullet_life_time.0.tick(time.delta());

        // check out of bounds / off-screen
        let out_of_bounds = transform.translation.x.abs() > SCREEN_WIDTH
            || transform.translation.y.abs() > SCREEN_HEIGHT;

        // despawn if off-screen or lifetime expired
        if out_of_bounds || bullet_life_time.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn shoot_with_mouse(
    mouse: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    player_query: Query<&Transform, With<Player>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let Ok(player_tf) = player_query.single() else {
            return;
        };
        let Ok(window) = windows.single() else { return };

        if let Some(cursor_pos) = window.cursor_position() {
            let Ok((camera, camera_tf)) = camera_query.single() else {
                return;
            };

            if let Ok(world_pos) = camera.viewport_to_world(camera_tf, cursor_pos) {
                let direction = (world_pos.origin - player_tf.translation)
                    .xy()
                    .normalize_or_zero();
                let velocity = direction * 500.0;

                commands.spawn((
                    Sprite {
                        color: Color::linear_rgb(0.5, 0.5, 0.5),
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        ..default()
                    },
                    Transform::from_translation(player_tf.translation),
                    Bullet,
                    BulletVelocity(velocity),
                    BulletLifeTime(Timer::from_seconds(0.2, TimerMode::Once)),
                ));
            }
        }
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_bullets.run_if(in_state(GameState::Playing)));
    }
}
