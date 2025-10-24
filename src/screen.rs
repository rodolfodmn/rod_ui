use crate::button::Button;
use std::io::{self, Write};

pub struct Screen {
    buttons: Vec<Button>,
    focused_index: Option<usize>,
}

impl Screen {
    /// Cria uma nova tela
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
            focused_index: None,
        }
    }

    /// Adiciona um botão à tela
    pub fn add_button(&mut self, button: Button) -> usize {
        self.buttons.push(button);
        self.buttons.len() - 1
    }

    /// Limpa a tela
    pub fn clear(&self) {
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap();
    }

    /// Desenha todos os botões
    pub fn draw(&self) {
        for button in &self.buttons {
            button.draw();
        }
    }

    /// Move o foco para o próximo botão
    pub fn focus_next(&mut self) {
        if self.buttons.is_empty() {
            return;
        }

        // Remove foco do botão atual
        if let Some(idx) = self.focused_index {
            self.buttons[idx].focused = false;
        }

        // Calcula próximo índice
        let next = match self.focused_index {
            None => 0,
            Some(idx) => (idx + 1) % self.buttons.len(),
        };

        self.focused_index = Some(next);
        self.buttons[next].focused = true;
    }

    /// Move o foco para o botão anterior
    pub fn focus_prev(&mut self) {
        if self.buttons.is_empty() {
            return;
        }

        if let Some(idx) = self.focused_index {
            self.buttons[idx].focused = false;
        }

        let prev = match self.focused_index {
            None => self.buttons.len() - 1,
            Some(0) => self.buttons.len() - 1,
            Some(idx) => idx - 1,
        };

        self.focused_index = Some(prev);
        self.buttons[prev].focused = true;
    }

    /// Retorna o índice do botão focado
    pub fn get_focused(&self) -> Option<usize> {
        self.focused_index
    }
}
