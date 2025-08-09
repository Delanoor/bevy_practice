use crate::{
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    player::{FireCooldown, Player},
    state::GameState,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct BulletVelocity(pub Vec2);

#[derive(Component)]
pub struct BulletLifeTime(pub Timer);

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
    mut cooldown_query: Query<&mut FireCooldown, With<Player>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        println!("Mouse left click detected!");

        let Ok(player_tf) = player_query.single() else {
            println!("No player found!");
            return;
        };
        let Ok(window) = windows.single() else {
            println!("No window found!");
            return;
        };
        let Ok(mut cd) = cooldown_query.single_mut() else {
            println!("No cooldown found!");
            return;
        };

        // dbg!("cd.0.finished: {:?}", cd.0.finished());

        // if !cd.0.finished() {
        //     return;
        // }

        if let Some(cursor_pos) = window.cursor_position() {
            println!("Cursor position: {:?}", cursor_pos);
            let Ok((camera, camera_tf)) = camera_query.single() else {
                println!("No camera found!");
                return;
            };

            if let Ok(world_pos) = camera.viewport_to_world(camera_tf, cursor_pos) {
                println!("World position: {:?}", world_pos.origin);
                let direction = (world_pos.origin - player_tf.translation)
                    .xy()
                    .normalize_or_zero();
                let velocity = direction * 500.0;

                commands.spawn((
                    Sprite {
                        color: Color::linear_rgb(1.0, 1.0, 0.0),
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        ..default()
                    },
                    Transform::from_translation(player_tf.translation),
                    Bullet,
                    BulletVelocity(velocity),
                    BulletLifeTime(Timer::from_seconds(2.5, TimerMode::Once)),
                ));

                println!(
                    "Bullet spawned at {:?} with velocity {:?}",
                    player_tf.translation, velocity
                );
                cd.0.reset();
            } else {
                println!("Failed to convert viewport to world!");
            }
        } else {
            println!("No cursor position!");
        }
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_bullets.run_if(in_state(GameState::Playing)))
            .add_systems(
                Update,
                shoot_with_mouse.run_if(in_state(GameState::Playing)),
            );
    }
}
