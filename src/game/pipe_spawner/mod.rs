use crate::AppState;
use bevy::{math::vec3, prelude::*};
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

use crate::game::GameState;
use crate::game::{degrees_to_radians, Game};

const SPAWNPOS_X: f32 = 1200.;
const MAXHEIGHT: f32 = 375.;

pub struct PipeSpawnerPlugin;

impl Plugin for PipeSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PipeTimer { ..default() })
            //.add_systems(Startup, setup)
            .add_systems(
                Update,
                spawn_pipe
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(AppState::Playing)),
            )
            .add_systems(
                Update,
                pipe_slow_down
                    .run_if(in_state(GameState::Dead))
                    .run_if(in_state(AppState::Playing)),
            );
    }
}

#[derive(Bundle)]
struct PipeBundle {
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub velocity: Velocity,
    pub pipe: PipeGap,
    pub game: Game,
}

#[derive(Component)]
pub struct PipeGap {
    pub interacted: bool,
}

#[derive(Component)]
pub struct Pipe;

impl Default for PipeBundle {
    fn default() -> Self {
        PipeBundle {
            rigidbody: RigidBody::KinematicVelocityBased,
            collider: Collider::cuboid(0.01, 150.),
            velocity: Velocity {
                linvel: Vec2::new(-200., 0.),
                angvel: 0.,
            },
            pipe: PipeGap { interacted: false },
            game: Game,
        }
    }
}

#[derive(Resource)]
struct PipeTimer {
    time_since_last_spawn: f32,
    time_needed_to_spawn: f32,
}

impl Default for PipeTimer {
    fn default() -> Self {
        PipeTimer {
            time_since_last_spawn: 3.,
            time_needed_to_spawn: 2.5,
        }
    }
}

fn spawn_pipe(
    mut pipe_timer: ResMut<PipeTimer>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if pipe_timer.time_since_last_spawn > pipe_timer.time_needed_to_spawn {
        pipe_timer.time_since_last_spawn = 0.;
        let height: f32 = thread_rng().gen_range(-MAXHEIGHT..MAXHEIGHT);
        let pipe_centre = commands
            .spawn(PipeBundle { ..default() })
            .insert(TransformBundle::from(Transform::from_xyz(
                SPAWNPOS_X, height, 0.,
            )))
            .insert(Sensor)
            .id();
        let pipe1 = commands
            .spawn(SpriteBundle {
                texture: asset_server.load("Sprites/pipe.png"),
                ..default()
            })
            .insert(Transform {
                translation: vec3(0., 588., 0.),
                scale: vec3(0.4, 0.4, 1.),
                ..default()
            })
            .insert(Collider::cuboid(230., 1090.))
            .insert(Pipe)
            .id();
        let pipe2 = commands
            .spawn(SpriteBundle {
                texture: asset_server.load("Sprites/pipe.png"),
                ..default()
            })
            .insert(Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                    vec3(0.4, 0.4, 1.),
                    Quat::from_rotation_z(degrees_to_radians(180.)),
                    vec3(0., -588., 0.),
                ),
            ))
            .insert(Collider::cuboid(230., 1090.))
            .insert(Pipe)
            .id();
        commands.entity(pipe_centre).add_child(pipe1);
        commands.entity(pipe_centre).add_child(pipe2);
    }
    pipe_timer.time_since_last_spawn += time.delta_seconds()
}

fn pipe_slow_down(mut pipes: Query<&mut Velocity, With<PipeGap>>) {
    for mut velocity in pipes.iter_mut() {
        if velocity.linvel.x < 0. {
            velocity.linvel.x += 3.;
        } else {
            velocity.linvel.x = 0.;
        }
    }
}
