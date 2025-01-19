use super::auto::AutoSelector;
use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem},
};

pub struct TraitSelector {
    auto_selector: AutoSelector, // Compose with AutoSelector
    pub number_input: String,
    pub selected_values: Vec<(String, u32)>, // (trait_name, count)
    pub is_entering_number: bool,
}

impl TraitSelector {
    pub fn new(available_traits: Vec<String>) -> Self {
        Self {
            auto_selector: AutoSelector::new(available_traits),
            number_input: String::new(),
            selected_values: Vec::new(),
            is_entering_number: false,
        }
    }

    pub fn on_key(&mut self, key: KeyCode) {
        if self.is_entering_number {
            match key {
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    self.number_input.push(c);
                }
                KeyCode::Backspace => {
                    if !self.number_input.is_empty() {
                        self.number_input.pop();
                    } else {
                        // If number input is empty, go back to trait name input
                        self.is_entering_number = false;
                    }
                    self.number_input.pop();
                }
                KeyCode::Enter => {
                    if let Ok(number) = self.number_input.parse::<u32>() {
                        // Get the trait name from auto_selector's input buffer
                        let trait_name = self.auto_selector.input_buffer.clone();
                        if !trait_name.is_empty() {
                            self.selected_values.push((trait_name, number));
                            self.auto_selector.input_buffer.clear();
                            self.number_input.clear();
                            self.is_entering_number = false;
                        }
                    }
                }
                KeyCode::Esc => {
                    self.number_input.clear();
                    self.auto_selector.input_buffer.clear();
                    self.is_entering_number = false;
                }
                _ => {}
            }
        } else {
            match key {
                KeyCode::Backspace => {
                    if !self.auto_selector.input_buffer.is_empty() {
                        self.auto_selector.on_key(key);
                    } else if !self.selected_values.is_empty() {
                        let (trait_name, _) = self.selected_values.pop().unwrap();
                        self.auto_selector.input_buffer = trait_name;
                        self.is_entering_number = false;
                    }
                }
                KeyCode::Enter => {
                    if !self.auto_selector.suggestions.is_empty() {
                        if let Some(selected) = self
                            .auto_selector
                            .suggestions
                            .get(self.auto_selector.selected_suggestion)
                        {
                            self.auto_selector.input_buffer = selected.clone();
                            self.auto_selector.suggestions.clear();
                            self.is_entering_number = true;
                        }
                    }
                }
                _ => self.auto_selector.on_key(key),
            }
        }
    }

    pub fn render_popup<'a>(
        &'a self,
        chunks: &[Rect],
        area: Rect,
        active_selector: usize,
    ) -> Option<(Rect, List<'a>)> {
        if self.is_entering_number {
            None
        } else {
            self.auto_selector
                .render_popup(chunks, area, active_selector)
        }
    }

    pub fn render_main_text(&self) -> Line {
        if self.selected_values.is_empty() {
            Line::from(vec![Span::raw(
                "Trait Requirements: [Press Enter to add...]",
            )])
        } else {
            let traits = self
                .selected_values
                .iter()
                .map(|(name, count)| format!("{}: {}", name, count))
                .collect::<Vec<_>>()
                .join(", ");

            Line::from(vec![Span::raw(format!("Trait Requirements: [{}]", traits))])
        }
    }

    pub fn render_editing_text(&self) -> Line {
        let current = if self.selected_values.is_empty() {
            String::new()
        } else {
            let traits = self
                .selected_values
                .iter()
                .map(|(name, count)| format!("{}: {}", name, count))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}, ", traits)
        };

        let input_display = if self.is_entering_number {
            format!(
                "{} ({})",
                self.auto_selector.input_buffer, self.number_input
            )
        } else {
            self.auto_selector.input_buffer.clone()
        };

        Line::from(vec![
            Span::raw("Trait Requirements: ["),
            Span::raw(current),
            Span::raw(format!("{}_", input_display)),
            Span::raw("]"),
        ])
    }
}
