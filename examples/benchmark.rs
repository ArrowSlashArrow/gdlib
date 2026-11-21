use std::time::Instant;

use gdlib::cclocallevels::gdlevel::CCLocalLevels;
fn benchmark<F: FnMut() -> R, R>(name: &str, mut f: F) -> R {
    let start_time = Instant::now();
    let result = f();
    let end_time = Instant::now();
    println!(
        "{name}: {:.9}s",
        (end_time - start_time).as_nanos() as f64 / 1_000_000_000.0
    );
    result
}

fn main() {
    let mut cc = benchmark("Read CCLocalLevels", || {
        CCLocalLevels::from_local().unwrap()
    });
    let _parsed = benchmark("Serialise CCLocalLevels", || {
        CCLocalLevels::from_decrypted(cc.serialise_to_string()).unwrap()
    });
}
