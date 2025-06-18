use ratatui::style::Color;

pub fn parse_color(name: &str) -> Color {
    match name.to_lowercase().as_str() {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        "yellow" => Color::Yellow,
        "cyan" => Color::Cyan,
        "magenta" => Color::Magenta,
        "gray" => Color::Gray,
        "white" => Color::White,
        _ => Color::White,
    }
}