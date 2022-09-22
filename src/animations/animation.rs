// New Way
use bevy::prelude::*;
use benimator::*;
use crate::actions::Actions;

use crate::{GameState};
use crate::player;

pub struct InternalAnimationPlugin;

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

// This plugin manages every animation in the game
impl Plugin for InternalAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(animate)
                .with_system(update_player_animation)
                .with_system(flip_sprites)
        )
        ;
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
    pub attack: Animation,
}

impl Default for PlayerAnimations {
    fn default() -> Self {
        PlayerAnimations {
            idle: Animation(benimator::Animation::from_indices(
                0..=3,
                FrameRate::from_fps(12.0),
            )),
            run: Animation(benimator::Animation::from_indices(
                8..=13,
                FrameRate::from_fps(12.0),
            )),
            jump: Animation(benimator::Animation::from_indices(
                14..=23,
                FrameRate::from_fps(12.0),
            )),
            dash: Animation(benimator::Animation::from_indices(
                24..=28,
                FrameRate::from_fps(12.0),
            )),
            fall: Animation(benimator::Animation::from_indices(
                22..=23,
                FrameRate::from_fps(12.0),
            )),
            crouch: Animation(benimator::Animation::from_indices(
                4..=7,
                FrameRate::from_fps(12.0),
            )),
            attack: Animation(benimator::Animation::from_indices(
                55..=57,
                FrameRate::from_fps(12.0),
            )),
        }
    }
}

fn animate(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut TextureAtlasSprite, &Animation)>,
) {
    for (
        mut player, 
        mut texture, 
        animation
    ) in query.iter_mut() {
        // Update the state
        player.update(animation, time.delta());

        // Update the texture atlas
        texture.index = player.frame_index();
    };
}

fn flip_sprites(
    actions: Res<Actions>,
    mut query: Query<(&mut player::Direction, &mut TextureAtlasSprite)>
) {
    let dir = actions.player_movement.unwrap_or(Vec2::ZERO);
    for (mut direction, mut sprite) in query.iter_mut() {
        if dir.x != 0. {
            direction.orientation = dir.x;

            if sprite.flip_x && direction.orientation > 0. {
                sprite.flip_x = false;
            } else if !sprite.flip_x && direction.orientation < 0. {
                sprite.flip_x = true;
            }
        }
    }
}

fn update_player_animation(
    mut query: Query<(&player::PlayerState, &PlayerAnimations, &mut Animation)>,
) {
    for (
        player_state, 
        player_animations, 
        mut animation,
    ) in query.iter_mut() {
        let new_animation = match player_state {
            player::PlayerState::Idle => player_animations.idle.clone(),
            player::PlayerState::Move => player_animations.run.clone(),
            player::PlayerState::Jump => player_animations.jump.clone(),
            player::PlayerState::Fall => player_animations.fall.clone(),
            player::PlayerState::Crouch => player_animations.crouch.clone(),
            player::PlayerState::Attack => player_animations.attack.clone(),
            player::PlayerState::Dash => player_animations.dash.clone()
        };

        // TODO(MO): How do we compare them?
        // Does this do what we think it does?
        // if current_animation == new_animation {
        //     return;
        // }
        *animation = new_animation;
    }
}