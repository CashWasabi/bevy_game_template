use bevy::prelude::{Vec2};
use statig::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PlayerStateMachine {
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

impl Default for PlayerStateMachine {
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

pub enum Event {
    Idle,
    Walk,
    Run,
    Jump,
    Crouch,
    Dash,
    Push,
    Pull
}

impl StateMachine for PlayerStateMachine { 
    type State = State;
    
    type Superstate<'a> = Superstate;
    
    type Event = Event;
    
    const INIT_STATE: State = State::idle();
}

// Transition(State::move())
// Super
// Handled
#[state_machine]
impl PlayerStateMachine {
    #[action]
    fn enter_grounded(&mut self, event: &Event) {
        // here we should remove command
    }

    #[action]
    fn exit_grounded(&mut self, event: &Event) {
        // here we should remove command
    }

    #[superstate(entry_action="enter_grounded", exit_action="exit_grounded")]
    fn grounded(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Idle => Transition(State::idle()),
            Event::Walk => Transition(State::walk()),
            Event::Run => Transition(State::run()),
            Event::Dash => Transition(State::dash()),
            Event::Crouch => Transition(State::crouch()),
            Event::Push => Transition(State::push()),
            Event::Pull => Transition(State::pull()),
            _ => Super
        }
    }

    #[action]
    fn enter_jump(&mut self, event: &Event) {}

    #[action]
    fn exit_jump(&mut self, event: &Event) {}

    #[superstate(entry_action="enter_jump", exit_action="exit_jump")]
    fn jump(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Push => Transition(State::push()),
            Event::Pull => Transition(State::pull()),
            _ => Super
        }
    }

    #[action]
    fn enter_attack(&mut self, event: &Event) {}

    #[action]
    fn exit_attack(&mut self, event: &Event) {}

    #[superstate(entry_action="enter_attack", exit_action="exit_attack")]
    fn attack(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Push => Transition(State::push()),
            Event::Pull => Transition(State::pull()),
            _ => Super
        }
    }

    #[state(superstate="grounded")]
    fn idle(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Idle => Handled,
            _ => Super,
        }
    }

    #[state(superstate="grounded")]
    fn walk(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Walk => Handled,
            _ => Super,
        }
    }

    #[state(superstate="grounded")]
    fn run(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Run => Handled,
            _ => Super,
        }
    }

    #[state(superstate="grounded")]
    fn crouch(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Crouch => Handled,
            _ => Super,
        }
    }

    #[action]
    fn enter_dash(&mut self, event: &Event) {}

    #[action]
    fn exit_dash(&mut self, event: &Event) {}

    #[state(superstate="grounded", entry_action="enter_dash", exit_action="exit_dash")]
    fn dash(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Dash => Handled,
            _ => Super,
        }
    }

    #[state(superstate="attack")]
    fn push(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Push => Handled,
            _ => Super,
        }
    }

    #[state(superstate="attack")]
    fn pull(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Pull => Handled,
            _ => Super,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_states() {
        let mut state_machine = PlayerStateMachine::default().state_machine().init();
        state_machine.handle(&Event::Idle);
        //  event should be Idle
        println!("State: {:?}", state_machine.state()); // State: Idle
        assert_eq!(format!("{:?}", state_machine.state()), "Idle");
    }
}
