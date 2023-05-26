pub mod state_machine;
pub mod components;
pub mod systems;

use crate::GameState;

use bevy::prelude::*;
use seldom_state::StateMachinePlugin;


pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement and animations
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(StateMachinePlugin)
            .add_systems(
                (
                    systems::update_player_animation,
                    systems::update_player_state,
                    systems::update_player_movement,
                ).in_set(OnUpdate(GameState::Playing))
            )
        ;
    }
}
