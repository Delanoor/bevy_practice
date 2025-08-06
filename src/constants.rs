use bevy::prelude::*;

// Entity sizes for collision detection and sprite rendering
pub const PLAYER_SIZE: Vec2 = Vec2::new(160.0, 120.5);
pub const ENEMY_SIZE: Vec2 = Vec2::new(66.9, 48.9);
pub const BULLET_SIZE: Vec2 = Vec2::new(10.0, 30.0);

// Game constants
pub const PLAYER_SPEED: f32 = 300.0;
pub const ENEMY_SPEED: f32 = 100.0;
