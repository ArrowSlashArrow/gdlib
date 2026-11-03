# Missing `GDLevel` properties

The pasted property list checked against `GDLevel` in `src/cclocallevels/gdlevel/mod.rs`, using the working tree on `dev`.

**Result:** 45 keys have no typed field, 2 are stored only as raw strings, and 1 is mapped to a conflicting meaning. The rest of the list has a field, though section 2 notes a few differences.

These keys aren't lost today. `from_dict` puts them in `unknowns.other`, and `to_dict` writes them back unchanged. "Missing" here means there is no typed access to them.

## Evidence and confidence

| Label | Meaning |
|---|---|
| **Confirmed** | Documented on [boomlings.dev](https://boomlings.dev/resources/client/level), or seen in a real level in `test_gmds/` |
| **gmdkit only** | Comes only from the pasted list and the [gmdkit](https://github.com/UHDanke/gmdkit) source (`casting/level_loader.py`, `models/prop/*.py`). Plausible, but not checked against real data |
| **Unsure** | Inferred by me, or the sources disagree. The reason is given in each case |

**k126–k155 are all gmdkit only.** boomlings.dev doesn't document them, and no level in `test_gmds/` or `GMDS/` contains them. gmdkit also doesn't state the plist tag (`<i>`, `<s>`, `<t />`), so the tags for those keys are inferred from its Python type annotations.

## 1. Missing properties

### Conventions used below

- **Optional or not:** a gmdkit `DictField` with no `default` suggests that absent differs from zero, so I've used `Option<T>`. This follows maintainer note 3 in `mod.rs`. It's my inference (**Unsure**) for each key.
- **Plain ints:** parse with `v.as_signed_integer()? as i32`. Store with `serialise_fields` (omits default values) or `serialise_optional_fields`.
- **Bools:** stored as `<t />`, with absent meaning false. Parse with `v.as_boolean()?` and store with `serialise_bool_fields`.
- **Stripped on export:** `GDLVL_NOT_EXPORTED_KEYS` covers all of these keys except k102, k103, k125, k147–k150, k153, k155 and k156. That constant isn't used anywhere yet, and I haven't verified its contents.

### Upload dates

| Key | Pasted name | Suggested type | Confidence |
|---|---|---|---|
| k98 | `upload_date` | `Option<i32>` | **Unsure** |
| k99 | `update_date` | `Option<i32>` | **Unsure** |

- **Why unsure:** boomlings.dev doesn't document either key for levels, and no sample level has them. gmdkit types them as ints.
- **Supporting evidence:** `GDList` reads the same keys as ints, and `Summer Vacation.gmdl` holds Unix timestamps there (`1779094660`). That only shows how lists store them.
- **Possible conflict:** from memory, the Geode bindings declare the level fields as strings (`gd::string m_uploadDate`), and the server sends text like "3 years". I haven't verified this. Accept both forms, or leave non-integers in `other`, until a real level confirms the type.

### Preview lock

| Key | Pasted name | Suggested type | Confidence |
|---|---|---|---|
| k102 | `preview_lock_x` | `i32` | **Confirmed** by docs (integer); no sample |
| k103 | `preview_lock_y` | `i32` | **Confirmed** by docs (integer); no sample |

### Platformer bests and Click Between Steps

| Key | Pasted name | Suggested type | On disk | Confidence |
|---|---|---|---|---|
| k115 | `ticks_best_time` | `Option<i32>` | int, ticks at 240 TPS | **Confirmed** (docs and sample) |
| k116 | `ticks_best_points` | `Option<i32>` | int, ticks at 240 TPS | **Confirmed** (docs) |
| k117 | `clicks_best_time` | `Option<i32>` | int | **Confirmed** (docs and sample) |
| k118 | `clicks_best_points` | `Option<i32>` | int | **Confirmed** (docs) |
| k119 | `replay_best_time` | `Option<Vec<u8>>`, same as k34 | string, see [replay strings](#replay-strings) | **Confirmed** as a string; contents are gmdkit only |
| k120 | `replay_best_points` | `Option<Vec<u8>>`, same as k34 | string, see [replay strings](#replay-strings) | **Confirmed** as a string; docs say it "seems to be unused" |
| k121 | `coins_best_time` | `Option<i32>` | int | **Confirmed** (docs and sample) |
| k122 | `coins_best_points` | `Option<i32>` | int | **Confirmed** (docs) |
| k123 | `saved_best_time` | `bool` | bool | **Confirmed** (docs and sample) |
| k124 | `saved_best_points` | `bool` | bool | **Confirmed** (docs) |
| k125 | `toggle_cbs` | new `repr_t!` enum: `Default = 0, On = 1, Off = 2` | int | **Confirmed** (docs and gmdkit agree on the values) |

Use a non-panicking conversion for k125 so an unknown value can't fail the load. Omit it when it's `Default`.

### Autobackup and platformer save

| Key | Pasted name | Suggested type | On disk | Confidence |
|---|---|---|---|---|
| k126 | `autobackup_id` | `Option<String>` | string; gmdkit generates a 16-character ID | gmdkit only. Character set is unknown, and I don't know whether new levels must have one |
| k127 | `last_autobackup_time` | `i64` | int | gmdkit only. **Unsure** of the unit (probably a Unix timestamp) |
| k128 | `platformer_save` | `Option<String>`, kept raw | gzip + URL-safe base64, see below | gmdkit only |
| k129 | `enable_platformer_save` | `bool` | bool | gmdkit only |
| k130 | `platformer_save_level_version` | `i32` | int | gmdkit only |
| k131 | `platformer_save_replay` | `Option<Vec<u8>>`, same as k34 | string, see [replay strings](#replay-strings) | gmdkit only |
| k132 | `platformer_save_format_version` | `i32` | int | gmdkit only |

**k128 format:** after decompressing, it's a `~`-separated checkpoint of about 25 named fields (`player1`, `player2`, `game_state`, and so on), many of them nested `,`-separated structs. Trailing tokens are kept as `extra_tokens`. See `models/prop/platformer_save.py` in gmdkit. I'd keep it raw rather than write a parser: it's large, tied to the format version in k132, and rewriting it could corrupt a save.

### Death tracker and sessions

| Key | Pasted name | Suggested type | Confidence |
|---|---|---|---|
| k133 | `death_tracker_format_version` | `i32` | gmdkit only |
| k134 | `death_counts` | `Vec<(i32, i32)>` or `BTreeMap<i32, i32>` | gmdkit only |
| k135 | `startpos_death_counts` | `Vec<StartPosDeathCount>` | gmdkit only |
| k136 | `attempt_history` | `Vec<Attempt>` | gmdkit only |
| k137 | `startpos_attempt_history` | `Vec<StartPosAttempt>` | gmdkit only |
| k138 | `session_start` | `Option<i64>` | gmdkit only. **Unsure** of the unit (probably a Unix timestamp) |
| k139 | `session_last_played` | `Option<i64>` | gmdkit only. **Unsure** of the unit (probably a Unix timestamp) |
| k140 | `death_markers` | `Vec<DeathMarker>` | gmdkit only |
| k141 | `death_data_pending_upload` | `bool` | gmdkit only |
| k142 | `session_play_time` | `Option<i32>` | gmdkit only. **Unsure** of the unit |
| k143 | `total_play_time` | `Option<i32>` | gmdkit only. **Unsure** of the unit |
| k145 | `completions` | `Option<i32>` | gmdkit only. See the k12 note in section 2 |
| k146 | `archived_sessions` | `Option<String>`, kept raw at first | gmdkit only |

All of the list keys are plist strings, and an empty string should parse to an empty list:

| Key | Format | Notes |
|---|---|---|
| k134 | Flat `key,count,key,count,...` | gmdkit types both as int and doesn't name them. Percent → death count is my guess (**Unsure**) |
| k135 | Flat groups of 3: `start_percent,percent,count` | |
| k136 | `,`-separated items, each `percent` or `percentxcount` | A missing count means 1. Keep the same form when writing so the output stays identical |
| k137 | Flat groups of 2: `start_percent,percent` | |
| k140 | Flat groups of 4: `x,y,count,percent` | gmdkit multiplies x and y by 10 on read and writes `round(x / 10)`. Keep the stored values as they are, or apply the same scaling in both directions |
| k146 | gzip + URL-safe base64, then `\|`-separated sessions | Each session is `start_timestamp:history:startpos_history:playtime:best_percent`. `history` uses the k136 format and `startpos_history` uses the k137 format |

### Controls

| Key | Pasted name | Suggested type | On disk | Confidence |
|---|---|---|---|---|
| k147 | `creator_controls` | `Option<String>`, kept raw at first | base64 string, see below | gmdkit only |
| k148 | `creator_controls_version` | `Option<i32>` | int | gmdkit only |
| k149 | `custom_controls` | `Option<String>`, kept raw at first | base64 string, see below | gmdkit only |
| k150 | `custom_controls_source` | new enum: `Creator = 0, Custom = 1` | int | gmdkit only |
| k153 | `edited_custom_controls` | `bool` | bool | gmdkit only |
| k155 | `custom_controls_type` | `i32` or a bitflags type: `Keyboard = 1, Touch = 2, Controller = 4` | int | gmdkit only |

k155 holds combinations, so `repr_t!` doesn't fit. gmdkit computes the same flags from which control sections are populated, so k155 may be derived from k147 or k149 (**Unsure**).

**k147 and k149 format:** URL-safe base64 with no compression, decoding to:

```text
1:<version>|2:<keyboard>|3:<touch>|4:<flip_platformer_controls>|5:<controller>
```

| Part | Format |
|---|---|
| `keyboard`, `controller` | `;`-separated entries of `id,key,key,...`, sorted by `id`. gmdkit doesn't define what `id` means (**Unsure**) |
| Key values | Ints: the low 16 bits are the key (`0xFFFF` mask), with `0x10000` Ctrl, `0x20000` Alt and `0x40000` Shift. Controller IDs are 1001–1044 |
| `touch` | `<canvas>;<slot>;<slot>...` |
| `canvas` | `cw=<float>,ch=<float>` |
| `slot` | `,`-separated `key=value` pairs: `ax, ay, rx, ry, ox, oy, w, h, s, o, dz, rd, mb, sr, ns, ob`. gmdkit's defaults are `w=280, h=120, s=1.0, o=255, rd=10.0`, and 0 or false for the rest |

**Padding problem:** gmdkit adds missing `=` padding before decoding, which suggests the game may write these strings unpadded. `b64_decode` in `core/mod.rs` uses `URL_SAFE`, which requires canonical padding, so it would return an error on such a string. gmdkit writes the padding back on encode, and I don't know whether the game cares (**Unsure**).

### Global death stats and upload rate

| Key | Pasted name | Suggested type | Confidence |
|---|---|---|---|
| k151 | `death_upload_rate` | `i32` | gmdkit only. **Unsure** of the meaning |
| k152 | `global_death_stats` | `Option<String>`, kept raw at first | gmdkit only |
| k154 | `practice_attempts` | `i32` | gmdkit only |

**k152 format:** a plist string of `level_id,data_version,<death_string>`, split on the first two commas only.

`death_string` is gzip + URL-safe base64, and decompresses to `<markers>|<flag>`:

- `markers` is flat groups of 9, separated by `;`: `x,y,count,death_share,wall_share,unk_5,stars,death_glow,wall_glow`. gmdkit notes that the game also accepts `,` between values, and scales x and y as in k140.
- `flag` is an optional int. It's treated as enabled when absent or above 0.

`unk_5` is unidentified, and the field names are gmdkit's own.

### Editor stats

| Key | Pasted name | Suggested type | On disk | Confidence |
|---|---|---|---|---|
| k156 | `editor_stats` | `Option<EditorStats>` | string, 23 comma-separated values | **Confirmed**: documented, and in 2 real `.gdlvl` samples |

Real samples:

```text
1,0,0,0,0,0,0,0,0,216,0,0,0,0,0,0,0,0,2,1,1792372422,79918576,1
1,0,0,0,0,0,0,0,0,3920,0,0,0,0,0,0,0,0,37,2,1793411113,0,0
```

| Position | gmdkit name | Meaning | Status |
|---|---|---|---|
| 1 | `format_version` | Format version, `1` | Confirmed |
| 2 | `objects_created` | Objects created | Confirmed |
| 3 | `objects_pasted` | Objects pasted | Confirmed |
| 4 | `objects_deleted` | Objects deleted | Confirmed |
| 5 | `transform_actions` | Transform commands | Confirmed |
| 6 | `edit_popups_opened` | Edit Special, Edit Group, Edit Object and Object Options actions | Confirmed |
| 7 | `undo_redo` | Undo and redo calls | Confirmed |
| 8 | `playtest_count` | Playtests started | Confirmed |
| 9 | `playtest_time` | Total playtest time, whole seconds | Confirmed |
| 10 | `editor_frames` | Activity clock sample | **Unsure**: docs say source and unit are unknown, so gmdkit's name is unconfirmed |
| 11–18 | `action_gap_*` | Histogram of gaps between actions: `0_1, 2_3, 4_7, 8_15, 16_31, 32_63, 64_127, 128_plus` | Confirmed |
| 19 | `editor_inputs` | Probably editor inputs or actions | **Unsure** |
| 20 | `editor_sessions` | Possibly saves, opens or exits | **Unsure** |
| 21 | `first_open` | First-open Unix timestamp in seconds. Written as `u64` | Confirmed |
| 22 | `original_level_id` | Level ID this was copied from | Confirmed |
| 23 | `original_level_version` | Level version this was copied from | Confirmed |

Parse by splitting on commas into ints, with `first_open` as `u64`. Write the values back in the same order. The docs page is marked WIP and describes the 2.2091 Android client, so check `format_version` and keep a raw fallback for any other value or field count.

### Replay strings

k34, k119, k120 and k131 all use the same type in gmdkit. `GDLevel` already stores k34 as raw bytes, so the simplest approach is to store the others the same way. Parsing can be added later as a separate step.

The value is gzip + URL-safe base64, with no XOR. It decompresses to:

```text
<events>#<metadata>
```

| Part | Format |
|---|---|
| `events` | `;`-separated inputs, each `delta_step,event_id:event_data`. The event part is optional. Event IDs `98` and `99` are checkpoint events with their own data format |
| `metadata` | `seed,attempt,game_version,game_binary,level_version,persistent_data` |
| `persistent_data` | Item and timer data, in two parts separated by `@` |

The container format is gmdkit only, and I haven't decoded a real replay to check it.

## 2. Present, but different from the list

### Needs a decision

| Key | In `GDLevel` | In the list | Finding |
|---|---|---|---|
| **k94** | `content.is_platformer` (`mod.rs:1038`) | `friends_only` | **Probably a bug in `GDLevel`.** boomlings.dev also says "Unlisted (Friends Only)". Platformer levels are already marked by `k23 = 5` (`Length::Platformer`). Suggested fix: rename to `flags.friends_only`, and derive `is_platformer` from `content.length`. `src/tests/mod.rs:179` sets this field and would need updating |
| **k9 / k10** | k9 is `DifficultyRating`, k10 is `i32` (`mod.rs:934-937`) | k9 is `int`, k10 is the `LevelRating` enum | **Probably a bug in `GDLevel`.** The one real sample with these keys has `k9 = 10, k10 = 50`, so it reads as Easy with a sum of 50. If k9 is the denominator and k10 the numerator, as the list suggests, that level is Insane. Based on a single sample, so check another rated level first |
| **k91** | `unknowns.k91: Option<String>` | `locked_layers: IntList` | Stored raw. Docs: comma-separated list of locked layers. Suggested fix: `editor_state.locked_layers: Vec<i32>`, with an empty string giving an empty list |
| **k101** | `unknowns.k101: Option<String>` | `saved_camera_positions: PositionList` | Stored raw. Docs: 10 saved points as `x,y,x,y,...`, which matches the 20 zeros in every sample. So the comment at `mod.rs:697` linking it to k47 looks wrong. Suggested fix: `Vec<(f32, f32)>`, with float parsing from gmdkit. Docs give only the format, and every sample is zero, so float output formatting is **Unsure** |

### Naming or type differences only

| Key | `GDLevel` | List | Note |
|---|---|---|---|
| k35 | `is_playable` | `not_downloaded` | Docs call it `isHidden` ("deleted (hidden) from Saved"). These three names don't agree, and `is_playable` reads as the opposite of the other two. No sample contains it. **Unsure** which is right |
| k41 | `Option<i32>` | `str` | Docs say integer, and the real sample is `<i>1</i>`. `GDLevel` looks correct |
| k12 | `player_stats.completions` | Not listed; k145 is `completions` | Docs describe k12 as level completions. Both may exist, so k145 needs a distinct field name |
| k7 | `i32` | `LevelDifficulty` enum | Works as is, but has no typed values |
| k61–k63 | `coinN_acquired` | `coin_N_verified` | Docs: "acquired during verification". Both names fit |
| k69, k93 | `high_object_count`, `unlimited_objects` | `bypass_high_objs`, `bypass_unlimited_objs` | Same keys, different names. Docs match `GDLevel` |
| kI7 | `f32` | `int` | Docs say float, but the real sample is `<i>-1</i>`. `to_float` reads both. Note that `GDLevel` defaults to 0 and always writes a real number if it's non-zero |

`start` and `objects` in the list are aliases into the level string, and are covered by `GDLevelData.headers` and `.objects`.

`GDLevel` also has keys the list doesn't: k12, k39, k40, k51–k54, k92, k106 and kCEK. k144 is in neither.

## 3. Things to watch when adding these

- **A strict parse can lose the whole savefile.** Each `?` in `from_dict` returns `None` on a wrong type, and `CCLocalLevels::from_decrypted` then fails entirely (`mod.rs:131`). For any key that isn't **Confirmed**, leave the raw value in `unknowns.other` on a mismatch.
- **`parse_csv` needs fixing first.** It panics on non-strings, and `""` returns `None`, which fails the level (`mod.rs:1540`). Most of the new list keys can reasonably be empty.
- **Keep raw strings where you can.** k128, k146, k147, k149 and k152 are the riskiest to re-encode. `GDLevelState` already has this split with `Encrypted` and `Decrypted`, and the same pattern would round-trip them exactly unless they're edited.
- **`as i32` truncates silently.** That's why I suggest `i64` for the timestamps with an unknown unit. A millisecond timestamp wouldn't fit in `i32`.
- **Stale comments to update:**
  - `mod.rs:1042`: `96 - 100 are unused`, but k98 and k99 are in the list
  - `mod.rs:1044`: the 102–103 todo
  - `mod.rs:1068`: the 115–125 todo
  - `mod.rs:1080`: the field type table in `to_dict`
- **Tests:** the field-by-field comparison in `src/tests/mod.rs` will need the new fields added.
