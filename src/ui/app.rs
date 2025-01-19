use ::crossterm::event::KeyCode;
use std::collections::HashSet;

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub should_quit: bool,
    pub selected_size: usize,
    pub max_cost: u32,
    pub active_selector: usize,
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub core_units: Vec<String>,
    pub available_champions: HashSet<String>,
    pub selected_suggestion: usize,
    pub autocomplete_suggestions: Vec<String>,
}

impl App {
    pub fn new(champion_names: Vec<String>) -> Self {
        Self {
            should_quit: false,
            selected_size: 7,
            max_cost: 6,
            active_selector: 0,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            core_units: Vec::new(),
            available_champions: champion_names.into_iter().collect(),
            selected_suggestion: 0,
            autocomplete_suggestions: Vec::new(),
        }
    }

    fn update_suggestions(&mut self) {
        if self.input_buffer.is_empty() {
            self.autocomplete_suggestions.clear();
            return;
        }

        let input_lowercase = self.input_buffer.to_lowercase();
        self.autocomplete_suggestions = self
            .available_champions
            .iter()
            .filter(|name| name.to_lowercase().starts_with(&input_lowercase))
            .cloned()
            .collect();

        self.autocomplete_suggestions.sort();
        self.selected_suggestion = 0
    }

    pub fn on_key(&mut self, key: crossterm::event::KeyEvent) {
        match self.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('q') => self.should_quit = true,
                KeyCode::Up => {
                    self.active_selector = self.active_selector.saturating_sub(1);
                }
                KeyCode::Down => {
                    self.active_selector = (self.active_selector + 1).min(4);
                }
                KeyCode::Left | KeyCode::Right => match self.active_selector {
                    0 => {
                        if key.code == KeyCode::Left && self.selected_size > 1 {
                            self.selected_size -= 1;
                        } else if key.code == KeyCode::Right && self.selected_size < 10 {
                            self.selected_size += 1;
                        }
                    }
                    2 => {
                        if key.code == KeyCode::Left && self.max_cost > 1 {
                            self.max_cost -= 1;
                        } else if key.code == KeyCode::Right && self.max_cost < 6 {
                            self.max_cost += 1;
                        }
                    }
                    _ => {}
                },
                KeyCode::Enter => {
                    if self.active_selector == 1 {
                        self.input_mode = InputMode::Editing;
                        self.input_buffer.clear();
                    }
                }
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    self.input_buffer.clear();
                    self.autocomplete_suggestions.clear(); // Clear suggestions on exit
                }
                KeyCode::Up => {
                    if !self.autocomplete_suggestions.is_empty() {
                        self.selected_suggestion = self.selected_suggestion.saturating_sub(1);
                    }
                }
                KeyCode::Down => {
                    if !self.autocomplete_suggestions.is_empty() {
                        self.selected_suggestion = (self.selected_suggestion + 1)
                            .min(self.autocomplete_suggestions.len() - 1);
                    }
                }
                KeyCode::Enter => {
                    if !self.autocomplete_suggestions.is_empty() {
                        // If we have suggestions, use the selected one
                        if let Some(selected) =
                            self.autocomplete_suggestions.get(self.selected_suggestion)
                        {
                            self.core_units.push(selected.clone());
                            self.input_buffer.clear();
                            self.autocomplete_suggestions.clear();
                        }
                    } else if !self.input_buffer.is_empty() {
                        // Fallback to old behavior if no suggestions
                        self.core_units.push(self.input_buffer.clone());
                        self.input_buffer.clear();
                    }
                }
                KeyCode::Char(c) => {
                    self.input_buffer.push(c);
                    self.update_suggestions();
                }
                KeyCode::Backspace => {
                    if !self.input_buffer.is_empty() {
                        self.input_buffer.pop();
                        self.update_suggestions();
                    } else if !self.core_units.is_empty() {
                        self.input_buffer = self.core_units.pop().unwrap();
                        self.update_suggestions();
                    }
                }
                _ => {}
            },
        }
    }
}
