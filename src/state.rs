use crate::collision::collide;
use crate::constants::{ENEMY_SIZE, PLAYER_SIZE};
use crate::enemy::Enemy;
use crate::player::Player;
use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

pub fn check_player_hit(
    mut next_state: ResMut<NextState<GameState>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_tf) = player_query.single() {
        let player_pos = player_tf.translation;

        for enemy in &enemy_query {
            if collide(enemy.translation, ENEMY_SIZE, player_pos, PLAYER_SIZE) {
                next_state.set(GameState::GameOver);
                dbg!("GAME OVER");
                break;
            }
        }
    }
}
