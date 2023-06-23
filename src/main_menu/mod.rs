use bevy::prelude::*;

pub mod styles;
pub mod components;
pub mod systems;

use crate::AppState;
use systems::interactions::{
    interact_with_play_button,
    interact_with_quit_button
};
use systems::layouts::{
    spawn_main_menu,
    despawn_main_menu,
};


pub struct MainMenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_systems(
                (
                    interact_with_play_button,
                    interact_with_quit_button
                ).in_set(OnUpdate(AppState::MainMenu))
            )
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}
