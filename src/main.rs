use bevy::prelude::*;

mod bullet;
mod collision;
mod enemy;
mod player;
mod score;

use bullet::move_bullet;
use collision::bullet_enemy_collision;
use enemy::{SpawnTimer, move_enemies, timed_enemy_spawner};
use player::PlayerPlugin;
use score::Score;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
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
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
