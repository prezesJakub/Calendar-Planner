use std::io;
use ratatui::{
    prelude::*,
    widgets::*
};
use crossterm::event;
use crossterm::event::{KeyCode, EnableMouseCapture, DisableMouseCapture, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use chrono::{DateTime, Local, NaiveDateTime, Duration, TimeZone};

use crate::model::event::{Event, Recurrence};

struct EventForm {
    title: String,
    start: String,
    duration: String,
    color: String,
    recurrence: String,
    active_field: usize,
}

impl EventForm {
    fn new() -> Self {
        Self {
            title: String::new(),
            start: String::new(),
            duration: String::new(),
            color: String::new(),
            recurrence: String::new(),
            active_field: 0,
        }
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(area);

        let title = Paragraph::new(self.title.as_str())
            .block(Block::default().title("Tytuł").borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        let start = Paragraph::new(self.start.as_str())
            .block(Block::default().title("Data rozpoczęcia (RRRR-MM-DD HH:MM)").borders(Borders::ALL));
        f.render_widget(start, chunks[1]);

        let duration = Paragraph::new(self.duration.as_str())
            .block(Block::default().title("Czas trwania w godzinach").borders(Borders::ALL));
        f.render_widget(duration, chunks[2]);

        let color = Paragraph::new(self.color.as_str())
            .block(Block::default().title("Kolor (opcjonalnie)").borders(Borders::ALL));
        f.render_widget(color, chunks[3]);

        let recurrence = Paragraph::new(self.recurrence.as_str())
            .block(Block::default().title("Powtarzanie (None/Daily/Weekly/etc.)").borders(Borders::ALL));
        f.render_widget(recurrence, chunks[4]);
    }

    fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {
                match self.active_field {
                    0 => self.title.push(c),
                    1 => self.start.push(c),
                    2 => self.duration.push(c),
                    3 => self.color.push(c),
                    4 => self.recurrence.push(c),
                    _ => {}
                }
            }
            KeyCode::Backspace => {
                match self.active_field {
                    0 => { self.title.pop(); }
                    1 => { self.start.pop(); }
                    2 => { self.duration.pop(); }
                    3 => { self.color.pop(); }
                    4 => { self.recurrence.pop(); }
                    _ => {}
                }
            }
            KeyCode::Enter => {
                self.active_field = (self.active_field + 1) % 5; // Przełącz na następne pole
            }
            _ => {}
        }
    }

    fn get_event(&self) -> Option<Event> {
        let naive = match NaiveDateTime::parse_from_str(&self.start, "%Y-%m-%d %H:%M") {
            Ok(dt) => dt,
            Err(_) => return None,
        };

        let start: DateTime<Local> = match Local.from_local_datetime(&naive).single() {
            Some(dt) => dt,
            None => return None,
        };

         let hours: i64 = match self.duration.parse() {
            Ok(h) => h,
            Err(_) => return None,
        };

        let end = start + Duration::hours(hours);

        let color = if self.color.is_empty() {
            None
        } else {
            Some(self.color.clone())
        };

        let recurrence = match self.recurrence.to_lowercase().as_str() {
            "daily" => Recurrence::Daily,
            "weekly" => Recurrence::Weekly,
            "biweekly" => Recurrence::Biweekly,
            "monthly" => Recurrence::Monthly,
            "yearly" => Recurrence::Yearly,
            _ => Recurrence::None,
        };

        Some(Event {
            title: self.title.clone(),
            start,
            end,
            color,
            recurrence,
        })
    }
}

pub fn run_ui(events: &mut Vec<Event>) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut show_form = false;
    let mut form = EventForm::new();

    loop {
        terminal.draw(|f| {
            let size = f.area();

            if show_form {
                let form_area = Rect::new(0, 0, size.width, size.height);
                form.render(f, form_area);
            } else {
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
            }
        })?;

        if let event::Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                let key = key_event.code;

                if show_form {
                    match key {
                        KeyCode::Esc => {
                            show_form = false;
                            form = EventForm::new();
                        }
                        KeyCode::Enter => {
                            if form.active_field == 4 {
                                if let Some(event) = form.get_event() {
                                    events.push(event);
                                    crate::storage::save_events(events)?;
                                    show_form = false;
                                    form = EventForm::new();
                                }
                            } else {
                                form.handle_input(key);
                            }
                        }
                        _ => {
                            form.handle_input(key);
                        }
                    }
                } else {
                    match key {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('a') => {
                            show_form = true;
                        }
                        _ => {}
                    }
                }
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