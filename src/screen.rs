use crate::button::Button;
use std::io::{self, Read, Write, stdin};

pub struct Screen {
    buttons: Vec<Button>,
    focused_index: Option<usize>,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
            focused_index: None,
        }
    }

    pub fn add_button(&mut self, button: Button) -> usize {
        self.buttons.push(button);
        self.buttons.len() - 1
    }

    pub fn clear(&self) {
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap();
    }

    pub fn draw(&self) {
        for button in &self.buttons {
            button.draw();
        }
    }

    pub fn focus_next(&mut self) {
        if self.buttons.is_empty() {
            return;
        }

        if let Some(idx) = self.focused_index {
            self.buttons[idx].focused = false;
        }

        let next = match self.focused_index {
            None => 0,
            Some(idx) => (idx + 1) % self.buttons.len(),
        };

        self.focused_index = Some(next);
        self.buttons[next].focused = true;
    }

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

    pub fn focus_by_key(&mut self, key: char) -> bool {
        if let Some(idx) = self.focused_index {
            self.buttons[idx].focused = false;
        }

        for (idx, button) in self.buttons.iter_mut().enumerate() {
            if button.matches_key(key) {
                button.focused = true;
                self.focused_index = Some(idx);
                return true;
            }
        }

        if let Some(idx) = self.focused_index {
            self.buttons[idx].focused = true;
        }

        false
    }

    pub fn get_focused(&self) -> Option<usize> {
        self.focused_index
    }

    // Espera por uma tecla sem mostrar no terminal
    pub fn wait_for_key(&mut self) -> io::Result<char> {
        // Entra em modo raw
        Self::enable_raw_mode()?;

        let mut buffer = [0; 1];
        stdin().read_exact(&mut buffer)?;
        let key = buffer[0] as char;

        // Sai do modo raw
        Self::disable_raw_mode()?;

        Ok(key)
    }

    fn enable_raw_mode() -> io::Result<()> {
        use std::process::Command;
        Command::new("stty").args(&["raw", "-echo"]).status()?;
        Ok(())
    }

    fn disable_raw_mode() -> io::Result<()> {
        use std::process::Command;
        Command::new("stty").arg("sane").status()?;
        Ok(())
    }
}
