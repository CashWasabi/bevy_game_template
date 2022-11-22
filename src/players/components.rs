use std::collections::HashSet;

use crate::animations::animation::{Animation, AnimationState, PlayerAnimations};
use crate::physics::components::ColliderBundle;
use crate::states::StateMachineComponent;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Debug, Clone)]
pub enum PlayerStateEvent {
    Idle,
    Walk,
    Run,
    Jump,
    Crouch,
    Dash,
    Push,
    Pull,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
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
    pub player_animations: PlayerAnimations,
    pub animation: Animation,
    pub animation_state: AnimationState,

    // TODO(MO): Do a PlayerMovement Bundle maybe?
    pub direction: PlayerDirection,
    pub player: Player,
    pub state_machine: StateMachineComponent,

    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
    #[worldly]
    pub worldly: Worldly,
    pub ground_detection: GroundDetection,
    pub wall_detection: WallDetection,
    // The whole EntityInstance can be stored directly as an EntityInstance component
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Clone, Default, Component, Deref, DerefMut)]
pub struct GroundDetection(pub bool);

#[derive(Component)]
pub struct GroundSensor {
    pub ground_detection_entity: Entity,
    pub intersecting_ground_entities: HashSet<Entity>,
}

#[derive(Clone, Default, Component, Deref, DerefMut)]
pub struct WallDetection(pub bool);

#[derive(Component)]
pub struct WallSensor {
    pub wall_detection_entity: Entity,
    pub intersecting_wall_entities: HashSet<Entity>,
}

#[derive(Component, Default, Clone, PartialEq, PartialOrd, Deref, DerefMut)]
pub struct PlayerDirection(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct DashTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct JumpBufferTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct CoyoteTimer(pub Timer);
