use bevy::prelude::Vec2;
use statig::prelude::*;

#[derive(Debug, Clone)]
pub enum Event {
    Idle,
    Run,
    Dash,
    MindMeld
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Dasher {
    pub velocity: Vec2,

    pub move_speed: f32,
    pub dash_range: f32
}

impl Default for Dasher {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,

            move_speed: 2.0,
            dash_range: 5.0,
        }
    }
}

impl StateMachine for Dasher {
    type State = State;

    type Superstate<'a> = Superstate;

    type Event = Event;

    const INIT_STATE: State = State::idle();
}

#[state_machine]
impl Dasher {
    #[state]
    fn idle(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Run => Transition(State::run()),
            Event::Dash => Transition(State::dash()),
            _ => Super,
        }
    }

    #[state]
    fn run(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Run => Handled,
            _ => Super,
        }
    }

    #[state]
    fn dash(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Dash => Handled,
            _ => Super,
        }
    }
}
