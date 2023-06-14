use bevy::prelude::*;
use crate::actions::components::Action;
// use crate::players::state_machine::CharacterController;
use crate::animations::components::{
    Animation,
    AnimationState,
};

pub struct GameOverEvent;
pub struct JumpStartedEvent;
pub struct LandedEvent;
pub struct CrouchStartedEvent;

pub struct DashStartedEvent;
pub struct DashEndedEvent;

pub struct AttackEvent;
pub struct HitEvent;

// action events
pub struct ActionEvent{
    pub entity: Entity,
    pub action: Action,
}

// player events
// pub struct CharacterControllerEvent{
//     pub entity: Entity,
//     pub state: CharacterController::State,
//     pub superstate: CharacterController::Superstate,
// }

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
            // .add_event::<CharacterControllerEvent>()
            .add_event::<AnimationEvent>()
        ;
    }
}
