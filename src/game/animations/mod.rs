pub mod systems;
pub mod components;

use bevy::prelude::*;

use crate::AppState;
use crate::game::GameState;

pub struct InternalAnimationPlugin;

// This plugin manages every animation in the game
impl Plugin for InternalAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                systems::animate,
                // systems::flip_sprites,
                // systems::update_player_animation,
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(GameState::Running))
        )
        ;
    }
}