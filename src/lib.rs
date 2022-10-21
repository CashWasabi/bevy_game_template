pub mod actions;
pub mod audio;
pub mod loading;
pub mod menu;
pub mod levels;
pub mod enemy;
pub mod animations;
pub mod players;
pub mod physics;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::physics::physic::PhysicsPlugin;
use crate::levels::level::LevelPlugin;
use crate::animations::animation::InternalAnimationPlugin;
use crate::enemy::EnemyPlugin;
use crate::players::player::PlayerPlugin;
use crate::players::components::PlayerData;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};

use bevy::app::App;
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

#[derive(Inspectable, Default)]
pub struct InspectorData {
    #[inspectable(collapse)]
    player_data: PlayerData
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PhysicsPlugin)
            .add_plugin(LevelPlugin)
            .add_plugin(InternalAnimationPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(InspectorPlugin::<InspectorData>::new())
            ;
    }
}
