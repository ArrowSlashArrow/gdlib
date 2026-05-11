//! Implementations and uses of Pseudo-RNG in Geometry Dash.

use crate::cclocallevels::gdobj::structs::{GDValue, Group};

const LCG_MULTIPLIER: u64 = 214013;
const LCG_CONSTANT: u64 = 2531011;

/// Determines the next seed from a starting seed as generated in Geometry Dash.
#[inline(always)]
pub fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(LCG_MULTIPLIER).wrapping_add(LCG_CONSTANT)
}

/// Mutating version of [`next_seed`]
#[inline(always)]
pub fn next_seed_mut(seed: &mut u64) {
    *seed = seed.wrapping_mul(LCG_MULTIPLIER).wrapping_add(LCG_CONSTANT);
}

/// Function used by GD to generate a new seed. Internally known as `fast_rand_0_1`.
/// Unlike the actual PRNG used in GD, this function *DOES NOT* automatically update the seed.
#[inline(always)]
pub fn fast_rand_bits(seed: u64) -> u64 {
    (next_seed(seed) >> 16) & 0x7fff
}

/// Utility function which normalises result from [`fast_rand_bits`] to the range \[0.0, 1.0].
#[inline(always)]
pub fn fast_rand_bits_norm(seed: u64) -> f64 {
    fast_rand_bits(seed) as f64 / 32767.0
}

/// Checks if the seed will activate group 1 or 2 in a random trigger.
/// The chance must be given as a float in the range \[0.0, 1.0].
/// The function returns true if the group 1 will be activated, and false if group 2 will be activated.
///
/// Note: this function does not automatically update the seed. To do so, refer to [`next_seed`].
/// This is a key difference between this function and GD's version,
/// since the official one automatically updates the seed when called.
#[inline(always)]
pub fn check_seed_random(seed: u64, chance: f64) -> bool {
    // Compare against the chance threshold
    fast_rand_bits_norm(seed) < chance
}

/// Determines the group that an advanced random trigger will activate based on an input seed
/// and a list of the trigger's activation probabilities per group as a [`GDValue::ProbabilitiesList`].
///
/// For maximal accuracy, please do not sort or prune the list in any way.
/// Doing so may affect the results of the check.
///
/// Note: this function does not automatically update the seed. To do so, refer to [`next_seed`].
/// This is a key difference between this function and GD's version,
/// since the official one automatically updates the seed when called.
///
/// **WARNING**: This function may rarely, for an unknown reason, erroneously determine that
/// a seed will activate a group when in reality, it won't. Please be mindful of this when checking seeds.
pub fn check_seed_advanced_random(seed: u64, probabilities: &GDValue) -> Option<Group> {
    let prob_list;
    if let GDValue::ProbabilitiesList(probs) = probabilities {
        prob_list = probs;
    } else {
        // Skip evaluation of an irrelevant value.
        return None;
    }

    // There are no probabilities to choose from.
    if prob_list.is_empty() {
        return None;
    }

    // If only one group can be triggered, then only that group may be activated.
    if prob_list.len() == 1 {
        return Some(Group::Regular(prob_list[0].0));
    }

    // Get total chance and threshold
    let total_chance: i32 = prob_list.iter().map(|(_, chance)| chance).sum();
    // threshold is in the range \[0, total_chance]
    let threshold = fast_rand_bits_norm(seed) * total_chance as f64;

    let mut cumulative_chance = 0;
    let mut chosen_group = prob_list.last().unwrap().0;

    // Iterate through all groups until the threshold is reached.
    // If the threshold is never reached, the last group is activated.
    for (group, chance) in prob_list.iter() {
        cumulative_chance += chance;
        if cumulative_chance as f64 >= threshold {
            chosen_group = *group;
            break;
        }
    }

    Some(Group::Regular(chosen_group))
}
