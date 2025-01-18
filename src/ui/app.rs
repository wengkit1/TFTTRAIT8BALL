use ::crossterm::event::KeyCode;
pub struct App {
    pub should_quit: bool,
    pub selected_size: usize,
    pub input_buffer: String, // For handling number input
    pub input_mode: InputMode,
}

pub enum InputMode {
    Normal,
    Typing,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            selected_size: 7, // Default size
            input_buffer: String::new(),
            input_mode: InputMode::Normal,
        }
    }

    pub fn on_key(&mut self, key: crossterm::event::KeyEvent) {
        match self.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('q') => self.should_quit = true,
                KeyCode::Left => {
                    if self.selected_size > 7 {
                        self.selected_size -= 1;
                    }
                }
                KeyCode::Right => {
                    if self.selected_size < 10 {
                        self.selected_size += 1;
                    }
                }
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    self.input_mode = InputMode::Typing;
                    self.input_buffer.clear();
                    self.input_buffer.push(c);
                }
                _ => {}
            },
            InputMode::Typing => match key.code {
                KeyCode::Enter => {
                    if let Ok(size) = self.input_buffer.parse::<usize>() {
                        if (1..=10).contains(&size) {
                            self.selected_size = size;
                        }
                    }
                    self.input_buffer.clear();
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Esc => {
                    self.input_buffer.clear();
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    self.input_buffer.push(c);
                }
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                }
                _ => {}
            },
        }
    }
}
