use std::collections::HashSet;

use crate::animations::animation::{Animation, AnimationState, PlayerAnimations};
use crate::physics::components::ColliderBundle;
use crate::players::states;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug, Default, Component)]
pub struct Player;

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
    // pub player_data: PlayerData,

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

#[derive(Copy, Clone, PartialEq, Debug, Component)]
pub struct PlayerData {
    pub last_frame_force: Vec2,
    pub last_frame_speed: Vec2,

    pub move_speed: f32,
    pub run_speed: f32,
    pub jump_force: f32,
    pub fall_speed: f32,
    pub dash_force: f32,
    pub dash_duration: u64,
    pub air_jump_counter: u64,
    pub coyote_duration: u64,
    pub jump_buffer_duration: u64,

    pub jump_active: bool,
    pub dash_active: bool,
    pub coyote_time_active: bool,
    pub jump_buffer_active: bool,

    pub player_state: states::PlayerState,
}

impl Default for PlayerData {
    fn default() -> Self {
        PlayerData {
            last_frame_force: Vec2::ZERO,
            last_frame_speed: Vec2::ZERO,

            move_speed: 200.0,
            run_speed: 350.0,
            jump_force: 3500.0,
            fall_speed: 150.0,
            dash_force: 500.0,
            dash_duration: 200,
            air_jump_counter: 2,
            coyote_duration: 150,
            jump_buffer_duration: 150,

            // states
            jump_active: false,
            dash_active: false,
            coyote_time_active: false,
            jump_buffer_active: false,

            player_state: PlayerState::default(),
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum PlayerState {
    Idle,
    Move,
    Jump,
    Fall,
    Crouch,
    Dash,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Idle
    }
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
