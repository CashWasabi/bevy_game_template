use crate::actions::game_control::Action;
use crate::actions::components::DasherController;
use crate::states::DasherStateMachine;
use crate::states::dasher::Event as DasherEvent;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;


pub fn update_dasher_controller(
    mut query: Query<(&ActionState<Action>, &mut DasherStateMachine), With<DasherController>>,
) {
    for (action_state, mut state_machine) in &mut query {
        if action_state.pressed(Action::Dash) {
            state_machine.handle(&DasherEvent::DashPressed);
        } 
        else if action_state.pressed(Action::Left) {
            state_machine.handle(&DasherEvent::WalkPressed);
        }
        else if action_state.pressed(Action::Right) {
            state_machine.handle(&DasherEvent::WalkPressed);
        }
    }
}
