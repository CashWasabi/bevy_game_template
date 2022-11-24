pub mod player;
pub mod candle;
pub mod dasher;

use crate::states::player::Player;
use crate::states::candle::Candle;
use crate::states::dasher::Dasher;

use bevy::prelude::*;

use statig::{InitializedStatemachine, StateMachineContext};

// Player
#[derive(Component, Deref, DerefMut)]
pub struct PlayerStateMachine(InitializedStatemachine<Player>);

impl Default for PlayerStateMachine {
    fn default() -> Self {
        Self(Player::default().state_machine().init())
    }
}

impl Clone for PlayerStateMachine {
    fn clone(&self) -> Self {
        let context = *self.0;
        Self(context.clone().state_machine().init())
    }
}

// Candle
#[derive(Component, Deref, DerefMut)]
pub struct CandleStateMachine(InitializedStatemachine<Candle>);

impl Default for CandleStateMachine {
    fn default() -> Self {
        Self(Candle::default().state_machine().init())
    }
}

impl Clone for CandleStateMachine {
    fn clone(&self) -> Self {
        let context = *self.0;
        Self(context.clone().state_machine().init())
    }
}

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
