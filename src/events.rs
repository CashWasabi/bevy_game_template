use bevy::prelude::*;
use crate::actions::components::Action;
use crate::animations::components::{
    Animation,
    AnimationState,
};

// action events
pub struct ActionEvent{
    entity: Entity,
    action: Action,
}

// player events
pub struct CharacterControllerEvent{
    entity: Entity,
}

// animation events
pub struct AnimationEvent{
    entity: Entity,
    animation: Animation,
    animation_state: AnimationState,
}

pub struct InternalEventPlugin;

impl Plugin for InternalEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ActionEvent>()
            .add_event::<CharacterControllerEvent>()
            .add_event::<AnimationEvent>()
        ;
    }
}
