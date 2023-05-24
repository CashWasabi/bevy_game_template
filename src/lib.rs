// pub mod actions;
pub mod animations;
pub mod audio;
pub mod levels;
pub mod loading;
pub mod menu;
pub mod physics;
pub mod players;

// use crate::actions::ActionsPlugin;
use crate::animations::InternalAnimationPlugin;
use crate::audio::InternalAudioPlugin;
use crate::levels::LevelPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::physics::PhysicsPlugin;
use crate::players::PlayerPlugin;

use bevy::app::App;
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
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
            // .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PhysicsPlugin)
            .add_plugin(LevelPlugin)
            .add_plugin(InternalAnimationPlugin)
            .add_plugin(PlayerPlugin)
            ;
    }
}
