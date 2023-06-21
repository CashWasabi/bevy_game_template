use bevy::prelude::*;

pub mod styles;
pub mod components;
pub mod systems;

use crate::GameState;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(systems::spawn_main_menu.in_schedule(OnEnter(GameState::Menu)))
            .add_systems(
                (
                    systems::click_play_button,
                    systems::click_exit_button
                ).in_set(OnUpdate(GameState::Menu))
            )
            .add_system(systems::despawn_main_menu.in_schedule(OnExit(GameState::Menu)));
    }
}
