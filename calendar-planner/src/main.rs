mod app;
mod model;
mod ui;
mod storage;
mod utils;

use ui::run_ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut events = crate::storage::load_events();
    run_ui(&mut events)?;
    crate::storage::save_events(&events)?;
    Ok(())
}