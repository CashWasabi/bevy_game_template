use bevy::prelude::*;
use crate::actions::components::Action;
use crate::players::state_machine::{
    State as CharacterState,
    Superstate as CharacterSuperstate
};
use crate::animations::components::{
    Animation,
    AnimationState,
};

// action events
pub struct ActionEvent{
    pub entity: Entity,
    pub action: Action,
}

// player events
pub struct CharacterControllerEvent{
    pub entity: Entity,
    pub state: CharacterState,
    pub superstate: CharacterSuperstate,
}

// animation events
pub struct AnimationEvent{
    pub entity: Entity,
    pub animation: Animation,
    pub animation_state: AnimationState,
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
