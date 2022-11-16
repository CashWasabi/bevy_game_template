pub mod animation;

use bevy::prelude::*;

use crate::GameState;

pub struct InternalAnimationPlugin;

// This plugin manages every animation in the game
impl Plugin for InternalAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(animation::animate)
                .with_system(animation::update_player_animation)
                .with_system(animation::flip_sprites),
        );
    }
}
