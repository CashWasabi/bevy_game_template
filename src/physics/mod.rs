pub mod components;
pub mod systems;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GameState;

pub struct PhysicsPlugin;

// This plugin is responsible to control the game levels
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            // TODO: Try to use standard gravity
            // .insert_resource(RapierConfiguration {
            //     gravity: Vec2::new(0.0, -2000.0),
            //     ..Default::default()
            // })
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(systems::spawn_ground_sensor)
                    .with_system(systems::spawn_wall_sensor)
                    .with_system(systems::ground_detection)
                    .with_system(systems::wall_detection)
            );
    }
}
