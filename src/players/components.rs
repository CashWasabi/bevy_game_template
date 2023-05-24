use crate::animations::animation::{
    Animation,
    AnimationState,
    PlayerAnimations
};
use crate::physics::components::{
    ColliderBundle,
    GroundDetection,
    WallDetection
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use leafwing_input_manager::{
    prelude::*,
    axislike::VirtualAxis
};
use seldom_state::prelude::*;

const JUMP_VELOCITY: f32 = 500.;
const PLAYER_SPEED: f32 = 200.;
const GRAVITY: f32 = -1000.;

#[derive(Copy, Clone, PartialEq, Default, Component)]
pub struct Player;

#[derive(Component, Default, Clone, PartialEq, PartialOrd, Deref, DerefMut)]
pub struct PlayerDirection(pub f32);

// #[derive(Component, Deref, DerefMut)]
// pub struct DashTimer(pub Timer);
//
// #[derive(Component, Deref, DerefMut)]
// pub struct JumpBufferTimer(pub Timer);
//
// #[derive(Component, Deref, DerefMut)]
// pub struct CoyoteTimer(pub Timer);

#[derive(Actionlike, Clone, Reflect)]
pub enum Action {
    Move,
    Jump,
    Dash,
}

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub enum Grounded {
    Left = -1,
    Idle = 0,
    Right = 1,
}

#[derive(Reflect)]
pub struct GroundedTrigger;

impl BoolTrigger for GroundedTrigger {
    type Param<'w, 's> = Query<'w, 's, (&'static Transform, &'static Falling)>;

    fn trigger(&self, entity: Entity, fallings: Self::Param<'_, '_>) -> bool {
        let (transform, falling) = fallings.get(entity).unwrap();
        transform.translation.y <= 0. && falling.velocity <= 0.
    }
}

#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Falling {
    velocity: f32,
}

fn walk(mut groundeds: Query<(&mut Transform, &Grounded)>, time: Res<Time>) {
    for (mut transform, grounded) in &mut groundeds {
        transform.translation.x += *grounded as i32 as f32 * time.delta_seconds() * PLAYER_SPEED;
    }
}

fn fall(mut fallings: Query<(&mut Transform, &mut Falling)>, time: Res<Time>) {
    for (mut transform, mut falling) in &mut fallings {
        let dt = time.delta_seconds();
        falling.velocity += dt * GRAVITY;
        transform.translation.y += dt * falling.velocity;
    }
}

#[derive(Clone, Bundle)]
pub struct PlayerStateBundle {
    pub input: InputManagerBundle<Action>,
    pub state_machine: StateMachine,
    pub player: Player,
    pub player_animations: PlayerAnimations,
    pub animation: Animation,
    pub animation_state: AnimationState,
    pub direction: PlayerDirection,
}

impl Default for PlayerStateBundle {
    fn default() -> Self {
        let input = InputManagerBundle {
            input_map: InputMap::default()
                .insert(VirtualAxis::horizontal_arrow_keys(), Action::Move)
                .insert(
                    SingleAxis::symmetric(GamepadAxisType::LeftStickX, 0.),
                    Action::Move,
                )
                .insert(KeyCode::Space, Action::Jump)
                .insert(GamepadButtonType::South, Action::Jump)
                .build(),
            ..default()
        };

        let state_machine = StateMachine::default()
            // Whenever the player presses jump, jump
            .trans::<Grounded>(
                JustPressedTrigger(Action::Jump),
                Falling {
                    velocity: JUMP_VELOCITY,
                },
            )
            // When the player hits the ground, idle
            .trans::<Falling>(GroundedTrigger, Grounded::Idle)
            // When the player is grounded, set their movement direction
            .trans_builder(
                ValueTrigger::unbounded(Action::Move),
                |_: &Grounded, value| {
                    Some(match value {
                        value if value > 0.5 => Grounded::Right,
                        value if value < -0.5 => Grounded::Left,
                        _ => Grounded::Idle,
                    })
                },
            );

        Self {
            input,
            state_machine,
            ..Default::default()
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[bundle]
    pub player_state_bundle: PlayerStateBundle,

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
