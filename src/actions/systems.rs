use crate::actions::components::{CandleController, DasherController};
use crate::actions::game_control::Action;
use crate::states::{CandleStateMachine, DasherStateMachine};
use crate::states::candle::Event as CandleEvent;
use crate::states::dasher::Event as DasherEvent;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn update_candle_controller(
    mut query: Query<(&ActionState<Action>, &mut CandleStateMachine), With<CandleController>>,
) {
    for (action_state, mut state_machine) in &mut query {
        if action_state.pressed(Action::MindMeld) {
            state_machine.handle(&CandleEvent::MindMeld);
        }
    }
}

pub fn update_dasher_controller(
    mut query: Query<(&ActionState<Action>, &mut DasherStateMachine), With<DasherController>>,
) {
    for (action_state, mut state_machine) in &mut query {
        if action_state.pressed(Action::MindMeld) {
            state_machine.handle(&DasherEvent::MindMeld);
        } else if action_state.pressed(Action::Run) {
            state_machine.handle(&DasherEvent::Run);
        } else if action_state.pressed(Action::PrimaryAbility) {
            state_machine.handle(&DasherEvent::Dash);
        }
    }
}
