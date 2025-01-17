mod api;
mod models;
mod optimiser;
use models::champions::OptimalComp;
use optimiser::constrained_finder::find_optimal_comp_with_requirements;
use std::fs;

fn save_results(comps: &[OptimalComp], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(comps)?;
    fs::write(filename, json)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;

    let champions = rt.block_on(api::fetch::fetch_tft_data())?;
    let traits = rt.block_on(api::fetch::fetch_trait_data())?;

    let mut optimal_comps = Vec::new();
    for size in 7..=10 {
        let core_units = &["Nami", "Gangplank", "Swain"];
        let trait_bonuses = &[("Chem-Baron", 1)];
        let trait_requirements = &[("Chem-Baron", 3)];
        let comp = find_optimal_comp_with_requirements(
            &champions,
            &traits,
            size,
            trait_requirements,
            trait_bonuses,
            5,
            core_units,
        );

        optimal_comps.push(comp);
    }

    let filtered_comps: Vec<OptimalComp> =
        optimal_comps.into_iter().filter_map(|comp| comp).collect();

    save_results(&filtered_comps, "optimal_comps_2.json")?;

    Ok(())
}
