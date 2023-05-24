pub mod animation;

use bevy::prelude::*;

use crate::GameState;

pub struct InternalAnimationPlugin;

// This plugin manages every animation in the game
impl Plugin for InternalAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                animation::animate,
                animation::flip_sprites,
                // animation::update_player_animation,
            ).in_set(OnUpdate(GameState::Playing))
        )
        ;
    }
}
