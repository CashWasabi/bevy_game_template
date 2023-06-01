use statig::prelude::*;
use bevy::math::Vec2;
use bevy::prelude::{Reflect, Component};

#[derive(Debug)]
pub enum State {
    Idle,
    Walk,
    Run,
    Crouch,
    Jump,
    Dash,
    GroundedAttack,
    AirborneAttack,
    Fall,
}

#[derive(Debug)]
pub enum Superstate {
    Grounded,
    Airborne,
}

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

#[derive(Default)]
pub struct CharacterController {
    pub speed: Vec2,
}

impl IntoStateMachine for CharacterController {
    type State = State;

    type Superstate<'sub> = Superstate;

    type Event<'evt> = Event;

    type Context<'ctx> = ();

    const INITIAL: State = State::Idle;

    const ON_TRANSITION: fn(&mut Self, &Self::State, &Self::State) = |_shared, source, target| {
        println!("Transitioning from {:?} to {:?}", source, target);
    };}

impl blocking::State<CharacterController> for State {
    fn call_handler(
        &mut self,
        character_controller: &mut CharacterController,
        event: &Event,
        _: &mut ()
    ) -> Response<Self> {
        match self {
            State::Idle => character_controller.idle(event),
            State::Walk => character_controller.walk(event),
            State::Run => character_controller.run(event),
            State::Crouch => character_controller.crouch(event),
            State::Jump => character_controller.jump(event),
            State::Dash => character_controller.dash(event),
            State::GroundedAttack => character_controller.grounded_attack(event),
            State::AirborneAttack => character_controller.airborne_attack(event),
            State::Fall => character_controller.fall(event),
        }
    }

    fn superstate(&mut self) -> Option<Superstate> {
        match self {
            State::Idle => Some(Superstate::Grounded),
            State::Walk => Some(Superstate::Grounded),
            State::Run => Some(Superstate::Grounded),
            State::Crouch => Some(Superstate::Grounded),
            State::Jump => Some(Superstate::Grounded),
            State::Dash => Some(Superstate::Grounded),
            State::GroundedAttack => Some(Superstate::Grounded),
            State::AirborneAttack => Some(Superstate::Airborne),
            State::Fall => Some(Superstate::Airborne),
        }
    }
}

impl blocking::Superstate<CharacterController> for Superstate {
    fn call_handler(&mut self, character_controller: &mut CharacterController, event: &Event, _: &mut ()) -> Response<State> {
        match self {
            Superstate::Grounded => character_controller.grounded(event),
            Superstate::Airborne => character_controller.airborne(event),
        }
    }
}

impl CharacterController {
    fn idle(&mut self, event: &Event) -> Response<State> {
        self.speed = Vec2::ZERO;
        match event {
            _ => Super
        }
    }

    fn walk(&mut self, event: &Event) -> Response<State> {
        self.speed = Vec2::new(100.0, 0.0);
        match event {
            _ => Super
        }
    }

    fn run(&mut self, event: &Event) -> Response<State> {
        self.speed = Vec2::new(200.0, 0.0);
        match event {
            _ => Super
        }
    }

    fn crouch(&mut self, event: &Event) -> Response<State> {
        self.speed = Vec2::new(50.0, 0.0);
        match event {
            _ => Super
        }
    }

    fn dash(&mut self, event: &Event) -> Response<State> {
        self.speed = Vec2::new(400.0, 0.0);
        match event {
            _ => Super
        }
    }

    fn jump(&mut self, event: &Event) -> Response<State> {
        match event {
            _ => Super
        }
    }

    fn grounded_attack(&mut self, event: &Event) -> Response<State> {
        self.speed = Vec2::ZERO;
        match event {
            _ => Super
        }
    }

    fn grounded(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Move => Transition(State::Walk),
            Event::Run => Transition(State::Run),
            Event::Crouch => Transition(State::Crouch),
            Event::Dash => Transition(State::Dash),
            Event::Jump => Transition(State::Jump),
            Event::Fall => Transition(State::Fall),
            Event::Attack => Transition(State::GroundedAttack),
            Event::Grounded => Transition(State::Idle),
            _ => Super
        }
    }

    fn fall(&mut self, event: &Event) -> Response<State> {
        match event {
            _ => Super
        }
    }

    fn airborne_attack(&mut self, event: &Event) -> Response<State> {
        self.speed = Vec2::ZERO;
        match event {
            _ => Super
        }
    }

    fn airborne(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Grounded => Transition(State::Idle),
            Event::Attack => Transition(State::AirborneAttack),
            Event::Airborne => Transition(State::Fall),
            _ => Super
        }
    }
}
