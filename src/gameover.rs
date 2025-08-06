use bevy::prelude::*;

use crate::{bullet::Bullet, enemy::Enemy, player::Player, score::Score, state::GameState};

#[derive(Component)]
pub struct GameOverText;

pub fn show_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/WonderBoys-Regular.ttf");

    // Spawn a container that covers the full screen
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            GameOverText,
        ))
        .with_children(|parent| {
            // Spawn the text as a child of the container
            parent.spawn((
                Text::new("GAME OVER, YOU DEAD\nPress R to Restart"),
                TextFont {
                    font_size: 80.0,
                    font,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });
}

pub fn hide_game_over(mut commands: Commands, query: Query<Entity, With<GameOverText>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn restart_on_key(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut score: ResMut<Score>,
    mut state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    query_player: Query<Entity, With<Player>>,
    query_enemy: Query<Entity, With<Enemy>>,
    query_bullet: Query<Entity, With<Bullet>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        // clean up

        score.0 = 0;

        for e in query_player {
            commands.entity(e).despawn();
        }

        for e in query_enemy {
            commands.entity(e).despawn();
        }

        for e in query_bullet {
            commands.entity(e).despawn();
        }

        state.set(GameState::Playing);
    }
}
