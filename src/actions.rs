use crate::GameState;
use bevy::prelude::*;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing)
            .with_system(set_actions),
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActionState {
    JustPressed,
    Pressed,
    JustReleased,
    Released
}

impl Default for ActionState {
    fn default() -> Self {
        Self::Released
    }
}

#[derive(Default, Debug)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
    pub run: ActionState,
    pub jump: ActionState,
    pub dash: ActionState,
    pub crouch: ActionState,
    pub attack: ActionState
}

fn set_action_state(control: GameControl, keyboard_input: &Res<Input<KeyCode>>) -> ActionState {
    if control.just_pressed(keyboard_input) {
        ActionState::JustPressed
    }
    else if control.pressed(keyboard_input) {
        ActionState::Pressed
    }
    else if control.just_released(keyboard_input) {
        ActionState::JustReleased
    }
    else {
        ActionState::Released
    }
}

fn set_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    if GameControl::Up.just_released(&keyboard_input)
        || GameControl::Up.pressed(&keyboard_input)
        || GameControl::Left.just_released(&keyboard_input)
        || GameControl::Left.pressed(&keyboard_input)
        || GameControl::Down.just_released(&keyboard_input)
        || GameControl::Down.pressed(&keyboard_input)
        || GameControl::Right.just_released(&keyboard_input)
        || GameControl::Right.pressed(&keyboard_input)
    {
        let mut player_movement = Vec2::ZERO;

        if GameControl::Up.just_released(&keyboard_input)
            || GameControl::Down.just_released(&keyboard_input)
        {
            if GameControl::Up.pressed(&keyboard_input) {
                player_movement.y = 1.;
            } else if GameControl::Down.pressed(&keyboard_input) {
                player_movement.y = -1.;
            } else {
                player_movement.y = 0.;
            }
        } else if GameControl::Up.just_pressed(&keyboard_input) {
            player_movement.y = 1.;
        } else if GameControl::Down.just_pressed(&keyboard_input) {
            player_movement.y = -1.;
        } else {
            player_movement.y = actions.player_movement.unwrap_or(Vec2::ZERO).y;
        }

        if GameControl::Right.just_released(&keyboard_input)
            || GameControl::Left.just_released(&keyboard_input)
        {
            if GameControl::Right.pressed(&keyboard_input) {
                player_movement.x = 1.;
            } else if GameControl::Left.pressed(&keyboard_input) {
                player_movement.x = -1.;
            } else {
                player_movement.x = 0.;
            }
        } else if GameControl::Right.just_pressed(&keyboard_input) {
            player_movement.x = 1.;
        } else if GameControl::Left.just_pressed(&keyboard_input) {
            player_movement.x = -1.;
        } else {
            player_movement.x = actions.player_movement.unwrap_or(Vec2::ZERO).x;
        }

        if player_movement != Vec2::ZERO {
            player_movement = player_movement.normalize();
            actions.player_movement = Some(player_movement);
        }
    }
    else {
        actions.player_movement = None;
    }

    actions.run = set_action_state(GameControl::Running, &keyboard_input);
    actions.crouch = set_action_state(GameControl::Crouching, &keyboard_input);
    actions.jump = set_action_state(GameControl::Jumping, &keyboard_input);
    actions.dash = set_action_state(GameControl::Dashing, &keyboard_input);
    actions.attack = set_action_state(GameControl::Attacking, &keyboard_input);
}

enum GameControl {
    Up,
    Down,
    Left,
    Right,
    Running,
    Jumping,
    Crouching,
    Dashing,
    Attacking
}

impl GameControl {
    fn just_released(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.just_released(KeyCode::W)
                    || keyboard_input.just_released(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.just_released(KeyCode::S)
                    || keyboard_input.just_released(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.just_released(KeyCode::A)
                    || keyboard_input.just_released(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.just_released(KeyCode::D)
                    || keyboard_input.just_released(KeyCode::Right)
            }
            GameControl::Running => {
                keyboard_input.just_released(KeyCode::LShift)
            }
            GameControl::Jumping => {
                keyboard_input.just_released(KeyCode::Space)
            }
            GameControl::Crouching => {
                keyboard_input.just_released(KeyCode::LControl)
            }
            GameControl::Dashing => {
                keyboard_input.just_released(KeyCode::V)
            }
            GameControl::Attacking => {
                keyboard_input.just_released(KeyCode::F)
            }
        }
    }

    fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right)
            }
            GameControl::Running => {
                keyboard_input.pressed(KeyCode::LShift)
            }
            GameControl::Jumping => {
                keyboard_input.pressed(KeyCode::Space)
            }
            GameControl::Crouching => {
                keyboard_input.pressed(KeyCode::C)
            }
            GameControl::Dashing => {
                keyboard_input.pressed(KeyCode::V)
            }
            GameControl::Attacking => {
                keyboard_input.pressed(KeyCode::F)
            }
        }
    }

    fn just_pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.just_pressed(KeyCode::W) || keyboard_input.just_pressed(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.just_pressed(KeyCode::S)
                    || keyboard_input.just_pressed(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.just_pressed(KeyCode::A)
                    || keyboard_input.just_pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.just_pressed(KeyCode::D)
                    || keyboard_input.just_pressed(KeyCode::Right)
            }
            GameControl::Running => {
                keyboard_input.just_pressed(KeyCode::LShift)
            }
            GameControl::Jumping => {
                keyboard_input.just_pressed(KeyCode::Space)
            }
            GameControl::Crouching => {
                keyboard_input.just_pressed(KeyCode::C)
            }
            GameControl::Dashing => {
                keyboard_input.just_pressed(KeyCode::V)
            }
            GameControl::Attacking => {
                keyboard_input.just_pressed(KeyCode::F)
            }
        }
    }
}
