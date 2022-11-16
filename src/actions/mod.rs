mod game_control;

use bevy::prelude::*;
use crate::actions::game_control::{get_movement, GameControl};
use crate::GameState;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(set_movement_actions),
        );
    }
}

#[derive(Default)]
pub struct Actions {
    pub movement: Option<Vec2>,
    pub jump: bool,
    pub run: bool,
    pub dash: bool,
    pub crouch: bool,
    pub push: bool,
    pub pull: bool,
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
    actions.jump = GameControl::Jump.just_pressed(&keyboard_input);
    actions.run = GameControl::Run.pressed(&keyboard_input);
    actions.dash = GameControl::Dash.pressed(&keyboard_input);
    actions.crouch = GameControl::Crouch.just_pressed(&keyboard_input);
    actions.push = GameControl::Push.just_pressed(&keyboard_input);
    actions.pull = GameControl::Pull.just_pressed(&keyboard_input);
}
