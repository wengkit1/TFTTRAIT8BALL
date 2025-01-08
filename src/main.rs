use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use futures::StreamExt;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem}, // Add these specific widgets
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::stdout;

#[derive(Debug, Deserialize)]
struct Champion {
    name: String,
    traits: Vec<String>,
    cost: u32,
}

#[derive(Debug, Deserialize)]
struct SetData {
    champions: Vec<Champion>,
}

async fn fetch_tft_data() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://raw.communitydragon.org/latest/cdragon/tft/en_us.json";
    let resp = client.get(url).send().await?;
    println!("Got response status: {}", resp.status());

    // Get just the first chunk
    let text = resp.text().await?;
    println!(
        "First 100 chars of response: {}",
        &text[..100.min(text.len())]
    );

    let data: serde_json::Value = serde_json::from_str(&text)?;
    Ok(data)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    let mut stdout = stdout();
    stdout.execute(EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Just fetch and print data
    let rt = tokio::runtime::Runtime::new()?;
    println!("About to fetch data...");
    let tft_data = rt.block_on(fetch_tft_data())?;
    if let Some(sets) = tft_data.as_object() {
        println!(
            "Available keys: {:?}",
            sets.keys().take(5).collect::<Vec<_>>()
        );
        // Maybe peek at first item's structure
        if let Some(first_set) = sets.values().next() {
            println!("\nFirst item structure: {:#?}", first_set);
        }
    }

    // Wait for 'q' to quit
    loop {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    // Cleanup
    terminal::disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;

    Ok(())
}
