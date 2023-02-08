use bevy::prelude::Vec2;
use statig::prelude::*;
use statig::StateOrSuperstate;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Event {
    Grounded,
    Airborne,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Platformer {
    pub last_frame_force: Vec2,
    pub last_frame_speed: Vec2,

    pub move_speed: f32,
    pub run_speed: f32,
    pub jump_force: f32,
    pub fall_speed: f32,
    pub dash_force: f32,
    pub dash_duration: u64,
    pub air_jump_counter: u64,
    pub coyote_duration: u64,
    pub jump_buffer_duration: u64,

    pub is_grounded: bool,
    pub on_wall: bool,
    pub jump_active: bool,
    pub dash_active: bool,
    pub coyote_time_active: bool,
    pub jump_buffer_active: bool,
}

impl Default for Platformer {
    fn default() -> Self {
        Self {
            last_frame_force: Vec2::ZERO,
            last_frame_speed: Vec2::ZERO,

            move_speed: 200.0,
            run_speed: 350.0,
            jump_force: 3500.0,
            fall_speed: 150.0,
            dash_force: 500.0,
            dash_duration: 200,
            air_jump_counter: 2,
            coyote_duration: 150,
            jump_buffer_duration: 150,

            is_grounded: false,
            on_wall: false,
            jump_active: false,
            dash_active: false,
            coyote_time_active: false,
            jump_buffer_active: false,
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
impl Platformer {
    fn on_transition(&mut self, source: &State, target: &State) {
        println!("transitioned from `{source:?}` to `{target:?}`");
    }

    fn on_dispatch(&mut self, state: StateOrSuperstate<Self>, event: &Event) {
        on_dispatch(self, state, event);
    }

    #[state]
    fn idle(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Grounded => Handled,
            _ => Super,
        }
    }

    // #[superstate]
    // fn grounded(&mut self, event: &Event) -> Response<State> {
    //     match event {
    //         Event::Idle => Transition(State::idle()),
    //         Event::Walk => Transition(State::walk()),
    //         Event::Run => Transition(State::run()),
    //         Event::Dash => Transition(State::dash()),
    //         Event::Crouch => Transition(State::crouch()),
    //         Event::Push => Transition(State::push()),
    //         Event::Pull => Transition(State::pull()),
    //         _ => Super,
    //     }
    // }
    //
    // #[superstate]
    // fn jump(&mut self, event: &Event) -> Response<State> {
    //     match event {
    //         Event::Push => Transition(State::push()),
    //         Event::Pull => Transition(State::pull()),
    //         _ => Super,
    //     }
    // }
    //
    // #[superstate]
    // fn attack(&mut self, event: &Event) -> Response<State> {
    //     match event {
    //         Event::Push => Transition(State::push()),
    //         Event::Pull => Transition(State::pull()),
    //         _ => Super,
    //     }
    // }
    //
    // #[state(superstate = "grounded")]
    // fn idle(&mut self, event: &Event) -> Response<State> {
    //     match event {
    //         Event::Idle => Handled,
    //         _ => Super,
    //     }
    // }
    //
    // #[state(superstate = "grounded")]
    // fn walk(&mut self, event: &Event) -> Response<State> {
    //     match event {
    //         Event::Walk => Handled,
    //         _ => Super,
    //     }
    // }
    //
    // #[state(superstate = "grounded")]
    // fn run(&mut self, event: &Event) -> Response<State> {
    //     match event {
    //         Event::Run => Handled,
    //         _ => Super,
    //     }
    // }
    //
    // #[state(superstate = "grounded")]
    // fn crouch(&mut self, event: &Event) -> Response<State> {
    //     match event {
    //         Event::Crouch => Handled,
    //         _ => Super,
    //     }
    // }
    //
    // #[state(superstate = "grounded")]
    // fn dash(&mut self, event: &Event) -> Response<State> {
    //     match event {
    //         Event::Dash => Handled,
    //         _ => Super,
    //     }
    // }
    //
    // #[state(superstate = "attack")]
    // fn push(&mut self, event: &Event) -> Response<State> {
    //     match event {
    //         Event::Push => Handled,
    //         _ => Super,
    //     }
    // }
    //
    // #[state(superstate = "attack")]
    // fn pull(&mut self, event: &Event) -> Response<State> {
    //     match event {
    //         Event::Pull => Handled,
    //         _ => Super,
    //     }
    // }
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
        let mut state_machine = Platformer::default().state_machine().init();
        state_machine.handle(&Event::Grounded);
        //  event should be Idle
        // println!("State: {:?}", state_machine.state()); // State: Idle
        // assert_eq!(format!("{:?}", state_machine.state()), "Idle");
    }
}

