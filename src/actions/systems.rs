use crate::physics::components::GroundDetection;
use crate::actions::game_control::Action;
use crate::players::components::PlayerDirection;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;


// pub fn update_dasher_controller(
//     mut query: Query<(&ActionState<Action>, &mut DasherStateMachine, &GroundDetection, &mut PlayerDirection), With<DasherController>>,
// ) {
//     for (action_state, mut state_machine, is_grounded, mut direction) in &mut query {
//
//         if is_grounded.0 {
//             state_machine.handle(&DasherEvent::Grounded);
//         } else {
//             state_machine.handle(&DasherEvent::Airborne);
//         }
//
//         if action_state.pressed(Action::Left) && !action_state.pressed(Action::Right) {
//             direction.0 = -1.0;
//             state_machine.handle(&DasherEvent::WalkPressed);
//         }
//         else if  !action_state.pressed(Action::Left) && action_state.pressed(Action::Right) {
//             direction.0 = 1.0;
//             state_machine.handle(&DasherEvent::WalkPressed);
//         }
//
//         if action_state.released(Action::Left) && action_state.released(Action::Right) {
//             state_machine.handle(&DasherEvent::WalkReleased);
//         }
//         else if  action_state.released(Action::Left) && action_state.released(Action::Right) {
//             state_machine.handle(&DasherEvent::WalkReleased);
//         }
//
//         if action_state.pressed(Action::Dash) {
//             state_machine.handle(&DasherEvent::DashPressed);
//         }
//         if action_state.released(Action::Dash) {
//             state_machine.handle(&DasherEvent::DashReleased);
//         }
//     }
// }
