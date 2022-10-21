use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

// This plugin is responsible to control the game levels
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -2000.0),
            ..Default::default()
        })
        .add_plugin(RapierDebugRenderPlugin::default())
        ;
    }
}
