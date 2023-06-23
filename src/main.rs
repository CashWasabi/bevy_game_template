pub mod loading;
pub mod events;
pub mod configs;
mod game;
mod main_menu;
mod systems;

use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_editor_pls::prelude::EditorPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use game::GamePlugin;
use loading::LoadingPlugin;
use main_menu::MainMenuPlugin;
use events::InternalEventPlugin;

use systems::{
    spawn_camera,
    transition_to_game_state,
    transition_to_main_menu_state,
    handle_game_over,
};

fn main() {
    App::new()
        .add_state::<AppState>() .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        // Plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: configs::GAME_TITLE.to_string(),
                resolution: (800., 600.).into(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(LoadingPlugin)
        .add_plugin(InternalEventPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(DebugLinesPlugin::default())
        // .add_plugin(EditorPlugin::default())

        // Startup Systems
        .add_startup_system(spawn_camera)

        // Systems
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(handle_game_over)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    Game,
    GameOver,
}
