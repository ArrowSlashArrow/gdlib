# Documentation review log

Review of every documented item in `src/` (excluding `src/tests/` and the build-script-generated `gdobj::ids` constants), done on 2026-11-05.

Docs that were accurate and clear enough were left alone, including terse `Internal key: kXX` field docs, which match the convention the module docs describe. The changes below fix docs that were wrong, misleading, truncated, broken in rendering, or missing something a caller would need.

## Verification

- `cargo doc --no-deps`: **0 warnings or errors.** Before this review it **failed to build** with 9 broken intra-doc links (the crate has `#![deny(rustdoc::broken_intra_doc_links)]`), plus 1 warning. The same failure happens on a clean checkout of `HEAD`.
- `cargo test --doc`: 17 passed, 0 failed. The `constructors` module example called a function that no longer exists, and is now fixed.
- `cargo test --lib`: 1052 passed, 108 failed, the same count as before these edits. Only doc comments were changed.

## Changes made

### Broken doc links (these stopped `cargo doc` from building)

| File | Fix |
|---|---|
| `cclocallevels/mod.rs` | The "Serializer and deserializer functions" list was written as markdown link definitions (`* [x]: path`), so it rendered as empty bullets. One line had trailing text, which broke its definition, and it named `export_to_file_with_backup`, which doesn't exist. Replaced it with one sentence about `export_to_savefile` / `export_to_savefile_with_backup` and a block of valid link definitions, which also fixes the links in the file-format table. |
| `gdobj/mod.rs` | `[`constructors::text`]` (removed function) now links to `constructors::TextObject` with `GDObject::from_config`. |
| `gdobj/mod.rs` (`ids` module docs) | `objects`, `level_header` and `metadata` didn't resolve, and `properties` silently linked to the **wrong** module (`cclocallevels::properties` instead of `ids::properties`). All four now use full paths. Also removed the redundant-link warning. |
| `gdobj/constructors/mod.rs` | `[`GDObjConfig`]` wasn't in scope, so it now uses the full path. |
| `gdlevel/mod.rs` | `[`core::io::linux_gd_files_path`]` is now `crate::core::...`. |

### Broken doctest

- `gdobj/constructors/mod.rs`: the module example used `constructors::text(...)`, which no longer exists. It now builds a `TextObject` and passes it to `GDObject::from_config`.

### Wrong or misleading docs

| Item | What was wrong |
|---|---|
| `core::io::decrypt_file` | Said "Returns CCLocalLevels.dat decrypted". It decrypts the file at the given path (either savefile), and **strips every non-ASCII byte**, which is now stated. |
| `core::rand::fast_rand_bits` | Said it "generate[s] a new seed". It returns the 15-bit random value (0–32767) derived from the next seed. |
| `core::GDError::AhoCorasick` | Said "(UTF-8 parsing issues)". It's an automaton build error, and the crate can no longer produce it now that the automaton is a `LazyLock`. |
| `core::GDError::DecodeError` | "Data could not be parsed from its raw form" is now "Base64 decoding failed". |
| `GDLevel::from_gmd`, `from_gdlvl` | Listed "AhoCorasick error" as a failure mode, which can't happen now. `from_gdlvl` was missing the base64/gzip decode failure. |
| `GDLevel::get_decrypted_data`, `get_decrypted_data_ref` | Said they return a `GDLevelContents`. They return `GDLevelData`. Also documented that the first works on a copy, the second stores the decrypted data in `self`, and when each returns `None`. |
| `GDLevel::format_to_gmd`, `format_to_gdlvl` | "raw level data" sounded like the `k4` level string. They take a stringified level dict (`Self::stringified`). |
| `GDLevel::serialise_to_gdlvl_stripped_keys` | Doc was cut off mid-sentence ("returns the level serialized"). |
| `GDLevelUnknowns::k91`, `k92` | `@nodoc` isn't rustdoc syntax and rendered literally. These now have real one-line docs with their keys. |
| `GDVersion::from_binary_version` | Said the value is preserved by `to_game_version_id`. It's `to_binary_version_id`. |
| `ColourString::copy_opacity` | `@nodoc` rendered literally. It now has a doc (property 17). |
| `GDObjConfig::add_group(s)` | Said "Groups are not automatically re-sorted". They are sorted and deduplicated, as are `with_groups`/`set_groups`. |
| `RotationConfig::Custom` | Said to use `RotationConfig::Default` for 0°/s, but `Default` means GD's random rotation. It now points to `Disabled`. |
| `TargetMove` / `DirectionalMove::target_group_id` | Pointed to `POS_PLAYER1`/`POS_PLAYER2` consts, which don't exist. They now point to `MoveTarget::Player1`/`Player2`. |
| `DirectionalMove` | Had a copy of `TargetMove`'s summary ("moving to a specific target"). |
| `CompareOperand` | Called the item compare trigger "the item control trigger". `modifier` was described as a "Multiplier", but it's combined using `mod_op`. |
| `SpeedPortal` | Pointed to `trigger_cfg.multiactivate`, but the field is `multitriggerable`. |
| `gameplay` module table | Listed `Force` as what selects a force object. That's `ForceObject`; `Force` is the extra option. |
| `ScaleTrigger::easing` | Linked `MoveEasing`, but the field is `Easing`. |
| `GDStatistics::achievements` | Said values are 0–100, which contradicts `AchievementProgress` (up to 10,000), and didn't say how the array is indexed. |
| `GDConfig::saved_levels_foldernames`, `local_levels_foldernames` | Said unnamed folders are stored as `None`, but the type is `Vec<(i32, String)>`. Now says (folder number, name) pairs, sorted. |
| `GDStatistics::set_achievement_by_ident` | Referred to `get_vanilla_achievement` and said "looked up in" for a setter. |
| `GDAccount::hashed_password` | Said "encrypted with GJP2". It's a hash. |
| `core::io::encrypt_level_str` | Called it encryption of "a GDLevel object string". It gzips and URL-safe base64-encodes any string. |
| `core::io::decompress` | Called the output "decompressed base64ed gzip". Now says what it decodes, that it strips null bytes, and which error it returns for each failure. |
| `core::io::stringify_xml` | Said "Parses". It serialises. Now documents `root`, the escaping, and that unsupported value types write a key with **no value** (see code issues). |
| `core::io::linux_gd_files_path` | Now says it's the Steam Proton path, isn't checked for existence, and is `None` only without `HOME`. |
| `core::get_cclocallevels_path`, `get_ccgamemanager_path` | Now say which locations are checked, in order. |
| `core::b64_encode`, `b64_decode` | Didn't mention the URL-safe alphabet. |
| `core::proper_plist_tags` | Notes that it currently always returns `Ok`. |

### Truncated docs and typos that changed the meaning

- `core` module docs: "types shared by more than one savefil" was cut off.
- `structs::Easing`: "How an easing" was cut off.
- `GDValue::Easing`: "See [`Easing`] for a more" was cut off.
- `CameraGuide`: "in the edito" is now "editor".
- `gdlist::GDList::from_dictionary`: "Parses a plist dictionary a GD list object" was missing "to". Also documented when it returns `None`.
- `defaults::default_object`: the fallback order stopped after the first case.
- `GDLevelIntegrity`: "checking the integrity of the level's integrity".
- `GuidelineString`: "guildline" is now "guideline".

### Rendering fixes

Lists indented by 4 spaces directly after a paragraph line were merged into the paragraph. They now render as lists in `GDLevelHeaderKey::parse`, `SpinningObject` and `TimeTrigger::dont_override`. `GDLevelContents::data` was also missing a blank line, so "See GDLevelState Internal key: k4" rendered as one line.

### Smaller additions

- `GDObject::parse_str`: never fails, and unparsable values become defaults.
- `GDObject::get_property`: covers both config and object-specific properties, and says when it returns `None`. It used to say "from this object's configuration".
- `GDObject::get_properties`: object-specific only, sorted by ID.
- `GDLevel::export_to_gdlvl`: keeps every key, unlike GD.
- `GDLevelData::parse`: takes the encoded `k4` string.
- `GDLevelData::get_unused_groups`: covers groups 1–9999.
- `GDList::levels`: stored as a dict keyed by ID, and levels that fail to parse are skipped.
- `round_3dp`: clamps beyond about ±2,147,483.647.
- `parse_lists_from_value`, `serialise_*_fields` (crate-private): describe what's actually parsed and skipped.

## Docs I couldn't fix confidently

- **`AchievementProgress`** contradicts itself. It says 100 means "completed regardless of format", but also that values on the 0–10,000 scale are hundredths of a percent, where 100 would be 1%.
- **`GDLevelContents::is_platformer` (`k94`):** `missing_gdlevel_properties.md` already flags that `k94` is probably "friends only", not platformer.
- **`GDLevelRatings::rating` / `rating_sum` (`k9`/`k10`):** possibly swapped, also flagged in `missing_gdlevel_properties.md`.
- **README** (used as the crate root docs): "Full support for read from and writing to .dat" isn't true for CCGameManager.dat, which is read-only. The minimal example's comment says `get_decrypted_data_ref` returns `None` "only if it hasn't been initialized", but it also returns `None` when decryption fails.

## Undocumented items

### Public items with no docs

The compiler doesn't flag these because of `#[allow(missing_docs)]`, including the one built into `repr_t!` and `object_descriptor!`. Many are self-explanatory (colour channels, `Player1`/`Player2`), but they are listed for completeness.

**Types with no doc comment at all:**
- `GDListType` (`gdlevel/enums.rs`)
- `DifficultyRating` (`gdlevel/enums.rs`)
- `GDLevelHeaderValue` (`gdlevel/leveldata.rs`)
- `ZLayer` (`gdobj/structs.rs`)
- `BPMSpeed` (`gdobj/structs.rs`)
- `AdvancedRandomTrigger` (`gdobj/constructors/triggers.rs`)

**Struct fields:**
- `StartposConfig` (`triggers.rs:208-216`): `starting_gamemode`, `starting_as_mini`, `starting_as_dual`, `starting_mirrored`, `reset_camera`, `rotate_gameplay`, `reverse_gameplay`, `target_order`, `target_channel`
- `AreaStopTrigger`: `effect_id`
- `PickupTrigger`: `count`
- `InstantCountTrigger`: `item_id`, `target_group`, `target_count`
- `Colour`: `red`, `green`, `blue`

**Fields that render the literal text `@nodoc`** (not a rustdoc feature):
- `GDObjAttributes`: all 24 flags (`meta.rs:410-456`)
- `TeleportConfig`: `ignore_x`, `ignore_y`, `snap_ground`
- `TeleportForce::Static`: `force`, `is_additive`
- `TeleportForce::Redirect`: `min`, `max`
- `Force::Range`: `minimum`, `maximum`
- `TeleportTrigger`: `teleport_config`

**Enum variants with no docs:**
- `gdlevel/enums.rs`:
  - every variant of `GDListType`, `GDListDifficulty`, `EpicRating`, `GDLevelType`, `DemonType`, `Length`, `FeatureRating`, `DifficultyRating`
  - all 41 variants of `OfficialSong`
- `gdlevel/leveldata.rs`:
  - every variant of `GDLevelHeaderValue`, `GuidelineColour`, `PlayerColour` and `GDLevelHeaderValType`
- `core/structs.rs`:
  - every variant of `KCEKValue`
- `gdobj/constructors/`:
  - every variant of `GDPadType`, `GDOrbType`, `ForceObject`, `SpeedPortal`, `SpinningObject` (74)
  - all `GameplayPortal` variants except `UnlinkedBlueTeleportal`, `LinkedTeleportals` and `LinkedOrangeTeleportal`
  - `OrbOptions::DashOrb` and `OrbOptions::Teleporb`
- `gdobj/structs.rs`:
  - all variants of `animation_ids::{BigBeast, Bat, Spikeball}`
  - `Item` (`Counter`, `Timer`, `Points`, `Attempts`, `MainTime`)
  - `ItemType` (`Timer`, `Points`, `MainTime`, `Attempts`)
  - `CounterMode` (`Attempts`, `Points`, `MainTime`)
  - most of `GDObjPropType`
  - `ColourChannel::Channel` and `ColourChannel::Background`
  - every variant of `ZLayer`, `MoveEasing`, `Gamemode`, `StopMode`, `ItemAlign`, `TransitionMode`, `TransitionType`, `Speed`, `MiddleGround`, `ArrowDirection`, `Toggle`, `PickupTriggerMode`, `InstantCountComparison`
  - all 77 variants of `Event`
  - `Op` (`Add`, `Sub`, `Mul`, `Div`)
  - `RoundMode` (`Nearest`, `Floor`, `Ceiling`)
  - `SignMode` (`Absolute`, `Negative`)
  - `UIReferencePos` (all 8)
  - `AxisOnlyMove::Unset`
  - `TeleportGravity` (`Unset`, `Normal`, `Flipped`)
  - `MoveTarget::Player1` and `MoveTarget::Player2`
  - `MoveLock::Player` and `MoveLock::Camera`
  - `RotationPlayerTarget::Player1` and `RotationPlayerTarget::Player2`
- `ccgamemanager/structs.rs`:
  - every variant of `TextureQuality` (`Auto`, `Low`, `Medium`, `High`)
  - the variants of `Resolution`

### Private and crate-visible items with no docs

These came from `clippy::missing_docs_in_private_items`, 50 in total. Only the non-obvious ones are worth documenting.

- `core/mod.rs`:
  - `count_exprs!` and `build_plist_tags!` macros
  - `PLIST_TAGS_AC`
  - `from_error!` macro
- `core/io.rs`:
  - `zlib_compress`
  - `vec_as_str_lossy`
- `core/crypto.rs`: `PASSWORD_SALT`
- `core/rand.rs`: `LCG_MULTIPLIER`, `LCG_CONSTANT`
- `cclocallevels/mod.rs`: `LEVEL_HEADER_PROP_ID_OFFSET`
- `gdlevel/mod.rs`:
  - `to_float`
  - `to_csv`
- `gdlevel/leveldata.rs`: `parse!` macro
- `gdobj/mod.rs`:
  - `parse!`, `prop_value!`, `object_field_match!` and `object_descriptor!` macros (the last three have explanatory `//` comments but no doc comments)
  - `KA_PROPERTY_OFFSET`
  - `set_property_raw`
  - `get_attr_as_gdvalue`
- `gdobj/meta.rs`:
  - `config_fn!` macro
  - `dedup_groups`
  - `GDOBJ_ATTR_FIELDS`
  - `GDOBJ_ATTR_PROPSTR_ALLOCSIZE`
  - `serialise_fields`
- `gdobj/structs.rs`:
  - `LIST_ALLOCSIZE`
  - `parse!`, `fmt_intlist!`, `fmt_inttuples!` and `set_value!` macros
  - `parse_sibling_items`
- `gdobj/constructors/triggers.rs`:
  - `get_property!` macro
  - `add_easing`
- `ccgamemanager/mod.rs`:
  - `IntMap`
  - `parse_dict`, `parse_values`, `parse_val`, `parse_level_dict`, `parse_foldernames`
- `ccgamemanager/structs.rs`: the 11 `GDPlatformerUI::parse_*` helpers (`parse_width` to `parse_split`)
