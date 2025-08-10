use bevy::prelude::*;

mod bullet;
mod collision;
mod constants;
mod enemy;
mod gameover;
mod player;
mod score;
mod state;
mod ui_cooldown;

use player::PlayerPlugin;
use score::Score;
use state::GameState;

use crate::{
    bullet::BulletPlugin,
    collision::CollisionPlugin,
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    enemy::EnemyPlugin,
    gameover::GameOverPlugin,
    score::ScorePlugin,
    ui_cooldown::CooldownUiPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Shooter".into(),
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            PlayerPlugin,
            ScorePlugin,
            BulletPlugin,
            EnemyPlugin,
            CollisionPlugin,
            CooldownUiPlugin,
            GameOverPlugin,
        ))
        .insert_state(GameState::Playing)
        .insert_resource(Score(0))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
