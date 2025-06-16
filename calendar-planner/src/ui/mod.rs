use std::io;
use ratatui::{prelude::*, widgets::*};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crate::model::event::Event;

pub fn run_ui(events: &[Event]) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let items: Vec<ListItem> = events.iter().map(|e| {
                ListItem::new(format!(
                    "{}\n{} - {}\n",
                    e.title,
                    e.start.format("%Y-%m-%d %H:%M"),
                    e.end.format("%Y-%m-%d %H:%M")
                ))
            }).collect();

            let list = List::new(items)
                .block(Block::default().title("Wydarzenia").borders(Borders::ALL))
                .highlight_symbol(">> ");

            f.render_widget(list, size);
        })?;

        if let CEvent::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}