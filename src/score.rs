use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_score_text);
    }
}

#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct ScoreText;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/WonderBoys-Regular.ttf");

    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font,
            font_size: 50.0,

            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(15.0),
            top: Val::Px(15.0),
            ..default()
        },
        ScoreText,
    ));
}

pub fn update_score_text(score: ResMut<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if let Ok(mut text) = query.single_mut() {
        text.0 = format!("Score: {}", score.0);
    }
}
