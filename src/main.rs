mod api;
mod models;
mod optimiser;

use models::champions::OptimalComp;
use std::fs;

fn save_results(comps: &[OptimalComp], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(comps)?;
    fs::write(filename, json)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create runtime
    let rt = tokio::runtime::Runtime::new()?;

    // Run async functions inside runtime
    let champions = rt.block_on(api::fetch::fetch_tft_data())?;
    let traits = rt.block_on(api::fetch::fetch_trait_data())?;

    println!("Champions loaded: {}", champions.len());

    let mut optimal_comps = Vec::new();
    for size in 7..=10 {
        println!("Calculating optimal comp for size {}", size);
        let core_units = &["Nami", "Gangplank", "Swain"];
        let comp = optimiser::greedy::find_optimal_comp_greedy(
            &champions, &traits, core_units, size, // This is the final team size we want
        );
        optimal_comps.push(comp);
    }

    save_results(&optimal_comps, "optimal_comps_2.json")?;

    Ok(())
}
