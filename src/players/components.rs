use crate::actions::components::Action;
use crate::animations::components::{Animation, AnimationState, PlayerAnimations};
use crate::physics::components::{ColliderBundle, GroundDetection, WallDetection};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use statig::prelude::*;

#[derive(Copy, Clone, PartialEq, Default, Component)]
pub struct Player;

#[derive(Component, Default, Clone, PartialEq, PartialOrd, Deref, DerefMut)]
pub struct PlayerDirection(pub f32);

#[derive(Debug, Clone, Reflect, Component)]
pub enum Event {
    Move,
    Run,
    Jump,
    Dash,
    Crouch,
    Grounded,
    Fall,
    Attack,
}

#[derive(Component)]
pub struct PlayerStateMachine(pub StateMachine<CharacterController>);

#[derive(Default)]
pub struct CharacterController;

#[state_machine(initial = "State::idle()")]
impl CharacterController {
    #[state(superstate = "grounded")]
    fn idle(event: &Event) -> Response<State> {
        match event {
            _ => Super
        }
    }

    #[state(superstate = "grounded")]
    fn walk(event: &Event) -> Response<State> {
        match event {
            _ => Super
        }
    }

    #[state(superstate = "grounded")]
    fn run(event: &Event) -> Response<State> {
        match event {
            _ => Super
        }
    }

    #[state(superstate = "grounded")]
    fn crouch(event: &Event) -> Response<State> {
        match event {
            _ => Super
        }
    }

    #[state(superstate = "grounded")]
    fn dash(event: &Event) -> Response<State> {
        match event {
            _ => Super
        }
    }

    #[state(superstate = "grounded")]
    fn jump(event: &Event) -> Response<State> {
        match event {
            _ => Super
        }
    }

    #[state(superstate = "grounded")]
    fn ground_attack(event: &Event) -> Response<State> {
        match event {
            _ => Super
        }
    }

    #[superstate]
    fn grounded(event: &Event) -> Response<State> {
        match event {
            Event::Move => Transition(State::walk()),
            Event::Run => Transition(State::run()),
            Event::Crouch => Transition(State::crouch()),
            Event::Dash => Transition(State::dash()),
            Event::Jump => Transition(State::jump()),
            Event::Fall => Transition(State::fall()),
            Event::Attack => Transition(State::ground_attack()),
            _ => Super
        }
    }

    #[state(superstate = "airborne")]
    fn fall(event: &Event) -> Response<State> {
        match event {
            _ => Super
        }
    }

    #[state(superstate = "airborne")]
    fn air_attack(event: &Event) -> Response<State> {
        match event {
            _ => Super
        }
    }

    #[superstate]
    fn airborne(event: &Event) -> Response<State> {
        match event {
            Event::Grounded => Transition(State::idle()),
            Event::Attack => Transition(State::air_attack()),
            _ => Super
        }
    }
}

#[derive(Bundle)]
pub struct PlayerStateBundle {
    pub player: Player,
    pub character_controller: PlayerStateMachine,
    pub action_state: ActionState<Action>,
    pub player_animations: PlayerAnimations,
    pub animation: Animation,
    pub animation_state: AnimationState,
    pub direction: PlayerDirection,
}


impl Default for PlayerStateBundle {
    fn default() -> Self {
        let player = Player::default();
        let character_controller = PlayerStateMachine(
            CharacterController::default().state_machine()
        );
        let action_state = ActionState::<Action>::default();
        let player_animations = PlayerAnimations::default();
        let animation = Animation::default();
        let animation_state = AnimationState::default();
        let direction = PlayerDirection::default();

        PlayerStateBundle {
            player,
            character_controller,
            action_state,
            player_animations,
            animation,
            animation_state,
            direction,
        }
    }
}

#[derive(Default, Bundle, LdtkEntity)]
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

    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,

    #[worldly]
    pub worldly: Worldly,

    #[bundle]
    pub player_state_bundle: PlayerStateBundle,

    pub ground_detection: GroundDetection,
    pub wall_detection: WallDetection,

    // The whole EntityInstance can be stored directly as an EntityInstance component
    #[from_entity_instance]
    entity_instance: EntityInstance,
}
