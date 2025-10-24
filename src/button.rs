use std::io::{self, Write};

pub struct Button {
    pub label: String,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub focused: bool,
}

impl Button {
    /// Cria um novo botão
    pub fn new(label: &str, x: u16, y: u16) -> Self {
        let width = label.len() as u16 + 4; // padding
        Self {
            label: label.to_string(),
            x,
            y,
            width,
            focused: false,
        }
    }

    /// Define se o botão está focado
    pub fn set_focus(&mut self, focused: bool) {
        self.focused = focused;
    }

    /// Desenha o botão na tela
    pub fn draw(&self) {
        let reset = "\x1B[0m";

        // Cores diferentes para estado focado/normal
        let (border_top, border_mid, border_bot, text_color) = if self.focused {
            (
                "\x1B[38;2;74;222;128m", // verde claro
                "\x1B[38;2;34;197;94m",  // verde médio
                "\x1B[38;2;22;163;74m",  // verde escuro
                "\x1B[1;97m",            // branco bold
            )
        } else {
            (
                "\x1B[38;2;100;116;139m", // cinza
                "\x1B[38;2;71;85;105m",
                "\x1B[38;2;51;65;85m",
                "\x1B[37m", // branco normal
            )
        };

        // Linha superior
        print!("\x1B[{};{}H", self.y, self.x);
        print!(
            "{}┌{}┐{}",
            border_top,
            "─".repeat(self.width as usize),
            reset
        );

        // Linha do meio com label
        print!("\x1B[{};{}H", self.y + 1, self.x);
        let padding = (self.width as usize - self.label.len()) / 2;
        print!(
            "{}│{}{:^width$}{}│{}",
            border_mid,
            text_color,
            self.label,
            border_mid,
            reset,
            width = self.width as usize
        );

        // Linha inferior
        print!("\x1B[{};{}H", self.y + 2, self.x);
        print!(
            "{}└{}┘{}",
            border_bot,
            "─".repeat(self.width as usize),
            reset
        );

        io::stdout().flush().unwrap();
    }

    /// Verifica se o botão foi clicado (placeholder para implementação futura)
    pub fn is_hovered(&self, cursor_x: u16, cursor_y: u16) -> bool {
        cursor_x >= self.x
            && cursor_x <= self.x + self.width + 1
            && cursor_y >= self.y
            && cursor_y <= self.y + 2
    }
}
