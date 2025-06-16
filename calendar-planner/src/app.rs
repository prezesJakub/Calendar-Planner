use chrono::{Duration, Local};
use crate::model::event::{Event, Recurrence};

pub struct App {
    pub events: Vec<Event>,
}

impl App {
    pub fn new() -> Self {
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
    }
}