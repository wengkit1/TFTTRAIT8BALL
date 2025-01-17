// use crate::models::champions::{Champion, OptimalComp, Trait};
// use crate::optimiser::trait_calc::trait_activation::calculate_trait_activations;
// use itertools::Itertools;

// pub fn find_optimal_comp_brute(
//     champions: &[Champion],
//     traits: &[Trait],
//     team_size: usize,
//     trait_bonuses: &[(&str, u32)],
// ) -> OptimalComp {
//     let mut best_comp = OptimalComp {
//         units: Vec::new(),
//         activated_traits: Vec::new(),
//         total_traits_activated: 0,
//     };

//     for team in champions.iter().combinations(team_size) {
//         let trait_activations = calculate_trait_activations(&team[..], traits, trait_bonuses);
//         let total_active = trait_activations.len();

//         if total_active > best_comp.total_traits_activated {
//             best_comp = OptimalComp {
//                 units: team.iter().map(|c| &c.name).cloned().collect(),
//                 activated_traits: trait_activations,
//                 total_traits_activated: total_active,
//             };
//         }
//     }

//     best_comp
// }
