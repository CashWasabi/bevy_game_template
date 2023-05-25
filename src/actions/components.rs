use bevy::prelude::*;
use leafwing_input_manager::{axislike::VirtualAxis, prelude::*};


#[derive(Debug, Clone, Actionlike, Reflect, Component)]
pub enum Action {
    Move,
    Run,
    Jump,
    Dash,
    Crouch,
    Attack,
}

pub fn default_input_map() -> InputMap<Action> {
    // This allows us to replace `Action::Up` with `Up`,
    // significantly reducing boilerplate
    use Action::*;
    InputMap::default()
        // Movement
        .insert(VirtualAxis::horizontal_arrow_keys(), Move)
        .insert(SingleAxis::symmetric(GamepadAxisType::LeftStickX, 0.), Move)
        // Abilities
        .insert(KeyCode::LShift, Dash)
        .insert(KeyCode::Space, Jump)
        .insert(KeyCode::C, Crouch)
        .insert(GamepadButtonType::South, Action::Jump)
        .build()
}
