// @url https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=ee3e4df093c136ced7b394dc7ffb78e1

#[derive(Debug, PartialEq)]
enum State {
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
            (State::Waiting { waiting_time }, Event::NoEvent) => State::Waiting {
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

#[derive(Debug, Clone, Copy)]
enum Event {
    NoEvent,
    BottleInserted,
    BottleFull,
    BottleEjected,
}

fn main() {
    let mut state = State::new();

    // We'll iterate over a series of events instead
    // of simulating events over a period of time
    let events = [
        Event::NoEvent,
        Event::NoEvent,
        Event::BottleInserted,
        Event::BottleFull,
        Event::BottleEjected,
        Event::NoEvent,
        Event::BottleFull,
    ];

    for event in events.into_iter() {
        print!("-- Transitioning from {:?}", state);
        state = state.next(event);
        print!(" into {:?}", state);
        println!();
        state.run();
    }
}
