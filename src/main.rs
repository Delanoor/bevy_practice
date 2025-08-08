use bevy::prelude::*;

mod bullet;
mod collision;
mod constants;
mod enemy;
mod gameover;
mod player;
mod score;
mod state;

use player::PlayerPlugin;
use score::Score;
use state::{GameState, check_player_hit};

use crate::{
    bullet::BulletPlugin,
    collision::CollisionPlugin,
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    enemy::EnemyPlugin,
    gameover::{hide_game_over, restart_on_key, show_game_over},
    score::ScorePlugin,
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
        ))
        .insert_state(GameState::Playing)
        .insert_resource(Score(0))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            check_player_hit.run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnEnter(GameState::GameOver), show_game_over)
        .add_systems(OnExit(GameState::GameOver), hide_game_over)
        .add_systems(Update, restart_on_key.run_if(in_state(GameState::GameOver)))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
