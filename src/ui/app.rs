use crate::ui::selector::{auto::AutoSelector, trait_selector::TraitSelector};
use ::crossterm::event::KeyCode;
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
    pub champion_selector: AutoSelector,
    pub trait_selector: TraitSelector,
}

impl App {
    pub fn new(champion_names: Vec<String>, trait_names: Vec<String>) -> Self {
        Self {
            should_quit: false,
            selected_size: 7,
            max_cost: 6,
            active_selector: 0,
            input_mode: InputMode::Normal,
            champion_selector: AutoSelector::new(champion_names),
            trait_selector: TraitSelector::new(trait_names),
        }
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
                KeyCode::Enter => match self.active_selector {
                    1 => {
                        self.input_mode = InputMode::Editing;
                        self.champion_selector.input_buffer.clear();
                    }
                    3 => {
                        self.input_mode = InputMode::Editing;
                        self.trait_selector.number_input.clear();
                    }
                    _ => {}
                },
                _ => {}
            },
            InputMode::Editing => match self.active_selector {
                1 => match key.code {
                    KeyCode::Esc => {
                        self.input_mode = InputMode::Normal;
                        self.champion_selector.input_buffer.clear();
                    }
                    _ => self.champion_selector.on_key(key.code),
                },
                3 => match key.code {
                    KeyCode::Esc => {
                        self.input_mode = InputMode::Normal;
                        self.trait_selector.number_input.clear();
                    }
                    _ => self.trait_selector.on_key(key.code),
                },
                _ => {}
            },
        }
    }
}
