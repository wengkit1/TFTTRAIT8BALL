use ::crossterm::event::KeyCode;
pub struct App {
    pub should_quit: bool,
    pub selected_size: usize,
    pub active_selector: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            selected_size: 7,
            active_selector: 0,
        }
    }

    pub fn on_key(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Up => {
                self.active_selector = self.active_selector.saturating_sub(1);
            }
            KeyCode::Down => {
                self.active_selector = (self.active_selector + 1).min(1);
            }
            KeyCode::Left => {
                if self.active_selector == 0 && self.selected_size > 1 {
                    self.selected_size -= 1;
                }
            }
            KeyCode::Right => {
                if self.active_selector == 0 && self.selected_size < 10 {
                    self.selected_size += 1;
                }
            }
            _ => {}
        }
    }
}
