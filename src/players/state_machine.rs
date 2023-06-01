use statig::prelude::*;
use bevy::math::Vec2;
use bevy::prelude::{Reflect, Component};


#[derive(Debug, Clone, Reflect, Component)]
pub enum Event {
    Move,
    Run,
    Jump,
    Dash,
    Crouch,
    Fall,
    Attack,
    Grounded,
    Airborne,
}

#[derive(Debug, Default)]
pub struct CharacterController {
    pub ground_detected: bool,
    pub wall_detected: bool,
    pub speed: Vec2,
}

#[state_machine(
    // This sets the initial state to `led_on`.
    initial = "State::idle()",
    // Derive the Debug trait on the `State` enum.
    state(derive(Debug)),
    // Derive the Debug trait on the `Superstate` enum.
    superstate(derive(Debug)),
    // Set the `on_transition` callback.
    on_transition = "Self::on_transition",
    // Set the `on_dispatch` callback.
    on_dispatch = "Self::on_dispatch"
)]
impl CharacterController {
    #[state(superstate="grounded")]
    fn idle(&mut self, event: &Event) -> Response<State> {
        self.speed = Vec2::ZERO;
        match event {
            _ => Super
        }
    }

    #[state(superstate="grounded")]
    fn walk(&mut self, event: &Event) -> Response<State> {
        self.speed = Vec2::new(100.0, 0.0);
        match event {
            _ => Super
        }
    }

    #[state(superstate="grounded")]
    fn run(&mut self, event: &Event) -> Response<State> {
        self.speed = Vec2::new(200.0, 0.0);
        match event {
            _ => Super
        }
    }

    #[state(superstate="grounded")]
    fn crouch(&mut self, event: &Event) -> Response<State> {
        self.speed = Vec2::new(50.0, 0.0);
        match event {
            _ => Super
        }
    }

    #[state(superstate="grounded")]
    fn dash(&mut self, event: &Event) -> Response<State> {
        self.speed = Vec2::new(600.0, 0.0);
        match event {
            _ => Super
        }
    }

    #[action]
    fn enter_jump(&mut self) {
        self.speed = Vec2::new(800.0, 100.0);
    }

    #[state(superstate="grounded", entry_action = "enter_jump")]
    fn jump(&mut self, event: &Event) -> Response<State> {
        match event {
            _ => Super
        }
    }

    #[state(superstate="grounded")]
    fn grounded_attack(&mut self, event: &Event) -> Response<State> {
        self.speed = Vec2::ZERO;
        match event {
            _ => Super
        }
    }

    #[superstate]
    fn grounded(&mut self, event: &Event) -> Response<State> {
        self.ground_detected = true;
        match event {
            Event::Move => Transition(State::walk()),
            Event::Run => Transition(State::run()),
            Event::Crouch => Transition(State::crouch()),
            Event::Dash => Transition(State::dash()),
            Event::Jump => Transition(State::jump()),
            Event::Fall => Transition(State::fall()),
            Event::Attack => Transition(State::grounded_attack()),
            Event::Grounded => Transition(State::idle()),
            _ => Super
        }
    }

    #[state(superstate="airborne")]
    fn fall(&mut self, event: &Event) -> Response<State> {
        match event {
            _ => Super
        }
    }

    #[state(superstate="airborne")]
    fn airborne_attack(&mut self, event: &Event) -> Response<State> {
        self.speed = Vec2::ZERO;
        match event {
            _ => Super
        }
    }

    #[superstate]
    fn airborne(&mut self, event: &Event) -> Response<State> {
        self.ground_detected = false;
        match event {
            Event::Grounded => Transition(State::idle()),
            Event::Attack => Transition(State::airborne_attack()),
            Event::Airborne => Transition(State::fall()),
            _ => Super
        }
    }
}

impl CharacterController {
    fn on_transition(&mut self, source: &State, target: &State) {
        println!("transitioned from `{source:?}` to `{target:?}`");
    }

    fn on_dispatch(&mut self, state: StateOrSuperstate<Self>, event: &Event) {
        println!("dispatching `{event:?}` to `{state:?}`");
    }
}
