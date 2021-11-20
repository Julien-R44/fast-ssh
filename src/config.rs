use tui::style::Color;

pub struct Theme {
    pub primary_color: Color,
    pub secondary_color: Color,
}

pub struct Config {
    pub theme: Theme,
}

impl Config {}

pub fn resolve_config() -> Config {
    Config {
        theme: Theme {
            primary_color: Color::Red,
            secondary_color: Color::Green,
        },
    }
}
