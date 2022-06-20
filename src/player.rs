use std::time::Duration;
use crate::actions::{Actions, ActionState};
use crate::animations::animation::Animations;
use crate::levels::components::ColliderBundle;
use crate::loading::TextureAssets;
use crate::GameState;
use benimator::{Play, SpriteSheetAnimation};
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_ecs_ldtk::{LevelSelection, LdtkLevel};
use heron::prelude::*;
use bevy_prototype_lyon::prelude::*;

// https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/helpers/texture.rs
use bevy::render::render_resource::TextureUsages;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

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
pub struct PlayCam;


#[derive(Component, Default)]
pub struct Direction {
    orientation: f32
}

/// This plugin handles player related stuff like movement and animations
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(ShapePlugin)
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_camera)
                .with_system(spawn_player)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(camera_fit_inside_current_level)
                .with_system(flip_sprites)
                .with_system(spawn_ground_sensor)
                .with_system(ground_detection)
                .with_system(update_level_selection)
                // .with_system(idle_system)
                // .with_system(move_system)
                // .with_system(jump_system)
                // .with_system(fall_system)
                // .with_system(dash_system)
                // .with_system(crouch_system)
                // .with_system(attack_system)
                .with_system(check_grounded_system)
                .with_system(update_movement)
                .with_system(update_animation)
                // https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/helpers/texture.rs
                .with_system(set_texture_filters_to_nearest)
        );
    }
}

// https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/helpers/texture.rs
pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) = textures.get_mut(handle) {
                    texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_SRC
                        | TextureUsages::COPY_DST;
                }
            }
            _ => (),
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(camera).insert(PlayCam);
}

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    animations: Res<Animations>
) {
    let player_position = Vec3::new(200., 400., 10.);
    let player_size = Vec3::new(50., 37., 0.);
    // player sprite
    let player_sprite = SpriteSheetBundle {
        texture_atlas: texture_atlas.add(
            TextureAtlas::from_grid(
        textures.texture_player.clone().into(),
                Vec2::new(player_size.x, player_size.y),
                7,
                11
            )

        ),
        transform: Transform {
            translation: player_position.clone(),
            ..Transform::default()
        },
        ..Default::default()
    };

    // collider
    let rotation_constraints = RotationConstraints::lock();
    let collider = ColliderBundle {
        collider: CollisionShape::Cuboid {
            half_extends: Vec3::new(player_size.x / 2., player_size.y / 2., 0.),
            border_radius: None,
        },
        rigid_body: RigidBody::Dynamic,
        rotation_constraints,
        ..Default::default()
    };

    // debug for collider
    let shape = shapes::Rectangle {
        extents: Vec2::new(player_size.x, player_size.y),
        origin: RectangleOrigin::Center
    };
    let geometry = GeometryBuilder::build_as(
        &shape,
        DrawMode::Stroke(StrokeMode::new(Color::WHITE, 2.0)),
        Transform {
            translation: Vec3::new(player_position.x, player_position.y, player_position.z -1.0),
            ..Transform::default()
        },
    );

    commands.spawn()
        .insert_bundle(geometry)
        .insert_bundle(player_sprite)
        .insert_bundle(collider)
        .insert(Player)
        .insert(Direction::default())
        .insert(GroundDetection::default())
        .insert(PlayerState::Idle)
        .insert(animations.idle.clone())
        .insert(Play)
        ;
}

pub fn spawn_ground_sensor(
    mut commands: Commands,
    detect_ground_for: Query<(Entity, &CollisionShape, &Transform), Added<GroundDetection>>,
) {
    for (entity, shape, transform) in detect_ground_for.iter() {
        if let CollisionShape::Cuboid { half_extends, .. } = shape {
            let detector_shape = CollisionShape::Cuboid {
                half_extends: Vec3::new(half_extends.x / 2., 2., 0.),
                border_radius: None,
            };

            let sensor_translation = Vec3::new(0., -half_extends.y, 0.) / transform.scale;

            commands.entity(entity).with_children(|builder| {
                builder
                    .spawn()
                    .insert(RigidBody::Sensor)
                    .insert(detector_shape)
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
    rigid_bodies: Query<&RigidBody>,
) {
    for (entity, mut ground_sensor) in ground_sensors.iter_mut() {
        for collision in collisions.iter() {
            match collision {
                CollisionEvent::Started(a, b) => match rigid_bodies.get(b.rigid_body_entity()) {
                    Ok(RigidBody::Sensor) => {
                        // don't consider sensors to be "the ground"
                    }
                    Ok(_) => {
                        if a.rigid_body_entity() == entity {
                            ground_sensor
                                .intersecting_ground_entities
                                .insert(b.rigid_body_entity());
                        }
                    }
                    Err(_) => {
                        panic!("If there's a collision, there should be an entity")
                    }
                },
                CollisionEvent::Stopped(a, b) => {
                    if a.rigid_body_entity() == entity {
                        ground_sensor
                            .intersecting_ground_entities
                            .remove(&b.rigid_body_entity());
                    }
                }
            }
        }

        if let Ok(mut ground_detection) =
            ground_detectors.get_mut(ground_sensor.ground_detection_entity)
        {
            ground_detection.on_ground = ground_sensor.intersecting_ground_entities.len() > 0;
        }
    }
}

fn flip_sprites(
    actions: Res<Actions>,
    mut query: Query<(&mut Direction, &mut TextureAtlasSprite)>
) {
    let dir = actions.player_movement.unwrap_or(Vec2::ZERO);

    for (
        mut direction,
        mut sprite,
    ) in query.iter_mut() {
        if dir.x != 0. {
            direction.orientation = dir.x;

            if sprite.flip_x && direction.orientation > 0. {
                sprite.flip_x = false;
            } else if !sprite.flip_x && direction.orientation < 0. {
                sprite.flip_x = true;
            }
        }
    }
}

const ASPECT_RATIO: f32 = 16. / 9.;

pub fn camera_fit_inside_current_level(
    mut camera_query: Query<
        (
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        ),
        (
            With<PlayCam>,
            Without<Player>
        ),
    >,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Player>),
    >,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = *player_translation;

        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_handle) in level_query.iter() {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                if level_selection.is_match(&0, level) {
                    let level_ratio = level.px_wid as f32 / ldtk_level.level.px_hei as f32;

                    orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::None;
                    orthographic_projection.bottom = 0.;
                    orthographic_projection.left = 0.;
                    if level_ratio > ASPECT_RATIO {
                        // level is wider than the screen
                        orthographic_projection.top = (level.px_hei as f32 / 9.).round() * 9.;
                        orthographic_projection.right = orthographic_projection.top * ASPECT_RATIO;
                        camera_transform.translation.x = (player_translation.x
                            - level_transform.translation.x
                            - orthographic_projection.right / 2.)
                            .clamp(0., level.px_wid as f32 - orthographic_projection.right);
                        camera_transform.translation.y = 0.;
                    } else {
                        // level is taller than the screen
                        orthographic_projection.right = (level.px_wid as f32 / 16.).round() * 16.;
                        orthographic_projection.top = orthographic_projection.right / ASPECT_RATIO;
                        camera_transform.translation.y = (player_translation.y
                            - level_transform.translation.y
                            - orthographic_projection.top / 2.)
                            .clamp(0., level.px_hei as f32 - orthographic_projection.top);
                        camera_transform.translation.x = 0.;
                    }

                    camera_transform.translation.x += level_transform.translation.x;
                    camera_transform.translation.y += level_transform.translation.y;
                }
            }
        }
    }
}

pub fn update_level_selection(
    level_query: Query<(&Handle<LdtkLevel>, &Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    for (level_handle, level_transform) in level_query.iter() {
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            let level_bounds = Rect {
                bottom: level_transform.translation.y,
                top: level_transform.translation.y + ldtk_level.level.px_hei as f32,
                left: level_transform.translation.x,
                right: level_transform.translation.x + ldtk_level.level.px_wid as f32,
            };

            for player_transform in player_query.iter() {
                if player_transform.translation.x < level_bounds.right
                    && player_transform.translation.x > level_bounds.left
                    && player_transform.translation.y < level_bounds.top
                    && player_transform.translation.y > level_bounds.bottom
                    && !level_selection.is_match(&0, &ldtk_level.level)
                {
                    *level_selection = LevelSelection::Iid(ldtk_level.level.iid.clone());
                }
            }
        }
    }
}


// TODO: Implement these transitions correctly
#[derive(Component)]
pub struct Grounded;
pub fn transition_to_grounded(
    entity: Entity,
    commands: &mut Commands,
) {
    commands.entity(entity).insert(Grounded);
}
pub fn transition_from_grounded(
    entity: Entity,
    commands: &mut Commands,
) {
    commands.entity(entity).remove::<Grounded>();
}
pub fn check_grounded_system(
    mut commands: Commands,
    query: Query<(Entity, &GroundDetection)>,
) {
    for (entity, ground_sensor) in query.iter() {
        if ground_sensor.on_ground {
            transition_to_grounded(entity, &mut commands);
        } else {
            transition_from_grounded(entity, &mut commands);
        }
    }
}

////////////////////////////////////////////////
// TODO(MO): STATE UPDATES
////////////////////////////////////////////////
pub fn transition_to_idle(
    velocity: &mut Velocity,
    player_state: &mut PlayerState,
) {
    *player_state = PlayerState::Idle;
    velocity.linear.x = 0.;
}
pub fn transition_from_idle(_entity: Entity, _commands: &mut Commands) {}

// pub fn idle_system(
//     actions: Res<Actions>,
//     mut commands: Commands,
//     mut query: Query<(Entity, &mut PlayerState, &mut Velocity, &Grounded)>,
// ) {
//     for (
//         entity,
//         mut player_state,
//         mut velocity,
//         _,
//     ) in query.iter_mut() {
//         if actions.jump == ActionState::JustPressed {
//             println!("IDLE -> JUMP");
//             transition_from_idle(entity, &mut commands);
//             transition_to_jump(entity, &mut commands, &mut velocity, &mut player_state);
//         }
//         else if actions.attack == ActionState::JustPressed {
//             println!("IDLE -> ATTACK");
//             transition_from_idle(entity, &mut commands);
//             transition_to_attack(entity, &mut commands, &mut player_state);
//         }
//         else if actions.crouch == ActionState::JustPressed {
//             println!("IDLE -> CROUCH");
//             transition_from_idle(entity, &mut commands);
//             transition_to_crouch(&mut player_state);
//         }
//         else if actions.player_movement.unwrap_or(Vec2::ZERO).x != 0. {
//             println!("IDLE -> MOVE");
//             transition_from_idle(entity, &mut commands);
//             transition_to_move(&mut player_state);
//         }
//     }
// }

pub fn transition_to_move(player_state: &mut PlayerState) {
    *player_state = PlayerState::Move;
}
pub fn transition_from_move() {}

// pub fn move_system(
//     actions: Res<Actions>,
//     mut commands: Commands,
//     mut query: Query<(Entity, &mut PlayerState, &mut Velocity)>,
//     ground_check: Query<&Grounded>,
// ) {
//     for (
//         entity,
//         mut player_state,
//         mut velocity,
//     ) in query.iter_mut() {
//         if !ground_check.get(entity).is_ok() {
//             println!("MOVE -> FALL");
//             transition_from_move();
//             transition_to_fall(&mut player_state);
//             return;
//         }

//         if actions.jump == ActionState::JustPressed {
//             println!("MOVE -> JUMP");
//             transition_from_move();
//             transition_to_jump(entity, &mut commands, &mut velocity, &mut player_state);
//         }
//         else if actions.attack == ActionState::JustPressed {
//             println!("MOVE -> ATTACK");
//             transition_from_move();
//             transition_to_attack(entity, &mut commands, &mut player_state);
//         }
//         else if actions.crouch == ActionState::JustPressed {
//             println!("CROUCH -> ATTACK");
//             transition_from_move();
//             transition_to_crouch(&mut player_state);
//         }
//         else if actions.dash == ActionState::JustPressed {
//             println!("MOVE -> DASH");
//             transition_from_move();
//             transition_to_dash(entity, &mut commands, &mut player_state);
//         }
//         else if actions.player_movement.unwrap_or(Vec2::ZERO).x != 0. {
//             let mut speed = 200.;
//             if actions.run == ActionState::Pressed {
//                 speed = 300.;
//             }
//             let direction = actions.player_movement.unwrap_or(Vec2::ZERO);
//             velocity.linear.x = direction.x * speed;
//         } else {
//             println!("MOVE -> IDLE");
//             transition_from_move();
//             transition_to_idle(&mut velocity, &mut player_state);
//         }
//     }
// }

#[derive(Component)]
pub struct JumpTimer{
    timer: Timer
}
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

// pub fn jump_system(
//     actions: Res<Actions>,
//     mut commands: Commands,
//     time: Res<Time>,
//     mut query: Query<(Entity, &mut PlayerState, &mut Velocity, &mut JumpTimer)>,
//     ground_check: Query<&Grounded>,
// ) {
//     for (
//         entity,
//         mut player_state,
//         mut velocity,
//         mut jump_timer,
//     ) in query.iter_mut() {
//         if ground_check.get(entity).is_ok() && velocity.linear.y <= 0. {
//             println!("JUMP -> IDLE");
//             transition_from_jump(entity, &mut commands);
//             transition_to_idle(&mut velocity, &mut player_state);
//             // early return since we're grounded
//             return;
//         }

//         jump_timer.timer.tick(time.delta());

//         if actions.jump != ActionState::JustReleased && !jump_timer.timer.finished() {
//             velocity.linear.y *= 1.10;
//         }
//         else if velocity.linear.y < 0.0 {
//             println!("JUMP -> Fall");
//             transition_from_jump(entity, &mut commands);
//             transition_to_fall(&mut player_state);
//         }
//     }
// }

// TODO(MO): jump buffer should check if we're pressing the button a bit to early
// if we're grounded then or are able to jump in this time frame then execute
// Maybe we can even make this somewhat of a feature for inputs in general
pub struct JumpBufferTime(f32);

// TODO(MO): coyote time should be triggered once we are not grounded
// for a certain ammount of time we should still be able to jump
pub struct CoyoteTimeBuffer(f32);

// TODO(MO): Also use JumpApex (when higher in jump movement in x gets better)

pub fn transition_to_fall(
    player_state: &mut PlayerState,
) {
    *player_state = PlayerState::Fall;
}
pub fn transition_from_fall() {}


#[derive(Component)]
pub struct DashTimer {
    timer: Timer,
}

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


// TODO: trigger an attack event?
// Whoever listens to attack events can then consume it and check for collision?
#[derive(Component)]
pub struct AttackTimer {
    timer: Timer,
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

#[derive(Component)]
pub enum PlayerState {
    Idle,
    Move,
    Jump,
    Fall,
    Crouch,
    Attack,
    Dash
}

fn update_animation(
    animations: Res<Animations>,
    mut query: Query<(&PlayerState, &mut Handle<SpriteSheetAnimation>)>,
) {
    for (
        player_state,
        mut anim,
    ) in query.iter_mut() {
        let new_anim = match player_state {
            PlayerState::Idle => animations.idle.clone(),
            PlayerState::Move => animations.run.clone(),
            PlayerState::Jump => animations.jump.clone(),
            PlayerState::Fall => animations.fall.clone(),
            PlayerState::Crouch => animations.crouch.clone(),
            PlayerState::Attack => animations.attack.clone(),
            PlayerState::Dash => animations.dash.clone()
        };

        // TODO(MO): How do we compare them?
        // Does this do what we think it does?
        if anim.id == new_anim.id {
            return;
        }
        *anim = new_anim;
    }
}

// TODO(MO): Certain things like jump or dash are triggering sequences
// How can we solve this?
fn update_movement(
    time: Res<Time>,
    actions: Res<Actions>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut PlayerState, &mut Velocity, &mut Direction)>,
    mut jump_query: Query<&mut JumpTimer>,
    mut dash_query: Query<&mut DashTimer>,
    mut attack_query: Query<&mut AttackTimer>,
    ground_check: Query<&Grounded>,
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
                if !ground_check.get(entity).is_ok() {
                    println!("MOVE -> FALL");
                    transition_from_move();
                    transition_to_fall(&mut player_state);
                    return;
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
                if ground_check.get(entity).is_ok() {
                    println!("Fall -> IDLE");
                    transition_from_fall();
                    transition_to_idle(&mut velocity, &mut player_state);
                } else {
                    let speed = 200.;
                    let direction = actions.player_movement.unwrap_or(Vec2::ZERO);
                    velocity.linear.x = direction.x * speed;
                }
            },
            PlayerState::Crouch => {
                if !ground_check.get(entity).is_ok() {
                    println!("CROUCH -> FALL");
                    transition_from_crouch();
                    transition_to_fall(&mut player_state);
                    return;
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
                    if ground_check.get(entity).is_ok() && velocity.linear.y <= 0. {
                        println!("JUMP -> IDLE");
                        transition_from_jump(entity, &mut commands);
                        transition_to_idle(&mut velocity, &mut player_state);
                        // early return since we're grounded
                        return;
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
                        if !ground_check.get(entity).is_ok() {
                            println!("DASH -> FALL");
                            transition_from_dash(entity, &mut commands);
                            transition_to_fall(&mut player_state);
                        } else {
                            println!("DASH -> IDLE");
                            transition_from_dash(entity, &mut commands);
                            transition_to_idle(&mut velocity, &mut player_state);
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
                        if !ground_check.get(entity).is_ok() {
                            println!("ATTACK -> FALL");
                            transition_from_attack(entity, &mut commands);
                            transition_to_fall(&mut player_state);
                        } else {
                            println!("ATTACK -> IDLE");
                            transition_from_attack(entity, &mut commands);
                            transition_to_idle(&mut velocity, &mut player_state);
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
