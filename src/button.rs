use std::io::{self, Write};

pub struct Button {
    pub label: String,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub focused: bool,
    pub action_key: Option<char>,
}

impl Button {
    pub fn new(label: &str, x: u16, y: u16) -> Self {
        let width = label.len() as u16 + 4;
        Self {
            label: label.to_string(),
            x,
            y,
            width,
            focused: false,
            action_key: None,
        }
    }

    pub fn with_action_key(mut self, key: char) -> Self {
        self.action_key = Some(key);
        self
    }

    pub fn set_focus(&mut self, focused: bool) {
        self.focused = focused;
    }

    pub fn draw(&self) {
        let reset = "\x1B[0m";

        let (border_top, border_mid, border_bot, text_color) = if self.focused {
            (
                "\x1B[38;2;74;222;128m",
                "\x1B[38;2;34;197;94m",
                "\x1B[38;2;22;163;74m",
                "\x1B[1;97m",
            )
        } else {
            (
                "\x1B[38;2;100;116;139m",
                "\x1B[38;2;71;85;105m",
                "\x1B[38;2;51;65;85m",
                "\x1B[37m",
            )
        };

        print!("\x1B[{};{}H", self.y, self.x);
        print!(
            "{}┌{}┐{}",
            border_top,
            "─".repeat(self.width as usize),
            reset
        );

        print!("\x1B[{};{}H", self.y + 1, self.x);

        // Mostra a action key no label se existir
        let display_label = if let Some(key) = self.action_key {
            format!("{} ({})", self.label, key)
        } else {
            self.label.clone()
        };

        print!(
            "{}│{}{:^width$}{}│{}",
            border_mid,
            text_color,
            display_label,
            border_mid,
            reset,
            width = self.width as usize
        );

        print!("\x1B[{};{}H", self.y + 2, self.x);
        print!(
            "{}└{}┘{}",
            border_bot,
            "─".repeat(self.width as usize),
            reset
        );

        io::stdout().flush().unwrap();
    }

    pub fn is_hovered(&self, cursor_x: u16, cursor_y: u16) -> bool {
        cursor_x >= self.x
            && cursor_x <= self.x + self.width + 1
            && cursor_y >= self.y
            && cursor_y <= self.y + 2
    }

    pub fn matches_key(&self, key: char) -> bool {
        self.action_key == Some(key.to_ascii_lowercase())
    }
}
