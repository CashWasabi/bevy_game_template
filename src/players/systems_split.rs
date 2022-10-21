use std::time::Duration;
use std::collections::HashSet;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::actions::{Actions, ActionState};
use crate::players::components::*;

pub fn update_jump_buffer(
    time: Res<Time>,
    actions: Res<Actions>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut PlayerData, &mut JumpBufferTimer)>,
) {
    for (entity, mut player_data, timer) in query.iter_mut() {
        timer.tick(time.delta());
        if actions.jump == ActionState::JustPressed {
            let t = Duration::from_millis(player_data.jump_buffer_duration)
            timer.set_duration(t);
        };
    };

}

pub fn update_coyote_time(
    time: Res<Time>,
    mut query: Query<(Entity, &mut CoyoteTimer)>,
) {
    for (entity, timer) in query.iter_mut() {
        timer.tick(time.delta());
    };
}

pub fn update_movement(
    time: Res<Time>,
    actions: Res<Actions>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut PlayerData, &mut Velocity, &mut PlayerDirection)>,
    // all these timers can be seperate systems and fill player_data
    mut jump_query: Query<&mut JumpTimer>,
    mut dash_query: Query<&mut DashTimer>,
    mut fall_query: Query<&mut CoyoteTimer>,
    mut jump_buffer_query: Query<&mut JumpBufferTimer>,
    ground_check: Query<&GroundDetection>,
    wall_check: Query<&WallDetection>,
) {
    for (
        entity,
        mut player_data,
        mut velocity,
        direction
    ) in query.iter_mut() {
        let ground_sensor = ground_check.get(entity).unwrap();
        let wall_sensor = wall_check.get(entity).unwrap();
        let player_movement = actions.player_movement.unwrap_or(Vec2::ZERO);

        match player_data.player_state {
            PlayerState::Idle => {
                if actions.jump == ActionState::JustPressed {
                    println!("IDLE -> JUMP");
                    transition_from_idle();
                    transition_to_jump(entity, &mut commands, &mut velocity, &mut player_data);
                }
                else if actions.crouch == ActionState::JustPressed {
                    println!("IDLE -> CROUCH");
                    transition_from_idle();
                    transition_to_crouch(&mut player_data);
                }
                else if player_movement.x != 0. {
                    println!("IDLE -> MOVE");
                    transition_from_idle();
                    transition_to_move(&mut player_data);
                };
            },
            PlayerState::Move => {
                if !ground_sensor.on_ground {
                    println!("MOVE -> FALL");
                    transition_from_move();
                    transition_to_fall(entity, &mut commands, &mut player_data);
                    return;
                }

                if actions.jump == ActionState::JustPressed {
                    println!("MOVE -> JUMP");
                    transition_from_move();
                    transition_to_jump(entity, &mut commands, &mut velocity, &mut player_data);
                }
                else if actions.dash == ActionState::JustPressed {
                    println!("MOVE -> DASH");
                    transition_from_move();
                    transition_to_dash(entity, &mut commands, &mut player_data);
                }
                else if player_movement.x != 0. {
                    let mut speed = player_data.move_speed;
                    if actions.run == ActionState::Pressed {
                        speed = player_data.run_speed;
                    }
                    velocity.linvel.x = player_movement.x * speed;
                } else {
                    println!("MOVE -> IDLE");
                    transition_from_move();
                    transition_to_idle(&mut velocity, &mut player_data); 
                };
            },
            PlayerState::Fall => {
                // Add fall when touching wall and not grounded
                if wall_sensor.touching_wall && !ground_sensor.on_ground {
                    velocity.linvel.y = -player_data.fall_speed;
                }

                if player_movement.x != 0. {
                    let mut speed = player_data.move_speed;
                    if actions.run == ActionState::Pressed {
                        speed = player_data.run_speed;
                    }
                    velocity.linvel.x = player_movement.x * speed;
                }

                // TODO(MO): Is this bad? We always need a jump_buffer now
                let mut jump_buffer_timer = jump_buffer_query
                    .get_mut(entity)
                    .unwrap(); 

                jump_buffer_timer.tick(time.delta());

                if actions.jump == ActionState::JustPressed {
                    jump_buffer_timer.set_duration(
                        Duration::from_millis(player_data.jump_buffer_duration)
                    );
                };

                if ground_sensor.on_ground {
                    if !jump_buffer_timer.finished() {
                        println!("Fall -> JUMP");
                        transition_from_fall(entity, &mut commands);
                        transition_to_jump(entity, &mut commands, &mut velocity, &mut player_data);
                    } else {
                        println!("Fall -> IDLE");
                        transition_from_fall(entity, &mut commands);
                        transition_to_idle(&mut velocity, &mut player_data);
                    }
                } 
                else if let Ok(mut coyote_timer) = fall_query.get_mut(entity) {
                    coyote_timer.tick(time.delta());
                    if actions.jump == ActionState::JustPressed && !coyote_timer.finished() {
                        println!("Fall -> JUMP");
                        transition_from_fall(entity, &mut commands);
                        transition_to_jump(entity, &mut commands, &mut velocity, &mut player_data);
                    }
                    else {
                        let speed = player_data.fall_speed;
                        velocity.linvel.x = player_movement.x * speed;
                    }
                }
                else {
                    let speed = player_data.fall_speed;
                    velocity.linvel.x = player_movement.x * speed;
                };
            },
            PlayerState::Crouch => {
                if !ground_sensor.on_ground {
                    println!("CROUCH -> FALL");
                    transition_from_crouch();
                    transition_to_fall(entity, &mut commands, &mut player_data);
                    return;
                };

                if actions.crouch == ActionState::Released {
                    println!("CROUCH -> IDLE");
                    transition_from_crouch();
                    transition_to_idle(&mut velocity, &mut player_data);
                } else {
                    velocity.linvel.x = 0.;
                };
            },
            PlayerState::Jump => {
                if let Ok(mut jump_timer) = jump_query.get_mut(entity) {
                    // do something with the components
                    if ground_sensor.on_ground && velocity.linvel.y <= 0. {
                        println!("JUMP -> IDLE");
                        transition_from_jump(entity, &mut commands);
                        transition_to_idle(&mut velocity, &mut player_data);
                        // early return since we're grounded
                        return;
                    };

                    jump_timer.tick(time.delta());

                    // TODO(MO): Instead of using a timer we can do initial force
                    // if the player stops pressing we can add negative force
                    if actions.jump != ActionState::JustReleased && !jump_timer.finished() {
                        velocity.linvel.y *= 1.05;
                    }
                    else if velocity.linvel.y <= 0.0 {
                        println!("JUMP -> Fall");
                        transition_from_jump(entity, &mut commands);
                        transition_to_fall(entity, &mut commands, &mut player_data);
                        return;
                    } else {
                        // do nothing
                    }

                    if player_movement.x != 0. {
                        let mut speed = player_data.move_speed;
                        if actions.run == ActionState::Pressed {
                            speed = player_data.run_speed;
                        }
                        velocity.linvel.x = player_movement.x * speed;
                    }

                } else {
                    // the entity does not have the components from the query
                };
            },
            // has state and needs more infos
            // TODO(MO): Query for DashTimer with entity
            PlayerState::Dash => {
                if let Ok(mut dash_timer) = dash_query.get_mut(entity) {
                    // do something with the components
                    dash_timer.tick(time.delta());
                    if dash_timer.finished() {
                        if ground_sensor.on_ground {
                            println!("DASH -> FALL");
                            transition_from_dash(entity, &mut commands);
                            transition_to_fall(entity, &mut commands, &mut player_data);
                        } else {
                            println!("DASH -> IDLE");
                            transition_from_dash(entity, &mut commands);
                            transition_to_idle(&mut velocity, &mut player_data);
                        }
                    }
                    else {
                        let speed = 450.;
                        velocity.linvel.x = direction.0 * speed;
                    }
                } else {
                    // the entity does not have the components from the query
                };
            }
        };
    }
}

pub fn transition_to_idle(velocity: &mut Velocity, player_data: &mut PlayerData) {
    player_data.player_state = PlayerState::Idle;
    velocity.linvel.x = 0.;
}
pub fn transition_from_idle() {}

pub fn transition_to_move(player_data: &mut PlayerData) {
    player_data.player_state = PlayerState::Move;
}
pub fn transition_from_move() {}


pub fn transition_to_jump(
    entity: Entity,
    commands: &mut Commands,
    velocity: &mut Velocity,
    player_data: &mut PlayerData,
) {
    player_data.player_state = PlayerState::Jump;
    velocity.linvel.y = player_data.jump_force;
    let jump_timer = JumpTimer(Timer::new(Duration::from_millis(player_data.jump_duration), false));
    commands.entity(entity).insert(jump_timer);
}
pub fn transition_from_jump(
    entity: Entity,
    commands: &mut Commands,
) {
    commands.entity(entity).remove::<JumpTimer>();
}

pub fn transition_to_fall(
    entity: Entity,
    commands: &mut Commands,
    player_data: &mut PlayerData,
) {
    player_data.player_state = PlayerState::Fall;
    // NOTE(MO): repeat has to be set to true since it can be reset
    let jump_buffer_timer = JumpBufferTimer(Timer::new(Duration::from_millis(1), true));
    // TODO(MO): Get's also triggered when transition from EVERY state to fall
    // e.g. jump to fall but we don't actually want to have coyote time there!
    let coyote_timer = CoyoteTimer(Timer::new(Duration::from_millis(player_data.coyote_duration), false));
    commands
        .entity(entity)
        .insert(jump_buffer_timer)
        .insert(coyote_timer);
}
pub fn transition_from_fall(
    entity: Entity,
    commands: &mut Commands,
) {
    commands.entity(entity)
        .remove::<JumpBufferTimer>()
        .remove::<CoyoteTimer>();
}

pub fn transition_to_dash(
    entity: Entity,
    commands: &mut Commands,
    player_data: &mut PlayerData,
) {
    player_data.player_state = PlayerState::Dash;
    let dash_timer = DashTimer(Timer::new(Duration::from_millis(player_data.dash_duration), false));
    commands.entity(entity).insert(dash_timer);
}

pub fn transition_from_dash(
    entity: Entity,
    commands: &mut Commands,
) {
    commands.entity(entity).remove::<DashTimer>();
}

pub fn transition_to_crouch(player_data: &mut PlayerData) {
    player_data.player_state = PlayerState::Crouch;
}
pub fn transition_from_crouch() {}

pub fn spawn_ground_sensor(
    mut commands: Commands,
    detect_ground_for: Query<(Entity, &Collider, &Transform), Added<GroundDetection>>,
) {
    for (entity, shape, transform) in detect_ground_for.iter() {
        if let Some(cuboid) = shape.as_cuboid() {
            let Vec2 {x: hx, y: hy} = cuboid.half_extents();

            let detector_shape = Collider::cuboid (hx / 2.0, 12.5);

            let sensor_translation = Vec3::new(0., -hy, 0.) / transform.scale;
                
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
    for (entity, mut ground_sensor) in ground_sensors.iter_mut() {
        for collision in collisions.iter() {
            println!("{}", format!("Matching collision: {collision:?}"));
            // match also for Sensor collision
            match collision {
                CollisionEvent::Started(a, b, _event_flag
                ) => match colliders.get(*b) {
                    Ok(_) => {
                        if *a == entity {
                            println!("COLLISION STARTED!");
                            ground_sensor
                                .intersecting_ground_entities
                                .insert(*b);
                        }
                    }
                    Err(_) => {
                        panic!("If there's a collision, there should be an entity")
                    }
                },
                CollisionEvent::Stopped(a, b, _event_flag) => {
                    if *a == entity {
                        println!("COLLISION ENDED!");
                        ground_sensor
                            .intersecting_ground_entities
                            .remove(b);
                    }
                }
            }
        }
        if let Ok(mut ground_detection) = ground_detectors.get_mut(ground_sensor.ground_detection_entity)
        {
            ground_detection.on_ground = ground_sensor.intersecting_ground_entities.len() > 0;
        }
    }
}


pub fn spawn_wall_sensor(
    mut commands: Commands,
    detect_wall_for: Query<(Entity, &Collider, &Transform), Added<WallDetection>>,
) {
    for (entity, shape, transform) in detect_wall_for.iter() {
        if let Some(cuboid) = shape.as_cuboid() {
            let Vec2 {x: hx, y: hy} = cuboid.half_extents();

            let detector_shape = Collider::cuboid (hx * 2.0, hy / 4.0);

            let sensor_translation = Vec3::new(0., 0., 0.) / transform.scale;
                
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
    for (entity, mut wall_sensor) in wall_sensors.iter_mut() {
        for collision in collisions.iter() {
            println!("{}", format!("Matching collision: {collision:?}"));
            // match also for Sensor collision
            match collision {
                CollisionEvent::Started(a, b, _event_flag
                ) => match colliders.get(*b) {
                    Ok(_) => {
                        if *a == entity {
                            println!("COLLISION STARTED!");
                            wall_sensor
                                .intersecting_wall_entities
                                .insert(*b);
                        }
                    }
                    Err(_) => {
                        panic!("If there's a collision, there should be an entity")
                    }
                },
                CollisionEvent::Stopped(a, b, _event_flag) => {
                    if *a == entity {
                        println!("COLLISION ENDED!");
                        wall_sensor
                            .intersecting_wall_entities
                            .remove(b);
                    }
                }
            }
        }
        if let Ok(mut wall_detection) = wall_detectors.get_mut(wall_sensor.wall_detection_entity)
        {
            wall_detection.touching_wall = wall_sensor.intersecting_wall_entities.len() > 0;
        }
    }
}

