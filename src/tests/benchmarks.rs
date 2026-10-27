//! Timing tests. These print how long an operation takes; run them with `--nocapture` to see the output.
use std::time::Instant;

use super::{benchmark, generated_path};
use crate::cclocallevels::gdlevel::GDLevel;

#[test]
fn big_level_parse() {
    let level = GDLevel::from_gmd("test_gmds/big.gmd").unwrap();
    benchmark("Big level parse", || level.get_decrypted_data());
}

#[test]
fn ref_vs_copy_benchmark() {
    let count = 50;
    let mut ref_time: u128 = 0;
    let mut copy_time: u128 = 0;

    let level = GDLevel::from_gmd("test_gmds/All Object IDs.gmd").unwrap();
    for _ in 0..count {
        {
            let start = Instant::now();
            let _ = level.get_decrypted_data();
            copy_time += start.elapsed().as_nanos();
        }
        {
            let mut level = GDLevel::from_gmd("test_gmds/All Object IDs.gmd").unwrap();
            let start = Instant::now();
            let _ = level.get_decrypted_data_ref();
            ref_time += start.elapsed().as_nanos();
        }
    }
    let objs = level.get_decrypted_data().unwrap().objects.len();
    let avg_copy_time = copy_time as f64 / (1_000 * count) as f64;
    let avg_ref_time = ref_time as f64 / (1_000 * count) as f64;

    println!(
        "Objects: {objs}; {count} tests\nAverage copy time: {:.3}us\nAverage ref time: {:.3}us\nAverage copy time per object: {:.3}us\nAverage ref time per object: {:.3}us",
        avg_copy_time,
        avg_ref_time,
        avg_copy_time / objs as f64,
        avg_ref_time / objs as f64,
    );
}

#[test]
fn serialise_level_benchmark() {
    let mut level = GDLevel::from_gmd("test_gmds/big.gmd").unwrap();
    level.decrypt_level_data().unwrap();
    let _ = benchmark("big.gmd serialise", || {
        level.export_to_gmd(generated_path("big2.gmd"))
    });
}
