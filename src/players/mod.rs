pub mod components;
pub mod systems;

use crate::GameState;

use bevy::prelude::*;


pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement and animations
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(systems::update_jump_buffer)
                .with_system(systems::update_coyote_time)
                .with_system(systems::dash_cooldown)
                // .with_system(update_player),
        );
    }
}
