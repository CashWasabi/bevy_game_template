pub mod components;
pub mod game_control;
pub mod systems;

use crate::GameState;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<game_control::Action>::default())
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(systems::update_dasher_controller),
            );
    }
}
