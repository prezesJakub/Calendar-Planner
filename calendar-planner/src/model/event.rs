use chrono::{DateTime, Duration, Local, Datelike, Months};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Recurrence {
    None,
    Daily,
    Weekly,
    Biweekly,
    Monthly,
    Yearly
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub title: String,
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    pub color: Option<String>,
    pub recurrence: Recurrence,
}

impl Event {
    pub fn generate_occurrences(&self, until: DateTime<Local>) -> Vec<Event> {
        let mut events = Vec::new();

        if self.recurrence == Recurrence::None {
            if self.start <= until {
                events.push(self.clone());
            }
            return events;
        }

        let mut current_start = self.start;
        let duration = self.end - self.start;

        while current_start <= until {
            let mut event = self.clone();
            event.start = current_start;
            event.end = current_start + duration;
            events.push(event);

            current_start = match self.recurrence {
                Recurrence::Daily => current_start + Duration::days(1),
                Recurrence::Weekly => current_start + Duration::weeks(1),
                Recurrence::Biweekly => current_start + Duration::weeks(2),
                Recurrence::Monthly => {
                    if let Some(new_start) = current_start.checked_add_months(Months::new(1)) {
                        new_start
                    } else {
                        break;
                    }
                }
                Recurrence::Yearly => {
                    match current_start.with_year(current_start.year() + 1) {
                        Some(new_date) => new_date,
                        None => break,
                    }
                }
                Recurrence::None => break,
            };
        }
        
        events
    }
}