use std::collections::HashSet;
use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::actions::{ActionState, Actions};
use crate::players::components::*;

// TODO(MO): Build an input buffer so that we don't miss out on inputs
pub fn update_player(
    actions: Res<Actions>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut PlayerData,
        &mut Velocity,
        &mut ExternalForce,
        &PlayerDirection,
        &GroundDetection,
    )>,
) {
    let movement = actions.player_movement.unwrap_or(Vec2::ZERO);
    for (entity, mut data, mut velocity, mut external_force, direction, is_grounded) in &mut query {
        let mut force = Vec2::ZERO;
        let mut speed = Vec2::ZERO;

        if movement.x != 0. {
            speed.x = match actions.run {
                ActionState::JustPressed | ActionState::Pressed => data.run_speed,
                _ => data.move_speed,
            }
        };

        match data.player_state {
            PlayerState::Idle => {
                speed.x = 0.;

                if !is_grounded.0 {
                    println!("IDLE -> FALL");
                    transition_from_idle();
                    transition_to_fall(entity, &mut commands, &mut data);
                }
                else if actions.jump == ActionState::JustPressed {
                    println!("IDLE -> JUMP");
                    transition_from_idle();
                    transition_to_jump(&mut data, &mut force);
                    println!("FORCE VALUE: {}", force);
                }
                else if actions.crouch == ActionState::JustPressed {
                    println!("IDLE -> CROUCH");
                    transition_from_idle();
                    transition_to_crouch(&mut data);
                }
                else if movement.x != 0. {
                    println!("IDLE -> MOVE");
                    transition_from_idle();
                    transition_to_move(&mut data);
                }
            }
            PlayerState::Move => {
                if !is_grounded.0 {
                    println!("MOVE -> FALL");
                    transition_from_move();
                    transition_to_fall(entity, &mut commands, &mut data);
                } else if actions.jump == ActionState::JustPressed {
                        println!("MOVE -> JUMP");
                        transition_from_move();
                        transition_to_jump(&mut data, &mut force);
                } else if actions.dash == ActionState::JustPressed {
                    println!("MOVE -> DASH");
                    transition_from_move();
                    transition_to_dash(entity, &mut commands, &mut data);
                } else if movement.x == 0. {
                    println!("MOVE -> IDLE");
                    transition_from_move();
                    transition_to_idle(&mut data);
                }
            }
            PlayerState::Jump => {
                if is_grounded.0 && velocity.linvel.y <= 0.0 {
                    println!("JUMP -> IDLE");
                    transition_from_jump(&mut data);
                    transition_to_idle(&mut data);
                } else if velocity.linvel.y <= 0.0 {
                    println!("JUMP -> Fall");
                    transition_from_jump(&mut data);
                    transition_to_fall(entity, &mut commands, &mut data);
                } 
                else if !(actions.jump == ActionState::Pressed) {
                    // TODO(MO): Do we want it to be like this?
                    // reduce linear velocity if jump not pressed
                    velocity.linvel.y *= 0.8;
                }
            }
            PlayerState::Fall => {
                if is_grounded.0 {
                    if data.jump_buffer_active {
                        println!("Fall -> JUMP");
                        transition_from_fall(entity, &mut commands);
                        transition_to_jump(&mut data, &mut force);
                    } else {
                        println!("Fall -> IDLE");
                        transition_from_fall(entity, &mut commands);
                        transition_to_idle(&mut data);
                    };
                } else if data.coyote_time_active {
                    if actions.jump == ActionState::JustPressed {
                        println!("Fall -> JUMP");
                        transition_from_fall(entity, &mut commands);
                        transition_to_jump(&mut data, &mut force);
                    }
                } 
            }
            PlayerState::Crouch => {
                if !is_grounded.0 {
                    println!("CROUCH -> FALL");
                    transition_from_crouch();
                    transition_to_fall(entity, &mut commands, &mut data);
                } else if actions.crouch == ActionState::Released {
                    println!("CROUCH -> IDLE");
                    transition_from_crouch();
                    transition_to_idle(&mut data);
                } else {
                    speed.x = 0.;
                }
            }
            PlayerState::Dash => {
                if !data.dash_active {
                    if is_grounded.0 {
                        println!("DASH -> FALL");
                        transition_from_dash(entity, &mut commands);
                        transition_to_fall(entity, &mut commands, &mut data);
                    } else {
                        println!("DASH -> IDLE");
                        transition_from_dash(entity, &mut commands);
                        transition_to_idle(&mut data);
                    }
                } else {
                    speed.x = data.dash_force;
                }
            }
        };

        //
        // update physics
        //
        
        // only use direct vel on x
        velocity.linvel.x = speed.x * direction.0;

        // for horizontals (like jump and fall)
        external_force.force = force;
        
        // keep data for next frame
        data.force = force;
        data.speed = speed;
    }
}

pub fn transition_to_idle(data: &mut PlayerData) {
    data.player_state = PlayerState::Idle;
}
pub fn transition_from_idle() {}

pub fn transition_to_move(data: &mut PlayerData) {
    data.player_state = PlayerState::Move;
}
pub fn transition_from_move() {}

pub fn transition_to_jump(data: &mut PlayerData, force: &mut Vec2) {
    data.player_state = PlayerState::Jump;
    data.jump_active = true;
    force.y = data.jump_force;
}
pub fn transition_from_jump(data: &mut PlayerData) {
    data.jump_active = false;
}

pub fn transition_to_fall(entity: Entity, commands: &mut Commands, data: &mut PlayerData) {
    data.player_state = PlayerState::Fall;
    // NOTE(MO): repeat has to be set to true since it can be reset
    let jump_buffer_timer = JumpBufferTimer(Timer::new(Duration::from_millis(1), true));
    // TODO(MO): Get's also triggered when transition from EVERY state to fall
    // e.g. jump to fall but we don't actually want to have coyote time there!
    // Should be activated once we lose "grounded" state
    let coyote_timer = CoyoteTimer(Timer::new(
        Duration::from_millis(data.coyote_duration),
        false,
    ));
    commands
        .entity(entity)
        .insert(jump_buffer_timer)
        .insert(coyote_timer);
}
pub fn transition_from_fall(entity: Entity, commands: &mut Commands) {
    commands
        .entity(entity)
        .remove::<JumpBufferTimer>()
        .remove::<CoyoteTimer>();
}

pub fn transition_to_dash(entity: Entity, commands: &mut Commands, data: &mut PlayerData) {
    data.player_state = PlayerState::Dash;
    let dash_timer = DashTimer(Timer::new(Duration::from_millis(data.dash_duration), false));
    commands.entity(entity).insert(dash_timer);
}

pub fn transition_from_dash(entity: Entity, commands: &mut Commands) {
    commands.entity(entity).remove::<DashTimer>();
}

pub fn transition_to_crouch(data: &mut PlayerData) {
    data.player_state = PlayerState::Crouch;
}
pub fn transition_from_crouch() {}

pub fn update_jump_buffer(
    time: Res<Time>,
    actions: Res<Actions>,
    mut query: Query<(&mut PlayerData, &mut JumpBufferTimer)>,
) {
    for (mut data, mut timer) in &mut query {
        timer.tick(time.delta());

        match actions.jump {
            ActionState::JustPressed => {
                let t = Duration::from_millis(data.jump_buffer_duration);
                timer.set_duration(t);
            }
            _ => (),
        }

        if timer.finished() {
            data.jump_buffer_active = false;
        } else if !data.jump_buffer_active {
            data.jump_buffer_active = true;
        }
    }
}

pub fn update_coyote_time(time: Res<Time>, mut query: Query<(&mut PlayerData, &mut CoyoteTimer)>) {
    for (mut data, mut timer) in &mut query {
        timer.tick(time.delta());

        if timer.finished() {
            data.coyote_time_active = false;
        } else if !data.coyote_time_active {
            data.coyote_time_active = true;
        }
    }
}

pub fn dash_cooldown(time: Res<Time>, mut query: Query<(&mut PlayerData, &mut DashTimer)>) {
    for (mut data, mut timer) in &mut query {
        timer.tick(time.delta());

        data.dash_active = if timer.finished() { false } else { true };
    }
}

pub fn spawn_ground_sensor(
    mut commands: Commands,
    detect_ground_for: Query<(Entity, &Collider, &Transform), Added<GroundDetection>>,
) {
    for (entity, shape, transform) in &detect_ground_for {
        if let Some(cuboid) = shape.as_cuboid() {
            let Vec2 { x: hx, y: hy } = cuboid.half_extents();

            let detector_shape = Collider::cuboid(hx * 0.75, 2.0);

            let sensor_translation = Vec3::new(0., -hy * 3.0, 0.) / transform.scale;

            commands.entity(entity).with_children(|builder| {
                builder
                    .spawn()
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(detector_shape)
                    .insert(Sensor)
                    .insert(Transform::from_translation(sensor_translation))
                    .insert(GlobalTransform::default())
                    .insert(GroundSensor {
                        ground_detection_entity: entity,
                        intersecting_ground_entities: HashSet::new(),
                    });
            });
        }
    }
}

pub fn ground_detection(
    mut ground_detectors: Query<&mut GroundDetection>,
    mut ground_sensors: Query<(Entity, &mut GroundSensor)>,
    mut collisions: EventReader<CollisionEvent>,
    colliders: Query<&Collider>,
) {
    for (entity, mut ground_sensor) in &mut ground_sensors {
        for collision in collisions.iter() {
            match collision {
                CollisionEvent::Started(a, b, _event_flag) => match colliders.get(*b) {
                    Ok(_) => {
                        if *a == entity {
                            ground_sensor.intersecting_ground_entities.insert(*b);
                        }
                    }
                    Err(_) => {
                        panic!("If there's a collision, there should be an entity")
                    }
                },
                CollisionEvent::Stopped(a, b, _event_flag) => {
                    if *a == entity {
                        ground_sensor.intersecting_ground_entities.remove(b);
                    }
                }
            }
        }
        if let Ok(mut ground_detection) =
            ground_detectors.get_mut(ground_sensor.ground_detection_entity)
        {
            ground_detection.0 = !ground_sensor.intersecting_ground_entities.is_empty();
        }
    }
}

pub fn spawn_wall_sensor(
    mut commands: Commands,
    detect_wall_for: Query<(Entity, &Collider, &Transform), Added<WallDetection>>,
) {
    for (entity, shape, transform) in &detect_wall_for {
        if let Some(cuboid) = shape.as_cuboid() {
            let Vec2 { x: hx, y: hy } = cuboid.half_extents();

            let detector_shape = Collider::cuboid(2.0, hy / 4.0);

            let sensor_translation = Vec3::new(hx * 3.0, 0., 0.) / transform.scale;

            commands.entity(entity).with_children(|builder| {
                builder
                    .spawn()
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(detector_shape)
                    .insert(Sensor)
                    .insert(Transform::from_translation(sensor_translation))
                    .insert(GlobalTransform::default())
                    .insert(WallSensor {
                        wall_detection_entity: entity,
                        intersecting_wall_entities: HashSet::new(),
                    });
            });
        }
    }
}

pub fn wall_detection(
    mut wall_detectors: Query<&mut WallDetection>,
    mut wall_sensors: Query<(Entity, &mut WallSensor)>,
    mut collisions: EventReader<CollisionEvent>,
    colliders: Query<&Collider>,
) {
    for (entity, mut wall_sensor) in &mut wall_sensors {
        for collision in collisions.iter() {
            match collision {
                CollisionEvent::Started(a, b, _event_flag) => match colliders.get(*b) {
                    Ok(_) => {
                        if *a == entity {
                            wall_sensor.intersecting_wall_entities.insert(*b);
                        }
                    }
                    Err(_) => {
                        panic!("If there's a collision, there should be an entity")
                    }
                },
                CollisionEvent::Stopped(a, b, _event_flag) => {
                    if *a == entity {
                        wall_sensor.intersecting_wall_entities.remove(b);
                    }
                }
            }
        }
        if let Ok(mut wall_detection) = wall_detectors.get_mut(wall_sensor.wall_detection_entity) {
            wall_detection.0 = wall_sensor.intersecting_wall_entities.len() > 0;
        }
    }
}
