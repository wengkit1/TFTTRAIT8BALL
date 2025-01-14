use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Champion {
    pub name: String,
    pub traits: Vec<String>,
    pub cost: u32,
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

// For optimization results
#[derive(Debug, Serialize)]
pub struct OptimalComp {
    pub units: Vec<String>,
    pub activated_traits: Vec<TraitActivation>,
    pub total_traits_activated: usize,
}

#[derive(Debug, Serialize, Clone)]
pub struct TraitActivation {
    pub name: String,
    pub count: usize,
    pub breakpoint_hit: u32,
}
