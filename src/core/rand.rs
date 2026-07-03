//! Implementations and uses of Pseudo-RNG in Geometry Dash.

use crate::cclocallevels::gdobj::structs::{GDValue, Group};

const LCG_MULTIPLIER: u64 = 214_013;
const LCG_CONSTANT: u64 = 2_531_011;

/// Determines the next seed from a starting seed as generated in Geometry Dash.
#[inline(always)]
#[must_use]
pub fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(LCG_MULTIPLIER).wrapping_add(LCG_CONSTANT)
}

/// Mutating version of [`next_seed`]
#[inline(always)]
#[must_use]
pub fn next_seed_mut(seed: &mut u64) {
    *seed = seed.wrapping_mul(LCG_MULTIPLIER).wrapping_add(LCG_CONSTANT);
}

/// Function used by GD to generate a new seed. Internally known as `fast_rand`.
/// Unlike the actual PRNG used in GD, this function *DOES NOT* automatically update the seed.
#[inline(always)]
#[must_use]
pub fn fast_rand_bits(seed: u64) -> u64 {
    (next_seed(seed) >> 16) & 0x7fff
}

/// Utility function which normalises result from [`fast_rand_bits`] to the range \[0.0, 1.0].
/// Internally known as `fast_rand_0_1`
#[inline(always)]
#[must_use]
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
#[must_use]
pub fn check_seed_random(seed: u64, chance: f64) -> bool {
    // Compare against the chance threshold
    fast_rand_bits_norm(seed) < chance
}

/// Determines the group that an advanced random trigger will activate based on an input seed
/// and a list of the trigger's activation probabilities per group as a [`GDValue::ProbabilitiesList`].
/// Note that this is the same type as the advanced trigger's `RANDOM_PROBABILITIES_LIST` property.
/// If the given list of probabilities is empty, this method will return `None`.
///
/// For accuracy, please do not sort or prune the list in any way.
/// Doing so may and likely will affect the results of the check.
///
/// Note: this function does not automatically update the seed. To do so, refer to [`next_seed`] or [`next_seed_mut`].
///
/// This algorithm was sourced from the Andriod APK for GD and is verified to work as of GD 2.2801.
#[must_use]
pub fn check_seed_advanced_random(seed: u64, probabilities: &GDValue) -> Option<Group> {
    // tuples of (group, chance)
    let prob_list = match probabilities {
        GDValue::ProbabilitiesList(probs) => probs,
        _ => return None,
    };

    let total_chance: i32 = prob_list.iter().map(|(_, chance)| chance).sum();
    let rand_seed = fast_rand_bits_norm(seed) as f32;
    let mut buf_ptr = 0;

    if !prob_list.is_empty() {
        let mut accumulated_chance = prob_list[buf_ptr].1; // chance

        loop {
            if (rand_seed * total_chance as f32) as i32 <= accumulated_chance {
                return Some(Group::Regular(prob_list[buf_ptr].0)); // return the group id 
            }

            if buf_ptr == prob_list.len() - 1 {
                // since we have reached the end of the list but still need to pick a group
                // we choose the last one of the list.
                return Some(Group::Regular(prob_list[buf_ptr].0));
            }

            buf_ptr += 1;
            accumulated_chance = accumulated_chance + prob_list[buf_ptr].1;
        }
    }
    return None; // fallback
}
