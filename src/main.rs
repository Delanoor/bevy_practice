use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SpawnTimer(Timer::from_seconds(1.5, TimerMode::Repeating)))
        .add_systems(Startup, setup_game)
        .add_systems(Update, move_square)
        .add_systems(Update, (move_enemies, timed_enemy_spawner, move_bullet))
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Bullet;

fn setup_game(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.1, 1.0, 1.0),
            custom_size: Some(Vec2::new(140.0, 140.0)),
            ..default()
        },
        Transform::default(), // Position at (0, 0, 0)
        // GlobalTransform::default(),
        Player,
    ));

    // for _ in 0..5 {
    //     let x = rand::random::<f32>() * 800.0 - 400.0;
    //     let y = rand::random::<f32>() * 600.0 - 300.0;

    //     commands.spawn((
    //         Sprite {
    //             color: Color::linear_rgb(1.0, 0.0, 0.0),
    //             custom_size: Some(Vec2::new(100.0, 100.0)),
    //             ..default()
    //         },
    //         Transform::from_xyz(x, y, 0.0),
    //         // GlobalTransform::default(),
    //         Enemy,
    //     ));
    // }
}

fn move_square(
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
            commands.spawn((
                Sprite {
                    color: Color::linear_rgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(pos.x, pos.y + 50.0, pos.z)),
                Bullet,
            ));
        }

        let speed = 300.0;

        transform.translation += direction * speed * time.delta_secs();
    }
}

fn move_enemies(mut query: Query<&mut Transform, With<Enemy>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.translation.y -= 100.0 * time.delta_secs();
    }
}

#[derive(Resource)]
struct SpawnTimer(Timer);

fn timed_enemy_spawner(mut commands: Commands, time: Res<Time>, mut timer: ResMut<SpawnTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        let x = rand::random::<f32>() * 800.0 - 400.0;
        let y = 300.0;

        commands.spawn((
            spawn_enenemy_sprite(),
            Transform::from_xyz(x, y, 0.0),
            Enemy,
        ));
    }
}

fn spawn_enenemy_sprite() -> Sprite {
    Sprite {
        color: Color::linear_rgb(1.0, 0.0, 0.0),
        custom_size: Some(Vec2::new(100.0, 100.0)),
        ..default()
    }
}

fn move_bullet(mut query: Query<&mut Transform, With<Bullet>>, time: Res<Time>) {
    const BULLET_VELOCITY: f32 = 400.0;
    for mut transform in &mut query {
        transform.translation.y += BULLET_VELOCITY * time.delta_secs();
    }
}
