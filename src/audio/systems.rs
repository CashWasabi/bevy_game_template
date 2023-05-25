use crate::actions::components::Action;
use crate::audio::components::FlyingAudio;
use crate::loading::AudioAssets;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn start_audio(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>
) {
    audio.pause();
    let handle = audio
        .play(audio_assets.flying.clone())
        .looped()
        .with_volume(0.3)
        .handle();
    commands.insert_resource(FlyingAudio(handle));
}

pub fn control_flying_sound(
    actions: ActionState<Action>,
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
