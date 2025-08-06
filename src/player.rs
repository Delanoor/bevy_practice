use crate::bullet::spawn_bullet;
use crate::constants::{PLAYER_SIZE, PLAYER_SPEED};
use crate::state::GameState;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player);
    }
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera
    commands.spawn(Camera2d);

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
    ));
}

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if keyboard_input.just_pressed(KeyCode::Space) {
            let pos = transform.translation;
            commands.spawn(spawn_bullet(pos));
        }

        let speed = PLAYER_SPEED;

        transform.translation += direction * speed * time.delta_secs();
    }
}
