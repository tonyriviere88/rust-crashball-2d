use crate::{
    ball::{ball_update_speed, Ball, BallState, BALL_RADIUS},
    corner::CORNER_RADIUS,
    util::{clamp, Rectangle},
    ArenaInfo, TIME_STEP,
};
use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_system(player_keyboard_event_system)
            .add_system(player_fire_energy)
            .add_system(player_update_energy)
            .add_system(player_energy_hit_ball);
    }
}

pub const PLAYER_BASE_SPEED: f32 = 350.0;
pub const PLAYER_ACCELERATE_SPEED: f32 = 550.0;
pub const PLAYER_RADIUS: f32 = 70.0;
pub const PLAYER_ENERGY_RADIUS: f32 = 80.0; // on top of PLAYER_RADIUS
pub const GAMEPAD_AXIS_THRESHOLD: f32 = 0.3;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerEnergy;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum PlayerAction {
    Move,
    MoveLeft,
    MoveRight,
    Accelerate,
    Energy,
}

impl PlayerAction {
    fn default_key_map() -> InputMap<PlayerAction> {
        let mut input_map = InputMap::default();

        // Move left
        input_map.insert(KeyCode::Left, PlayerAction::MoveLeft);
        input_map.insert(GamepadButtonType::DPadLeft, PlayerAction::MoveLeft);
        input_map.insert(
            SingleAxis::symmetric(GamepadAxisType::LeftStickX, DualAxis::DEFAULT_DEADZONE),
            PlayerAction::Move,
        );

        // Move right
        input_map.insert(KeyCode::Right, PlayerAction::MoveRight);
        input_map.insert(GamepadButtonType::DPadRight, PlayerAction::MoveRight);

        //input_map.insert(GamepadAxisType::LeftStickX, PlayerAction::Move);

        // Accelerate
        input_map.insert(KeyCode::LControl, PlayerAction::Accelerate);
        input_map.insert(GamepadButtonType::RightTrigger, PlayerAction::Accelerate);

        // Energy
        input_map.insert(KeyCode::Space, PlayerAction::Energy);
        input_map.insert(GamepadButtonType::West, PlayerAction::Energy);

        input_map
    }
}

fn player_spawn_system(mut commands: Commands, arena_info: Res<ArenaInfo>) {
    let shape = shapes::Circle {
        radius: PLAYER_RADIUS,
        ..Default::default()
    };

    let left_bound = arena_info.0.left() + PLAYER_RADIUS + CORNER_RADIUS;
    let right_bound = arena_info.0.right() - PLAYER_RADIUS - CORNER_RADIUS;

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(bevy_prototype_lyon::prelude::FillMode::color(Color::CYAN)),
            Transform {
                translation: arena_info.0.bottom_middle(),
                ..Default::default()
            },
        ))
        .insert(Player)
        .insert(RigidBody::KinematicPositionBased)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Friction {
            coefficient: 0.65,
            combine_rule: CoefficientCombineRule::Max,
        })
        .insert(Collider::ball(PLAYER_RADIUS))
        .insert_bundle(InputManagerBundle::<PlayerAction> {
            // Stores "which actions are currently pressed"
            action_state: ActionState::default(),
            // Describes how to convert from player inputs into those actions
            input_map: PlayerAction::default_key_map(),
        });
}

fn player_keyboard_event_system(
    mut player_query: Query<(&mut Transform, &ActionState<PlayerAction>), With<Player>>,
    area_info: Res<ArenaInfo>,
) {
    let left_bound = area_info.0.left() + CORNER_RADIUS + PLAYER_RADIUS;
    let right_bound = area_info.0.right() - CORNER_RADIUS - PLAYER_RADIUS;

    if let Ok((mut rb_trans, action_state)) = player_query.get_single_mut() {
        let x_axis = if (action_state.pressed(PlayerAction::MoveLeft)) {
            -1.0
        } else if (action_state.pressed(PlayerAction::MoveRight)) {
            1.0
        } else if (action_state.pressed(PlayerAction::Move)) {
            match (action_state.clamped_value(PlayerAction::Move)) {
                num if num > GAMEPAD_AXIS_THRESHOLD => 1.0,
                num if num < -GAMEPAD_AXIS_THRESHOLD => -1.0,
                _ => 0.0,
            }
        } else {
            0.0
        };

        let player_speed = if (action_state.pressed(PlayerAction::Accelerate)) {
            PLAYER_ACCELERATE_SPEED
        } else {
            PLAYER_BASE_SPEED
        };

        rb_trans.translation.x += x_axis as f32 * player_speed * TIME_STEP;
        rb_trans.translation.x = clamp(rb_trans.translation.x, left_bound, right_bound);
    }
}

fn player_fire_energy(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform, &ActionState<PlayerAction>), With<Player>>,
) {
    if let Ok((player_id, player_tf, action_state)) = player_query.get_single() {
        if action_state.just_pressed(PlayerAction::Energy) {
            // Spawn the energy effect
            let shape = shapes::Circle {
                radius: PLAYER_RADIUS,
                ..Default::default()
            };

            let energy = commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Stroke(StrokeMode::new(Color::WHITE, 5.0)),
                    Transform::default(),
                ))
                .insert(PlayerEnergy)
                .id();

            commands.entity(player_id).push_children(&[energy]);
        }
    }
}

fn player_update_energy(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<PlayerEnergy>>,
) {
    let scale_threshold = 1.0 + PLAYER_ENERGY_RADIUS / PLAYER_RADIUS;
    for (id, mut transform) in query.iter_mut() {
        transform.scale += scale_threshold * 2.0 * TIME_STEP;

        if transform.scale.x > scale_threshold {
            commands.entity(id).despawn();
        }
    }
}

fn player_energy_hit_ball(
    player_query: Query<(&Transform), With<Player>>,
    player_energy_query: Query<(&Transform), With<PlayerEnergy>>,
    mut balls_query: Query<(&mut BallState, &Transform, &mut Velocity), With<Ball>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for pe_transf in player_energy_query.iter() {
            let pe_radius = PLAYER_RADIUS * pe_transf.scale.x;
            let threashold_dist = pe_radius + BALL_RADIUS;

            // Move the balls in the range
            for (mut ball_state, ball_tf, mut velocity) in balls_query.iter_mut() {
                let vect = ball_tf.translation.truncate() - player_transform.translation.truncate();
                if vect.length() < threashold_dist {
                    ball_state.energize();
                    ball_update_speed(vect, &ball_state, &mut velocity);
                }
            }
        }
    }
}
