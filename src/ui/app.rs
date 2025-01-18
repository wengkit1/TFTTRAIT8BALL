use ::crossterm::event::KeyCode;
pub struct App {
    pub should_quit: bool,
    pub selected_size: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            selected_size: 7, // Default to 7
        }
    }

    pub fn on_key(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Left => {
                if self.selected_size > 1 {
                    self.selected_size -= 1;
                }
            }
            KeyCode::Right => {
                if self.selected_size < 10 {
                    self.selected_size += 1;
                }
            }
            _ => {}
        }
    }
}
