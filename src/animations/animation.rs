use crate::actions::Actions;
use benimator::*;
use bevy::prelude::*;

use crate::players::components::PlayerDirection;

// Create the animation component
// Note: you may make the animation an asset instead of a component
#[derive(Component, Deref, Clone)]
pub struct Animation(pub benimator::Animation);
impl Default for Animation {
    fn default() -> Self {
        Animation(benimator::Animation::from_indices(
            0..=1,
            FrameRate::from_fps(12.0),
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
    actions: Res<Actions>,
    mut query: Query<(&mut PlayerDirection, &mut TextureAtlasSprite)>,
) {
    let dir = actions.movement.unwrap_or(Vec2::ZERO);
    for (mut direction, mut sprite) in query.iter_mut() {
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

pub fn update_player_animation(mut query: Query<(&PlayerAnimations, &mut Animation)>) {
    for (player_animations, mut animation) in query.iter_mut() {
        // TODO(MO): Fix this! Add events
        // let new_animation = match player_data.player_state {
        //     PlayerState::Idle => player_animations.idle.clone(),
        //     PlayerState::Move => player_animations.run.clone(),
        //     PlayerState::Jump => player_animations.jump.clone(),
        //     PlayerState::Fall => player_animations.fall.clone(),
        //     PlayerState::Crouch => player_animations.crouch.clone(),
        //     PlayerState::Dash => player_animations.dash.clone(),
        // };
        //
        // *animation = new_animation;
    }
}
