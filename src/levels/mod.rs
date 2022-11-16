pub mod components;
pub mod systems;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::levels::components::WallBundle;
use crate::levels::systems::{
    camera_fit_inside_current_level, pause_physics_during_load, restart_level, setup,
    spawn_wall_collision, update_level_selection,
};
use crate::players::components::PlayerBundle;
use crate::GameState;
pub struct LevelPlugin;

// This plugin is responsible to control the game levels
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Uid(0))
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                set_clear_color: SetClearColor::FromLevelBackground,
                ..Default::default()
            })
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(pause_physics_during_load)
                    .with_system(spawn_wall_collision)
                    .with_system(camera_fit_inside_current_level)
                    .with_system(update_level_selection)
                    .with_system(restart_level),
            )
            .register_ldtk_int_cell::<WallBundle>(1)
            .register_ldtk_int_cell::<WallBundle>(3)
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}

