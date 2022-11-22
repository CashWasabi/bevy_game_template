mod game_control;

use crate::actions::game_control::{get_movement, GameControl};
use crate::GameState;
use bevy::prelude::*;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Actions::default()).add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(set_movement_actions),
        );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub movement: Option<Vec2>,
    pub jump_pressed: bool,
    pub jump_hold: bool,
    pub run_hold: bool,
    pub dash_pressed: bool,
    pub crouch_pressed: bool,
    pub push_pressed: bool,
    pub pull_pressed: bool,
}

fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    let player_movement = Vec2::new(
        get_movement(GameControl::Right, &keyboard_input)
            - get_movement(GameControl::Left, &keyboard_input),
        get_movement(GameControl::Up, &keyboard_input)
            - get_movement(GameControl::Down, &keyboard_input),
    );

    if player_movement != Vec2::ZERO {
        actions.movement = Some(player_movement.normalize());
    } else {
        actions.movement = None;
    }
    actions.jump_pressed = GameControl::Jump.just_pressed(&keyboard_input);
    actions.jump_hold = GameControl::Jump.pressed(&keyboard_input);
    actions.run_hold = GameControl::Run.pressed(&keyboard_input);
    actions.dash_pressed = GameControl::Dash.just_pressed(&keyboard_input);
    actions.crouch_pressed = GameControl::Crouch.just_pressed(&keyboard_input);
    actions.push_pressed = GameControl::Push.just_pressed(&keyboard_input);
    actions.pull_pressed = GameControl::Pull.just_pressed(&keyboard_input);
}
