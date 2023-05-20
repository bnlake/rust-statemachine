
#[derive(Debug, Clone, Copy)]
pub enum Event {
    None,
    BottleInserted,
    BottleFull,
    BottleEjected,
}