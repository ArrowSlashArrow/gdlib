# Proposal: better module documentation

GDLib's item docs are decent, but most module docs are one line that repeats the module's name. `gdobj::constructors::triggers` is the exception: it explains how to use the module, not just what's in it. This proposal applies that approach to all 26 public modules and includes a draft for each one.

**Scope:** this only covers `//!` module docs. Item-level docs (`///`) are out of scope, apart from the fixes in section 4.

**How the drafts were checked:** I pasted every draft into a throwaway copy of the working tree, then ran `cargo doc` and `cargo test --doc`. See [section 5](#5-validation) for the results. Nothing in `src/` has been changed.

## 1. What makes the triggers docs work

The current `triggers` docs answer the questions a reader actually has:

| Question | How `triggers` answers it |
|---|---|
| What is this for? | "Constructors for trigger objects" |
| How do I use it? | Build a struct, then call `GDObject::from_config` |
| What's the exception? | Triggers with no parameters use a function instead, and those are listed with links |

By comparison, here's what some other modules say today:

| Module | Current docs | What's missing |
|---|---|---|
| `gdlist` | "GD lists" | How to load one, and that its levels are partial summaries |
| `gdobj::structs` | "All structs and enums for values..." | Any way to find one of its 50+ types |
| `core` | "Various utilities for debugging and processing structs" | It actually holds the error type and savefile lookup |
| `ccgamemanager` | "Everything in the CCGameManager.dat file" | That it's read-only, can panic, and holds account credentials |
| `leveldata` | "Operations with individual levels" | That it's specifically about the `k4` level string, and its format |

There are also no code examples in any `src/` docs. The only example in the rendered docs is the one from the README.

## 2. Proposed standard

Every module doc starts with one sentence saying what the module is for. After that, add only the sections that have something useful to say:

1. **How to use it:** the usual workflow and its entry points, with links.
2. **Choosing between things:** a table or list when a module has many items or several ways to do something.
3. **Format:** how the data looks on disk, when the module reads or writes GD data.
4. **Gotchas:** behaviour that would surprise someone, such as panics, silent no-ops and lost data.
5. **References:** external sources for the format.

| Module type | Expected docs | Examples |
|---|---|---|
| **Entry point** | Everything relevant, plus a tested example | `gdlevel`, `gdobj`, `ccgamemanager` |
| **Domain module** | Purpose, format or selection guide, gotchas | `leveldata`, `meta`, `structs`, `io` |
| **Small or lookup module** | A few lines, plus an example if it's quick | `crypto`, `consts`, `defaults` |

**Conventions:**
- Write examples as doctests. Use `no_run` for anything touching files or savefiles, so tests never read or overwrite a real save.
- Say "module", not "file", since rustdoc shows modules.
- Link to items instead of naming them in plain text, so renames break the docs build instead of leaving stale text.
- Keep gotchas to what's true now. When an entry in `unresolved-issues.md` is fixed, remove the matching gotcha. The gotchas in these drafts reflect the current working tree.
- Don't hardcode counts of things that can change. "There are 6 of these" followed by a list of 7 is already the case in `triggers`.

## 3. Module drafts

The drafts are in priority order, so you can stop after any tier and still have something useful:

| Tier | Modules | Why |
|---|---|---|
| **1. Entry points** | crate root, `prelude`, `cclocallevels`, `gdlevel`, `gdobj`, `ccgamemanager` | Where every new reader starts |
| **2. Domain modules** | `leveldata`, `meta`, `gdobj::structs`, `constructors`, `gameplay`, `triggers`, `io`, `version`, `gdlist` | Most of the day-to-day API |
| **3. Small and lookup modules** | Everything else | Quick to write, and rarely the first place people look |

---

### Tier 1: entry points

#### Crate root

**File:** `src/lib.rs` (add after the `#![doc = include_str!("../README.md")]` line)

```rust
//! # Module map
//!
//! | If you want to... | Start at |
//! |---|---|
//! | Load, edit or save a single level | [`cclocallevels::gdlevel::GDLevel`] |
//! | Load or save all of a player's levels | [`cclocallevels::gdlevel::CCLocalLevels`] |
//! | Create or edit objects and triggers | [`cclocallevels::gdobj`] |
//! | Work with level lists | [`cclocallevels::gdlist::GDList`] |
//! | Read player stats, settings and account data | [`ccgamemanager::CCGameManager`] |
//! | Predict random triggers | [`core::rand`] |
//! | Handle errors | [`core::GDError`] |
//!
//! Commonly used items are re-exported from [`prelude`].
//!
//! # Feature flags
//!
//! - `parallel`: uses Rayon to parse and serialise objects, and to run the savefile XOR pass, in parallel.
```

#### `prelude`

**Inline in:** `src/lib.rs`, `pub mod prelude`

```rust
//! Common types and traits, for glob importing.
//!
//! ```
//! use gdlib::prelude::*;
//!
//! let level = GDLevel::new_latest_version();
//! assert!(level.get_objects_ref().is_some());
//! ```
```

#### `cclocallevels`

**File:** `src/cclocallevels/mod.rs`

```rust
//! Everything stored in CCLocalLevels.dat, the savefile that holds a player's created levels and lists.
//!
//! # How the data is organised
//!
//! ```text
//! CCLocalLevels                     the whole savefile
//! ├── levels: Vec<GDLevel>          level metadata: name, creator, stats, editor state...
//! │   └── content.data              the level string (`k4`), parsed on demand
//! │       ├── headers               start settings: colours, speed, gamemode...
//! │       └── objects: Vec<GDObject>
//! └── lists: Vec<GDList>
//! ```
//!
//! # Modules
//!
//! - [`gdlevel`]: levels and the savefile itself. Most programs start here.
//! - [`gdobj`]: objects and triggers.
//! - [`gdlist`]: level lists.
//! - [`properties`]: lookup tables for object names and property types.
//! - [`consts`]: numeric constants from the game.
//!
//! # File formats
//!
//! | Extension | Contains | Encoding | Read with | Write with |
//! |---|---|---|---|---|
//! | `.dat` | All levels and lists | gzip, then base64, then XOR | [`CCLocalLevels::from_local`] | [`CCLocalLevels::export_to_file`] |
//! | `.gmd` | One level | Plain plist | [`GDLevel::from_gmd`] | [`GDLevel::export_to_gmd`] |
//! | `.gdlvl` | One level | gzip, then base64 | [`GDLevel::from_gdlvl`] | [`GDLevel::export_to_gdlvl`] |
//! | `.gmdl` | One list | Plain plist | [`GDList::from_gmdl`] | [`GDList::export_to_gmdl`] |
//!
//! GD's plists use shortened tags, such as `<k>` for `<key>` and `<s>` for `<string>`. See [`crate::core::proper_plist_tags`].
//!
//! [`CCLocalLevels::from_local`]: gdlevel::CCLocalLevels::from_local
//! [`CCLocalLevels::export_to_file`]: gdlevel::CCLocalLevels::export_to_file
//! [`GDLevel::from_gmd`]: gdlevel::GDLevel::from_gmd
//! [`GDLevel::export_to_gmd`]: gdlevel::GDLevel::export_to_gmd
//! [`GDLevel::from_gdlvl`]: gdlevel::GDLevel::from_gdlvl
//! [`GDLevel::export_to_gdlvl`]: gdlevel::GDLevel::export_to_gdlvl
//! [`GDList::from_gmdl`]: gdlist::GDList::from_gmdl
//! [`GDList::export_to_gmdl`]: gdlist::GDList::export_to_gmdl
```

#### `cclocallevels::gdlevel`

**File:** `src/cclocallevels/gdlevel/mod.rs`

```rust
//! Levels, and the CCLocalLevels.dat savefile that contains them.
//!
//! [`GDLevel`] is one level, and [`CCLocalLevels`] is the whole savefile.
//!
//! # Loading and creating levels
//!
//! - From the savefile: [`CCLocalLevels::from_local`], then look through [`CCLocalLevels::levels`].
//! - From a file: [`GDLevel::from_gmd`] or [`GDLevel::from_gdlvl`].
//! - From scratch: [`GDLevel::new_latest_version`] or [`GDLevel::with_level_data`].
//!
//! # Editing objects
//!
//! Level data is left compressed when a level is loaded, because decompressing it is slow. Decompress it before working with objects:
//!
//! ```no_run
//! use gdlib::cclocallevels::gdlevel::GDLevel;
//!
//! # fn main() -> Result<(), gdlib::core::GDError> {
//! let mut level = GDLevel::from_gmd("level.gmd")?;
//!
//! // Decompresses and parses the level data on first use.
//! if let Some(data) = level.get_decrypted_data_ref() {
//!     for object in data.objects.iter_mut() {
//!         object.config.add_group(42);
//!     }
//! }
//!
//! level.export_to_gmd("edited.gmd")?;
//! # Ok(())
//! # }
//! ```
//!
//! # How fields are organised
//!
//! [`GDLevel`] splits a level's properties into groups: [`identity`](GDLevel::identity), [`content`](GDLevel::content), [`ratings`](GDLevel::ratings), [`coins`](GDLevel::coins), [`player_stats`](GDLevel::player_stats), [`flags`](GDLevel::flags), [`editor_state`](GDLevel::editor_state), [`meta`](GDLevel::meta), [`integrity`](GDLevel::integrity) and [`unknowns`](GDLevel::unknowns).
//!
//! Most field docs name the key they're stored under (such as `k2`). Keys that gdlib doesn't recognise are kept in [`GDLevelUnknowns::other`] and written back unchanged, so they survive a round trip.
//!
//! # Gotchas
//!
//! - **Use [`GDLevel::new_latest_version`] for new levels.** [`GDLevel::default`] leaves the binary version at the oldest known game version, and GD needs a valid value for that key to open the level.
//! - **Object methods can do nothing on compressed data.** [`GDLevel::add_object`] and [`GDLevel::add_objects`] silently do nothing, and [`GDLevel::get_objects_ref`] returns `None`, until the data is decompressed. Call [`GDLevel::decrypt_level_data`] first.
//! - **One bad level fails the whole savefile.** If any level can't be parsed, [`CCLocalLevels::from_decrypted`] returns an error.
//! - **Writing to the real savefile overwrites it.** Prefer [`CCLocalLevels::export_to_savefile_with_backup`] over [`CCLocalLevels::export_to_savefile`], and close GD first, since it may overwrite your changes when it saves.
//!
//! # References
//!
//! - <https://boomlings.dev/resources/client/level>
```

#### `cclocallevels::gdobj`

**File:** `src/cclocallevels/gdobj/mod.rs`

```rust
//! Objects: the blocks, triggers and decorations that make up a level.
//!
//! A [`GDObject`] has three parts:
//!
//! - [`id`](GDObject::id): which object it is. Constants are in [`ids::objects`], and names are in [`OBJECT_NAMES`].
//! - [`config`](GDObject::config): properties every object can have, such as position and groups. See [`meta::GDObjConfig`].
//! - Object-specific properties: read and change them with [`GDObject::get_property`] and [`GDObject::set_property`], using IDs from [`ids::properties`] and values from [`structs::GDValue`].
//!
//! # Creating objects
//!
//! | You want... | Use |
//! |---|---|
//! | A trigger | A struct from [`constructors::triggers`] with [`GDObject::from_config`] |
//! | A pad, orb, portal or force block | A struct from [`constructors::gameplay`] with [`GDObject::from_config`] |
//! | Text | [`constructors::text`] |
//! | Any other object, with its in-game defaults | [`GDObject::default_from_id`] |
//! | Full manual control | [`GDObject::new`] |
//! | An object from a raw string | [`GDObject::parse_str`] |
//!
//! Constructor structs and functions are also re-exported from this module.
//!
//! # Example
//!
//! ```
//! use gdlib::cclocallevels::gdobj::{GDObject, ids::{objects, properties}, structs::GDValue};
//!
//! let mut block = GDObject::default_from_id(1);
//! block.config.set_pos(30.0, 15.0);
//! block.config.add_group(42);
//!
//! // Objects serialise to GD's raw `key,value,key,value;` format.
//! let parsed = GDObject::parse_str(block.serialise_to_string());
//! assert_eq!(parsed.config.groups, vec![42]);
//!
//! // Object-specific properties are set by ID.
//! let mut trigger = GDObject::default_from_id(objects::MOVE_TRIGGER);
//! trigger.set_property(properties::DURATION_GROUP_TRIGGER_CHANCE, GDValue::Float(2.0));
//! ```
//!
//! # Converting back to a struct
//!
//! [`ObjectProperties::from_object`] converts a [`GDObject`] back into its constructor struct. Not every struct supports this yet, and those that don't return `None`.
//!
//! # Gotchas
//!
//! - **[`GDObject::set_property`] only changes object-specific properties.** [`GDObject::get_property`] can also read [`config`](GDObject::config) values such as position, but change those through `config` itself.
//! - **GDLib doesn't check that objects are valid in-game.** Using the wrong [`structs::GDValue`] type for a property can produce an object GD handles incorrectly. [`get_obj_property_type`] gives the expected type.
```

#### `ccgamemanager`

**File:** `src/ccgamemanager/mod.rs`

```rust
//! Read access to CCGameManager.dat, the savefile that holds a player's profile, stats and settings.
//!
//! [`CCGameManager`] is the top-level struct. Its fields split the file up by area:
//!
//! | Field | Contains |
//! |---|---|
//! | [`player_info`](CCGameManager::player_info) | Username, user ID, selected icons |
//! | [`stats`](CCGameManager::stats) | Statistics and achievement progress |
//! | [`config`](CCGameManager::config) | Game settings |
//! | [`account`](CCGameManager::account) | Account details, including credentials |
//! | [`keybinds`](CCGameManager::keybinds) | Keybinding data, left as raw dictionaries |
//! | [`temp_state`](CCGameManager::temp_state) | Temporary values the game overwrites |
//! | [`song_config`](CCGameManager::song_config) | Song settings |
//! | [`other_properties`](CCGameManager::other_properties) | Keys gdlib doesn't parse yet |
//!
//! # Example
//!
//! ```no_run
//! use gdlib::ccgamemanager::{CCGameManager, achievements::GDAchievement};
//!
//! # fn main() -> Result<(), gdlib::core::GDError> {
//! let save = CCGameManager::from_local()?;
//! println!("{}", save.player_info.username);
//! println!("{}", save.stats.get_vanilla_achievement(GDAchievement::Attempt01));
//! # Ok(())
//! # }
//! ```
//!
//! # Gotchas
//!
//! - **This module can only read.** There's currently no way to write a [`CCGameManager`] back to a file.
//! - **Malformed files can panic.** [`CCGameManager::from_raw_string`] returns errors for some problems, but it panics on others, such as mistyped values.
//! - **This data is sensitive.** [`account`](CCGameManager::account) can hold the player's password in plaintext and in hashed form, and [`CCGameManager`]'s `Debug` output prints both. Don't log it or share it.
```

---

### Tier 2: domain modules

#### `cclocallevels::gdlevel::leveldata`

**File:** `src/cclocallevels/gdlevel/leveldata.rs`

```rust
//! The level string: a level's start settings and objects, stored in key `k4` of a [`GDLevel`](super::GDLevel).
//!
//! # Format
//!
//! The string is gzip-compressed and base64-encoded. Decoded, it looks like this:
//!
//! ```text
//! <headers>;<object>;<object>;...
//! ```
//!
//! `headers` is a flat `key,value,key,value` list of start settings, such as colours, speed and gamemode. See [`GDLevelHeaderKey`], [`GDLevelHeaderValue`] and [`LEVEL_HEADER_PROPERTIES`](crate::cclocallevels::properties::LEVEL_HEADER_PROPERTIES). Each object is described in [`gdobj`](crate::cclocallevels::gdobj).
//!
//! # Main types
//!
//! - [`GDLevelState`]: the level string as either [`Encrypted`](GDLevelState::Encrypted) (still raw) or [`Decrypted`](GDLevelState::Decrypted) (parsed). Levels load in the raw state because parsing is slow.
//! - [`GDLevelData`]: the parsed form, with `headers` and `objects`.
//! - [`ColourString`] and [`GuidelineString`]: header values with their own formats.
//!
//! # Common tasks
//!
//! - Parse the encoded string: [`GDLevelData::parse`]. Most programs should use [`GDLevel::get_decrypted_data_ref`](super::GDLevel::get_decrypted_data_ref) instead.
//! - Create level data for new objects: [`GDLevelData::with_default_headers`].
//! - Find free group IDs: [`GDLevelData::get_used_groups`] and [`GDLevelData::get_unused_groups`].
//!
//! # Example
//!
//! ```
//! use gdlib::cclocallevels::gdlevel::{GDLevel, leveldata::GDLevelData, version::GDVersion};
//! use gdlib::cclocallevels::gdobj::GDObject;
//!
//! let data = GDLevelData::with_default_headers([GDObject::default_from_id(1)]);
//! let level = GDLevel::with_level_data(GDVersion::LATEST, data);
//! assert_eq!(level.get_objects_ref().unwrap().len(), 1);
//! ```
```

#### `cclocallevels::gdobj::meta`

**File:** `src/cclocallevels/gdobj/meta.rs`

```rust
//! Properties shared by all objects: position, scale, groups, colours and more.
//!
//! - [`GDObjConfig`]: the shared properties themselves.
//! - [`TriggerConfig`]: how a trigger is activated (touch, spawn, multi-trigger).
//! - [`GDObjAttributes`]: on/off flags that apply to objects, stored as bit flags.
//!
//! # Building a config
//!
//! Most properties have a pair of methods: `with_*` consumes and returns the config for chaining, and `set_*` changes it in place.
//!
//! ```
//! use gdlib::cclocallevels::gdobj::meta::GDObjConfig;
//!
//! // Chained, for new configs.
//! let config = GDObjConfig::new().with_pos(150.0, 105.0).with_groups([1i16, 2]);
//!
//! // In place, for existing ones.
//! let mut config = config;
//! config.set_pos(300.0, 105.0);
//! config.add_group(3);
//! assert_eq!(config.groups, vec![1, 2, 3]);
//! ```
//!
//! # Gotchas
//!
//! - **Group literals may need a type.** `with_groups([1, 2])` doesn't compile, because the literals default to `i32`. Write `[1i16, 2]`.
//! - **Default values aren't written out.** Properties that match their defaults are left out of the raw object string.
```

#### `cclocallevels::gdobj::structs`

**File:** `src/cclocallevels/gdobj/structs.rs`

```rust
//! The value types used by object properties and constructors.
//!
//! The most important type is [`GDValue`], which holds any object property value. Its expected variant for each property is given by [`GDObjPropType`] and the [`properties`](crate::cclocallevels::properties) tables.
//!
//! # Finding a type
//!
//! | Area | Types |
//! |---|---|
//! | Property values | [`GDValue`], [`GDObjPropType`] |
//! | Colours | [`ColourChannel`], [`Colour`], [`HSVColour`], [`CopyColourConfig`] |
//! | Layers | [`ZLayer`] |
//! | Movement | [`MoveMode`], [`DefaultMove`], [`TargetMove`], [`DirectionalMove`], [`MoveLock`], [`MoveTarget`], [`AxisOnlyMove`] |
//! | Easing | [`Easing`], [`MoveEasing`] |
//! | Pulse | [`PulseMode`], [`PulseTarget`], [`PulseChannel`], [`PulseGroup`], [`PulseHSV`] |
//! | Rotation and scale | [`RotationMode`], [`RotationNormal`], [`RotationAim`], [`RotationPlayerTarget`], [`ScaleConfig`] |
//! | Items and counters | [`Item`], [`ItemType`], [`CounterMode`], [`CompareOperand`] |
//! | Collision | [`ColliderConfig`] |
//! | Teleports | [`TeleportConfig`], [`TeleportForce`] |
//! | Events | [`Event`] |
//!
//! # Enums and unknown values
//!
//! Most enums here have an `Unrecognized` variant, which keeps values gdlib doesn't know about so they can be written back unchanged. Use `to_num` to get the raw value.
//!
//! Some enums don't have that variant. Converting an unknown value to one of those either returns an error or falls back to a default, so the original value can be lost.
```

#### `cclocallevels::gdobj::constructors`

**File:** `src/cclocallevels/gdobj/constructors/mod.rs`

```rust
//! Typed constructors for objects that have their own properties.
//!
//! Most constructors are structs that implement [`ObjectProperties`]. Fill in the struct, then pass it to [`GDObject::from_config`] along with a [`GDObjConfig`] for the shared properties.
//!
//! - [`triggers`]: triggers and the objects that work with them.
//! - [`gameplay`]: pads, orbs, portals and force blocks.
//! - [`text`]: text objects.
//! - [`SpinningObjectConfig`]: objects that spin by themselves.
//!
//! All of these are also re-exported from [`gdobj`](crate::cclocallevels::gdobj).
//!
//! # Example
//!
//! ```
//! use gdlib::cclocallevels::gdobj::{constructors::text, ids::objects, meta::GDObjConfig};
//!
//! let label = text(&GDObjConfig::new().with_pos(90.0, 90.0), "hello", 0);
//! assert_eq!(label.id, objects::TEXT);
//! ```
//!
//! # Adding a constructor
//!
//! Objects that map directly from fields to properties can use the internal `object_descriptor!` macro, which also generates `from_object`. Objects that need custom logic should implement [`ObjectProperties`] by hand.
```

#### `cclocallevels::gdobj::constructors::gameplay`

**File:** `src/cclocallevels/gdobj/constructors/gameplay.rs`

```rust
//! Constructors for parameterized gameplay objects: pads, orbs, portals and force blocks.
//!
//! | Object | Config struct | Selects the object with | Extra options |
//! |---|---|---|---|
//! | Pads | [`GDPad`] | [`GDPadType`] | None |
//! | Orbs | [`GDOrb`] | [`GDOrbType`] | [`OrbOptions`], including [`DashOrbSettings`] |
//! | Force blocks | [`ForceObjectConfig`] | [`Force`] | None |
//! | Portals | [`GameplayPortalConfig`] | [`GameplayPortal`] | [`PortalOptions`], including [`PortalCameraOptions`] |
//!
//! # Example
//!
//! ```
//! use gdlib::cclocallevels::gdobj::{GDObject, GDPad, GDPadType, ids::objects, meta::GDObjConfig};
//!
//! let config = GDObjConfig::new().with_pos(150.0, 105.0);
//! let pad = GDObject::from_config(&config, GDPad::new(GDPadType::Yellow));
//! assert_eq!(pad.id, objects::YELLOW_JUMP_PAD);
//! ```
//!
//! # Gotchas
//!
//! - **Converting back is limited.** Only [`GDPad`] currently implements [`ObjectProperties::from_object`]. The others return `None`.
```

#### `cclocallevels::gdobj::constructors::triggers`

**File:** `src/cclocallevels/gdobj/constructors/triggers.rs`

This keeps the existing approach. It fixes the "6" count, adds a way to find a trigger, and notes the `from_object` gaps.

```rust
//! Constructors for trigger objects, and for the objects that work directly with them.
//!
//! Since most triggers have parameters, this module defines a struct for each one that implements [`ObjectProperties`].
//! Fill in the struct, then create the object with [`GDObject::from_config`]:
//!
//! ```
//! use gdlib::cclocallevels::gdobj::{
//!     GDObject, MoveTrigger, ids::objects, meta::GDObjConfig,
//!     structs::{DefaultMove, Easing, MoveMode},
//! };
//!
//! let trigger = MoveTrigger {
//!     move_config: MoveMode::Default(DefaultMove { dx: 30.0, dy: 0.0, x_lock: None, y_lock: None }),
//!     time: 0.5,
//!     target_group: 2,
//!     silent: false,
//!     dynamic: false,
//!     easing: Easing::default(),
//! };
//!
//! let config = GDObjConfig::new().with_pos(150.0, 105.0);
//! let object = GDObject::from_config(&config, trigger);
//! assert_eq!(object.id, objects::MOVE_TRIGGER);
//! ```
//!
//! # Triggers without parameters
//!
//! These have no struct, because they have nothing to configure. Call the function to get a [`GDObject`] directly:
//!
//! - [`show_player`] and [`hide_player`]
//! - [`show_player_trail`] and [`hide_player_trail`]
//! - [`bg_effect_on`] and [`bg_effect_off`]
//! - [`reverse_gameplay`]
//!
//! # Finding a trigger
//!
//! | Area | Types |
//! |---|---|
//! | Movement and transforms | [`MoveTrigger`], [`RotateTrigger`], [`ScaleTrigger`], [`FollowTrigger`], [`FollowPlayerYTrigger`], [`TeleportTrigger`] |
//! | Visuals | [`ColourTrigger`], [`PulseTrigger`], [`AlphaTrigger`], [`TransitionTrigger`], [`ShakeTrigger`], [`ParticleSpawnTrigger`], [`AnimateTrigger`], [`LinkVisibleTrigger`] |
//! | Camera | [`CameraZoomTrigger`], [`CameraGuide`], [`RotateGameplayTrigger`] |
//! | Control flow | [`SpawnTrigger`], [`ToggleTrigger`], [`StopTrigger`], [`GroupResetTrigger`], [`RandomTrigger`], [`AdvancedRandomTrigger`], [`OnDeathTrigger`], [`TouchTrigger`], [`EventTrigger`], [`EndTrigger`] |
//! | Items and counters | [`PickupTrigger`], [`CountTrigger`], [`InstantCountTrigger`], [`ItemEditTrigger`], [`ItemCompareTrigger`], [`PersistentItemTrigger`], [`CounterLabel`] |
//! | Collision | [`CollisionTrigger`], [`InstantCollTrigger`], [`CollisionBlock`], [`StateBlock`], [`ToggleBlock`], [`AreaStopTrigger`] |
//! | Time | [`TimewarpTrigger`], [`TimeTrigger`], [`TimeControlTrigger`], [`TimeEventTrigger`] |
//! | Player and level | [`GravityTrigger`], [`PlayerControlTrigger`], [`BGSpeedTrigger`], [`MGSpeedTrigger`], [`MiddleGroundTrigger`], [`MiddleGroundConfigTrigger`], [`UIConfigTrigger`], [`StartposConfig`], [`BPMGuide`] |
//!
//! # Converting back to a struct
//!
//! Most structs support [`ObjectProperties::from_object`]. These don't yet, and return `None`: [`MoveTrigger`], [`ColourTrigger`], [`PulseTrigger`], [`ParticleSpawnTrigger`], [`RotateTrigger`] and [`TeleportTrigger`].
```

#### `core::io`

**File:** `src/core/io.rs`

```rust
//! Low-level encoding and decoding for GD savefiles and level strings.
//!
//! Most programs don't need this module, because [`GDLevel`](crate::cclocallevels::gdlevel::GDLevel) and [`CCLocalLevels`](crate::cclocallevels::gdlevel::CCLocalLevels) call it for you.
//!
//! # Encoding layers
//!
//! | Data | Writing | Reading |
//! |---|---|---|
//! | Level strings and `.gdlvl` files | gzip, then URL-safe base64: [`encrypt_level_str`] | [`decompress`] |
//! | Savefiles (`.dat`) | The above, then XOR with [`XOR_KEY_PLAYER_DATA`]: [`encrypt_savefile_str`] | [`decrypt`], or [`decrypt_file`] for a path |
//!
//! Plists are written in GD's shortened tag format by [`stringify_xml`]. To read them, convert the tags with [`proper_plist_tags`](crate::core::proper_plist_tags) first.
//!
//! # Gotchas
//!
//! - **[`decrypt_file`] removes all non-ASCII bytes.** It does this to cope with stray characters in CCGameManager.dat, but it also damages non-ASCII text such as level names.
//! - **[`decompress`] assumes a gzip header.** It skips a fixed 10 bytes rather than reading the header.
```

#### `cclocallevels::gdlevel::version`

**File:** `src/cclocallevels/gdlevel/version.rs`

```rust
//! GD release versions, and the IDs that the savefile format uses for them.
//!
//! A version has up to three numeric IDs, available through [`GDVersion`]'s methods:
//!
//! | ID | Method | Stored in |
//! |---|---|---|
//! | Game version | [`GDVersion::to_game_version_id`] | `k17` on levels |
//! | Binary version | [`GDVersion::to_binary_version_id`] | `k50` on levels, and `LLM_02` in CCLocalLevels.dat |
//! | Manifest version | [`GDVersion::to_manifest_version`] | Unknown |
//!
//! Several releases can share an ID, so converting from an ID picks the first match and may not give you the exact release. IDs that don't match any known release are kept as [`GDVersion::Unknown`].
//!
//! ```
//! use gdlib::cclocallevels::gdlevel::version::GDVersion;
//!
//! assert_eq!(GDVersion::from_binary_version(51), GDVersion::GD22091);
//! assert_eq!(GDVersion::from_binary_version(9999), GDVersion::Unknown(9999));
//! ```
//!
//! Use [`GDVersion::LATEST`] for new levels. [`GDVersion::default`] is the oldest version, not the newest.
```

#### `cclocallevels::gdlist`

**File:** `src/cclocallevels/gdlist/mod.rs`

```rust
//! Level lists: named, ordered collections of levels.
//!
//! [`GDList`] is one list. Lists in a savefile are in [`CCLocalLevels::lists`](crate::cclocallevels::gdlevel::CCLocalLevels::lists), and single lists are stored in `.gmdl` files, which you can read with [`GDList::from_gmdl`] and write with [`GDList::export_to_gmdl`].
//!
//! # Example
//!
//! ```no_run
//! use gdlib::cclocallevels::gdlist::GDList;
//!
//! # fn main() -> Result<(), gdlib::core::GDError> {
//! let list = GDList::from_gmdl("list.gmdl")?;
//! println!("{}: {} levels", list.name, list.level_ids.len());
//! # Ok(())
//! # }
//! ```
//!
//! # Gotchas
//!
//! - **[`GDList::levels`] holds summaries, not full levels.** They're missing many properties, so don't treat them as complete [`GDLevel`]s.
//! - **[`GDList::level_ids`] and [`GDList::levels`] are stored separately.** Keep them in sync if you change either one.
```

---

### Tier 3: small and lookup modules

#### `core`

**File:** `src/core/mod.rs`

```rust
//! Shared foundations: the error type, savefile lookup and encoding helpers.
//!
//! - [`GDError`]: the error type used across the crate.
//! - [`get_cclocallevels_path`] and [`get_ccgamemanager_path`]: find GD's savefiles on Windows and Linux.
//! - [`proper_plist_tags`]: converts GD's shortened plist tags into standard ones.
//! - [`b64_encode`] and [`b64_decode`]: URL-safe base64, as GD uses.
//!
//! # Modules
//!
//! - [`io`]: savefile and level string encoding.
//! - [`rand`]: GD's random number generator.
//! - [`crypto`]: password hashing.
//! - [`structs`]: types shared by more than one savefile.
```

#### `core::crypto`

**File:** `src/core/crypto.rs`

```rust
//! Password hashing, as used by GD's servers and savefiles.
//!
//! [`generate_gjp2_hexdigest`] produces a GJP2 hash: the SHA-1 hex digest of the password followed by a fixed salt. It's the format stored in [`GDAccount::hashed_password`](crate::ccgamemanager::structs::GDAccount::hashed_password).
//!
//! ```
//! use gdlib::core::crypto::generate_gjp2_hexdigest;
//!
//! let hash = generate_gjp2_hexdigest("password".to_string());
//! assert_eq!(hash.len(), 40);
//! ```
//!
//! GJP2 is a compatibility format, not secure password storage. Treat these hashes as sensitive anyway.
```

#### `core::rand`

**File:** `src/core/rand.rs`

```rust
//! GD's pseudo-random number generator, for predicting random triggers.
//!
//! GD uses a linear congruential generator: `seed = seed * 214013 + 2531011`, with wrapping arithmetic.
//!
//! # Advancing the seed
//!
//! Unlike the game, the functions here **don't advance the seed for you**. Call [`next_seed`] or [`next_seed_mut`] yourself between checks:
//!
//! ```
//! use gdlib::core::rand::{check_seed_random, next_seed};
//!
//! let seed = 1234;
//! let _first_goes_to_group_1 = check_seed_random(seed, 0.5);
//!
//! let seed = next_seed(seed);
//! assert_eq!(seed, 266_623_053);
//! let _second_goes_to_group_1 = check_seed_random(seed, 0.5);
//! ```
//!
//! # Which function to use
//!
//! - Random trigger: [`check_seed_random`].
//! - Advanced random trigger: [`check_seed_advanced_random`]. Pass the probabilities in their original order, because sorting or pruning them changes the result.
//! - Raw generator output: [`fast_rand_bits`] and [`fast_rand_bits_norm`].
//!
//! See `examples/seed_crack.rs` for a program that searches for a seed matching a set of expected results.
```

#### `core::structs`

**File:** `src/core/structs.rs`

```rust
//! Types found in more than one part of GD's savefiles.
//!
//! Currently this is just [`KCEKValue`], the value of the `kCEK` key, which appears to tell GD what kind of object a dictionary holds. Levels must keep [`KCEKValue::GJGameLevel`] to be recognised as levels.
//!
//! Types used in only one area belong in that area's module instead, such as [`gdobj::structs`](crate::cclocallevels::gdobj::structs).
```

#### `ccgamemanager::structs`

**File:** `src/ccgamemanager/structs.rs`

```rust
//! The structs that make up [`CCGameManager`](super::CCGameManager), grouped by area of the savefile.
//!
//! | Struct | Contains |
//! |---|---|
//! | [`GDPlayerInfo`] | Username, UDID, user ID, selected icons |
//! | [`GDStatistics`] | Statistics and achievement progress |
//! | [`GDConfig`] | Game settings, including [`Resolution`], [`TextureQuality`] and [`GDPlatformerUI`] |
//! | [`GDAccount`] | Account details and credentials |
//! | [`GDCurrentValues`] | Temporary values the game overwrites |
//! | [`GDSongConfig`] | Song settings |
//! | [`GDSearchFilters`] | Saved level search filters, with [`GDSearchFilterFlags`] |
//!
//! Most fields name the savefile key they're stored under, such as `GJA_001`.
//!
//! # Achievements
//!
//! [`GDStatistics`] has two ways to access achievement progress:
//!
//! - By variant, for vanilla achievements: [`GDStatistics::get_vanilla_achievement`] and [`GDStatistics::set_vanilla_achievement`].
//! - By identifier string, which also covers non-vanilla ones: [`GDStatistics::get_achievement_by_ident`] and [`GDStatistics::set_achievement_by_ident`].
```

#### `ccgamemanager::achievements`

**File:** `src/ccgamemanager/achievements.rs`

```rust
//! All vanilla GD achievements.
//!
//! [`GDAchievement`] has one variant per achievement. Convert to and from the game's string identifiers with [`GDAchievement::to_ident`] and [`GDAchievement::parse_str`]. Identifiers that aren't vanilla achievements return `None`.
//!
//! ```
//! use gdlib::ccgamemanager::achievements::GDAchievement;
//!
//! let ident = GDAchievement::Attempt01.to_ident();
//! assert_eq!(GDAchievement::parse_str(ident), Some(GDAchievement::Attempt01));
//! ```
//!
//! Progress values are read and set through [`GDStatistics`](super::structs::GDStatistics).
```

#### `cclocallevels::consts`

**Inline in:** `src/cclocallevels/mod.rs`, `pub mod consts`

```rust
//! Numeric constants from GD, either derived from its source or taken directly from it.
//!
//! - `SPEED_*`: the player's movement speed in units per second, for each speed setting. Useful for converting between time and position in a level.
//! - [`SPAWN_ORDER`]: how many units per second a spawn trigger moves.
//!
//! Reference: <https://github.com/UHDanke/gmdkit/blob/main/src/gmdkit/constants/game/speed/__init__.py>
```

#### `cclocallevels::properties`

**File:** `src/cclocallevels/properties.rs`

```rust
//! Lookup tables for object names and property types.
//!
//! - [`PROPERTY_TABLE`]: object property ID to name and type. Use [`get_obj_property_type`] to get just the type.
//! - [`LEVEL_HEADER_PROPERTIES`]: the same for level headers. Use [`get_level_header_property_type`] to get just the type.
//! - [`OBJECT_NAMES`]: object ID to name.
//!
//! The build script also generates the constants in [`ids`](crate::cclocallevels::gdobj::ids) from this module, so a new entry here becomes a new constant.
//!
//! ```
//! use gdlib::cclocallevels::properties::OBJECT_NAMES;
//!
//! assert!(OBJECT_NAMES.get(&1).is_some());
//! ```
```

#### `cclocallevels::gdlevel::enums`

**File:** `src/cclocallevels/gdlevel/enums.rs`

```rust
//! Enums for the fields of [`GDLevel`](super::GDLevel) and [`GDList`](crate::cclocallevels::gdlist::GDList).
//!
//! | Area | Enums |
//! |---|---|
//! | Level details | [`GDLevelType`], [`Length`], [`OfficialSong`] |
//! | Ratings | [`DifficultyRating`], [`DemonType`], [`EpicRating`], [`FeatureRating`] |
//! | Lists | [`GDListType`], [`GDListDifficulty`] |
//!
//! Enums convert from their raw value with `from` or `try_from`, and back with `to_num`. Most keep unknown values in an `Unrecognized` variant, so they can be written back unchanged:
//!
//! ```
//! use gdlib::cclocallevels::gdlevel::enums::Length;
//!
//! assert_eq!(Length::from(5), Length::Platformer);
//! assert_eq!(Length::from(99).to_num(), 99);
//! ```
//!
//! Many of these were sourced from <https://github.com/UHDanke/gmdkit/blob/main/src/gmdkit/utils/enums.py>. Thanks to HDanke for providing this resource.
```

#### `cclocallevels::gdobj::ids`

**Inline in:** `src/cclocallevels/gdobj/mod.rs`, `pub mod ids`

The current docs say "this file" about a generated module, and use `///` on the outside of it.

```rust
//! Generated constants for object IDs and property IDs.
//!
//! - [`objects`]: object IDs, for [`GDObject::id`](crate::cclocallevels::gdobj::GDObject::id).
//! - [`properties`]: object property IDs, for [`GDObject::get_property`](crate::cclocallevels::gdobj::GDObject::get_property) and [`GDObject::set_property`](crate::cclocallevels::gdobj::GDObject::set_property).
//! - [`level_header`]: level header property IDs.
//! - [`metadata`]: lists of properties that share a trait, such as holding groups.
//!
//! The build script generates this module from the tables in [`properties`](crate::cclocallevels::properties), so change those tables rather than this module.
```

#### `cclocallevels::gdobj::defaults`

**File:** `src/cclocallevels/gdobj/defaults.rs`

```rust
//! The in-game default state of each object.
//!
//! [`default_object`] returns an object as it would be when first placed in the editor. [`GDObject::default_from_id`](super::GDObject::default_from_id) calls the same function.
//!
//! A default is looked up in this order:
//!
//! 1. [`OBJECT_DEFAULTS_UNIQUE`], for objects with their own default string.
//! 2. [`check_common_suffix`], for the many objects that share one.
//! 3. An object with only the ID set, for IDs with no known default.
//!
//! [`default_object_string`] does the same lookup, but returns the raw string.
```

## 4. Item-level fixes to make at the same time

These all came up while reviewing the modules.

### Broken links

`cargo doc` currently reports these 7 warnings:

| Location | Problem |
|---|---|
| `README.md:69` | `[LICENSE]` link, which works on GitHub but not in rustdoc. An absolute URL fixes both |
| `src/cclocallevels/gdlevel/mod.rs:249` | Links to `LINUX_GD_FILES`, which you've replaced with `core::io::linux_gd_files_path` |
| `src/cclocallevels/gdobj/structs.rs:513, 520, 663` | Link to a `Group` type that doesn't exist |
| `src/cclocallevels/properties.rs:335, 336` | `GDLevelHeaderKey` isn't in scope at these links. Use the full path |

### Likely mistakes

| Location | Current text | Likely fix |
|---|---|---|
| `src/core/rand.rs` (`check_seed_advanced_random`) | "verified to work as of GD 2.2901" | Probably 2.2091 |
| `README.md` | "Latest supported version of GD: 2.2081" | `GDVersion::LATEST` is `GD22091`. Check which you mean to claim |
| `README.md` | "As of v0.4.3" and "As of v0.4.1" | Version-stamped claims go stale; consider removing the numbers |
| `src/cclocallevels/gdobj/constructors/gameplay.rs` (`GDOrb`) | Fields say "What type of pad" and "Allows the pad" | Should say "orb" |
| `src/cclocallevels/gdobj/defaults.rs` (`check_common_suffix`) | Says suffixes end with `;` | `default_object_string(1)` returned `1,1,2,0,3,0` with no `;`. Either the doc or the code needs changing. `default_object_strings_end_with_semicolon` is also among the failing tests, so this may already be on your list |
| `src/core/io.rs` | "CCGameManager.day" | "CCGameManager.dat" |
| `src/cclocallevels/gdobj/constructors/mod.rs` (`text`) | "Text in the objecty" | "object" |
| `src/cclocallevels/gdlevel/mod.rs` (`from_dict`) | "This function panics if any of the documented keys have a value which does not fit its expected type" | Most paths return `None` (also noted in `unresolved-issues.md`) |

### Stopping docs from breaking again

Add to `src/lib.rs`:

```rust
#![deny(rustdoc::broken_intra_doc_links)]
```

Fix the warnings above first, or `cargo doc` will fail. It only takes effect when rustdoc runs, so also add this to CI:

```text
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

Your workflows run `cargo test`, which already includes doctests, so the examples in these drafts would be checked automatically.

## 5. Validation

I applied all 26 drafts to a throwaway copy of the working tree and ran rustdoc and the doctests.

| Check | Result |
|---|---|
| Drafts applied | All 26 |
| Doctests | All 16 from the drafts pass: 13 run and 3 compile only (`no_run`). The existing README doctest also passes |
| `cargo doc` | No new warnings. Only the 7 existing warnings from section 4 remain |

- **Not checked by running:** the `no_run` examples only compile. That's deliberate, because they'd otherwise read or overwrite real savefiles.
- **Written from reading the code:** the gotchas and descriptions come from the source and your existing docs, not from testing each claim against the game.
- **Least certain wording:**
  - `version`: I couldn't determine where the manifest version is used, and your own comment says the same.
  - `core::structs`: the role of `kCEK` is hedged the same way your existing docs hedge it.
  - `gdlevel`: "close GD first, since it may overwrite your changes" is my understanding of how GD saves, not something I tested.

## 6. Suggested rollout

1. Fix the broken links in section 4.
2. Add tier 1, since it's the most visible and most of the examples are there.
3. Add tiers 2 and 3.
4. Add `broken_intra_doc_links` and the CI check.

Each step can be its own PR, and none of them changes any behaviour.
