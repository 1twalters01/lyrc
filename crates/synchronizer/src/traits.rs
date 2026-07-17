use chrono::Duration;

pub trait Synchronizer {
    type Event;

    fn update(&mut self, position: Duration) -> Option<Self::Event>;
}
