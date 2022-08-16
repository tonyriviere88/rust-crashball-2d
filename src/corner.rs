use crate::{util::Rectangle, ArenaInfo};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CornerPlugin;

impl Plugin for CornerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, corner_spawn_system);
    }
}

pub const CORNER_RADIUS: f32 = 70.0;

#[derive(Component)]
struct Corner;

fn corner_spawn_system(mut commands: Commands, arena_info: Res<ArenaInfo>) {
    let shape = shapes::Circle {
        radius: CORNER_RADIUS,
        ..Default::default()
    };

    for pos in [
        arena_info.0.top_left(),
        arena_info.0.top_right(),
        arena_info.0.bottom_left(),
        arena_info.0.bottom_right(),
    ] {
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                DrawMode::Fill(bevy_prototype_lyon::prelude::FillMode::color(
                    Color::DARK_GRAY,
                )),
                Transform {
                    translation: pos,
                    ..Default::default()
                },
            ))
            .insert(Corner)
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
            .insert(Collider::ball(CORNER_RADIUS));
    }
}
