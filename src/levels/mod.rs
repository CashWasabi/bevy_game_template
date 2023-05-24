pub mod components;
pub mod systems;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::levels::components::WallBundle;
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
            .add_system(
                systems::setup.in_schedule(OnEnter(GameState::Playing))
            )
            .add_systems(
                (
                    systems::pause_physics_during_load,
                    systems::spawn_wall_collision,
                    systems::camera_fit_inside_current_level,
                    systems::update_level_selection,
                    systems::restart_level,
                ).in_set(OnUpdate(GameState::Playing))
            )
            .register_ldtk_int_cell::<WallBundle>(1)
            .register_ldtk_int_cell::<WallBundle>(3)
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}
