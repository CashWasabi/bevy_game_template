use std::collections::HashSet;

use bevy::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Copy, Clone, PartialEq, Debug, Component)]
pub struct PlayerData {
    pub move_speed: f32,
    pub run_speed: f32,
    pub jump_force: f32,
    pub jump_duration: u64,
    pub fall_speed: f32,
    pub dash_force: f32,
    pub dash_duration: u64,
    pub air_jump_counter: f32,
    pub coyote_duration: u64,
    pub jump_buffer_duration: u64,
    pub player_state: PlayerState,
}

impl Default for PlayerData {
    fn default() -> Self {
        PlayerData {
            move_speed: 200.0,
            run_speed: 300.0,
            jump_force: 300.0,
            jump_duration: 50,
            fall_speed: 200.0,
            dash_force: 500.0,
            dash_duration: 200,
            air_jump_counter: 1.0,
            coyote_duration: 150,
            jump_buffer_duration: 1150,
            player_state: PlayerState::default()
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

#[derive(Component)]
pub struct Grounded;


#[derive(Component, Default, Clone, PartialEq, PartialOrd, Deref, DerefMut)]
pub struct PlayerDirection (pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct DashTimer (pub Timer);

// TODO: Use Deref?
#[derive(Component, Deref, DerefMut)]
pub struct JumpTimer (pub Timer);

// TODO(MO): jump buffer should check if we're pressing the button a bit to early
// if we're grounded then or are able to jump in this time frame then execute
// Maybe we can even make this somewhat of a feature for inputs in general
#[derive(Component, Deref, DerefMut)]
pub struct JumpBufferTimer(pub Timer);

// TODO(MO): coyote time should be triggered once we are not grounded
// for a certain ammount of time we should still be able to jump

#[derive(Component, Deref, DerefMut)]
pub struct CoyoteTimer(pub Timer);
