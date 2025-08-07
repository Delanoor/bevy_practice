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
    mut player_query: Query<(&Transform, &mut Sprite), With<Player>>,
) {
    if let Ok((player_tf, mut sprite)) = player_query.single_mut() {
        let player_pos = player_tf.translation;

        for enemy in &enemy_query {
            if collide(enemy.translation, ENEMY_SIZE, player_pos, PLAYER_SIZE) {
                next_state.set(GameState::GameOver);
                sprite.color = Color::linear_rgb(0.0, 0.0, 0.0);
                dbg!("GAME OVER");
                break;
            }
        }
    }
}
