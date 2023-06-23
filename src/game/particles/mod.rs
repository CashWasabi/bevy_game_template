pub mod components;
pub mod systems;

use bevy::prelude::*;
use bevy_hanabi::*;


pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HanabiPlugin)
        ;
    }
}

