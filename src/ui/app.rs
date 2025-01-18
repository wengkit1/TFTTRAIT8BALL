pub struct App {
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self { should_quit: false }
    }

    pub fn on_key(&mut self, key: crossterm::event::KeyEvent) {
        if let crossterm::event::KeyCode::Char('q') = key.code {
            self.should_quit = true;
        }
    }
}
