pub mod components;
pub mod systems;

use crate::GameState;

use bevy::prelude::*;


pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement and animations
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            systems::update_player.in_set(OnUpdate(GameState::Playing))
        )
        // .add_system(
        //     systems::update_jump_buffer.in_set(OnUpdate(GameState::Playing))
        // )
        // .add_system(
        //     systems::update_coyote_time.in_set(OnUpdate(GameState::Playing))
        // )
        // .add_system(
        //     systems::dash_cooldown.in_set(OnUpdate(GameState::Playing))
        // )
        ;
    }
}
