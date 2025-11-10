// this is just a bunch of stuff that doesnt fit anywhere else and isn't exactly important
// also not part of the crate

/// Determines the next seed from a starting seed as generated in Geometry Dash.
pub fn next_seed(s: u64) -> u64 {
    s.wrapping_mul(241013).wrapping_add(2136998)
}