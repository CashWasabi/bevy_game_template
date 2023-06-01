use crate::players::state_machine::CharacterController;
use crate::actions::components::{Action, default_input_map};
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


#[derive(Component)]
pub struct PlayerStateMachine(pub StateMachine<CharacterController>);

#[derive(Bundle)]
pub struct PlayerStateBundle {
    pub input_manager_bundle: InputManagerBundle<Action>,
    pub player: Player,
    pub character_controller: PlayerStateMachine,
    pub player_animations: PlayerAnimations,
    pub animation: Animation,
    pub animation_state: AnimationState,
    pub direction: PlayerDirection,
}


impl Default for PlayerStateBundle {
    fn default() -> Self {
        let input_manager_bundle = InputManagerBundle::<Action> {
            action_state: ActionState::<Action>::default(),
            input_map: default_input_map(),
        };
        let player = Player::default();
        let mut state_machine = CharacterController::default().state_machine();
        state_machine.init();
        let character_controller = PlayerStateMachine(state_machine);
        let player_animations = PlayerAnimations::default();
        let animation = Animation::default();
        let animation_state = AnimationState::default();
        let direction = PlayerDirection::default();

        PlayerStateBundle {
            input_manager_bundle,
            player,
            character_controller,
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
