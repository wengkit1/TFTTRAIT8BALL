use crate::models::champions::{Champion, OptimalComp, Trait, TraitActivation};
use crate::optimiser::trait_calc::trait_activation::calculate_trait_activations;
use itertools::Itertools;

pub fn find_optimal_comp_greedy(
    champions: &[Champion],
    traits: &[Trait],
    required_champions: &[&str],
    team_size: usize,
    trait_bonuses: &[(&str, u32)],
) -> Option<OptimalComp> {
    let mut team: Vec<&Champion> = Vec::new();
    for core_name in required_champions {
        if let Some(champ) = champions.iter().find(|c| &c.name == core_name) {
            team.push(champ);
        }
    }

    let mut available: Vec<&Champion> = champions
        .iter()
        .filter(|c| !required_champions.contains(&c.name.as_str()))
        .collect();

    while team.len() < team_size && !available.is_empty() {
        let mut best_score = 0;
        let mut best_idx = 0;

        for (idx, candidate) in available.iter().enumerate() {
            let mut test_team = team.clone();
            test_team.push(candidate);

            let activations = calculate_trait_activations(&test_team, traits, trait_bonuses);
            let score = calculate_combined_score(&test_team, &activations, candidate, traits);

            if score > best_score {
                best_score = score;
                best_idx = idx;
            }
        }

        team.push(available.remove(best_idx));
    }

    let activations = calculate_trait_activations(&team, traits, trait_bonuses);
    Some(OptimalComp {
        units: team.iter().map(|c| c.name.clone()).collect(),
        activated_traits: activations.clone(),
        total_traits_activated: activations.len(),
    })
}

fn calculate_combined_score(
    team: &[&Champion],
    activations: &[TraitActivation],
    candidate: &Champion,
    traits: &[Trait],
) -> usize {
    let current_trait_score = activations
        .iter()
        .map(|activation| match activation.breakpoint_hit {
            1 => 3,
            2 => 4,
            3 => 5,
            _ => 5,
        })
        .sum::<usize>();

    let unique_trait_score = candidate
        .traits
        .iter()
        .map(|trait_name| {
            let existing_count = team
                .iter()
                .filter(|c| c.traits.contains(trait_name))
                .count();

            if let Some(trait_def) = traits.iter().find(|t| t.name == *trait_name) {
                match existing_count {
                    0 => 3,
                    n if is_near_breakpoint(n, trait_def) => 1,
                    _ => 1,
                }
            } else {
                0
            }
        })
        .sum::<usize>();

    current_trait_score + unique_trait_score
}

fn is_near_breakpoint(count: usize, trait_def: &Trait) -> bool {
    trait_def
        .effects
        .iter()
        .any(|effect| effect.min_units as usize == count + 1)
}

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
