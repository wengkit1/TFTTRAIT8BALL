use crate::models::champions::{Champion, ChampionId, Trait};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct ChampionRaw {
    name: String,
    traits: Vec<String>,
    cost: u32,
}

impl From<ChampionRaw> for Champion {
    fn from(raw: ChampionRaw) -> Self {
        Champion {
            id: ChampionId(raw.name.clone()),
            name: raw.name,
            traits: raw.traits,
            cost: raw.cost,
        }
    }
}

pub async fn fetch_tft_data() -> Result<Vec<Champion>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://raw.communitydragon.org/latest/cdragon/tft/en_us.json";
    let resp = client.get(url).send().await?;
    let text = resp.text().await?;
    let data: serde_json::Value = serde_json::from_str(&text)?;

    if let Some(sets_obj) = data.get("sets") {
        if let Some(set13) = sets_obj.get("13") {
            if let Some(champions) = set13.get("champions") {
                let raw_champions: Vec<ChampionRaw> = serde_json::from_value(champions.clone())?;
                let champions: Vec<Champion> = raw_champions
                    .into_iter()
                    .map(Champion::from)
                    .filter(|champion| !champion.traits.is_empty())
                    .collect();
                return Ok(champions);
            }
        }
    }
    Err("Could not find Set 13 champions".into())
}

pub async fn fetch_trait_data() -> Result<Vec<Trait>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://raw.communitydragon.org/latest/cdragon/tft/en_us.json";
    let resp = client.get(url).send().await?;
    let text = resp.text().await?;
    let data: serde_json::Value = serde_json::from_str(&text)?;

    if let Some(sets_obj) = data.get("sets") {
        if let Some(set13) = sets_obj.get("13") {
            if let Some(traits) = set13.get("traits") {
                let traits: Vec<Trait> = serde_json::from_value(traits.clone())?;
                return Ok(traits);
            }
        }
    }
    Err("Could not find trait data in Set 13".into())
}
