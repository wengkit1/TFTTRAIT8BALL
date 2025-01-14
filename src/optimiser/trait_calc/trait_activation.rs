use crate::models::champions::{Champion, Trait, TraitActivation};
use std::collections::HashMap;

pub fn calculate_trait_activations(team: &[&Champion], traits: &[Trait]) -> Vec<TraitActivation> {
    let mut activated = Vec::new();
    let mut trait_counts: HashMap<&String, usize> = HashMap::new();

    for champ in team {
        for trait_name in &champ.traits {
            *trait_counts.entry(trait_name).or_insert(0) += 1;
        }
    }

    for trait_data in traits {
        if let Some(&count) = trait_counts.get(&trait_data.name) {
            for effect in trait_data.effects.iter().rev() {
                if count >= effect.min_units as usize {
                    activated.push(TraitActivation {
                        name: trait_data.name.clone(),
                        count,
                        breakpoint_hit: effect.min_units,
                    });
                    break;
                }
            }
        }
    }

    activated
}
