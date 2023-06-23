use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::game::movements::components::{
    JumpMovement,
    RunMovement,
    DashMovement,
    CrouchMovement,
};

use crate::game::actions::components::Action;
use crate::game::physics::components::GroundDetection;

pub fn run_system(
    mut query: Query<(
        &ActionState<Action>,
        &GroundDetection,
        // &WallDetection,
        &mut RunMovement,
        // &mut ExternalForce,
        &mut Velocity,
    )>,
) {
    for (
        action_state,
        ground_detection,
        // wall_detection,
        mut run_movement,
        // external_force,
        mut velocity,
    ) in &mut query
    {
        if !ground_detection.0 {
            run_movement.can_run = false;
            run_movement.is_running = false;
            continue;
        }

        // we can run since we're grounded!
        run_movement.can_run = true;

        let axis_data = match action_state.axis_pair(Action::Move) {
            Some(axis_data) => axis_data.xy(),
            None => Vec2::ZERO,
        };

        if axis_data.x != 0. {
            run_movement.is_running = true;
            velocity.linvel.x = run_movement.speed;
        }
    }
}

pub fn jump_system(
    mut query: Query<(
        &ActionState<Action>,
        &GroundDetection,
        // &WallDetection,
        &mut JumpMovement,
        // &mut ExternalForce,
        &mut Velocity,
    )>,
) {
    for (
        action_state,
        ground_detection,
        // wall_detection,
        mut jump_movement,
        // external_force,
        mut velocity,
    ) in &mut query
    {
        if ground_detection.0 {
            jump_movement.jump_count = 0;
            jump_movement.can_jump = true;
            jump_movement.is_jumping = false;
        } else {
            if jump_movement.jump_count >= 2 {
                jump_movement.can_jump = false;
                jump_movement.is_jumping = false;
                continue;
            }
            jump_movement.can_jump = true;
        }

        if action_state.just_pressed(Action::Jump) && jump_movement.can_jump {
            jump_movement.is_jumping = true;
            jump_movement.jump_count += 1;
            velocity.linvel.y = jump_movement.force;
        }
    }
}

pub fn dash_system(
    time: Res<Time>,
    mut query: Query<(
        &ActionState<Action>,
        &mut DashMovement,
        &mut Velocity,
    )>,
) {
    for (
        action_state,
        mut dash_movement,
        mut velocity,
    ) in &mut query
    {
        if dash_movement.cooldown > 0.0 && !dash_movement.can_dash {
            // TODO(MO): Use real timer Resource here
            dash_movement.cooldown -= time.delta_seconds();
            continue;
        } else {
            dash_movement.can_dash = true;
            dash_movement.cooldown = 5.0;
        }

        if action_state.just_pressed(Action::Dash) {
            dash_movement.is_dashing = true;
            velocity.linvel.y = dash_movement.force;
        }
    }
}


pub fn crouch_system(
    mut query: Query<(
        &ActionState<Action>,
        &mut CrouchMovement,
        &mut Velocity,
    )>,
) {
    for (
        action_state,
        mut crouch_movement,
        mut velocity,
    ) in &mut query
    {
        if !crouch_movement.can_crouch {
            continue;
        }

        if action_state.pressed(Action::Crouch) {
            crouch_movement.is_crouching = true;
            velocity.linvel.x *= crouch_movement.speed_reduction;
        } else {
            crouch_movement.is_crouching = false;
        }
    }
}
