mod bird;
use bird::{Bird, BirdPlugin};
mod cloud;
mod pipe_spawner;
mod ui;

use crate::AppState;
use bevy::{math::f32, prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum GameState {
    Playing,
    Dead,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .insert_state(GameState::Playing)
            .insert_resource(GameOverTimer::default())
            .add_plugins((
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "FlappyBird".to_string(),
                        resolution: WindowResolution::new(1920., 1080.),
                        focused: true,
                        mode: bevy::window::WindowMode::BorderlessFullscreen,
                        ..default()
                    }),
                    ..default()
                }),
                BirdPlugin,
                ui::UiPlugin,
                pipe_spawner::PipeSpawnerPlugin,
                cloud::CloudPlugin,
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            ))
            .add_plugins((
                RapierDebugRenderPlugin::default(),
                WorldInspectorPlugin::new(),
            ))
            .add_systems(
                OnEnter(AppState::Playing),
                (
                    set_sky_colour,
                    reset_game_state.run_if(in_state(GameState::Dead)),
                ),
            )
            .add_systems(
                Update,
                delete_offscreen_entities
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(AppState::Playing)),
            )
            .add_systems(
                Update,
                bird_pipe_collide
                    .run_if(in_state(AppState::Playing))
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                gameover
                    .run_if(in_state(GameState::Dead))
                    .run_if(in_state(AppState::Playing)),
            )
            .add_systems(OnExit(AppState::Playing), exit);
    }
}

fn set_sky_colour(mut sky_colour: ResMut<ClearColor>) {
    sky_colour.0 = Color::rgb_u8(135, 206, 235);
}

fn reset_game_state(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Playing);
}

pub fn degrees_to_radians(deg: f32) -> f32 {
    deg * (3.14159265 / 180.)
}

fn delete_offscreen_entities(
    entities: Query<(Entity, &Transform), Without<Bird>>,
    mut commands: Commands,
) {
    for (entity, transform) in entities.iter() {
        if transform.translation.x < -1200. {
            commands.entity(entity).despawn();
        }
    }
}

fn bird_pipe_collide(
    bird: Query<Entity, With<Bird>>,
    pipes: Query<Entity, With<pipe_spawner::Pipe>>,
    mut pipe_gaps: Query<(Entity, &mut pipe_spawner::PipeGap)>,
    rapier_context: Res<RapierContext>,
    mut game_state: ResMut<NextState<GameState>>,
    mut score: ResMut<Score>,
) {
    for bird in bird.iter() {
        for pipe in pipes.iter() {
            if let Some(contact_pair) = rapier_context.contact_pair(bird, pipe) {
                if contact_pair.has_any_active_contacts() {
                    game_state.set(GameState::Dead)
                }
            }
        }
        for (pipe_gap, mut pipe_gap_interaction) in pipe_gaps.iter_mut() {
            if rapier_context.intersection_pair(bird, pipe_gap) == Some(true)
                && pipe_gap_interaction.interacted == false
            {
                pipe_gap_interaction.interacted = true;
                score.0 += 1;
            }
        }
    }
}

#[derive(Component)]
pub struct Game;

#[derive(Resource)]
pub struct Score(u32);

#[derive(Resource)]
struct GameOverTimer {
    time_since_gameover: f32,
    time_needed_gameover: f32,
}

impl Default for GameOverTimer {
    fn default() -> Self {
        GameOverTimer {
            time_since_gameover: 0.,
            time_needed_gameover: 5.,
        }
    }
}

fn gameover(
    mut gameover_timer: ResMut<GameOverTimer>,
    time: Res<Time>,
    mut appstate: ResMut<NextState<AppState>>,
) {
    if gameover_timer.time_since_gameover > gameover_timer.time_needed_gameover {
        appstate.set(AppState::MainMenu);
        gameover_timer.time_since_gameover = 0.0;
    }
    gameover_timer.time_since_gameover += time.delta_seconds();
}

fn exit(mut items: Query<Entity, With<Game>>, mut commands: Commands) {
    for entity in items.iter_mut() {
        commands.entity(entity).despawn_descendants().despawn()
    }
}
