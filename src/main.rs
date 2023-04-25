// @url https://hoverbear.org/blog/rust-state-machine-pattern/

struct BottleFillingMachine<S> {
    shared_value: usize,
    state: S,
}

struct Waiting {
    waiting_time: std::time::Duration,
}
impl From<BottleFillingMachine<Done>> for BottleFillingMachine<Waiting> {
    fn from(value: BottleFillingMachine<Done>) -> Self {
        BottleFillingMachine::new(0)
    }
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

// These enum values are not to be confused with the
// various state structs
enum BottleFillingMachineWrapper {
    Waiting(BottleFillingMachine<Waiting>),
    Filling(BottleFillingMachine<Filling>),
    Done(BottleFillingMachine<Done>),
}
impl BottleFillingMachineWrapper {
    pub fn step(mut self) -> Self {
        match self {
            BottleFillingMachineWrapper::Waiting(val) => {
                BottleFillingMachineWrapper::Filling(val.into())
            }
            BottleFillingMachineWrapper::Filling(val) => {
                BottleFillingMachineWrapper::Done(val.into())
            }
            BottleFillingMachineWrapper::Done(val) => {
                BottleFillingMachineWrapper::Waiting(val.into())
            }
        }
    }
}

struct BottleFillingMachineFactory {
    bottle_filling_machine: BottleFillingMachineWrapper,
}
impl BottleFillingMachineFactory {
    pub fn new() -> Self {
        BottleFillingMachineFactory {
            bottle_filling_machine: BottleFillingMachineWrapper::Waiting(
                BottleFillingMachine::new(0),
            ),
        }
    }
}

fn main() {
    let mut state_machine_factory = BottleFillingMachineFactory::new();

    state_machine_factory.bottle_filling_machine =
        state_machine_factory.bottle_filling_machine.step();
}
