use statig::prelude::*;

#[derive(Debug, Clone)]
pub enum Event {
    Idle,
    MindMeld,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Candle {
    pub candle_scale: f32,
    pub candle_inactive_scale: f32,
    pub candle_active_scale: f32,
    pub psychic_range: f32
}

impl Default for Candle {
    fn default() -> Self {
        Self {
            candle_scale: 1.0,
            candle_inactive_scale: 1.0,
            candle_active_scale: 1.5,
            psychic_range: 10.0,
        }
    }
}

impl StateMachine for Candle {
    type State = State;

    type Superstate<'a> = Superstate;

    type Event = Event;

    const INIT_STATE: State = State::mind_meld();
}

#[state_machine]
impl Candle {
    #[state]
    fn idle(&mut self, event: &Event) -> Response<State> {
        self.candle_scale = self.candle_inactive_scale;
        match event {
            Event::MindMeld => {
                Transition(State::mind_meld())
            },
            _ => Super,
        }
    }

    #[state]
    fn mind_meld(&mut self, event: &Event) -> Response<State> {
        self.candle_scale = self.candle_active_scale;

        match event {
            Event::MindMeld => Handled,
            _ => Super,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_states() {
        let mut state_machine = Candle::default().state_machine().init();
        state_machine.handle(&Event::MindMeld);
        // println!("State: {:?}", state_machine.state()); // State: MindMeld
        // assert_eq!(format!("{:?}", state_machine.state()), "MindMeld");
    }
}
