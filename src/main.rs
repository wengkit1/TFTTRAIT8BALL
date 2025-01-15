mod api;
mod models;
mod optimiser;
use models::champions::OptimalComp;
use optimiser::greedy::find_optimal_comp_with_requirement;
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
        let _core_units = &["Nami", "Gangplank", "Swain"];
        let trait_bonuses = &[("Conqueror", 1)];
        let comp = find_optimal_comp_with_requirement(
            &champions,
            &traits,
            size,
            "Conqueror",
            4,
            trait_bonuses,
            3,
        );

        match &comp {
            Some(found_comp) => {
                println!("Found optimal comp:");
                println!("Units: {:?}", found_comp.units);
                println!("Traits: {:?}", found_comp.activated_traits);
            }
            None => println!("No valid composition found"),
        }

        println!("Found comp for size {}: {:?}", size, comp); // Debug print
        optimal_comps.push(comp);
    }

    let filtered_comps: Vec<OptimalComp> =
        optimal_comps.into_iter().filter_map(|comp| comp).collect();

    save_results(&filtered_comps, "optimal_comps_2.json")?;

    Ok(())
}
