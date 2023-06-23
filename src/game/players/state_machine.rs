use statig::prelude::*;
use bevy::math::Vec2;
use bevy::prelude::{Reflect, Component};

#[derive(Debug, Clone, Reflect, Component)]
pub enum Event {
    StartWalking,
    StopWalking,
    StartRunning,
    StopRunning,
    Jumping,
    Landing,
    Falling,
    StartDashing,
    StopDashing,
    StartCrouching,
    StopCrouching,
    StartAttacking,
    StopAttacking,
    Empty,
}

#[derive(Debug, Clone, Reflect, Component)]
pub struct ExternalContext {
    pub ground_detected: bool,
    pub wall_detected: bool,
    // pub external_velocity: Vec2,
}


#[derive(Debug, Default)]
pub struct CharacterController {
    pub attack_duration: f32,
    pub speed: Vec2,
}

#[state_machine(
    initial = "State::idle()",
    state(derive(Debug)),
    superstate(derive(Debug)),
    on_transition = "Self::on_transition",
    on_dispatch = "Self::on_dispatch"
)]
impl CharacterController {
    #[state(superstate="grounded")]
    fn idle(&mut self, context: &mut ExternalContext, event: &Event) -> Response<State> {
        self.speed = Vec2::ZERO;
        match event {
            _ if !context.ground_detected => Transition(State::fall()),
            _ => Super
        }
    }

    #[action]
    fn enter_walk(&mut self){
        self.speed.x = 200.0;
    }

    #[state(superstate="grounded", entry_action="enter_walk")]
    fn walk(&mut self, event: &Event) -> Response<State> {
        // TODO(MO): use min to slowly go up to max walk speed
        match event {
            Event::StartRunning => Transition(State::run()),
            Event::Jumping => Transition(State::jump()),
            Event::StartDashing => Transition(State::dash()),
            Event::StartCrouching => Transition(State::crouch()),
            Event::StopWalking => Super,
            _ => Handled
        }
    }

    #[action]
    fn enter_run(&mut self){
        self.speed.x = 400.0;
    }

    #[state(superstate="grounded", entry_action="enter_run")]
    fn run(&mut self, event: &Event) -> Response<State> {
        // TODO(MO): use min to slowly go up to max run speed
        match event {
            Event::Jumping => Transition(State::jump()),
            Event::StartDashing => Transition(State::dash()),
            Event::StartCrouching => Transition(State::crouch()),
            Event::StopRunning => Super,
            _ => Handled
        }
    }

    #[state(superstate="grounded")]
    fn crouch(&mut self, event: &Event) -> Response<State> {
        self.speed = Vec2::ZERO;
        match event {
            Event::StopCrouching => Super,
            _ => Handled
        }
    }

    #[action]
    fn enter_dash(&mut self) {
        self.speed.x = 600.0;
    }

    #[state(superstate="grounded", entry_action="enter_dash")]
    fn dash(&mut self, event: &Event) -> Response<State> {
        self.speed.x -= 10.0;

        match event {
            Event::StopDashing => Super,
            _ if self.speed.x <= 0.0 => Super,
            _ => Handled,
        }
    }

    #[action]
    fn enter_jump(&mut self){
        self.speed.y = 600.0;
    }

    #[state(superstate="grounded", entry_action="enter_jump")]
    fn jump(&mut self, event: &Event) -> Response<State> {
        self.speed.y -= 30.0;
        match event {
            Event::Falling => Super,
            _ if self.speed.y <= 0.0 => Transition(State::fall()),
            _ => Handled
        }
    }
    #[action]
    fn enter_attack(&mut self) {
        self.attack_duration = 50.0;
    }

    #[state(superstate="grounded", entry_action="enter_attack")]
    fn grounded_attack(&mut self, event: &Event) -> Response<State> {
        self.speed = Vec2::ZERO;
        self.attack_duration -= 1.0;
        match event {
            Event::StopAttacking => Super,
            _ if self.attack_duration <= 0.0 => Super,
            _ => Handled,
        }
    }

    #[state(superstate="airborne", entry_action="enter_attack")]
    fn airborne_attack(&mut self, event: &Event) -> Response<State> {
        self.speed = Vec2::ZERO;
        match event {
            Event::StopAttacking => Super,
            _ if self.attack_duration <= 0.0 => Super,
            _ => Handled,
        }
    }

    #[state(superstate="airborne")]
    fn fall(&mut self, context: &mut ExternalContext, event: &Event) -> Response<State> {
        let new_fall_speed: f32 = self.speed.y - 40.0;
        let max_fall_speed: f32 = -200.0;
        // choose fall speed max or new
        self.speed.y = max_fall_speed.max(new_fall_speed);
        match event {
            Event::Landing => Transition(State::idle()),
            _ if context.ground_detected => Transition(State::idle()),
            _ => Super
        }
    }

    #[superstate]
    fn grounded(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::StartWalking => Transition(State::walk()),
            Event::StartRunning => Transition(State::run()),
            Event::StartCrouching => Transition(State::crouch()),
            Event::StartDashing => Transition(State::dash()),
            Event::Jumping => Transition(State::jump()),
            Event::Falling => Transition(State::fall()),
            Event::StartAttacking => Transition(State::grounded_attack()),
            _ => Transition(State::idle()),
        }
    }

    #[superstate]
    fn airborne(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::StartAttacking => Transition(State::airborne_attack()),
            _ => Transition(State::fall())
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
