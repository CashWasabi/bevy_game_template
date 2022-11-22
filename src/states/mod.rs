pub mod states;
use crate::states::states::PlayerStateMachine;

use crate::GameState;

use bevy::prelude::*;

use statig::{InitializedStatemachine, StateMachineContext};

#[derive(Component, Deref, DerefMut)]
pub struct StateMachineComponent(InitializedStatemachine<PlayerStateMachine>);

impl Default for StateMachineComponent {
    fn default() -> Self {
        Self(PlayerStateMachine::default().state_machine().init())
    }
}

impl Clone for StateMachineComponent {
    fn clone(&self) -> Self {
        let context = *self.0;
        Self(context.clone().state_machine().init())
    }
}

pub fn debug_state_machine() {}
