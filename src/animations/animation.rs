use std::time::Duration;
use crate::GameState;
use bevy::prelude::*;
use benimator::*;

pub  struct InternalAnimationPlugin;

impl Plugin for InternalAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Animations>()
        .add_plugin(AnimationPlugin::default())
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(create_animations)
        );
    }
}

// Create a resource to store handles of the animations
#[derive(Default)]
pub struct Animations {
    pub idle: Handle<SpriteSheetAnimation>,
    pub run: Handle<SpriteSheetAnimation>,
    pub jump: Handle<SpriteSheetAnimation>,
    pub dash: Handle<SpriteSheetAnimation>,
    pub fall: Handle<SpriteSheetAnimation>,
    pub crouch: Handle<SpriteSheetAnimation>,
    pub attack: Handle<SpriteSheetAnimation>,

}

fn create_animations(
    mut handles: ResMut<Animations>,
    mut assets: ResMut<Assets<SpriteSheetAnimation>>,
) {

    // idle
    handles.idle = assets.add(SpriteSheetAnimation::from_range(
        0..=3,
        Duration::from_millis(500),
    ));

    // run
    handles.run = assets.add(SpriteSheetAnimation::from_range(
        8..=13,
        Duration::from_millis(150),
    ));

    // jump
    handles.jump = assets.add(SpriteSheetAnimation::from_range(
        14..=23,
        Duration::from_millis(500),
    ));

    // dash
    handles.dash = assets.add(SpriteSheetAnimation::from_range(
        24..=28,
        Duration::from_millis(150),
    ));

    // fall
    handles.fall = assets.add(
        SpriteSheetAnimation::from_range(22..=23,
        Duration::from_millis(150),
    ));

    // crouch
    handles.crouch = assets.add(SpriteSheetAnimation::from_range(
        4..=7, // TODO: Find correct frames
        Duration::from_millis(150),
    ));
    // attacks
    handles.attack = assets.add(SpriteSheetAnimation::from_range(
        55..=57, // TODO: find correct frames
        Duration::from_millis(150),
    ));
}