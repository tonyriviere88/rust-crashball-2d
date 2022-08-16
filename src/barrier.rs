use std::f32::consts::PI;

use crate::{util::Rectangle, ArenaInfo};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct BarrierPlugin;

impl Plugin for BarrierPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, barrier_spawn_system);
    }
}

const BARRIER_THICKNESS: f32 = 10.0;

#[derive(Component)]
struct Barrier;

fn barrier_spawn_system(mut commands: Commands, arena_info: Res<ArenaInfo>) {
    let shape = shapes::Rectangle {
        extents: Vec2 {
            x: arena_info.0.w,
            y: BARRIER_THICKNESS,
        },
        ..Default::default()
    };

    for (pos, angle) in [
        (arena_info.0.left_middle(), -90.0),
        (arena_info.0.top_middle(), 180.0),
        (arena_info.0.right_middle(), 90.0),
    ] {
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                DrawMode::Fill(bevy_prototype_lyon::prelude::FillMode::color(
                    Color::DARK_GRAY,
                )),
                Transform {
                    translation: pos,
                    rotation: Quat::from_rotation_z(angle / 180.0 * PI),
                    ..Default::default()
                },
            ))
            .insert(Barrier)
            .insert(RigidBody::Fixed)
            .insert(Restitution {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Min,
            })
            .insert(Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            })
            .insert(Velocity::zero())
            .insert(Collider::cuboid(
                arena_info.0.w / 2.0,
                BARRIER_THICKNESS / 2.0,
            ));
    }
}
