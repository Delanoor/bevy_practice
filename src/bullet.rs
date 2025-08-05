use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet;

pub fn move_bullet(mut query: Query<&mut Transform, With<Bullet>>, time: Res<Time>) {
    const BULLET_VELOCITY: f32 = 400.0;
    for mut transform in &mut query {
        transform.translation.y += BULLET_VELOCITY * time.delta_secs();
    }
}

pub fn spawn_bullet(pos: Vec3) -> (Sprite, Transform, Bullet) {
    (
        Sprite {
            color: Color::linear_rgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(10.0, 10.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(pos.x, pos.y + 50.0, pos.z)),
        Bullet,
    )
}
