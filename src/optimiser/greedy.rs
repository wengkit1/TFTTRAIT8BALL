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
