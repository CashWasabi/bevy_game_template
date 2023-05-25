use crate::events::CharacterControllerEvent;
use crate::actions::components::Action;
use crate::players::components::{
    PlayerDirection,
    PlayerStateMachine,
};
use crate::animations::components::{
    Animation,
    AnimationState,
    PlayerAnimations,
};

use bevy::prelude::*;


pub fn animate(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut TextureAtlasSprite, &Animation)>,
) {
    for (mut player, mut texture, animation) in &mut query {
        // Update the state
        player.update(animation, time.delta());

        // Update the texture atlas
        texture.index = player.frame_index();
    }
}


// TODO(MO): Should we directly link action_state to animations?
// Maybe only link by events
// pub fn flip_sprites(
//     mut query: Query<(
//         &ActionState<Action>,
//         &mut PlayerDirection,
//         &mut TextureAtlasSprite,
//     )>,
// ) {
//     for (
//         action_state,
//         mut direction,
//         mut sprite,
//     ) in query.iter_mut()
//     {
//         TODO(MO): fix stuff
//         let axis_pair = action_state
//             .clamped_axis_pair(Action::Move)
//             .unwrap_or(Vec2::ZERO);
//
//         if axis_pair.x() != 0. {
//             direction.0 = axis_pair.x();
//
//             if sprite.flip_x && direction.0 > 0. {
//                 sprite.flip_x = false;
//             } else if !sprite.flip_x && direction.0 < 0. {
//                 sprite.flip_x = true;
//             }
//         }
//     }
// }

pub fn update_player_animation(
    mut events: EventReader<CharacterControllerEvent>,
    mut query: Query<(&mut Animation, &PlayerAnimations)>
) {
    for _event in events.iter() {
        // if let (mut animation, player_animations) = query.single_mut() {
        //     let new_animation = match event.action {
        //         event.action::Run => player_animations.run.clone(),
        //         event.action::Dash => player_animations.dash.clone(),
        //         event.action::Jump => player_animations.jump.clone(),
        //         _ => player_animations.idle.clone(),
        //     };
        //     *animation = new_animation;
        // }
    }
}
