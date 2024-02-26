use crate::game::Game;
use crate::game::GameState;
use crate::AppState;
use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use bevy_rapier2d::prelude::*;
use rand::{self, thread_rng, Rng};

pub struct CloudPlugin;

impl Plugin for CloudPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CloudTimer { ..default() }).add_systems(
            Update,
            (
                cloud_spawn.run_if(in_state(AppState::Playing)),
                cloud_slow_down.run_if(in_state(GameState::Dead)),
            ),
        );
    }
}

#[derive(Resource)]
struct CloudTimer {
    time_since_last_spawn: f32,
    time_needed_to_spawn: f32,
}

impl Default for CloudTimer {
    fn default() -> Self {
        CloudTimer {
            time_needed_to_spawn: 0.6,
            time_since_last_spawn: 0.,
        }
    }
}

#[derive(Component)]
struct Cloud;

const CLOUDPATH: &str = "Sprites/cloud.png";
const SPAWNPOS_X: f32 = 1200.;
const HEIGHT: f32 = 540.;

fn cloud_spawn(
    time: Res<Time>,
    mut cloud_timer: ResMut<CloudTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if cloud_timer.time_since_last_spawn > cloud_timer.time_needed_to_spawn {
        let mut cloud = commands.spawn((RigidBody::KinematicVelocityBased, Cloud, Game));
        let distance = thread_rng().gen_range(1..4);
        match distance {
            1 => {
                cloud
                    .insert((
                        Velocity {
                            linvel: vec2(-150., 0.),
                            ..default()
                        },
                        SpriteBundle {
                            texture: asset_server.load(CLOUDPATH),
                            sprite: Sprite {
                                color: Color::Rgba {
                                    red: 255.,
                                    green: 255.,
                                    blue: 255.,
                                    alpha: 0.5,
                                },
                                ..default()
                            },
                            ..default()
                        },
                    ))
                    .insert(Transform {
                        scale: vec3(0.2, 0.2, 1.),
                        translation: vec3(
                            SPAWNPOS_X,
                            thread_rng().gen_range((-HEIGHT)..HEIGHT),
                            -10.,
                        ),
                        ..default()
                    });
            }
            2 => {
                cloud
                    .insert((
                        Velocity {
                            linvel: vec2(-125., 0.),
                            ..default()
                        },
                        SpriteBundle {
                            texture: asset_server.load(CLOUDPATH),
                            sprite: Sprite {
                                color: Color::Rgba {
                                    red: 255.,
                                    green: 255.,
                                    blue: 255.,
                                    alpha: 0.3,
                                },
                                ..default()
                            },
                            ..default()
                        },
                    ))
                    .insert(Transform {
                        scale: vec3(0.4, 0.4, 1.),
                        translation: vec3(
                            SPAWNPOS_X,
                            thread_rng().gen_range((-HEIGHT)..HEIGHT),
                            -15.,
                        ),
                        ..default()
                    });
            }
            _ => (),
        }
        cloud_timer.time_since_last_spawn = 0.;
    }
    cloud_timer.time_since_last_spawn += time.delta_seconds();
}

fn cloud_slow_down(mut clouds: Query<&mut Velocity, With<Cloud>>) {
    for mut velocity in clouds.iter_mut() {
        if velocity.linvel.x < -25. {
            velocity.linvel.x += 3.;
        } else {
            velocity.linvel.x = -25.;
        }
    }
}
