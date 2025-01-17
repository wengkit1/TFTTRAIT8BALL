use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Champion {
    pub id: ChampionId,
    pub name: String,
    pub traits: Vec<String>,
    pub cost: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChampionId(pub String);

#[derive(Debug)]
pub struct ChampionPool {
    pub by_id: HashMap<ChampionId, Champion>,
    pub all: Vec<Champion>,
}

impl ChampionPool {
    pub fn with_data(champions: Vec<Champion>) -> Self {
        let by_id = champions
            .iter()
            .map(|c| (c.id.clone(), c.clone()))
            .collect::<HashMap<_, _>>();

        ChampionPool {
            by_id,
            all: champions,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Trait {
    pub name: String,
    pub effects: Vec<TraitEffect>,
}

#[derive(Debug, Deserialize)]
pub struct TraitEffect {
    #[serde(rename = "minUnits")]
    pub min_units: u32,
}

#[derive(Debug, Serialize)]
pub struct OptimalComp {
    pub units: Vec<ChampionId>,
    pub activated_traits: Vec<TraitActivation>,
    pub total_traits_activated: usize,
}

#[derive(Debug, Serialize, Clone)]
pub struct TraitActivation {
    pub name: String,
    pub count: usize,
    pub breakpoint_hit: u32,
}
