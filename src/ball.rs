use std::f32::consts::PI;

use crate::{corner::CORNER_RADIUS, util::Rectangle, ArenaInfo};
use bevy::{prelude::*, time::FixedTimestep};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(ball_spawn_system),
        )
        .add_system(ball_despawn_system)
        .add_system(ball_speed_control_system)
        .insert_resource(BallCounter(0));
    }
}

pub const MAX_BALL_COUNT: u32 = 5;
pub const BALL_SPEED: f32 = 400.0;
pub const BALL_RADIUS: f32 = 20.0;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct BallState {
    has_energy: bool,
}

impl BallState {
    pub fn energize(self: &mut Self) {
        self.has_energy = true;
    }
}

impl Default for BallState {
    fn default() -> Self {
        Self { has_energy: false }
    }
}

#[derive(Component)]
struct BallCounter(u32);

fn ball_spawn_system(
    mut commands: Commands,
    mut ball_counter: ResMut<BallCounter>,
    time: Res<Time>,
    arena_info: Res<ArenaInfo>,
) {
    if ball_counter.0 == MAX_BALL_COUNT {
        return;
    }

    let shape = shapes::Circle {
        radius: BALL_RADIUS,
        ..Default::default()
    };

    let spawn_points = [
        (arena_info.0.top_left(), -45.0 as f32),
        (arena_info.0.top_right(), -135.0 as f32),
        (arena_info.0.bottom_left(), 45.0 as f32),
        (arena_info.0.bottom_right(), 135.0 as f32),
    ];

    let mut rng = thread_rng();
    let spawn_idx = rng.gen_range(0..4);

    let (spawn_point, base_angle) = spawn_points[spawn_idx];

    let initial_angle = rng.gen_range(base_angle - 30.0..base_angle + 30.0);
    let initial_angle = initial_angle / 180.0 * PI;
    let velocity_x = initial_angle.cos() * BALL_SPEED;
    let velocity_y = initial_angle.sin() * BALL_SPEED;

    let spawn_point = spawn_point
        + Vec3 {
            x: initial_angle.cos() * (CORNER_RADIUS + BALL_RADIUS),
            y: initial_angle.sin() * (CORNER_RADIUS + BALL_RADIUS),
            z: 0.0,
        };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(bevy_prototype_lyon::prelude::FillMode::color(Color::GRAY)),
            Transform {
                translation: Vec3 {
                    x: spawn_point.x,
                    y: spawn_point.y,
                    z: 10.0,
                },
                ..Default::default()
            },
        ))
        .insert(Ball)
        .insert(BallState::default())
        .insert(RigidBody::Dynamic)
        .insert(Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::linear(Vec2 {
            x: velocity_x,
            y: velocity_y,
        }))
        .insert(Collider::ball(BALL_RADIUS))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Ccd::enabled());

    ball_counter.0 += 1;
}

fn is_out_of_bound(pos: &Vec3, arena_info: &Res<ArenaInfo>) -> bool {
    return pos.x < arena_info.0.left() - CORNER_RADIUS
        || pos.x > arena_info.0.right() + CORNER_RADIUS
        || pos.y < arena_info.0.bottom() - CORNER_RADIUS
        || pos.y > arena_info.0.top() + CORNER_RADIUS;
}

fn ball_despawn_system(
    mut commands: Commands,
    mut ball_counter: ResMut<BallCounter>,
    mut query: Query<(Entity, &Transform)>,
    arena_info: Res<ArenaInfo>,
) {
    for (entity, transform) in query.iter_mut() {
        if is_out_of_bound(&transform.translation, &arena_info) {
            commands.entity(entity).despawn();
            ball_counter.0 -= 1;
        }
    }
}

pub fn ball_update_speed(vect: Vec2, ball_state: &BallState, velocity: &mut Velocity) {
    // ensure constant speed
    let speed = if ball_state.has_energy {
        BALL_SPEED * 2.0
    } else {
        BALL_SPEED
    };
    let v = vect.normalize() * speed;
    *velocity = Velocity::linear(v);
}

fn ball_speed_control_system(mut query: Query<(&BallState, &mut Velocity), With<Ball>>) {
    for (ball_state, mut velocity) in query.iter_mut() {
        // ensure constant speed
        ball_update_speed(velocity.linvel, &ball_state, &mut velocity);
    }
}

fn ball_collision_system(
    mut events: EventReader<CollisionEvent>,
    mut balls: Query<(&mut Velocity), With<Ball>>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(a, b, _) => {}

            CollisionEvent::Stopped(a, b, _) => {
                if let Ok(mut velocity) = balls.get_mut(*a) {
                    let v = velocity.linvel.normalize() * BALL_SPEED;

                    *velocity = Velocity::linear(v);
                }

                if let Ok(mut velocity) = balls.get_mut(*b) {
                    let v = velocity.linvel.normalize() * BALL_SPEED;

                    *velocity = Velocity::linear(v);
                }
            }
        }
    }
}
