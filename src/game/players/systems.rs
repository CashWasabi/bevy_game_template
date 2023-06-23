use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use statig::prelude::IntoStateMachine;
use leafwing_input_manager::prelude::ActionState;

use crate::game::players::components::*;
use crate::game::actions::components::Action;
use crate::game::animations::components::*;
use crate::game::physics::components::{
    GroundDetection,
    WallDetection
};
use crate::game::players::state_machine::CharacterController;
use crate::game::players::state_machine::Event as CharacterEvent;


pub fn flip_sprites(
    mut query: Query<(
        &ActionState<Action>,
        &mut PlayerDirection,
        &mut TextureAtlasSprite,
    )>,
) {
    for (
        action_state,
        mut direction,
        mut sprite,
    ) in query.iter_mut()
    {
        let axis_data = match action_state.axis_pair(Action::Move) {
            Some(axis_data) => axis_data.xy(),
            None => Vec2::ZERO,
        };

        if axis_data.x != 0. {
            direction.0 = axis_data.x;

            if sprite.flip_x && direction.0 > 0. {
                sprite.flip_x = false;
            } else if !sprite.flip_x && direction.0 < 0. {
                sprite.flip_x = true;
            }
        }
    }
}

pub fn update_player_animation(
    mut query: Query<(&PlayerStateMachine, &mut Animation, &PlayerAnimations)>
) {
    for (
        player_state_machine,
        mut animation,
        player_animations
    ) in &mut query {
        type CharacterState = <CharacterController as IntoStateMachine>::State;

        let new_animation = match player_state_machine.0.state() {
            CharacterState::Walk{ .. } => player_animations.run.clone(),
            CharacterState::Run{ .. } => player_animations.run.clone(),
            CharacterState::Dash{ .. } => player_animations.dash.clone(),
            CharacterState::Jump{ .. } => player_animations.jump.clone(),
            CharacterState::Idle{ .. } => player_animations.idle.clone(),
            CharacterState::Crouch{ .. } => player_animations.crouch.clone(),
            CharacterState::GroundedAttack{ .. } => player_animations.grounded_attack.clone(),
            CharacterState::AirborneAttack{ .. } => player_animations.airborne_attack.clone(),
            _ => player_animations.fall.clone(),
        };

        *animation = new_animation;
    }
}


pub fn update_player_state(
    mut _commands: Commands,
    mut query: Query<(
        &ActionState<Action>,
        &GroundDetection,
        &WallDetection,
        &mut GravityScale,
        &mut PlayerStateMachine,
        &mut CharacterControllerExternalContext,
    )>,
) {
    for (
        action_state,
        ground_detection,
        wall_detection,
        _gravity_scale,
        mut player_state_machine,
        mut character_controller_external_context,
    ) in &mut query {
        type CharacterState = <CharacterController as IntoStateMachine>::State;

        // update our current situation
        character_controller_external_context.0.ground_detected = ground_detection.0;
        character_controller_external_context.0.wall_detected = wall_detection.0;

        // update on key inputs
        if action_state.just_pressed(Action::Run)  {
            player_state_machine.0.handle_with_context(
                &CharacterEvent::StartRunning,
                &mut character_controller_external_context.0
            );
            return;
        }
        if action_state.just_released(Action::Run)  {
            player_state_machine.0.handle_with_context(
                &CharacterEvent::StopRunning,
                &mut character_controller_external_context.0
            );
            return;
        }

        if action_state.just_pressed(Action::Crouch)  {
            player_state_machine.0.handle_with_context(
                &CharacterEvent::StartCrouching,
                &mut character_controller_external_context.0
            );
            return;
        }
        if action_state.just_released(Action::Crouch)  {
            player_state_machine.0.handle_with_context(
                &CharacterEvent::StopCrouching,
                &mut character_controller_external_context.0
            );
            return;
        }

        if action_state.just_pressed(Action::Jump)  {
            player_state_machine.0.handle_with_context(
                &CharacterEvent::Jumping,
                &mut character_controller_external_context.0
            );
            return;
        }

        if action_state.just_pressed(Action::Dash)  {
            player_state_machine.0.handle_with_context(
                &CharacterEvent::StartDashing,
                &mut character_controller_external_context.0
            );
            return;
        } else if action_state.just_released(Action::Dash) {
            player_state_machine.0.handle_with_context(
                &CharacterEvent::StopDashing,
                &mut character_controller_external_context.0
            );
            return;
        }

        if action_state.just_pressed(Action::Attack)  {
            player_state_machine.0.handle_with_context(
                &CharacterEvent::StartAttacking,
                &mut character_controller_external_context.0
            );
            return;
        }

        if action_state.just_released(Action::Attack)  {
            player_state_machine.0.handle_with_context(
                &CharacterEvent::StopAttacking,
                &mut character_controller_external_context.0
            );
            return;
        }

        if action_state.pressed(Action::Move)  {
            match player_state_machine.0.state() {
                CharacterState::Walk{ .. } => (),
                _ => {
                    player_state_machine.0.handle_with_context(
                        &CharacterEvent::StartWalking,
                        &mut character_controller_external_context.0
                    );
                }
            };
            return;
        }

        if action_state.just_released(Action::Move)  {
            player_state_machine.0.handle_with_context(
                &CharacterEvent::StopWalking,
                &mut character_controller_external_context.0
            );
            return;
        }

        player_state_machine.0.handle_with_context(
            &CharacterEvent::Empty,
            &mut character_controller_external_context.0
        );
    }
}

pub fn update_player_movement(
    mut _commands: Commands,
    mut query: Query<(
        Entity,
        &ActionState<Action>,
        &mut ExternalForce,
        &mut Velocity,
        &GroundDetection,
        &WallDetection,
        &mut GravityScale,
        &PlayerDirection,
        &PlayerStateMachine,
    )>,
    _time: Res<Time>,
) {
    for (
        _entity,
        action_state,
        mut _external_force,
        mut velocity,
        mut _gravity_scale,
        _ground_detection,
        _wall_detection,
        player_direction,
        player_state_machine,
    ) in &mut query {
        let axis_data = match action_state.axis_pair(Action::Move) {
            Some(axis_data) => axis_data.xy(),
            None => Vec2::ZERO,
        };
        let y_force = player_state_machine.0.speed.y;
        if y_force != 0.0 {
            velocity.linvel.y = player_state_machine.0.speed.y;
            velocity.linvel.x = axis_data.x * player_state_machine.0.speed.x;
        } else {
            velocity.linvel.x = player_direction.0 * player_state_machine.0.speed.x;
        }
    }
}
