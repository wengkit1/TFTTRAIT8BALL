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
        app.champion_selector.render_editing_text()
    } else {
        app.champion_selector.render_main_text()
    };

    let cost_text = Line::from(vec![Span::raw(format!("Max Cost: {} ↔", app.max_cost))]);

    let trait_req_text = if app.input_mode == InputMode::Editing && app.active_selector == 3 {
        app.trait_selector.render_editing_text()
    } else {
        app.trait_selector.render_main_text()
    };

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
    if app.input_mode == InputMode::Editing && app.active_selector == 3 {
        if let Some((popup_area, suggestions_list)) =
            app.trait_selector
                .render_popup(&chunks, area, app.active_selector)
        {
            frame.render_widget(Clear, popup_area);
            frame.render_widget(suggestions_list, popup_area);
        }
    }

    if app.input_mode == InputMode::Editing && app.active_selector == 1 {
        if let Some((popup_area, suggestions_list)) =
            app.champion_selector
                .render_popup(&chunks, area, app.active_selector)
        {
            frame.render_widget(Clear, popup_area);
            frame.render_widget(suggestions_list, popup_area);
        }
    }
}
