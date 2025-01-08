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

async fn fetch_tft_data() -> Result<Vec<Champion>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://raw.communitydragon.org/latest/cdragon/tft/en_us.json";
    let resp = client.get(url).send().await?;
    let text = resp.text().await?;
    let data: serde_json::Value = serde_json::from_str(&text)?;

    // Navigate to Set 13 champions
    if let Some(sets_obj) = data.get("sets") {
        if let Some(set13) = sets_obj.get("13") {
            if let Some(champions) = set13.get("champions") {
                let champions: Vec<Champion> = serde_json::from_value(champions.clone())?;
                // Filter out champions with empty traits
                let filtered_champions: Vec<Champion> = champions
                    .into_iter()
                    .filter(|champion| !champion.traits.is_empty())
                    .collect();
                println!("Total champions with traits: {}", filtered_champions.len());
                return Ok(filtered_champions);
            }
        }
    }

    Err("Could not find Set 13 champions".into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    let champions = rt.block_on(fetch_tft_data())?;

    // Print champions in a readable format
    for champion in champions {
        println!(
            "Name: {}, Cost: {}, Traits: {:?}",
            champion.name, champion.cost, champion.traits
        );
    }

    Ok(())
}
