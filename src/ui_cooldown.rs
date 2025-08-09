use bevy::prelude::*;

use crate::{player::FireCooldown, state::GameState};

#[derive(Component)]
pub struct CooldownBar;

#[derive(Component)]
pub struct CooldownFill;

pub fn spawn_cooldown_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(16.0),
                bottom: Val::Px(16.0),
                width: Val::Px(180.0),
                height: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::linear_rgb(0.15, 0.15, 0.18)),
            BorderRadius::all(Val::Px(16.0)),
            CooldownBar,
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(0.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::linear_rgb(0.2, 0.8, 0.3)),
                BorderRadius::all(Val::Px(8.0)),
                CooldownFill,
            ));
        });
}

pub fn update_cooldown_ui(
    mut fill_q: Query<&mut Node, With<CooldownFill>>,
    cd_q: Query<&FireCooldown>,
) {
    let Ok(mut node) = fill_q.single_mut() else {
        return;
    };
    let Ok(cd) = cd_q.single() else {
        return;
    };

    let percent = (cd.0.fraction() * 100.0).clamp(0.0, 100.0);
    node.width = Val::Percent(percent);
}

pub struct CooldownUiPlugin;

impl Plugin for CooldownUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_cooldown_ui)
            .add_systems(
                Update,
                update_cooldown_ui.run_if(in_state(GameState::Playing)),
            );
    }
}
