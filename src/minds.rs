use crate::animations::animation::{Animation, AnimationState, PlayerAnimations};
use crate::physics::components::{ColliderBundle, GroundDetection, WallDetection};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

// TODO: The traveller is our old player we had before
#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct TravellerBundle {
    #[sprite_sheet_bundle(
        "characters/Adventurer-1.5/adventurer-v1.5-Sheet.png",
        50.0, // tile height
        37.0, // tile width
        7,  // columns
        11, // rows
        0.0, // padding
        0.0, // offset
        0 // index
    )]
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    // TODO(MO): Do an animation Bundle maybe?
    pub player_animations: TravellerAnimations,
    pub animation: Animation,
    pub animation_state: AnimationState,

    // TODO(MO): Do a PlayerMovement Bundle maybe?
    // Maybe we don't need that at atll?
    pub direction: LookDirection,

    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
    pub ground_detection: GroundDetection,
    pub wall_detection: WallDetection,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct CandleBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,

    // TODO(MO): Do an animation Bundle maybe?
    pub player_animations: CandleAnimations,
    pub animation: Animation,
    pub animation_state: AnimationState,

    // TODO(MO): Do a PlayerMovement Bundle maybe?
    pub direction: LookDirection,

    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
    pub ground_detection: GroundDetection,
    pub wall_detection: WallDetection,
    #[from_entity_instance]
    entity_instance: EntityInstance,

}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct ClimberBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,

    // TODO(MO): Do an animation Bundle maybe?
    pub player_animations: CandleAnimations,
    pub animation: Animation,
    pub animation_state: AnimationState,

    // TODO(MO): Do a PlayerMovement Bundle maybe?
    pub direction: LookDirection,

    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
    pub ground_detection: GroundDetection,
    pub wall_detection: WallDetection,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct DasherBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,

    // TODO(MO): Do an animation Bundle maybe?
    pub player_animations: CandleAnimations,
    pub animation: Animation,
    pub animation_state: AnimationState,

    // TODO(MO): Do a PlayerMovement Bundle maybe?
    pub direction: LookDirection,

    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
    pub ground_detection: GroundDetection,
    pub wall_detection: WallDetection,
    #[from_entity_instance]
    entity_instance: EntityInstance,

}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct JumperBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,

    // TODO(MO): Do an animation Bundle maybe?
    pub player_animations: CandleAnimations,
    pub animation: Animation,
    pub animation_state: AnimationState,

    // TODO(MO): Do a PlayerMovement Bundle maybe?
    pub direction: LookDirection,

    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
    pub ground_detection: GroundDetection,
    pub wall_detection: WallDetection,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}


