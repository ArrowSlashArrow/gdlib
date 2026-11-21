# Failing tests

Results from `cargo test --lib` (2026-11-18): 1135 passed, 24 failed, 5 ignored.

The 24 failures are the 22 listed in the request plus two `game_logic` tests that were not in that list:

- `level_seed_classic_large_values_do_not_overflow`
- `level_seed_platformer_negative_time_does_not_panic`

They only panic in debug builds, because those builds check for integer overflow.

The failures are grouped below by root cause. Each entry gives the test, what it observed, and the code responsible.

---

## 6. `GLM_12` key parsing `unwrap()`s a missing segment (1 test)

| Test | Observed |
|---|---|
| `deserialisers::gamemanager_short_glm12_key_does_not_panic` | panic `Option::unwrap() on None` at `ccgamemanager/mod.rs:295` |

The parser expects keys shaped like `likes_a_b_c_d` and calls `split.next().unwrap()` four times. The key `likes_1` has only one segment after `likes`.

**Fix direction:** `split.next().and_then(|s| s.parse().ok()).unwrap_or_default()`.