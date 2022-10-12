use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GameState;
use crate::levels::components;
use crate::levels::systems;
pub struct LevelPlugin;

// This plugin is responsible to control the game levels
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(LdtkPlugin)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -500.0),
            ..Default::default()
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(systems::setup)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(systems::spawn_wall_collision)
                .with_system(systems::camera_fit_inside_current_level)
                .with_system(systems::update_level_selection)
                .with_system(systems::restart_level)
        )
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_int_cell::<components::WallBundle>(3)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        ;
    }
}