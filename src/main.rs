mod api;
mod models;
mod optimiser;
use models::{
    champions::{ChampionId, ChampionPool, OptimalComp},
    state,
};
use optimiser::constrained_finder::find_optimal_comp_with_requirements;
use std::fs;
mod ui;

fn save_results(comps: &[OptimalComp], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(comps)?;
    fs::write(filename, json)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;

    let champions = rt.block_on(api::fetch::fetch_tft_data())?;
    let champion_pool = ChampionPool::with_data(champions);
    let traits = rt.block_on(api::fetch::fetch_trait_data())?;

    state::init(champion_pool.clone(), traits.clone());
    // println!("{:#?}", traits);
    let champion_names: Vec<String> = champion_pool.all.iter().map(|c| c.id.0.clone()).collect();
    let trait_names: Vec<String> = traits.iter().map(|t| t.name.clone()).collect();

    let app = ui::app::App::new(champion_names, trait_names);
    ui::tui::run(app)?;

    // let mut optimal_comps = Vec::new();
    // for size in 7..=10 {
    //     let core_unit_ids: Vec<ChampionId> = vec![
    //         // ChampionId("Nami".to_string()),
    //         ChampionId("Gangplank".to_string()),
    //         // ChampionId("Swain".to_string()),
    //     ];

    //     let trait_bonuses = &[];
    //     let trait_requirements = &[];
    //     let comp = find_optimal_comp_with_requirements(
    //         size,
    //         trait_requirements,
    //         trait_bonuses,
    //         2,
    //         &core_unit_ids,
    //     );

    //     optimal_comps.push(comp);
    // }

    // let filtered_comps: Vec<OptimalComp> =
    //     optimal_comps.into_iter().filter_map(|comp| comp).collect();

    // save_results(&filtered_comps, "optimal_comps_2.json")?;

    Ok(())
}
