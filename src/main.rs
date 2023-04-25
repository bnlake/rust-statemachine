// @url https://hoverbear.org/blog/rust-state-machine-pattern/

struct BottleFillingMachine<S> {
    shared_value: usize,
    state: S,
}

struct Waiting {
    waiting_time: std::time::Duration,
}
impl BottleFillingMachine<Waiting> {
    pub fn new(shared_value: usize) -> Self {
        BottleFillingMachine {
            shared_value,
            state: Waiting {
                waiting_time: std::time::Duration::new(0, 0),
            },
        }
    }
}

struct Filling {
    rate: usize,
}
impl From<BottleFillingMachine<Waiting>> for BottleFillingMachine<Filling> {
    fn from(value: BottleFillingMachine<Waiting>) -> Self {
        BottleFillingMachine {
            shared_value: value.shared_value,
            state: Filling { rate: 1 },
        }
    }
}

struct Done;
impl From<BottleFillingMachine<Filling>> for BottleFillingMachine<Done> {
    fn from(value: BottleFillingMachine<Filling>) -> Self {
        BottleFillingMachine {
            shared_value: value.shared_value,
            state: Done,
        }
    }
}

fn main() {
    let in_waiting = BottleFillingMachine::<Waiting>::new(0);
    let in_filling = BottleFillingMachine::<Filling>::from(in_waiting);
}
