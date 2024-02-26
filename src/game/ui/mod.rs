use crate::{
    game::{Game, GameState, Score},
    AppState,
};
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), setup)
            .add_systems(
                Update,
                update_score
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(AppState::Playing)),
            );
    }
}

#[derive(Component)]
struct TextBase;

fn setup(mut commands: Commands, score: Res<Score>, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            score.0.to_string(),
            TextStyle {
                font_size: 100.,
                font: asset_server.load("fonts/blocky.ttf"),

                ..default()
            },
        )
        .with_style(Style {
            margin: UiRect::new(
                Val::Px(50.),
                Val::Percent(0.),
                Val::Percent(0.),
                Val::Percent(0.),
            ),
            ..default()
        }),
        TextBase,
        Game,
    ));
}

fn update_score(mut text: Query<&mut Text, With<TextBase>>, score: Res<Score>) {
    for mut text in text.iter_mut() {
        text.sections[0].value = score.0.to_string();
    }
}
