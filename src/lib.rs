pub mod configs;
pub mod animations;
pub mod audio;
pub mod ldtk;
pub mod loading;
pub mod main_menu;
pub mod physics;
pub mod players;
pub mod actions;
pub mod events;
pub mod movements;
// pub mod particles;

use crate::animations::InternalAnimationPlugin;
use crate::audio::InternalAudioPlugin;
use crate::events::InternalEventPlugin;
use crate::ldtk::LdtkImportPlugin;
use crate::loading::LoadingPlugin;
use crate::main_menu::MenuPlugin;
use crate::physics::PhysicsPlugin;
use crate::players::PlayerPlugin;
use crate::actions::ActionsPlugin;
use crate::movements::MovementPlugin;
// use crate::particles::ParticlesPlugin;

use bevy::app::App;
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(InternalEventPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(InternalAnimationPlugin)
            .add_plugin(PhysicsPlugin)
            .add_plugin(LdtkImportPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ActionsPlugin)
            ;
    }
}
