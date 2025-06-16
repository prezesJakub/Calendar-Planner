use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Recurrence {
    None,
    Daily,
    Weekly,
    Biweekly,
    Monthly,
    Yearly
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub title: String,
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    pub color: Option<String>,
    pub recurrence: Recurrence,
}