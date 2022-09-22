use std::time::Duration;

use bevy::prelude::*;
use heron::prelude::*;

use crate::actions::{Actions, ActionState};
use crate::levels::components::GroundDetection;
use crate::GameState;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component, Clone)]
pub enum PlayerState {
    Idle,
    Move,
    Jump,
    Fall,
    Crouch,
    Attack,
    Dash
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Idle
    }
}

#[derive(Component)]
pub struct Grounded;


#[derive(Component, Default, Clone)]
pub struct Direction {
    pub orientation: f32
}


#[derive(Component)]
pub struct DashTimer {
    pub timer: Timer,
}

// TODO: trigger an attack event?
// Whoever listens to attack events can then consume it and check for collision?
#[derive(Component)]
pub struct AttackTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct JumpTimer{
    pub timer: Timer
}

// TODO(MO): jump buffer should check if we're pressing the button a bit to early
// if we're grounded then or are able to jump in this time frame then execute
// Maybe we can even make this somewhat of a feature for inputs in general
#[derive(Component, Deref)]
pub struct JumpBufferTime(f32);

// TODO(MO): coyote time should be triggered once we are not grounded
// for a certain ammount of time we should still be able to jump

#[derive(Component, Deref)]
pub struct CoyoteTimeBuffer(f32);

// TODO(MO): Also use JumpApex (when higher in jump movement in x gets better)

/// This plugin handles player related stuff like movement and animations
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(update_movement)
        );
    }
}

pub fn transition_to_idle(
    velocity: &mut Velocity,
    player_state: &mut PlayerState,
) {
    *player_state = PlayerState::Idle;
    velocity.linear.x = 0.;
}
pub fn transition_from_idle(_entity: Entity, _commands: &mut Commands) {}

pub fn transition_to_move(player_state: &mut PlayerState) {
    *player_state = PlayerState::Move;
}
pub fn transition_from_move() {}


pub fn transition_to_jump(
    entity: Entity,
    commands: &mut Commands,
    velocity: &mut Velocity,
    player_state: &mut PlayerState,
) {
    *player_state = PlayerState::Jump;
    commands.entity(entity)
        .insert(JumpTimer{timer: Timer::new(Duration::from_millis(500), false)})
        ;
    let speed = 350.;
    velocity.linear.y = speed;
}
pub fn transition_from_jump(
    entity: Entity,
    commands: &mut Commands,
) {
    commands.entity(entity).remove::<JumpTimer>();
}

pub fn transition_to_fall(
    player_state: &mut PlayerState,
) {
    *player_state = PlayerState::Fall;
}
pub fn transition_from_fall() {}

pub fn transition_to_dash(
    entity: Entity,
    commands: &mut Commands,
    player_state: &mut PlayerState,
) {
    *player_state = PlayerState::Dash;
    commands.entity(entity)
        .insert(DashTimer{timer: Timer::new(Duration::from_millis(250), false)})
        ;
}

pub fn transition_from_dash(
    entity: Entity,
    commands: &mut Commands,
) {
    commands.entity(entity).remove::<DashTimer>();
}


pub fn transition_to_attack(
    entity: Entity,
    commands: &mut Commands,
    player_state: &mut PlayerState,
) {
    *player_state = PlayerState::Attack;
    commands.entity(entity)
        .insert(
            AttackTimer{
                timer: Timer::new(Duration::from_millis(250), false)
            }
        )
        ;
}
pub fn transition_from_attack(
    entity: Entity,
    commands: &mut Commands,
) {
    commands.entity(entity).remove::<AttackTimer>();
}


pub fn transition_to_crouch(
    player_state: &mut PlayerState
) {
    *player_state = PlayerState::Crouch;
}
pub fn transition_from_crouch() {}

// TODO(MO): Things changed fix this!
// Where do we want to put this information?

// TODO(MO): Certain things like jump or dash are triggering sequences
// How can we solve this? The current solution looks a bit brittle!
fn update_movement(
    time: Res<Time>,
    actions: Res<Actions>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut PlayerState, &mut Velocity, &mut Direction)>,
    mut jump_query: Query<&mut JumpTimer>,
    mut dash_query: Query<&mut DashTimer>,
    mut attack_query: Query<&mut AttackTimer>,
    ground_check: Query<&GroundDetection>,
) {
    for (
        entity,
        mut player_state,
        mut velocity,
        direction
    ) in query.iter_mut() {
        match *player_state {
            PlayerState::Idle => {
                if actions.jump == ActionState::JustPressed {
                    println!("IDLE -> JUMP");
                    transition_from_idle(entity, &mut commands);
                    transition_to_jump(entity, &mut commands, &mut velocity, &mut player_state);
                }
                else if actions.attack == ActionState::JustPressed {
                    println!("IDLE -> ATTACK");
                    transition_from_idle(entity, &mut commands);
                    transition_to_attack(entity, &mut commands, &mut player_state);
                }
                else if actions.crouch == ActionState::JustPressed {
                    println!("IDLE -> CROUCH");
                    transition_from_idle(entity, &mut commands);
                    transition_to_crouch(&mut player_state);
                }
                else if actions.player_movement.unwrap_or(Vec2::ZERO).x != 0. {
                    println!("IDLE -> MOVE");
                    transition_from_idle(entity, &mut commands);
                    transition_to_move(&mut player_state);
                }
            },
            PlayerState::Move => {
                if let Some(ground_sensor) = ground_check.get(entity).ok() {
                    if !ground_sensor.on_ground {
                        println!("MOVE -> FALL");
                        transition_from_move();
                        transition_to_fall(&mut player_state);
                        return;
                    }
                }

                if actions.jump == ActionState::JustPressed {
                    println!("MOVE -> JUMP");
                    transition_from_move();
                    transition_to_jump(entity, &mut commands, &mut velocity, &mut player_state);
                }
                else if actions.attack == ActionState::JustPressed {
                    println!("MOVE -> ATTACK");
                    transition_from_move();
                    transition_to_attack(entity, &mut commands, &mut player_state);
                }
                else if actions.crouch == ActionState::JustPressed {
                    println!("CROUCH -> ATTACK");
                    transition_from_move();
                    transition_to_crouch(&mut player_state);
                }
                else if actions.dash == ActionState::JustPressed {
                    println!("MOVE -> DASH");
                    transition_from_move();
                    transition_to_dash(entity, &mut commands, &mut player_state);
                }
                else if actions.player_movement.unwrap_or(Vec2::ZERO).x != 0. {
                    let mut speed = 200.;
                    if actions.run == ActionState::Pressed {
                        speed = 300.;
                    }
                    let direction = actions.player_movement.unwrap_or(Vec2::ZERO);
                    velocity.linear.x = direction.x * speed;
                } else {
                    println!("MOVE -> IDLE");
                    transition_from_move();
                    transition_to_idle(&mut velocity, &mut player_state);
                }
            },
            PlayerState::Fall => {
                if let Some(ground_sensor) = ground_check.get(entity).ok() {
                    if ground_sensor.on_ground {
                        println!("Fall -> IDLE");
                        transition_from_fall();
                        transition_to_idle(&mut velocity, &mut player_state);
                    } else {
                        let speed = 200.;
                        let direction = actions.player_movement.unwrap_or(Vec2::ZERO);
                        velocity.linear.x = direction.x * speed;
                    }
                }
            },
            PlayerState::Crouch => {
                if let Some(ground_sensor) = ground_check.get(entity).ok() {
                    if !ground_sensor.on_ground {
                        println!("CROUCH -> FALL");
                        transition_from_crouch();
                        transition_to_fall(&mut player_state);
                        return;
                    }
                }

                if actions.crouch == ActionState::Released {
                    println!("CROUCH -> IDLE");
                    transition_from_crouch();
                    transition_to_idle(&mut velocity, &mut player_state);
                } else {
                    velocity.linear.x = 0.;
                }
            },
            // has state and needs more infos
            // TODO(MO): Query for JumpTimer with entity
            PlayerState::Jump => {
                if let Ok(mut jump_timer) = jump_query.get_mut(entity) {
                    // do something with the components
                    if let Some(ground_sensor) = ground_check.get(entity).ok() {
                        if ground_sensor.on_ground && velocity.linear.y <= 0. {
                            println!("JUMP -> IDLE");
                            transition_from_jump(entity, &mut commands);
                            transition_to_idle(&mut velocity, &mut player_state);
                            // early return since we're grounded
                            return;
                        }
                    }

                    jump_timer.timer.tick(time.delta());

                    if actions.jump != ActionState::JustReleased && !jump_timer.timer.finished() {
                        velocity.linear.y *= 1.10;
                    }
                    else if velocity.linear.y < 0.0 {
                        println!("JUMP -> Fall");
                        transition_from_jump(entity, &mut commands);
                        transition_to_fall(&mut player_state);
                    }
                } else {
                    // the entity does not have the components from the query
                }
            },
            // has state and needs more infos
            // TODO(MO): Query for DashTimer with entity
            PlayerState::Dash => {
                if let Ok(mut dash_timer) = dash_query.get_mut(entity) {
                    // do something with the components
                    dash_timer.timer.tick(time.delta());
                    if dash_timer.timer.finished() {
                        if let Some(ground_sensor) = ground_check.get(entity).ok() {
                            if ground_sensor.on_ground {
                                println!("DASH -> FALL");
                                transition_from_dash(entity, &mut commands);
                                transition_to_fall(&mut player_state);
                            } else {
                                println!("DASH -> IDLE");
                                transition_from_dash(entity, &mut commands);
                                transition_to_idle(&mut velocity, &mut player_state);
                            }
                        }
                    }
                    else {
                        let speed = 450.;
                        velocity.linear.x = direction.orientation * speed;
                    }
                } else {
                    // the entity does not have the components from the query
                }
            }
            // has state and needs more infos
            PlayerState::Attack => {
                if let Ok(mut attack_timer) = attack_query.get_mut(entity) {
                    // do something with the components
                    attack_timer.timer.tick(time.delta());
                    if attack_timer.timer.finished() {
                        if let Some(ground_sensor) = ground_check.get(entity).ok() {
                            if ground_sensor.on_ground {
                                println!("ATTACK -> FALL");
                                transition_from_attack(entity, &mut commands);
                                transition_to_fall(&mut player_state);
                            } else {
                                println!("ATTACK -> IDLE");
                                transition_from_attack(entity, &mut commands);
                                transition_to_idle(&mut velocity, &mut player_state);
                            }
                        }
                    }
                    else {
                        velocity.linear.x = 0.;
                        velocity.linear.y = 0.;
                    }
                } else {
                    // the entity does not have the components from the query
                }
            },
        };
    }
}