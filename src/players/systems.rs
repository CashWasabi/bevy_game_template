use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::players::components::*;
use crate::actions::components::Action;
use crate::animations::components::*;
use crate::physics::components::{
    GroundDetection,
    WallDetection
};
use crate::players::state_machine::State as CharacterState;
use crate::players::state_machine::Event as CharacterEvent;


// TODO(MO): Should we directly link action_state to animations?
// Maybe only link by events
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
        let new_animation = match player_state_machine.0.state() {
            CharacterState::Walk => player_animations.run.clone(),
            CharacterState::Run => player_animations.run.clone(),
            CharacterState::Dash => player_animations.dash.clone(),
            CharacterState::Jump => player_animations.jump.clone(),
            CharacterState::Idle => player_animations.idle.clone(),
            CharacterState::Crouch => player_animations.crouch.clone(),
            CharacterState::GroundedAttack => player_animations.grounded_attack.clone(),
            CharacterState::AirborneAttack => player_animations.airborne_attack.clone(),
            _ => player_animations.fall.clone(),
        };

        *animation = new_animation;
    }
}


pub fn update_player_state(
    mut _commands: Commands,
    mut query: Query<(
        Entity,
        &ActionState<Action>,
        &GroundDetection,
        &WallDetection,
        &mut GravityScale,
        &mut PlayerStateMachine,
    )>,
) {
    for (
        _entity,
        action_state,
        ground_detection,
        _wall_detection,
        _gravity_scale,
        mut player_state_machine,
    ) in &mut query {
        if action_state.pressed(Action::Run)  {
            println!("RUN PRESSED!");
            player_state_machine.0.handle(&CharacterEvent::Run);
            return;
        }

        if action_state.pressed(Action::Crouch)  {
            println!("CROUCH PRESSED!");
            player_state_machine.0.handle(&CharacterEvent::Crouch);
            return;
        }

        if action_state.pressed(Action::Jump)  {
            println!("JUMP PRESSED!");
            player_state_machine.0.handle(&CharacterEvent::Jump);
            return;
        }

        if action_state.pressed(Action::Dash)  {
            println!("JUMP PRESSED!");
            player_state_machine.0.handle(&CharacterEvent::Dash);
            return;
        }

        if action_state.pressed(Action::Attack)  {
            println!("JUMP PRESSED!");
            player_state_machine.0.handle(&CharacterEvent::Attack);
            return;
        }

        if action_state.pressed(Action::Attack)  {
            println!("ATTACK PRESSED!");
            if ground_detection.0 {
                println!("GROUND ATTACK!");
                player_state_machine.0.handle(&CharacterEvent::Attack);

            } else {
                println!("AIRBORNE ATTACK!");
                player_state_machine.0.handle(&CharacterEvent::Attack);
            }
            return;
        }

        if action_state.pressed(Action::Move)  {
            println!("MOVE PRESSED!");
            player_state_machine.0.handle(&CharacterEvent::Move);
            return;
        }

        if ground_detection.0 {
            println!("IDLING!");
            player_state_machine.0.handle(&CharacterEvent::Grounded);

        } else {
            println!("FALLING!");
            player_state_machine.0.handle(&CharacterEvent::Fall);
        }

    }
}

pub fn update_player_movement(
    mut _commands: Commands,
    mut query: Query<(
        Entity,
        &ActionState<Action>,
        &mut ExternalForce,
        &mut Velocity,
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
        player_direction,
        player_state_machine,
    ) in &mut query {
        let axis_data = match action_state.axis_pair(Action::Move) {
            Some(axis_data) => axis_data.xy(),
            None => Vec2::ZERO,
        };
        println!("{}", axis_data);
        // TODO(MO): axis_data is only relevant when pressed
        // For dash and jump and others we want to do something else
        // velocity.linvel.x = axis_data.x * player_state_machine.0.speed.x;
        velocity.linvel.x = player_direction.0 * player_state_machine.0.speed.x;
    }
}
