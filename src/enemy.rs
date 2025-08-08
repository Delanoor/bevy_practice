use crate::constants::{ENEMY_SIZE, ENEMY_SPEED};
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

pub fn timed_enemy_spawner(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut timer: ResMut<SpawnTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let x = rand::random::<f32>() * 800.0 - 400.0;
        let y = 300.0;

        commands.spawn((
            spawn_enenemy_sprite(asset_server),
            Transform::from_xyz(x, y, 0.0),
            Enemy,
        ));
    }
}

pub fn spawn_enenemy_sprite(asset_server: Res<AssetServer>) -> Sprite {
    let texture = asset_server.load("cat_no.png");
    Sprite {
        image: texture,
        // color: Color::linear_rgb(1.0, 0.0, 0.0),
        custom_size: Some(ENEMY_SIZE),
        ..default()
    }
}
pub fn move_enemies(mut query: Query<&mut Transform, With<Enemy>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.translation.y -= ENEMY_SPEED * time.delta_secs();
    }
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(1.5, TimerMode::Repeating)))
            .add_systems(Update, (move_enemies, timed_enemy_spawner));
    }
}
