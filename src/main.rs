#![allow(unused)]

mod ball;
mod barrier;
mod corner;
mod player;
mod util;

use ball::BallPlugin;
use barrier::BarrierPlugin;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use corner::CornerPlugin;
use player::PlayerPlugin;
use util::{clamp, Rectangle};

// region:  -- Resources
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

pub struct ArenaInfo(Rectangle);

// endregion

// region:  -- Game constants

const TIME_STEP: f32 = 1.0 / 60.0;

// endregion

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Crash Ball".to_string(),
            width: 850.,
            height: 850.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(ShapePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CornerPlugin)
        .add_plugin(BarrierPlugin)
        .add_plugin(BallPlugin)
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physic)
        .add_startup_system(setup_arena)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // camera
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_physic(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    // Set gravity to 0.0 and spawn camera.
    rapier_config.gravity = Vec2::ZERO;
    rapier_config.physics_pipeline_active = true;
}

fn setup_arena(mut commands: Commands, mut windows: ResMut<Windows>) {
    // capture window size
    let mut window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    let arena_info = ArenaInfo(Rectangle {
        x: -win_w / 2.0 + 50.0,
        y: -win_h / 2.0 + 50.0,
        w: win_w - 100.0,
        h: win_h - 100.0,
    });
    commands.insert_resource(arena_info);

    // position window (temp for debug)
    window.set_position(IVec2::new(1700, 0));
}
