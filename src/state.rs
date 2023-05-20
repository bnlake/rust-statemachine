use crate::event::Event;

#[derive(Debug, PartialEq)]
pub enum State {
    Waiting { waiting_time: u8 },
    Filling { rate: u8 },
    Done,
}

impl State {
    pub fn new() -> Self {
        State::Waiting { waiting_time: 0 }
    }

    pub fn next(self, event: Event) -> Self {
        // Make a tuple and use match arms to control transitions
        match (self, event) {
            (State::Waiting { waiting_time }, Event::None) => State::Waiting {
                waiting_time: waiting_time + 1,
            },
            (State::Waiting { .. }, Event::BottleInserted) => State::Filling { rate: 10 },
            (State::Filling { .. }, Event::BottleFull) => State::Done,
            (State::Done, Event::BottleEjected) => State::Waiting { waiting_time: 0 },
            // Any other state and event are outside of the state machine boundaries
            (s, e) => {
                panic!("Invalid state and event: {:#?} {:#?}", s, e)
            }
        }
    }

    pub fn run(&self) {
        match self {
            State::Waiting { waiting_time } => {
                println!(
                    "We've been waiting for a bottle for {} events now",
                    waiting_time
                )
            }
            State::Filling { rate } => {
                println!("We're filling the bottle at a rate of {}", rate)
            }
            State::Done => {
                println!("Done filling")
            }
        }
    }
}
