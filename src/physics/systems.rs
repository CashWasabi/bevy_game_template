use crate::physics::components::{GroundDetection, WallDetection, GroundSensor, WallSensor};
use bevy::utils::HashSet;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn spawn_ground_sensor(
    mut commands: Commands,
    detect_ground_for: Query<(Entity, &Collider), Added<GroundDetection>>,
) {
    for (entity, shape) in &detect_ground_for {
        if let Some(cuboid) = shape.as_cuboid() {
            let Vec2 { x: hx, y: hy } = cuboid.half_extents();

            let detector_shape = Collider::cuboid(hx * 0.75, 2.0);

            let sensor_translation = Vec3::new(0., -hy * 2.0, 0.);

            commands.entity(entity).with_children(|builder| {
                builder
                    .spawn_empty()
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
    collidables: Query<Entity, (With<Collider>, Without<Sensor>)>,
) {
    for (entity, mut ground_sensor) in &mut ground_sensors {
        for collision in collisions.iter() {
            match collision {
                CollisionEvent::Started(a, b, _) => {
                    if *a == entity {
                        ground_sensor.intersecting_ground_entities.insert(*b);
                    }
                    if *b == entity {
                        ground_sensor.intersecting_ground_entities.insert(*a);
                    }
                    let (sensor, other) = if *a == entity {
                        (a, b)
                    } else if *b == entity {
                        (b, a)
                    } else {
                        continue;
                    };

                    if collidables.contains(*other) {
                        if *sensor == entity {
                            ground_sensor.intersecting_ground_entities.insert(*other);
                        }
                    }
                }
                CollisionEvent::Stopped(a, b, _) => {
                    let (sensor, other) = if *a == entity {
                        (a, b)
                    } else if *b == entity {
                        (b, a)
                    } else {
                        continue;
                    };

                    if collidables.contains(*other) {
                        if *sensor == entity {
                            ground_sensor.intersecting_ground_entities.remove(other);
                        }
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
    detect_wall_for: Query<(Entity, &Collider), Added<WallDetection>>,
) {
    for (entity, shape) in &detect_wall_for {
        if let Some(cuboid) = shape.as_cuboid() {
            let Vec2 { x: hx, y: hy } = cuboid.half_extents();

            let left_detector_shape = Collider::cuboid(2.0, hy);
            let right_detector_shape = Collider::cuboid(2.0, hy);

            let left_sensor_translation = Vec3::new(hx * -3.0, 0., 0.);
            let right_sensor_translation = Vec3::new(hx * 3.0, 0., 0.);

            commands.entity(entity).with_children(|builder| {
                builder
                    .spawn_empty()
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(left_detector_shape)
                    .insert(Sensor)
                    .insert(Transform::from_translation(left_sensor_translation))
                    .insert(GlobalTransform::default())
                    .insert(WallSensor {
                        wall_detection_entity: entity,
                        intersecting_wall_entities: HashSet::new(),
                    });
            });

            commands.entity(entity).with_children(|builder| {
                builder
                    .spawn_empty()
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(right_detector_shape)
                    .insert(Sensor)
                    .insert(Transform::from_translation(right_sensor_translation))
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
    collidables: Query<Entity, (With<Collider>, Without<Sensor>)>,
) {
    for (entity, mut wall_sensor) in &mut wall_sensors {
        for collision in collisions.iter() {
            match collision {
                CollisionEvent::Started(a, b, _) => {
                    let (sensor, other) = if *a == entity {
                        (a, b)
                    } else if *b == entity {
                        (b, a)
                    } else {
                        continue;
                    };

                    if collidables.contains(*other) {
                        if *sensor == entity {
                            wall_sensor.intersecting_wall_entities.insert(*other);
                        }
                    }
                }
                CollisionEvent::Stopped(a, b, _) => {
                    let (sensor, other) = if *a == entity {
                        (a, b)
                    } else if *b == entity {
                        (b, a)
                    } else {
                        continue;
                    };

                    if collidables.contains(*other) {
                        if *sensor == entity {
                            wall_sensor.intersecting_wall_entities.remove(other);
                        }
                    }
                }
            }
        }
        if let Ok(mut wall_detection) = wall_detectors.get_mut(wall_sensor.wall_detection_entity) {
            wall_detection.0 = wall_sensor.intersecting_wall_entities.is_empty();
        }
    }
}
