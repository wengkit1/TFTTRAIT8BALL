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
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(frame: &mut Frame, app: &App) {
    let area = frame.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(area);

    // Instructions
    let help_text = match app.input_mode {
        InputMode::Normal => "← → to move, Enter to select, type to input number, q to quit",
        InputMode::Typing => "Enter to confirm, Esc to cancel",
    };

    frame.render_widget(
        Paragraph::new(help_text).alignment(Alignment::Center),
        chunks[0],
    );

    let numbers: Vec<Span> = (7..=10)
        .map(|num| {
            if num == app.selected_size {
                Span::styled(
                    format!(" {} ", num),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled(format!(" {} ", num), Style::default().fg(Color::Gray))
            }
        })
        .collect();

    let mut spans = vec![Span::raw("Size: [ ")];
    spans.extend(numbers);
    spans.push(Span::raw(" ]"));

    let size_line = Line::from(spans);

    frame.render_widget(
        Paragraph::new(size_line).alignment(Alignment::Left),
        chunks[1],
    );

    // Status line
    let status = match app.input_mode {
        InputMode::Normal => format!("Selected size: {}", app.selected_size),
        InputMode::Typing => format!("Typing: {}", app.input_buffer),
    };

    frame.render_widget(Paragraph::new(status).alignment(Alignment::Left), chunks[2]);
}
