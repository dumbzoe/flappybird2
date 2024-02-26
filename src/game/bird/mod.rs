use crate as main;
use crate::{game::Game, AppState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), setup)
            .add_systems(
                Update,
                (
                    jump.run_if(in_state(main::game::GameState::Playing)),
                    rotate_based_on_velocity,
                ),
            );
    }
}

#[derive(Component)]
pub struct Bird;

#[derive(Bundle)]
struct FlappyBird {
    pub name: Name,
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub gravityscale: GravityScale,
    pub velocity: Velocity,
    pub bird: Bird,
    pub game: Game,
}

// The first Comment
//CIRCLERADIUS is a tuple so that we can have more forgiving hitboxes
//The first number is the width of the sprite/mesh and the second number is the offset so its more forgiving
const CIRCLERADIUS: (f32, f32) = (30., 5.);

const JUMPHEIGHT: f32 = 425.;

impl Default for FlappyBird {
    fn default() -> Self {
        let gravity: f32 = 10.;
        FlappyBird {
            name: Name::from("Bird"),
            rigidbody: RigidBody::Dynamic,
            collider: Collider::ball(CIRCLERADIUS.0 - CIRCLERADIUS.1),
            gravityscale: GravityScale(gravity),
            velocity: Velocity { ..default() },
            bird: Bird,
            game: Game,
        }
    }
}

//spawns the bird with a custom sprite
fn setup(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands
        .spawn((
            FlappyBird { ..default() },
            SpriteBundle {
                texture: assets_server.load("Sprites/BirdBody.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(60., 70.)),
                    anchor: bevy::sprite::Anchor::Custom(Vec2::new(-0.02, -0.08)),
                    ..default()
                },
                ..default()
            },
        ))
        .insert(TransformBundle::from(Transform::from_xyz(-400., 0., 0.)));
}

//jumps when you press space
fn jump(mut bird: Query<&mut Velocity, With<Bird>>, input: Res<ButtonInput<KeyCode>>) {
    for mut velocity in bird.iter_mut() {
        if input.just_pressed(KeyCode::Space) {
            velocity.linvel.y = JUMPHEIGHT;
        }
    }
}

//rotates the bird based on vertical velocity
fn rotate_based_on_velocity(mut bird: Query<(&mut Transform, &Velocity), With<Bird>>) {
    for (mut transform, velocity) in bird.iter_mut() {
        let mut angle = velocity.linvel.y / 10.;
        if angle > 45. {
            angle = 45.
        }
        if angle < -90. {
            angle = -90.;
        }

        transform.rotation = Quat::from_rotation_z(main::game::degrees_to_radians(angle));
    }
}
