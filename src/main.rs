// @url https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=ee3e4df093c136ced7b394dc7ffb78e1

mod state;
mod event;

use event::Event;
use state::State;

fn main() {
    let mut state = State::new();

    // We'll iterate over a series of events instead
    // of simulating events over a period of time
    let events = [
        Event::None,
        Event::None,
        Event::BottleInserted,
        Event::BottleFull,
        Event::BottleEjected,
        Event::None,
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
