use std::io;
use std::thread::current;
use ratatui::{
    prelude::*,
    widgets::*
};
use crossterm::event;
use crossterm::event::{KeyCode, EnableMouseCapture, DisableMouseCapture, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use chrono::{DateTime, Datelike, Duration, Local, NaiveDateTime, TimeZone};
use crate::model::event::{Event, Recurrence};
use crate::utils::color::parse_color;

struct EventForm {
    title: String,
    start: String,
    duration: String,
    color: String,
    recurrence: String,
    active_field: usize,
    error_message: Option<String>,
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
            error_message: None,
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
            .style(if self.active_field == 0 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            })
            .block(Block::default().title("Tytuł")
            .borders(Borders::ALL)
            .border_style(if self.active_field == 0 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            })
        );
        f.render_widget(title, chunks[0]);

        let start = Paragraph::new(self.start.as_str())
            .style(if self.active_field == 1 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            })
            .block(Block::default().title("Data rozpoczęcia (RRRR-MM-DD HH:MM)")
            .borders(Borders::ALL)
            .border_style(if self.active_field == 1 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            })
        );
        f.render_widget(start, chunks[1]);

        let duration = Paragraph::new(self.duration.as_str())
            .style(if self.active_field == 2 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            })
            .block(Block::default().title("Czas trwania w godzinach")
            .borders(Borders::ALL)
            .border_style(if self.active_field == 2 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            })
        );
        f.render_widget(duration, chunks[2]);

        let color = Paragraph::new(self.color.as_str())
            .style(if self.active_field == 3 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            })
            .block(Block::default().title("Kolor (opcjonalnie)")
            .borders(Borders::ALL)
            .border_style(if self.active_field == 3 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            })
        );
        f.render_widget(color, chunks[3]);

        let recurrence = Paragraph::new(self.recurrence.as_str())
            .style(if self.active_field == 4 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            })
            .block(Block::default().title("Powtarzanie (None/Daily/Weekly/etc.)")
            .borders(Borders::ALL)
            .border_style(if self.active_field == 4 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            })
        );
        f.render_widget(recurrence, chunks[4]);

        if let Some(ref msg) = self.error_message {
            let error_area = Rect {
                x: area.x,
                y: area.y + area.height.saturating_sub(3),
                width: area.width,
                height: 3,
            };
            let paragraph = Paragraph::new(msg.as_str())
                .style(Style::default().fg(Color::Red))
                .block(Block::default().title("Błąd").borders(Borders::ALL));
            f.render_widget(paragraph, error_area);
        }
    }

    fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {
                self.error_message = None;
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
                self.error_message = None;
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
                self.active_field = (self.active_field + 1) % 5;
            }
            KeyCode::Up => {
                if self.active_field > 0 {
                    self.active_field -= 1;
                }
            }
            KeyCode::Down => {
                if self.active_field < 4 {
                    self.active_field += 1;
                }
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

    fn from_event(event: &Event) -> Self {
        Self {
            title: event.title.clone(),
            start: event.start.format("%Y-%m-%d %H:%M").to_string(),
            duration: ((event.end - event.start).num_hours()).to_string(),
            color: event.color.clone().unwrap_or_default(),
            recurrence: format!("{:?}", event.recurrence),
            active_field: 0,
            error_message: None,
        }
    }

    fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Tytuł nie może być pusty.".to_string());
        }

        if NaiveDateTime::parse_from_str(&self.start, "%Y-%m-%d %H:%M").is_err() {
            return Err("Niepoprawny format daty rozpoczęcia.".to_string());
        }

        let hours: i64 = self.duration.parse().map_err(|_| "Czas trwania musi być liczbą.")?;
        if hours <= 0 {
            return Err("Czas trwania musi być większy niż 0.".to_string());
        }

        let allowed = ["none", "daily", "weekly", "biweekly", "monthly", "yearly"];
        if !self.recurrence.trim().is_empty()
            && !allowed.contains(&self.recurrence.to_lowercase().as_str()) {
                return Err("Niepoprawna wartość pola 'Powtarzanie'.".to_string());
        }

        Ok(())
    }
}

pub fn run_ui(events: &mut Vec<Event>) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    #[derive(PartialEq)]
    enum ViewMode {
        All,
        Week,
        Month,
        Year,
    }

    let mut show_form = false;
    let mut form = EventForm::new();
    let mut selected: usize = 0;
    let mut edit_index: Option<usize> = None;
    let mut sort_asc = true;
    let mut show_only_upcoming = false;
    let mut filter_color: Option<String> = None;
    let mut view_mode = ViewMode::All;
    let mut current_date = Local::now().date_naive();

    loop {
        let now = Local::now();
        let future_limit = Local::now() + Duration::weeks(200);
        let mut expanded: Vec<Event> = Vec::new();

        let mut index_map: Vec<(usize, Event)> = Vec::new();
        for (idx, event) in events.iter().enumerate() {
            let occurrences = event.generate_occurrences(future_limit);
            index_map.extend(occurrences.into_iter().map(|e| (idx, e)));
        }

        let visible_events: Vec<(usize, &Event)> = index_map
            .iter()
            .enumerate()
            .filter(|(_, (_, e))| {
                let matches_view = match view_mode {
                    ViewMode::All => true,
                    ViewMode::Week => {
                        let week_start = current_date
                            - chrono::Duration::days(current_date.weekday().num_days_from_monday() as i64);
                        let week_end = week_start + chrono::Duration::days(6);
                        e.start.date_naive() >= week_start && e.start.date_naive() <= week_end
                    }
                    ViewMode::Month => {
                        e.start.year() == current_date.year()
                            && e.start.month() == current_date.month()
                    }
                    ViewMode::Year => e.start.year() == current_date.year(),
                };

                matches_view
                    && (!show_only_upcoming || e.start >= now)
                    && (filter_color.is_none() || e.color.as_deref() == filter_color.as_deref())
                
            })
            .map(|(_, (original_index, e))| (*original_index, e))
            .collect();

        let mut visible_events = visible_events;
        visible_events.sort_by_key(|(_, e)| e.start);
        if !sort_asc {
            visible_events.reverse();
        }

        terminal.draw(|f| {
            let size = f.area();

            if show_form {
                let form_area = Rect::new(0, 0, size.width, size.height);
                form.render(f, form_area);
            } else {
                let items: Vec<ListItem> = visible_events.iter().map(|(_, e)| {
                    let style = if let Some(ref c) = e.color {
                        Style::default().fg(parse_color(c))
                    } else {
                        Style::default()
                    };

                    ListItem::new(format!(
                        "{}\n{} - {}\n",
                        e.title,
                        e.start.format("%Y-%m-%d %H:%M"),
                        e.end.format("%Y-%m-%d %H:%M")
                    )).style(style)
                }).collect();

                let mut state = ListState::default();
                if !visible_events.is_empty() {
                    state.select(Some(selected.min(visible_events.len() - 1)));
                }

                let list = List::new(items)
                    .block(Block::default().title("Wydarzenia").borders(Borders::ALL))
                    .highlight_style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">> ");

                let info = Paragraph::new(format!(
                    "[s] Sortowanie: {} | [f] Filtrowanie: {} | [c] Kolor: {} | [v] Widok: {}",
                    if sort_asc { "Rosnąco" } else { "Malejąco" },
                    if show_only_upcoming { "Tylko nadchodzące" } else { "Wszystkie" },
                    filter_color.clone().unwrap_or("Wszystkie".to_string()),
                    match view_mode {
                        ViewMode::All => "Wszystko",
                        ViewMode::Week => "Tydzień",
                        ViewMode::Month => "Miesiąc",
                        ViewMode::Year => "Rok",
                    }
                ))
                .style(Style::default().fg(Color::Gray));
                f.render_widget(info, Rect::new(0, 0, size.width, 1));

                let view_info = Paragraph::new(match view_mode {
                    ViewMode::All => "Widok: Ogólny".to_string(),
                    ViewMode::Week => {
                        let start = current_date - chrono::Duration::days(current_date.weekday().num_days_from_monday() as i64);
                        let end = start + chrono::Duration::days(6);
                        format!("Widok: Tydzień {} - {}", start.format("%Y-%m-%d"), end.format("%Y-%m-%d"))
                    }
                    ViewMode::Month => format!("Widok: Miesiąc {}", current_date.format("%Y-%m")),
                    ViewMode::Year => format!("Widok: Rok {}", current_date.format("%Y")),
                })
                .style(Style::default().fg(Color::Blue));

                f.render_widget(view_info, Rect::new(0, 1, size.width, 1));

                let list_area = Rect::new(0, 2, size.width, size.height - 2);
                f.render_stateful_widget(list, list_area, &mut state);
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
                            edit_index = None;
                        }
                        KeyCode::Enter => {
                            if form.active_field == 4 {
                                match form.validate() {
                                    Ok(()) => {
                                        if let Some(event) = form.get_event() {
                                            if let Some(index) = edit_index {
                                                events[index] = event;
                                                edit_index = None;
                                            } else {
                                                events.push(event);
                                                selected = events.len().saturating_sub(1);
                                            }
                                            crate::storage::save_events(events)?;
                                            show_form = false;
                                            form = EventForm::new();
                                        } else {
                                            form.error_message = Some("Nie udało się utworzyć wydarzenia.".to_string());
                                        }
                                    }
                                    Err(msg) => {
                                        form.error_message = Some(msg);
                                    }
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
                        KeyCode::Char('d') => {
                            if !visible_events.is_empty() && selected < visible_events.len() {
                                let index_in_events = visible_events[selected].0;
                                let recurring = events[index_in_events].recurrence != Recurrence::None;

                                if recurring {
                                    let event_to_remove = events[index_in_events].clone();
                                    events.retain(|e| e != &event_to_remove);
                                } else {
                                    events.remove(index_in_events);
                                }

                                crate::storage::save_events(events)?;
                                if events.is_empty() {
                                    selected = 0;
                                } else if selected >= visible_events.len() {
                                    selected = visible_events.len().saturating_sub(1);
                                }
                            }
                        }
                        KeyCode::Char('e') => {
                            if !visible_events.is_empty() && selected < visible_events.len() {
                                let index_in_events = visible_events[selected].0;
                                let original_event = &events[index_in_events];

                                form = EventForm::from_event(original_event);
                                show_form = true;
                                edit_index = Some(index_in_events);
                            }
                        }
                        KeyCode::Char('s') => {
                            sort_asc = !sort_asc;
                        }
                        KeyCode::Char('f') => {
                            show_only_upcoming = !show_only_upcoming;
                            selected = 0;
                        }
                        KeyCode::Char('c') => {
                            let mut colors: Vec<String> = events.iter()
                                .filter_map(|e| e.color.clone())
                                .collect();
                            colors.sort();
                            colors.dedup();

                            if colors.is_empty() {
                                filter_color = None;
                            } else {
                                let current = filter_color.clone();
                                let next = match current {
                                    None => Some(colors[0].clone()),
                                    Some(curr) => {
                                        let idx = colors.iter().position(|c| *c == curr);
                                        match idx {
                                            Some(i) if i + 1 < colors.len() => Some(colors[i + 1].clone()),
                                            _ => None,
                                        }
                                    }
                                };
                                filter_color = next;
                                selected = 0;
                            }
                        }
                        KeyCode::Char('v') => {
                            view_mode = match view_mode {
                                ViewMode::All => ViewMode::Week,
                                ViewMode::Week => ViewMode::Month,
                                ViewMode::Month => ViewMode::Year,
                                ViewMode::Year => ViewMode::All,
                            };
                            selected = 0;
                        }
                        KeyCode::Left => {
                            match view_mode {
                                ViewMode::Week => current_date -= chrono::Duration::weeks(1),
                                ViewMode::Month => current_date = current_date.with_day(1).unwrap()
                                    - chrono::Duration::days(1),
                                ViewMode::Year => current_date = current_date
                                    .with_month(1).unwrap()
                                    .with_day(1).unwrap()
                                    - chrono::Duration::days(1),
                                _ => {},
                            }
                        }
                        KeyCode::Right => {
                            match view_mode {
                                ViewMode::Week => current_date += chrono::Duration::weeks(1),
                                ViewMode::Month => current_date = current_date.with_day(1).unwrap()
                                    + chrono::Duration::days(32),
                                ViewMode::Year => current_date = current_date
                                    .with_month(1).unwrap()
                                    .with_day(1).unwrap()
                                    + chrono::Duration::days(366),
                                _ => {}, 
                            }
                        }
                        KeyCode::Up => {
                            if selected > 0 {
                                selected -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if selected + 1 < visible_events.len() {
                                selected += 1;
                            }
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