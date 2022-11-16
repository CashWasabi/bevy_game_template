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
                .with_system(spawn_ground_sensor.label("a"))
                .with_system(spawn_wall_sensor.label("a"))
                .with_system(ground_detection.label("a"))
                .with_system(wall_detection.label("a"))
                .with_system(update_jump_buffer.label("a"))
                .with_system(update_coyote_time.label("a"))
                .with_system(dash_cooldown.label("a"))
                .with_system(update_player.label("b").after("a")),
        );
    }
}

