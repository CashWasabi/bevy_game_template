use bevy::prelude::*;

// Game Events
pub struct GameStartedEvent;
pub struct GameOverEvent;

// Control Events
pub struct JumpStartedEvent;
pub struct LandedEvent;

pub struct CrouchStartedEvent;
pub struct CrouchEndedEvent;

pub struct DashStartedEvent;
pub struct DashEndedEvent;

pub struct AttackEvent;
pub struct HitEvent;


pub struct InternalEventPlugin;

impl Plugin for InternalEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GameStartedEvent>()
            .add_event::<GameOverEvent>()

            .add_event::<JumpStartedEvent>()
            .add_event::<LandedEvent>()

            .add_event::<CrouchStartedEvent>()
            .add_event::<CrouchEndedEvent>()

            .add_event::<DashStartedEvent>()
            .add_event::<DashEndedEvent>()
        ;
    }
}
