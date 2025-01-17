use crate::models::champions::{Champion, OptimalComp, Trait};
use crate::optimiser::{
    greedy::find_optimal_comp_greedy, trait_calc::trait_activation::calculate_trait_activations,
};
use itertools::Itertools;

pub fn find_optimal_comp_with_requirements(
    champions: &[Champion],
    traits: &[Trait],
    team_size: usize,
    trait_requirements: &[(&str, usize)],
    trait_bonuses: &[(&str, u32)],
    max_cost: u32,
) -> Option<OptimalComp> {
    let total_required: usize = trait_requirements.iter().map(|(_, count)| count).sum();

    if total_required > team_size {
        return None;
    }
    let filtered_champs: Vec<Champion> = champions
        .iter()
        .filter(|c| c.cost <= max_cost)
        .cloned()
        .collect();

    if trait_requirements.is_empty() {
        return find_optimal_comp_greedy(&filtered_champs, traits, &[], team_size, trait_bonuses);
    }

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

        // Check if this combination has duplicates
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
            let remaining_spots = team_size - required_champs.len();
            let remaining_champs: Vec<Champion> = filtered_champs
                .iter()
                .filter(|c| !required_champs.iter().any(|rc| rc.name == c.name))
                .cloned()
                .collect();

            if let Some(comp) = find_optimal_comp_greedy(
                &remaining_champs,
                traits,
                &required_champs
                    .iter()
                    .map(|c| c.name.as_str())
                    .collect::<Vec<_>>(),
                remaining_spots,
                trait_bonuses,
            ) {
                let all_units: Vec<String> = required_champs
                    .iter()
                    .map(|c| c.name.clone())
                    .chain(comp.units.into_iter())
                    .collect();

                let activated = calculate_trait_activations(
                    &all_units
                        .iter()
                        .filter_map(|name| champions.iter().find(|c| &c.name == name))
                        .collect::<Vec<_>>(),
                    traits,
                    trait_bonuses,
                );

                let score = activated.len();
                if score > best_score {
                    best_score = score;
                    best_comp = Some(OptimalComp {
                        units: all_units,
                        activated_traits: activated,
                        total_traits_activated: score,
                    });
                }
            }

            required_champs.clear();
        }
    }

    best_comp
}
