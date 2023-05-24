use bevy::prelude::Vec2;
use statig::prelude::*;
use statig::StateOrSuperstate;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Event {
    Idle,
    WalkPressed,
    WalkReleased,
    DashPressed,
    DashReleased,
    Grounded,
    Airborne,
}

#[derive(Copy, Clone, Debug)]
pub struct Dasher {
    pub velocity: Vec2,
    pub walk_speed: Vec2,
    pub dash_speed: Vec2,
}

impl Default for Dasher {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
            walk_speed: Vec2::new(100.0, 0.0),
            dash_speed: Vec2::new(300.0, 0.0),
        }
    }
}

#[state_machine(
    // This sets the initial state to `idle`.
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
impl Dasher {
    #[superstate]
    fn grounded(&mut self, event: &Event) -> Response<State> {
        match event {
            // TODO(MO): Use a timer here?
            Event::Idle => Transition(State::idle()),
            Event::DashPressed => Transition(State::dash()),
            Event::WalkPressed => Transition(State::walk()),
            Event::Airborne => Transition(State::fall()),
            _ => Super,
        }
    }

    #[state]
    fn fall(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Grounded => Transition(State::idle()),
            _ => Super,
        }
    }

    #[state(superstate= "grounded")]
    fn idle(&mut self, event: &Event) -> Response<State> {
        self.velocity = Vec2::ZERO;

        match event {
            Event::DashPressed => Transition(State::dash()),
            Event::WalkPressed => Transition(State::walk()),
            _ => Super,
        }
    }

    #[state(superstate= "grounded")]
    fn walk(&mut self, event: &Event) -> Response<State> {
        self.velocity.x = self.walk_speed.x;

        match event {
            // Event::WalkPressed => Handled,
            Event::WalkReleased => Transition(State::idle()),
            _ => Super,
        }
    }
    
    #[state(superstate= "grounded")]
    fn dash(&mut self, event: &Event) -> Response<State> {
        self.velocity.x = self.dash_speed.x;

        match event {
            Event::DashReleased => Transition(State::idle()),
            _ => Super,
        }
    }
}

impl Dasher {
    fn on_transition(&mut self, source: &State, target: &State) {
        println!("transitioned from `{source:?}` to `{target:?}`");
    }

    fn on_dispatch(&mut self, state: StateOrSuperstate<Self>, event: &Event) {
        on_dispatch(self, state, event);
    }
}

// The `on_dispatch` callback that will be called before an event is dispatched to a state or superstate.
fn on_dispatch<M, S, E>(state_machine: M, state: S, event: E)
where
    M: Debug,
    S: Debug,
    E: Debug,
{
    println!("{state_machine:?}: dispatching `{event:?}` to `{state:?}`");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_states() {
        let mut state_machine = Dasher::default().state_machine().init();
        state_machine.handle(&Event::DashPressed);
    }
}
