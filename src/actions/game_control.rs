use bevy::prelude::{Input, KeyCode, Res};

pub enum GameControl {
    Up,
    Down,
    Left,
    Right,
    Run,
    Jump,
    Dash,
    Crouch,
    Push,
    Pull,
}

impl GameControl {
    pub fn get_key_code(&self) -> KeyCode {
        match self {
            GameControl::Up => KeyCode::W,
            GameControl::Down => KeyCode::S,
            GameControl::Left => KeyCode::A,
            GameControl::Right => KeyCode::D,
            GameControl::Run => KeyCode::LShift,
            GameControl::Jump => KeyCode::Space,
            GameControl::Dash => KeyCode::R,
            GameControl::Crouch => KeyCode::S,
            GameControl::Push => KeyCode::Key1,
            GameControl::Pull => KeyCode::Key2,
        }
    }

    pub fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        keyboard_input.pressed(self.get_key_code())
    }

    pub fn just_pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        keyboard_input.just_pressed(self.get_key_code())
    }

    pub fn just_released(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        keyboard_input.just_released(self.get_key_code())
    }
}

pub fn get_movement(control: GameControl, input: &Res<Input<KeyCode>>) -> f32 {
    if control.pressed(input) {
        1.0
    } else {
        0.0
    }
}
