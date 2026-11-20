# Failing tests

Results from `cargo test --lib` (2026-11-18): 1135 passed, 24 failed, 5 ignored.

The 24 failures are the 22 listed in the request plus two `game_logic` tests that were not in that list:

- `level_seed_classic_large_values_do_not_overflow`
- `level_seed_platformer_negative_time_does_not_panic`

They only panic in debug builds, because those builds check for integer overflow.

The failures are grouped below by root cause. Each entry gives the test, what it observed, and the code responsible.

---

## 3. `set_raw_key` failures are ignored by `from_dict` (1 test)

| Test | Observed |
|---|---|
| `deserialisers::level_description_with_invalid_base64_returns_none` | `from_dict` returned `Some` |

For `k3`, `set_raw_key` correctly returns `None` when `b64_decode` fails (`mod.rs:1055`). However, `from_dict` calls `level.set_raw_key(k, v);` and throws the result away, then always returns `Some(level)`.

**Fix direction:** use `level.set_raw_key(k, v)?;`. Be aware that this makes *any* mistyped known key reject the whole level. Some other tests (e.g. `level_csv_with_non_string_value_does_not_panic`, plus the loop just above `level_empty_csv_is_an_empty_list`, which asserts `is_some()` for odd values) expect leniency. You may want to propagate failure only for `k3`, or loosen the test.

Actual fix: include a parameter for strict parsing in `from_dict`, that returns None if a single property fails to pass, and forward that through every constructor for GDLevel

## 4. `kI7` (editor layer) is read as an integer but stored as a real (1 test)

| Test | Observed |
|---|---|
| `deserialisers::level_fields` | `editor_state.editor_layer` is `0`, expected `2` |

The test supplies `kI7` as `Value::Real(2.0)`, which matches the `to_dict` comment block saying kIX floats are 1, 2, 3 and 7. The parser (`mod.rs:1038`) uses `v.as_signed_integer()`, which returns `None` for a real, so `unwrap_or_default()` gives 0. `to_dict` also writes `kI7` as an integer (`mod.rs:1308`).

**Fix direction:** read it with `to_float(v)` (or accept both types), and write it back in the same type GD uses.

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

## 8. Replay data serialised with a panicking UTF-8 conversion (1 test)

| Test | Observed |
|---|---|
| `roundtrips::level_with_non_ascii_replay_data` | panic `Utf8Error { valid_up_to: 1 }` at `core/io.rs:203` |

`to_dict` writes `k34`, `k119`, `k120` and `k131` (`Option<Vec<u8>>`) using `vec_as_str`, which is `from_utf8(..).unwrap()`. The bytes `[0x00, 0xff, 0x80, b'a']` are not valid UTF-8. The parser also stores these as `as_string().as_bytes()`, so arbitrary bytes can't round-trip through a plist string as things stand.

**Fix direction:** decide on an encoding. Either the fields are really strings (make them `String`), or they are bytes, in which case encode them in a lossless way, e.g. map each byte to a `char` as latin-1 when writing and back to a byte when reading.

## 9. `encrypt_level_str` writes an 8-byte ISIZE (1 test)

| Test | Observed |
|---|---|
| `serialisers::encrypted_level_string_is_valid_gzip` | gzip trailer CRC slot is `[23, 0, 0, 0]`, expected the CRC32 |

`core/io.rs:54`: `let size = s.len().to_le_bytes();`. `s.len()` is a `usize`, so this produces **8** bytes on 64-bit targets. The trailer ends up as CRC (4) + size (8). The last 8 bytes the test reads are therefore all size bytes, and the length `23` lands where the CRC should be. Lenient decoders (including flate2's reader in this test, and apparently GD) still decompress it, but the gzip stream is malformed.

**Fix direction:** `(s.len() as u32).to_le_bytes()`.

## 10. Level header serialisation issues (2 tests)

`GDLevelData::serialise_level_headers` (`gdlevel/leveldata.rs:246`):

| Test | Observed | Cause |
|---|---|---|
| `serialisers::level_headers_only_ks38_has_no_dangling_comma` | output `kS38,,;` | `kS38` is always written as `"kS38,{v},"` with a trailing comma, even when no other headers follow. |
| `serialisers::level_headers_are_deterministic` | the `kA*` keys come out in a different order on each call | `headers` is a `HashMap` (`leveldata.rs:74`), whose iteration order is randomised per instance. |

**Fix direction:** collect all the parts (kS38 first) into a `Vec` and `join(",")`. Then either sort the remaining keys or switch `headers` to a `BTreeMap` / `IndexMap`.

## 11. Unknown object keys collapse into one property (1 test)

| Test | Observed |
|---|---|
| `deserialisers::object_unparseable_keys_are_kept_apart` | `"1,1,foo,1,bar,2;"` gives 1 property `[(65535, "2")]`, expected 2 |

`GDObject::parse_str` (`gdobj/mod.rs:197`) maps every key it can't parse to `u16::MAX`. `set_property` then overwrites the earlier value.

**Fix direction:** keep unparseable keys in a separate list (e.g. `Vec<(String, String)>`), or give each one its own ID.

## 12. Property 87 (multitrigger) is written twice (1 test)

| Test | Observed |
|---|---|
| `serialisers::object_property_that_belongs_to_config_is_not_duplicated` | `1,1,2,0,3,0,87,1,87,0;` |

`GDObject::new` (`gdobj/mod.rs:559`) stores the property list as given. It doesn't redirect properties that belong to `GDObjConfig` (as `set_property` does), so `MULTITRIGGERABLE` stays in `properties`. `serialise_to_string` (`mod.rs:531`) then writes both the config's `87` and the list's `87`.

**Fix direction:** in `new`, route each property through `set_property`. Or, in `serialise_to_string`, skip list entries whose ID is a config property.

## 13. Smaller one-off bugs (5 tests)

| Test | Observed | Cause |
|---|---|---|
| `serialisers::level_display` | `<No desciption>` | Typo at `gdlevel/mod.rs:1669`. |
| `roundtrips::gdvalue_item_type_counter_default` | `ItemType(Unrecognized(0))`, expected `ItemType(Counter)` | `GDValue::from(ItemType, "")` uses `parse!(s => i32 => ItemType)` (`gdobj/structs.rs:674`). `""` parses to `0`, and `ItemType::try_from(0)` *succeeds* with `Unrecognized(0)`, so the `unwrap_or_default()` that should give `Counter` never runs. The `ItemType::Counter` doc comment promises the default. Fix: map an empty string (or 0) to `ItemType::default()` for this type. |
| `serialisers::level_binary_version_without_known_id_is_not_written_as_zero` | `k50 = 0` | `GD2207` (and 2.2071–2.2073) have `binary: None` (`version.rs:178`). `to_dict` writes `to_binary_version_id().unwrap_or_default()` (`mod.rs:1418`), which is 0. Fix: leave `k50` out when it's `None`, or fall back to the nearest known binary version. |
