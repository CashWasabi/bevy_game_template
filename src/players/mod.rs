pub mod components;
pub mod systems;

use bevy::prelude::*;

use crate::players::systems::*;
use crate::GameState;

pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement and animations
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(spawn_ground_sensor.before("last"))
                .with_system(spawn_wall_sensor.before("last"))
                .with_system(ground_detection.before("last"))
                .with_system(wall_detection.before("last"))
                .with_system(update_jump_buffer.before("last"))
                .with_system(update_coyote_time.before("last"))
                .with_system(dash_cooldown.before("last"))
                .with_system(update_player.label("last")),
        );
    }
}
