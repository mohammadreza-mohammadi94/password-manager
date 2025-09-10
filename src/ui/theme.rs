
use tui::style::Color;

#[derive(Clone)]
pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub error: Color,
    pub success: Color,
    pub warning: Color,
    pub border: Color,
    pub highlight_bg: Color,
    pub highlight_fg: Color,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::White,
            primary: Color::Cyan,
            secondary: Color::Yellow,
            accent: Color::Magenta,
            error: Color::Red,
            success: Color::Green,
            warning: Color::Yellow,
            border: Color::Gray,
            highlight_bg: Color::Blue,
            highlight_fg: Color::White,
        }
    }

    pub fn light() -> Self {
        Self {
            background: Color::White,
            foreground: Color::Black,
            primary: Color::Blue,
            secondary: Color::Green,
            accent: Color::Red,
            error: Color::Red,
            success: Color::Green,
            warning: Color::Yellow,
            border: Color::DarkGray,
            highlight_bg: Color::Yellow,
            highlight_fg: Color::Black,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}
