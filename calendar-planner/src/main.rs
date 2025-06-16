mod app;
mod model;
mod ui;

use app::App;
use ui::run_ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new();
    run_ui(&app.events)?;
    Ok(())
}