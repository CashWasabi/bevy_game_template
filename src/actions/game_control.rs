use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::orientation::Direction;

#[derive(Actionlike, Clone)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Run,
    Jump,
    Dash,
    Crouch,
    PrimaryAbility,
    MindMeld
}

impl Action {
    const DIRECTIONS: [Self; 4] = [
        Action::Up,
        Action::Down,
        Action::Left,
        Action::Right,
    ];

    fn direction(self) -> Option<Direction> {
        match self {
            Action::Up => Some(Direction::NORTH),
            Action::Down => Some(Direction::SOUTH),
            Action::Left => Some(Direction::EAST),
            Action::Right => Some(Direction::WEST),
            _ => None,
        }
    }
}

fn default_input_map() -> InputMap<Action> {
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
        (KeyCode::F, PrimaryAbility),
        (KeyCode::Space, MindMeld),
    ])
}
