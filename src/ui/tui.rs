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

fn ui(frame: &mut Frame, app: &App) {
    let area = frame.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .split(area);

    // Team size selector
    let size_text = Line::from(vec![Span::raw(format!(
        "Team Size: {} ↔",
        app.selected_size
    ))]);

    let trait_text = Line::from(vec![Span::raw("Trait: [Type to search traits...]")]);

    frame.render_widget(
        Paragraph::new(size_text).alignment(Alignment::Left).style(
            Style::default()
                .bg(if app.active_selector == 0 {
                    Color::Red
                } else {
                    Color::Reset
                })
                .fg(if app.active_selector == 0 {
                    Color::White
                } else {
                    Color::Gray
                }),
        ),
        chunks[0],
    );

    frame.render_widget(
        Paragraph::new(trait_text).alignment(Alignment::Left).style(
            Style::default()
                .bg(if app.active_selector == 1 {
                    Color::Red
                } else {
                    Color::Reset
                })
                .fg(if app.active_selector == 1 {
                    Color::White
                } else {
                    Color::Gray
                }),
        ),
        chunks[1],
    );
}
