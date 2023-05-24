// use crate::actions::game_control::Action;
use crate::players::components::PlayerDirection;

use benimator::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

// Create the animation component
// Note: you may make the animation an asset instead of a component
#[derive(Component, Deref, Clone)]
pub struct Animation(pub benimator::Animation);
impl Default for Animation {
    fn default() -> Self {
        Animation(benimator::Animation::from_indices(
            0..=1,
            FrameRate::from_fps(6.0),
        ))
    }
}

// Create the player component
#[derive(Component, Deref, DerefMut)]
pub struct AnimationState(benimator::State);
impl Default for AnimationState {
    fn default() -> Self {
        AnimationState(benimator::State::default())
    }
}

// TODO(MO): Can we do this somehow else?
impl Clone for AnimationState {
    fn clone(&self) -> Self {
        AnimationState(benimator::State::default())
    }
}

// Create a resource to store handles of the animations
#[derive(Component, Clone)]
pub struct PlayerAnimations {
    pub idle: Animation,
    pub run: Animation,
    pub jump: Animation,
    pub dash: Animation,
    pub fall: Animation,
    pub crouch: Animation,
}

impl Default for PlayerAnimations {
    fn default() -> Self {
        let frame_rate = 6.0;
        PlayerAnimations {
            idle: Animation(benimator::Animation::from_indices(
                0..=3,
                FrameRate::from_fps(frame_rate),
            )),
            run: Animation(benimator::Animation::from_indices(
                8..=13,
                FrameRate::from_fps(frame_rate),
            )),
            jump: Animation(benimator::Animation::from_indices(
                14..=23,
                FrameRate::from_fps(frame_rate),
            )),
            dash: Animation(benimator::Animation::from_indices(
                24..=28,
                FrameRate::from_fps(frame_rate),
            )),
            fall: Animation(benimator::Animation::from_indices(
                22..=23,
                FrameRate::from_fps(frame_rate),
            )),
            crouch: Animation(benimator::Animation::from_indices(
                4..=7,
                FrameRate::from_fps(frame_rate),
            )),
        }
    }
}

pub fn animate(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut TextureAtlasSprite, &Animation)>,
) {
    for (mut player, mut texture, animation) in query.iter_mut() {
        // Update the state
        player.update(animation, time.delta());

        // Update the texture atlas
        texture.index = player.frame_index();
    }
}

pub fn flip_sprites(
    mut query: Query<(
        // &ActionState<Action>,
        &mut PlayerDirection,
        &mut TextureAtlasSprite,
    )>,
) {
    for (
        // _action_state,
        mut direction,
        mut sprite
    ) in query.iter_mut() {
        // TODO(MO): fix stuff
        // let dir = action_state.direction().unwrap_or(Vec2::ZERO);
        let dir = Vec2::ZERO;
        if dir.x != 0. {
            direction.0 = dir.x;

            if sprite.flip_x && direction.0 > 0. {
                sprite.flip_x = false;
            } else if !sprite.flip_x && direction.0 < 0. {
                sprite.flip_x = true;
            }
        }
    }
}

// pub fn update_player_animation(mut query: Query<(&PlayerAnimations, &mut Animation, &DasherStateMachine)>) {
//     for (player_animations, mut animation, state_machine) in query.iter_mut() {
//         // TODO(MO): Fix this! use StateMachine States now!
//         let new_animation = match state_machine.state() {
//             DasherState::Idle{..} => player_animations.idle.clone(),
//             DasherState::Walk{..} => player_animations.run.clone(),
//             DasherState::Dash{..} => player_animations.dash.clone(),
//             DasherState::Fall{..} => player_animations.fall.clone(),
//         };
//
//         *animation = new_animation;
//     }
// }
