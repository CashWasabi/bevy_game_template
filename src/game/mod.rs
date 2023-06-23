pub mod animations;
pub mod audio;
pub mod ldtk;
pub mod physics;
pub mod players;
pub mod actions;
pub mod movements;
pub mod particles;

use crate::game::animations::InternalAnimationPlugin;
use crate::game::audio::InternalAudioPlugin;
use crate::game::ldtk::LdtkImportPlugin;
use crate::game::physics::PhysicsPlugin;
use crate::game::players::PlayerPlugin;
use crate::game::actions::ActionsPlugin;
use crate::game::movements::MovementPlugin;
use crate::game::particles::ParticlesPlugin;

use bevy::app::App;
use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(InternalAudioPlugin)
            .add_plugin(InternalAnimationPlugin)
            .add_plugin(PhysicsPlugin)
            .add_plugin(ParticlesPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(LdtkImportPlugin)
            ;
    }
}
