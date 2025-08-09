use crate::constants::{PLAYER_SIZE, PLAYER_SPEED};
use crate::state::GameState;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct FireConfig {
    pub cooldown_secs: f32,
}

#[derive(Component)]
pub struct FireCooldown(pub Timer);

pub fn attach_fire_cooldown(
    mut commands: Commands,
    q: Query<Entity, With<Player>>,
    cfg: Res<FireConfig>,
) {
    if let Ok(player) = q.single() {
        commands
            .entity(player)
            .insert(FireCooldown(Timer::from_seconds(
                cfg.cooldown_secs,
                TimerMode::Once,
            )));
    }
}

pub fn tick_fire_cooldown(time: Res<Time>, mut q: Query<&mut FireCooldown, With<Player>>) {
    if let Ok(mut cd) = q.single_mut() {
        cd.0.tick(time.delta());
    }
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FireConfig {
            cooldown_secs: 0.25,
        })
        .add_systems(OnEnter(GameState::Playing), spawn_player)
        .add_systems(Update, move_player)
        .add_systems(
            Update,
            rotate_player_to_mouse.run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            tick_fire_cooldown.run_if(in_state(GameState::Playing)),
        );
    }
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, cfg: Res<FireConfig>) {
    // Spawn camera

    let texture = asset_server.load("main_sc.png");

    commands.spawn((
        Sprite {
            flip_x: false,
            flip_y: false,
            color: Color::WHITE,
            rect: None,
            custom_size: Some(PLAYER_SIZE),
            image: texture,
            image_mode: SpriteImageMode::Auto,

            ..default()
        },
        Transform::from_xyz(0.0, -200.0, 0.0),
        Player,
        FireCooldown(Timer::from_seconds(cfg.cooldown_secs, TimerMode::Once)),
    ));
}

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyE) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyF) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        let speed = PLAYER_SPEED;

        transform.translation += direction * speed * time.delta_secs();
    }
}

// pub fn move_player_to_mouse(
//     windows: Query<&Window>,
//     camera_query: Query<(&Camera, &GlobalTransform)>,
//     mut query: Query<&mut Transform, With<Player>>,
// ) {
//     let Ok(mut player_tf) = query.single_mut() else {
//         return;
//     };
//     let Ok(window) = windows.single() else { return };

//     if let Some(cursor_pos) = window.cursor_position() {
//         let Ok((camera, camera_tf)) = camera_query.single() else {
//             return;
//         };

//         if let Ok(world_pos) = camera.viewport_to_world(camera_tf, cursor_pos) {
//             player_tf.translation = world_pos.origin;
//         }
//     }
// }

pub fn rotate_player_to_mouse(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(&mut Transform, &mut Sprite), With<Player>>,
) {
    let Ok((mut transform, mut sprite)) = query.single_mut() else {
        return;
    };
    let Ok(window) = windows.single() else { return };

    if let Some(cursor_pos) = window.cursor_position() {
        let Ok((camera, camera_tf)) = camera_query.single() else {
            return;
        };

        if let Ok(world_pos) = camera.viewport_to_world(camera_tf, cursor_pos) {
            let dir = world_pos.origin - transform.translation;

            // Calculate angle to mouse
            let angle = dir.y.atan2(dir.x);

            // Flip sprite based on mouse position
            if dir.x > 0.0 {
                sprite.flip_x = true; // Flip when mouse is on the right
                transform.rotation = Quat::from_rotation_z(angle);

            // Adjust rotation for flipped sprite
            } else {
                // sprite.flip_x = false; // Normal (left-facing) when mouse is on the left
                // Normal rotation for unflipped sprite
                sprite.flip_x = false;
                transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::PI)
            }
        }
    }
}
