pub mod platformer;
pub mod dasher;

use crate::states::dasher::Dasher;

use bevy::prelude::*;

use statig::InitializedStatemachine;
use statig::StateMachineSharedStorage;

// Dasher
#[derive(Component, Deref, DerefMut)]
pub struct DasherStateMachine(InitializedStatemachine<Dasher>);

impl Default for DasherStateMachine {
    fn default() -> Self {
        Self(Dasher::default().state_machine().init())
    }
}

impl Clone for DasherStateMachine {
    fn clone(&self) -> Self {
        let context = *self.0;
        Self(context.clone().state_machine().init())
    }
}
