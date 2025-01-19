use crate::ui::app::InputMode;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use std::io;

use super::app::App;

pub fn run(mut app: App) -> io::Result<()> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|frame| ui(frame, &app))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.on_key(key);
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

// in tui.rs
fn ui(frame: &mut Frame, app: &App) {
    let area = frame.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .margin(1)
        .split(area);

    let size_text = Line::from(vec![Span::raw(format!(
        "Team Size: {} ↔",
        app.selected_size
    ))]);

    let core_text = if app.input_mode == InputMode::Editing && app.active_selector == 1 {
        let current_units = if app.core_units.is_empty() {
            String::new()
        } else {
            format!("{}, ", app.core_units.join(", "))
        };

        Line::from(vec![
            Span::raw("Core Units: ["),
            Span::raw(current_units),
            Span::raw(format!("{}_", app.input_buffer)),
            Span::raw("]"),
        ])
    } else {
        if app.core_units.is_empty() {
            Line::from(vec![Span::raw("Core Units: [Press Enter to add...]")])
        } else {
            Line::from(vec![Span::raw(format!(
                "Core Units: [{}]",
                app.core_units.join(", ")
            ))])
        }
    };
    let cost_text = Line::from(vec![Span::raw(format!("Max Cost: {} ↔", app.max_cost))]);
    let trait_req_text = Line::from(vec![Span::raw(
        "Trait Requirements: [Type to add traits...]",
    )]);

    let trait_bonus_text = Line::from(vec![Span::raw("Trait Emblems: [Type to add emblems...]")]);

    let texts = [
        size_text,
        core_text,
        cost_text,
        trait_req_text,
        trait_bonus_text,
    ];
    for (i, text) in texts.iter().enumerate() {
        frame.render_widget(
            Paragraph::new(text.clone())
                .alignment(Alignment::Left)
                .style(
                    Style::default()
                        .bg(if app.active_selector == i {
                            Color::Red
                        } else {
                            Color::Reset
                        })
                        .fg(if app.active_selector == i {
                            Color::White
                        } else {
                            Color::Gray
                        }),
                ),
            chunks[i],
        );
    }

    if app.input_mode == InputMode::Editing
        && app.active_selector == 1
        && !app.autocomplete_suggestions.is_empty()
    {
        let popup_area = Rect::new(
            chunks[1].x + 12,
            chunks[1].y + 1,
            (area.width as f32 * 0.3) as u16,
            7,
        );

        // Create suggestion list
        let suggestions: Vec<ListItem> = app
            .autocomplete_suggestions
            .iter()
            .enumerate()
            .take(7)
            .map(|(i, s)| {
                let style = if i == app.selected_suggestion {
                    Style::default().bg(Color::LightRed).fg(Color::White)
                } else {
                    Style::default().fg(Color::Gray)
                };
                ListItem::new(Line::from(vec![Span::styled(s, style)]))
            })
            .collect();

        let suggestions_list = List::new(suggestions).style(Style::default().bg(Color::DarkGray));

        frame.render_widget(Clear, popup_area);
        frame.render_widget(suggestions_list, popup_area);
    }
}
