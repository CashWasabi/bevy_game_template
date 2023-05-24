use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, Clone, Debug)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Run,
    Jump,
    Dash,
    Crouch,
}

pub fn default_input_map() -> InputMap<Action> {
    // This allows us to replace `Action::Up` with `Up`,
    // significantly reducing boilerplate
    use Action::*;
    InputMap::new([
        // Movement
        (KeyCode::Up, Up),
        (KeyCode::W, Up),

        (KeyCode::Down, Down),
        (KeyCode::S, Down),

        (KeyCode::Left, Left),
        (KeyCode::A, Left),

        (KeyCode::Right, Right),
        (KeyCode::D, Right),

        // Abilities
        (KeyCode::LShift, Dash),
        (KeyCode::Space, Jump),
        (KeyCode::C, Crouch),
    ])
}
