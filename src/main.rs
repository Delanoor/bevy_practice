use bevy::prelude::*;

mod bullet;
mod collision;
mod constants;
mod enemy;
mod gameover;
mod player;
mod score;
mod state;

use bullet::move_bullet;
use collision::bullet_enemy_collision;
use enemy::{SpawnTimer, move_enemies, timed_enemy_spawner};
use player::PlayerPlugin;
use score::Score;
use state::{GameState, check_player_hit};

use crate::{
    gameover::{hide_game_over, restart_on_key, show_game_over},
    score::ScorePlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(ScorePlugin)
        .insert_state(GameState::Playing)
        .insert_resource(SpawnTimer(Timer::from_seconds(1.5, TimerMode::Repeating)))
        .insert_resource(Score(0))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_enemies,
                timed_enemy_spawner,
                move_bullet,
                bullet_enemy_collision,
            ),
        )
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
