use crate::models::champions::{Champion, OptimalComp, Trait, TraitActivation};
use crate::optimiser::trait_calc::trait_activation::calculate_trait_activations;

pub fn find_optimal_comp_greedy(
    champions: &[Champion],
    traits: &[Trait],
    core_champions: &[&str],
    team_size: usize,
) -> OptimalComp {
    let mut team: Vec<&Champion> = Vec::new();
    for core_name in core_champions {
        if let Some(champ) = champions.iter().find(|c| &c.name == core_name) {
            team.push(champ);
        }
    }

    let mut available: Vec<&Champion> = champions
        .iter()
        .filter(|c| !core_champions.contains(&c.name.as_str()))
        .collect();

    while team.len() < team_size && !available.is_empty() {
        let mut best_score = 0;
        let mut best_idx = 0;

        for (idx, candidate) in available.iter().enumerate() {
            let mut test_team = team.clone();
            test_team.push(candidate);

            let activations = calculate_trait_activations(&test_team, traits);
            // New scoring that considers both current activations and future potential
            let score = calculate_combined_score(&activations, candidate);

            if score > best_score {
                best_score = score;
                best_idx = idx;
            }
        }

        team.push(available.remove(best_idx));
    }

    let activations = calculate_trait_activations(&team, traits);
    OptimalComp {
        units: team.iter().map(|c| c.name.clone()).collect(),
        activated_traits: activations.clone(),
        total_traits_activated: activations.len(),
    }
}

fn calculate_combined_score(activations: &[TraitActivation], candidate: &Champion) -> usize {
    let current_trait_score = activations
        .iter()
        .map(|activation| match activation.breakpoint_hit {
            1 => 1,
            2 => 3,
            3 => 6,
            _ => 10,
        })
        .sum::<usize>();
    let potential_score = candidate.traits.len() * 3;

    current_trait_score + potential_score
}
