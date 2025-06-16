use chrono::{Duration, Local};
use crate::model::event::{Event, Recurrence};
use crate::storage;

pub struct App {
    pub events: Vec<Event>,
}

impl App {
    pub fn new() -> Self {
        let events = storage::load_events();

        if events.is_empty() {
            let now = Local::now();
            let event = Event {
                title: "Test".into(),
                start: now,
                end: now + Duration::hours(2),
                color: Some("blue".into()),
                recurrence: Recurrence::None,
            };
            Self {
                events: vec![event],
            }
        } else {
            Self { events }
        }
    }

    pub fn save(&self) {
        storage::save_events(&self.events);
    }
}