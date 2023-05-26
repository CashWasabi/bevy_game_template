use benimator::FrameRate;
use bevy::prelude::*;

// Create the animation component
// Note: you may make the animation an asset instead of a component
#[derive(Clone, Component, Deref)]
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

