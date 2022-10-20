use std::collections::HashSet;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::Inspectable;
use crate::physics::components::ColliderBundle;
use crate::animations::animation::{
    PlayerAnimations,
    Animation,
    AnimationState,
};


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
    pub player_data: PlayerData,

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

#[derive(Copy, Clone, PartialEq, Debug, Component, Inspectable)]
pub struct PlayerData {
    #[inspectable(min=1.0, max=1000.0)]
    pub move_speed: f32,
    #[inspectable(min=1.0, max=1000.0)]
    pub run_speed: f32,
    #[inspectable(min=1.0, max=1000.0)]
    pub jump_force: f32,
    #[inspectable(min=1, max=1000)]
    pub jump_duration: u64,
    #[inspectable(min=1.0, max=1000.0)]
    pub fall_speed: f32,
    #[inspectable(min=1.0, max=1000.0)]
    pub max_fall_speed: f32,
    #[inspectable(min=1.0, max=1000.0)]
    pub dash_force: f32,
    #[inspectable(min=1, max=1000)]
    pub dash_duration: u64,
    #[inspectable(min=1, max=5)]
    pub air_jump_counter: u64,
    #[inspectable(min=1, max=1000)]
    pub coyote_duration: u64,
    #[inspectable(min=1, max=1000)]
    pub jump_buffer_duration: u64,

    #[inspectable()]
    pub player_state: PlayerState,
}

impl Default for PlayerData {
    fn default() -> Self {
        PlayerData {
            move_speed: 300.0,
            run_speed: 300.0,
            jump_force: 300.0,
            jump_duration: 50,
            fall_speed: 300.0,
            max_fall_speed: 400.0,
            dash_force: 500.0,
            dash_duration: 200,
            air_jump_counter: 2,
            coyote_duration: 150,
            jump_buffer_duration: 250,
            player_state: PlayerState::default()

            /*

            // maybe add?
            // everything could have its own system
            // using player_data as base and writing to it

            // activates when: jump_buffer has been added
            jump_buffered: bool,

            // activates when: coyote timer has been added
            coyote_active: bool,

            // activates when: dash has been activated
            dash_cooldown: bool,

            // counts down as long as not grounded
            // when grounded reset jumps
            air_jump_counter: u64
            */
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Inspectable)]
pub enum PlayerState {
    Idle,
    Move,
    Jump,
    Fall,
    Crouch,
    Dash
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Idle
    }
}

#[derive(Clone, Default, Component)]
pub struct GroundDetection {
    pub on_ground: bool,
}

#[derive(Component)]
pub struct GroundSensor {
    pub ground_detection_entity: Entity,
    pub intersecting_ground_entities: HashSet<Entity>,
}


#[derive(Clone, Default, Component)]
pub struct WallDetection {
    pub touching_wall: bool,
}

#[derive(Component)]
pub struct WallSensor {
    pub wall_detection_entity: Entity,
    pub intersecting_wall_entities: HashSet<Entity>,
}


#[derive(Component, Default, Clone, PartialEq, PartialOrd, Deref, DerefMut)]
pub struct PlayerDirection (pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct DashTimer (pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct JumpTimer (pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct JumpBufferTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct CoyoteTimer(pub Timer);
