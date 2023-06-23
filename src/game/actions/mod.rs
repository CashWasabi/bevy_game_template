pub mod components;
pub mod systems;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<components::Action>::default())
        ;
    }
}
