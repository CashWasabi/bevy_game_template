use bevy::prelude::*;

use crate::players::systems::*;
use crate::GameState;

pub struct PlayerPlugin;

// TODO(MO): Also use JumpApex (when higher in jump movement in x gets better)

/// This plugin handles player related stuff like movement and animations
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(spawn_ground_sensor)
                .with_system(spawn_wall_sensor)
                .with_system(update_movement)
                .with_system(ground_detection)
                .with_system(wall_detection)
        );
    }
}
