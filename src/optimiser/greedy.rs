use crate::models::champions::{
    Champion, ChampionId, ChampionPool, OptimalComp, Trait, TraitActivation,
};
use crate::models::state;
use crate::optimiser::trait_calc::trait_activation::calculate_trait_activations;

pub fn find_optimal_comp_greedy(
    champion_pool: &ChampionPool,
    required_champion_ids: &[ChampionId],
    team_size: usize,
    trait_bonuses: &[(&str, u32)],
) -> Option<OptimalComp> {
    let context = state::get();

    let traits = &context.traits;

    let mut team: Vec<&Champion> = required_champion_ids
        .iter()
        .filter_map(|id| champion_pool.by_id.get(id))
        .collect();

    let mut available: Vec<&Champion> = champion_pool
        .all
        .iter()
        .filter(|c| !required_champion_ids.iter().any(|id| id == &c.id))
        .collect();

    while team.len() < team_size && !available.is_empty() {
        let mut best_score = 0;
        let mut best_idx = 0;

        for (idx, candidate) in available.iter().enumerate() {
            let mut test_team = team.clone();
            test_team.push(candidate);

            let test_team_ids: Vec<ChampionId> = test_team.iter().map(|c| c.id.clone()).collect();

            let activations =
                calculate_trait_activations(champion_pool, &test_team_ids, traits, trait_bonuses);
            let score = calculate_combined_score(&test_team, &activations, candidate, traits);

            if score > best_score {
                best_score = score;
                best_idx = idx;
            }
        }

        team.push(available.remove(best_idx));
    }

    let final_team_ids: Vec<ChampionId> = team.iter().map(|c| c.id.clone()).collect();

    let activations =
        calculate_trait_activations(champion_pool, &final_team_ids, traits, trait_bonuses);

    Some(OptimalComp {
        units: final_team_ids,
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
            _ => 4,
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
                    0 => 1,
                    n if is_near_breakpoint(n, trait_def) => 2,
                    _ => 0,
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
