use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use crate::loading::AudioAssets;

#[derive(Resource)]
pub struct FlyingAudio(pub Handle<AudioInstance>);

impl FlyingAudio {
    pub fn new(
        audio: Res<Audio>,
        audio_assets: Res<AudioAssets>,
    ) -> Self {
        let handle = audio
            .play(audio_assets.flying.clone())
            .looped()
            .with_volume(0.3)
            .handle();
        FlyingAudio (handle)
    }
}
