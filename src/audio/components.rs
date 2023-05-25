use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

#[derive(Resource)]
pub struct FlyingAudio(pub Handle<AudioInstance>);

// TODO(MO): Find sounds for:
// - walking
// - running
// - crouching
// - jumping
// - landing
// - attacking
// - dashing
