use crossterm::event::KeyCode;
use std::collections::HashSet;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct ChampionSelector {
    pub input_buffer: String,
    pub selected_values: Vec<String>,
    pub available_options: HashSet<String>,
    pub suggestions: Vec<String>,
    pub selected_suggestion: usize,
}

impl ChampionSelector {
    pub fn new(available_champions: Vec<String>) -> Self {
        Self {
            input_buffer: String::new(),
            selected_values: Vec::new(),
            available_options: available_champions.into_iter().collect(),
            suggestions: Vec::new(),
            selected_suggestion: 0,
        }
    }

    fn update_suggestions(&mut self) {
        if self.input_buffer.is_empty() {
            self.suggestions.clear();
            return;
        }

        let input_lowercase = self.input_buffer.to_lowercase();
        self.suggestions = self
            .available_options
            .iter()
            .filter(|name| name.to_lowercase().starts_with(&input_lowercase))
            .cloned()
            .collect();

        self.suggestions.sort();
        self.selected_suggestion = 0;
    }

    pub fn on_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => {
                if !self.suggestions.is_empty() {
                    self.selected_suggestion = self.selected_suggestion.saturating_sub(1);
                }
            }
            KeyCode::Down => {
                if !self.suggestions.is_empty() {
                    self.selected_suggestion =
                        (self.selected_suggestion + 1).min(self.suggestions.len() - 1);
                }
            }
            KeyCode::Enter => {
                if !self.suggestions.is_empty() {
                    if let Some(selected) = self.suggestions.get(self.selected_suggestion) {
                        self.selected_values.push(selected.clone());
                        self.input_buffer.clear();
                        self.suggestions.clear();
                    }
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
                } else if !self.selected_values.is_empty() {
                    self.input_buffer = self.selected_values.pop().unwrap();
                    self.update_suggestions();
                }
            }
            _ => {}
        }
    }
}
