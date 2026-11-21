# Failing tests

Results from `cargo test --lib` (2026-11-18): 1135 passed, 24 failed, 5 ignored.

The 24 failures are the 22 listed in the request plus two `game_logic` tests that were not in that list:

- `level_seed_classic_large_values_do_not_overflow`
- `level_seed_platformer_negative_time_does_not_panic`

They only panic in debug builds, because those builds check for integer overflow.

The failures are grouped below by root cause. Each entry gives the test, what it observed, and the code responsible.

---


## 5. `GDPlatformerUI` parse functions `unwrap()` (3 tests)

`GDPlatformerUI::parse_*` (`src/ccgamemanager/structs.rs:409` onwards) call `s.parse::<i32>().unwrap()` / `parse::<f32>().unwrap()`. Any non-numeric or empty field panics.

| Test | Input | Panic |
|---|---|---|
| `deserialisers::platformer_ui_malformed_field_does_not_panic` | `"wide,tall"` | `ParseIntError { InvalidDigit }` at `structs.rs:410` |
| `deserialisers::gamemanager_malformed_dpad_does_not_panic` | `dpad01 = "a,b,c"` | same |
| `deserialisers::gamemanager_non_string_dpad_layout_does_not_panic` | `dpad_layout = <i>5</i>` | `ParseIntError { Empty }`: `ccgamemanager/mod.rs:216` turns the non-string into `""` via `unwrap_or_default()`, which then hits the same `unwrap` |

**Fix direction:** use `.unwrap_or_default()` (or keep the existing value) in every `parse_*` function.

## 6. `GLM_12` key parsing `unwrap()`s a missing segment (1 test)

| Test | Observed |
|---|---|
| `deserialisers::gamemanager_short_glm12_key_does_not_panic` | panic `Option::unwrap() on None` at `ccgamemanager/mod.rs:295` |

The parser expects keys shaped like `likes_a_b_c_d` and calls `split.next().unwrap()` four times. The key `likes_1` has only one segment after `likes`.

**Fix direction:** `split.next().and_then(|s| s.parse().ok()).unwrap_or_default()`.

## 7. Savefile reader strips every non-ASCII byte (1 test)

| Test | Observed |
|---|---|
| `files::cclocallevels_file_keeps_non_ascii_text` | name `"Niveau lev "`, expected `"Niveau élevé 日本"` |

`decrypt_file` (`src/core/io.rs:197`) runs `data.retain(|c| *c < 128)` before `String::from_utf8`. The doc comment admits this. The test expects UTF-8 text to survive.

**Fix direction:** drop the `retain` and use `String::from_utf8_lossy` (or only remove the specific junk bytes that `CCGameManager` contains).


## 9. `encrypt_level_str` writes an 8-byte ISIZE (1 test)

| Test | Observed |
|---|---|
| `serialisers::encrypted_level_string_is_valid_gzip` | gzip trailer CRC slot is `[23, 0, 0, 0]`, expected the CRC32 |

`core/io.rs:54`: `let size = s.len().to_le_bytes();`. `s.len()` is a `usize`, so this produces **8** bytes on 64-bit targets. The trailer ends up as CRC (4) + size (8). The last 8 bytes the test reads are therefore all size bytes, and the length `23` lands where the CRC should be. Lenient decoders (including flate2's reader in this test, and apparently GD) still decompress it, but the gzip stream is malformed.

**Fix direction:** `(s.len() as u32).to_le_bytes()`.

## 11. Unknown object keys collapse into one property (1 test)

| Test | Observed |
|---|---|
| `deserialisers::object_unparseable_keys_are_kept_apart` | `"1,1,foo,1,bar,2;"` gives 1 property `[(65535, "2")]`, expected 2 |

`GDObject::parse_str` (`gdobj/mod.rs:197`) maps every key it can't parse to `u16::MAX`. `set_property` then overwrites the earlier value.

**Fix direction:** keep unparseable keys in a separate list (e.g. `Vec<(String, String)>`), or give each one its own ID.

## 13. Smaller one-off bugs (5 tests)

| Test | Observed | Cause |
|---|---|---|
| `serialisers::level_binary_version_without_known_id_is_not_written_as_zero` | `k50 = 0` | `GD2207` (and 2.2071–2.2073) have `binary: None` (`version.rs:178`). `to_dict` writes `to_binary_version_id().unwrap_or_default()` (`mod.rs:1418`), which is 0. Fix: leave `k50` out when it's `None`, or fall back to the nearest known binary version. |
