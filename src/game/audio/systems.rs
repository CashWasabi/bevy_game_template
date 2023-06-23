use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
// use leafwing_input_manager::prelude::*;
// use crate::game::actions::components::Action;
use crate::loading::AudioAssets;
use crate::game::audio::components::{
    FlyingAudio,
    // WalkingAudio,
    // RunningAudio,
    // JumpingAudio,
    // DuckingAudio,
    // CrouchingAudio,
    // LandingAudio,
    // AirAttackAudio,
    // GroundAttackAudio,
};

pub fn start_audio(
    mut commands: Commands,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    audio.pause();
    commands.insert_resource(
        FlyingAudio::new(audio, audio_assets)
    );
}

// TODO(MO): use events to start and stop sound!
pub fn _control_flying_sound(
    // actions: ActionState<Action>,
    audio: Res<FlyingAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&audio.0) {
        match instance.state() {
            PlaybackState::Paused { .. } => {
                // if actions.player_movement.is_some() {
                //     instance.resume(AudioTween::default());
                // }
            }
            PlaybackState::Playing { .. } => {
                // if actions.player_movement.is_none() {
                //     instance.pause(AudioTween::default());
                // }
            }
            _ => {}
        }
    }
}
