use crossterm::event::KeyCode;
use ratatui::{prelude::*, widgets::*};
use std::collections::HashSet;

pub struct AutoSelector {
    pub input_buffer: String,
    pub selected_values: Vec<String>,
    pub available_options: HashSet<String>,
    pub suggestions: Vec<String>,
    pub selected_suggestion: usize,
}

impl AutoSelector {
    pub fn new(options: Vec<String>) -> Self {
        Self {
            input_buffer: String::new(),
            selected_values: Vec::new(),
            available_options: options.into_iter().collect(),
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

    pub fn render_main_text(&self) -> Line<'static> {
        if self.selected_values.is_empty() {
            Line::from(vec![Span::raw("Core Units: [Enter to add...]")])
        } else {
            Line::from(vec![Span::raw(format!(
                "Core Units: [{}]",
                self.selected_values.join(", ")
            ))])
        }
    }

    pub fn render_editing_text(&self) -> Line<'static> {
        let current_units = if self.selected_values.is_empty() {
            String::new()
        } else {
            format!("{}, ", self.selected_values.join(", "))
        };
        Line::from(vec![
            Span::raw("Core Units: ["),
            Span::raw(current_units),
            Span::raw(format!("{}_", self.input_buffer)),
            Span::raw("]"),
        ])
    }

    pub fn render_popup<'a>(
        &'a self,
        chunks: &[Rect],
        area: Rect,
        active_selector: usize,
    ) -> Option<(Rect, List<'a>)> {
        if self.suggestions.is_empty() {
            return None;
        }

        let popup_area = Rect::new(
            chunks[active_selector].x + 12,
            chunks[active_selector].y + 1,
            (area.width as f32 * 0.3) as u16,
            7,
        );

        let suggestions: Vec<ListItem> = self
            .suggestions
            .iter()
            .enumerate()
            .take(7)
            .map(|(i, s)| {
                let style = if i == self.selected_suggestion {
                    Style::default().bg(Color::LightRed).fg(Color::White)
                } else {
                    Style::default().fg(Color::Gray)
                };
                ListItem::new(Line::from(vec![Span::styled(s, style)]))
            })
            .collect();

        Some((
            popup_area,
            List::new(suggestions).style(Style::default().bg(Color::DarkGray)),
        ))
    }
}
