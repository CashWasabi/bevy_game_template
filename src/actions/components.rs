use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::actions::game_control::{Action, default_input_map};
use crate::states::DasherStateMachine;

#[derive(Component, Default, Clone)]
pub struct DasherController;

#[derive(Bundle)]
pub struct DasherControllerBundle {
    pub controller: DasherController,
    pub state_machine: DasherStateMachine,

    #[bundle]
    pub input_manager: InputManagerBundle<Action>,
}


impl Default for DasherControllerBundle {
    fn default() -> Self {
        let input_map = default_input_map();
        Self {
            controller: DasherController::default(),
            state_machine: DasherStateMachine::default(),
            input_manager: InputManagerBundle {
                input_map: input_map,
                ..default()
            },
        }
    }
}
