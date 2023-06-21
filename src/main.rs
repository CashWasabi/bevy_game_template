// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_editor_pls::prelude::*;
use kaginawa::GamePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Kaginawa".to_string(),
                resolution: (800., 600.).into(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(GamePlugin)
        .add_plugin(EditorPlugin::default())
        .add_system(bevy::window::close_on_esc)
        .run();
}
