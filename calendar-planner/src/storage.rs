use std::{fs::File, io::{BufReader, BufWriter}, path::Path};
use crate::model::event::Event;

const FILE_PATH: &str = "events.json";

pub fn load_events() -> Vec<Event> {
    if !Path::new(FILE_PATH).exists() {
        return Vec::new();
    }

    let file = File::open(FILE_PATH).expect("Nie udało się otworzyć pliku");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
}

pub fn save_events(events: &[Event]) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(FILE_PATH)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, events)?;
    Ok(())
}