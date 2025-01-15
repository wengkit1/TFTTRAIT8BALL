use crate::models::champions::{Champion, OptimalComp, Trait, TraitActivation};
use crate::optimiser::trait_calc::trait_activation::calculate_trait_activations;
use itertools::Itertools;

pub fn find_optimal_comp_greedy(
    champions: &[Champion],
    traits: &[Trait],
    core_champions: &[&str],
    team_size: usize,
    max_cost: u32,
    trait_bonuses: &[(&str, u32)],
) -> Option<OptimalComp> {
    let mut team: Vec<&Champion> = Vec::new();
    for core_name in core_champions {
        if let Some(champ) = champions.iter().find(|c| &c.name == core_name) {
            team.push(champ);
        }
    }

    let mut available: Vec<&Champion> = champions
        .iter()
        .filter(|c| !core_champions.contains(&c.name.as_str()))
        .filter(|c| c.cost >= max_cost) // Filter by cost
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

pub fn find_optimal_comp_with_requirement(
    champions: &[Champion],
    traits: &[Trait],
    team_size: usize,
    required_trait: &str,
    required_count: usize,
    trait_bonuses: &[(&str, u32)],
    max_cost: u32,
) -> Option<OptimalComp> {
    // Get number of emblems for the required trait
    let emblem_count = trait_bonuses
        .iter()
        .find(|(t, _)| *t == required_trait)
        .map(|(_, count)| *count as usize)
        .unwrap_or(0);

    let actual_champs_needed = required_count.saturating_sub(emblem_count);

    let mut trait_champions: Vec<&Champion> = champions
        .iter()
        .filter(|c| c.traits.contains(&required_trait.to_string()))
        .collect();

    println!(
        "Found {} champions with {} trait",
        trait_champions.len(),
        required_trait
    );

    let mut best_comp = None;
    let mut best_score = 0;

    trait_champions.sort_by(|a, b| {
        let score_a = a.traits.len() * 10 + a.cost as usize * 3;
        let score_b = b.traits.len() * 10 + b.cost as usize * 2;
        score_b.cmp(&score_a)
    });

    for required_combo in trait_champions.iter().combinations(actual_champs_needed) {
        let remaining_spots = team_size - actual_champs_needed;

        // Filter out used champions for remaining pool
        let remaining_champs: Vec<Champion> = champions
            .iter()
            .filter(|c| !required_combo.iter().any(|rc| rc.name == c.name))
            .filter(|c| c.cost >= max_cost)
            .cloned()
            .collect();

        if let Some(comp) = find_optimal_comp_greedy(
            &remaining_champs,
            traits,
            &required_combo
                .iter()
                .map(|c| c.name.as_str())
                .collect::<Vec<_>>(),
            remaining_spots,
            max_cost,
            trait_bonuses,
        ) {
            let all_units: Vec<String> = required_combo
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
    }

    best_comp
}
