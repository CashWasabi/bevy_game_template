use crate::game::animations::components::{
    Animation,
    AnimationState,
};

use bevy::prelude::*;


// TODO(MO): Use Events to start and stop animations!
pub fn animate(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut TextureAtlasSprite, &Animation)>,
) {
    for (mut animation_state, mut texture, animation) in &mut query {
        // Update the state
        animation_state.update(animation, time.delta());

        // Update the texture atlas
        texture.index = animation_state.frame_index();
    }
}
