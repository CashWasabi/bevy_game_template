pub mod components;
pub mod systems;

use crate::AppState;
use crate::game::GameState;

use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                systems::run_system,
                systems::crouch_system,
                systems::jump_system,
                systems::dash_system,
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(GameState::Running))
        )
        ;
    }
}
