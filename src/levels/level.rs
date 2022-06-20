use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use heron::prelude::*;

use crate::GameState;
use crate::levels::components;
use crate::levels::systems;

pub struct LevelPlugin;


// This plugin is responsible to control the game levels
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(LdtkPlugin)
        .insert_resource(Gravity::from(Vec3::new(0.0, -2000., 0.0)))
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(
            LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                set_clear_color: SetClearColor::FromLevelBackground,
                ..Default::default()
            }
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
            .with_system(systems::setup)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
            .with_system(systems::pause_physics_during_load)
            .with_system(systems::spawn_wall_collision)
            .with_system(systems::patrol)
        )
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_int_cell::<components::LadderBundle>(2)
        .register_ldtk_int_cell::<components::WallBundle>(3)
        .register_ldtk_entity::<components::MobBundle>("Mob")
        .register_ldtk_entity::<components::ChestBundle>("Chest")
        ;
    }
}
