use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::players::components::*;

pub fn update_player(
    mut _commands: Commands,
    mut query: Query<(
        Entity,
        &mut ExternalForce,
        &mut Velocity,
        &mut GravityScale,
        &PlayerDirection,
        &PlayerStateMachine,
    )>,
) {
    for (
        _entity,
        mut _external_force,
        mut velocity,
        mut _gravity_scale,
        direction,
        _state_machine,
    ) in &mut query {
        // TODO(MO): Fix this!
        // velocity.linvel.x = state_machine.velocity.x * direction.0;
        velocity.linvel.x = 50.0 * direction.0;
    }
}


// // TODO(MO): Add input buffer in general to inputs
// pub fn update_jump_buffer(time: Res<Time>, mut query: Query<&mut JumpBufferTimer>) {
//     for mut timer in &mut query {
//         timer.tick(time.delta());
//
//         // TODO(MO): Fix this!
//         // if timer.finished() {
//         //     (*state_machine).jump_buffer_active = false;
//         // } else if !(*state_machine).jump_buffer_active {
//         //     (*state_machine).jump_buffer_active = true;
//         // }
//     }
// }
//
// // TODO(MO): Add input buffer in general to inputs
// pub fn update_coyote_time(time: Res<Time>, mut query: Query<&mut CoyoteTimer>) {
//     for mut timer in &mut query {
//         timer.tick(time.delta());
//
//         // if timer.finished() {
//         //     (*state_machine).coyote_time_active = false;
//         // } else if !(*state_machine).coyote_time_active {
//         //     (*state_machine).coyote_time_active = true;
//         // }
//     }
// }
//
// // TODO(MO): Add skill cooldown in general! There might be a leafwing lib that does this already.
// pub fn dash_cooldown(time: Res<Time>, mut query: Query<&mut DashTimer>) {
//     for mut timer in &mut query {
//         timer.tick(time.delta());
//         // (*state_machine).dash_active = if timer.finished() { false } else { true };
//     }
// }
