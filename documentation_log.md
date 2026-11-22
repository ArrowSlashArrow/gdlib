# Documentation review log

Review of every documented item in `src/` (excluding `src/tests/` and the build-script-generated `gdobj::ids` constants), done on 2026-11-05.

Docs that were accurate and clear enough were left alone, including terse `Internal key: kXX` field docs, which match the convention the module docs describe. The changes below fix docs that were wrong, misleading, truncated, broken in rendering, or missing something a caller would need.

## Verification

- `cargo doc --no-deps`: **0 warnings or errors.** Before this review it **failed to build** with 9 broken intra-doc links (the crate has `#![deny(rustdoc::broken_intra_doc_links)]`), plus 1 warning. The same failure happens on a clean checkout of `HEAD`.
- `cargo test --doc`: 17 passed, 0 failed. The `constructors` module example called a function that no longer exists, and is now fixed.
- `cargo test --lib`: 1052 passed, 108 failed, the same count as before these edits. Only doc comments were changed.

## Docs I couldn't fix confidently

- **`AchievementProgress`** contradicts itself. It says 100 means "completed regardless of format", but also that values on the 0–10,000 scale are hundredths of a percent, where 100 would be 1%.
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
