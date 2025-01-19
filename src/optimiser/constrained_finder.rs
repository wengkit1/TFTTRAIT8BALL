use crate::models::champions::{Champion, ChampionId, ChampionPool, OptimalComp};
use crate::models::state;
use crate::optimiser::{
    greedy::find_optimal_comp_greedy, trait_calc::trait_activation::calculate_trait_activations,
};
use itertools::Itertools;
use std::collections::HashMap;

pub fn find_optimal_comp_with_requirements(
    team_size: usize,
    trait_requirements: &[(&str, usize)],
    trait_bonuses: &[(&str, u32)],
    max_cost: u32,
    core_unit_ids: &[ChampionId],
) -> Option<OptimalComp> {
    let context = state::get();
    let champion_pool = &context.champion_pool;
    let traits = &context.traits;

    let total_required: usize = trait_requirements.iter().map(|(_, count)| count).sum();

    if total_required + core_unit_ids.len() > team_size {
        return None;
    }

    let cost_filtered_champs: Vec<&Champion> = champion_pool
        .all
        .iter()
        .filter(|c| c.cost <= max_cost || core_unit_ids.contains(&c.id))
        .collect();

    if trait_requirements.is_empty() {
        let filtered_by_id: HashMap<ChampionId, Champion> = cost_filtered_champs
            .iter()
            .map(|c| (c.id.clone(), (*c).clone()))
            .collect();

        let cost_filtered_pool = ChampionPool {
            by_id: filtered_by_id,
            all: cost_filtered_champs.iter().map(|c| (**c).clone()).collect(),
        };
        return find_optimal_comp_greedy(
            &cost_filtered_pool,
            core_unit_ids,
            team_size,
            trait_bonuses,
        );
    }

    let filtered_champs: Vec<&Champion> = cost_filtered_champs
        .iter()
        .filter(|c| !core_unit_ids.contains(&c.id))
        .copied()
        .collect();

    let mut best_comp = None;
    let mut best_score = 0;

    let requirement_data: Vec<(Vec<&Champion>, usize)> = trait_requirements
        .iter()
        .map(|(trait_name, required_count)| {
            let emblem_count = trait_bonuses
                .iter()
                .find(|(t, _)| *t == &trait_name.to_string())
                .map(|(_, count)| *count as usize)
                .unwrap_or(0);

            let actual_count = required_count.saturating_sub(emblem_count);
            let trait_champions: Vec<&Champion> = filtered_champs
                .iter()
                .filter(|c| c.traits.contains(&trait_name.to_string()))
                .filter(|c| c.cost <= max_cost)
                .copied()
                .collect();

            (trait_champions, actual_count)
        })
        .collect();

    if requirement_data
        .iter()
        .any(|(champs, count)| champs.len() < *count)
    {
        return None;
    }

    let trait_combinations: Vec<Vec<Vec<&Champion>>> = requirement_data
        .iter()
        .map(|(trait_champs, count)| {
            trait_champs
                .iter()
                .combinations(*count)
                .map(|combo| combo.into_iter().copied().collect())
                .collect()
        })
        .collect();

    for combined_combo in trait_combinations.iter().multi_cartesian_product() {
        let mut required_champs = Vec::new();
        let mut valid = true;

        for combo in combined_combo {
            for champ in combo {
                if required_champs.contains(champ) {
                    valid = false;
                    break;
                }
                required_champs.push(*champ);
            }
            if !valid {
                break;
            }
        }

        if valid && required_champs.len() <= team_size {
            let remaining_spots = team_size - required_champs.len() - core_unit_ids.len();
            println!("Remaining team size: {}", remaining_spots);
            let filtered_by_id: HashMap<ChampionId, Champion> = filtered_champs
                .iter()
                .filter(|c| !required_champs.iter().any(|rc| rc.id == c.id))
                .map(|c| (c.id.clone(), (*c).clone()))
                .collect();

            let filtered_all: Vec<Champion> = filtered_by_id.values().cloned().collect();

            let remaining_champs = ChampionPool {
                by_id: filtered_by_id,
                all: filtered_all,
            };

            if let Some(comp) = find_optimal_comp_greedy(
                &remaining_champs,
                &required_champs
                    .iter()
                    .map(|c| c.id.clone())
                    .collect::<Vec<_>>(),
                remaining_spots,
                trait_bonuses,
            ) {
                let all_unit_ids: Vec<ChampionId> = core_unit_ids
                    .iter()
                    .cloned()
                    .chain(required_champs.iter().map(|c| c.id.clone()))
                    .chain(comp.units)
                    .collect();

                let activated = calculate_trait_activations(
                    champion_pool,
                    &all_unit_ids,
                    traits,
                    trait_bonuses,
                );

                let score = activated.len();
                if score > best_score {
                    best_score = score;
                    best_comp = Some(OptimalComp {
                        units: all_unit_ids,
                        activated_traits: activated,
                        total_traits_activated: score,
                    });
                }
            }
        }
    }

    best_comp
}
