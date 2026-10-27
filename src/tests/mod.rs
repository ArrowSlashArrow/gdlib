//! Unit tests for the crate.
//!
//! Tests are split up by what they primarily exercise:
//! * [`serialisers`]: structs -> raw strings / plist dictionaries
//! * [`deserialisers`]: raw strings / plist dictionaries -> structs, including malformed input
//! * [`roundtrips`]: serialise -> deserialise (and vice versa) must be lossless
//! * [`object_properties`]: every function of [`ObjectProperties`] for every struct that implements it
//! * [`enum_conversions`]: numeric <-> enum conversions of all `repr_t!` and similar enums
//! * [`accessors`]: getters, setters and builder methods on objects, configs and levels
//! * [`game_logic`]: evaluation of in-game behaviour (items, RNG, seeds, ...)
//! * [`files`]: reading and writing the level/list files in `test_gmds/`
//! * [`local_savefiles`]: tests that need a local GD installation
//! * [`benchmarks`]: timing tests
//!
//! [`ObjectProperties`]: crate::cclocallevels::gdobj::ObjectProperties
use std::{collections::HashMap, path::PathBuf, time::Instant};

use plist::Value;
use smallvec::smallvec;

use crate::cclocallevels::{
    gdlevel::{
        GDLevel, default_level_data,
        enums::{DemonType, DifficultyRating, EpicRating, GDLevelType, Length, OfficialSong},
        version::GDVersion,
    },
    gdlist::GDList,
    gdobj::{GDObject, ids::properties as p, meta::GDObjConfig},
};

mod accessors;
mod benchmarks;
mod deserialisers;
mod enum_conversions;
mod files;
mod game_logic;
mod local_savefiles;
mod object_properties;
mod roundtrips;
mod serialisers;

/// Property IDs that are owned by [`GDObjConfig`] (or are the object ID itself).
/// These must never show up in `GDObject.properties`, otherwise they get serialised twice.
const CONFIG_PROPERTY_IDS: &[u16] = &[
    p::OBJECT_ID,
    p::X_POS,
    p::Y_POS,
    p::ROTATION,
    p::TOUCH_TRIGGERABLE,
    p::SPAWN_TRIGGERABLE,
    p::MULTITRIGGERABLE,
    p::GROUPS,
    p::PARENT_GROUPS,
    p::X_SCALE,
    p::Y_SCALE,
    p::EDITOR_LAYER_1,
    p::EDITOR_LAYER_2,
    p::OBJECT_COLOUR,
    p::SECONDARY_COLOUR,
    p::Z_LAYER,
    p::Z_ORDER,
    p::ENTER_EFFECT_CHANNEL,
    p::OBJECT_MATERIAL,
    p::MATERIAL_CONTROL_ID,
    p::DONT_FADE,
    p::DONT_ENTER,
    p::NO_OBJECT_EFFECTS,
    p::IS_GROUP_PARENT,
    p::IS_AREA_PARENT,
    p::DONT_BOOST_X,
    p::DONT_BOOST_Y,
    p::IS_HIGH_DETAIL,
    p::NO_TOUCH,
    p::PASSABLE,
    p::HIDDEN,
    p::NONSTICK_X,
    p::NONSTICK_Y,
    p::EXTRA_STICKY,
    p::HAS_EXTENDED_COLLISION,
    p::IS_ICE_BLOCK,
    p::GRIP_SLOPE,
    p::NO_GLOW,
    p::NO_PARTICLES,
    p::SCALE_STICK,
    p::NO_AUDIO_SCALE,
    p::SINGLE_PLAYER_TOUCH,
    p::CENTER_EFFECT,
    p::REVERSES_GAMEPLAY,
];

/// Runs `f` and prints how long it took.
fn benchmark<F: Fn() -> R, R>(name: &str, f: F) -> R {
    let start = Instant::now();
    let result = f();
    println!(
        "{name}: {:.3}ms",
        start.elapsed().as_micros() as f64 / 1000.0
    );
    result
}

/// Splits a raw object string into its `(key, value)` pairs, in order.
fn object_kv_pairs(s: &str) -> Vec<(String, String)> {
    let tokens = s
        .trim_end_matches(';')
        .trim_start_matches(',')
        .split(',')
        .collect::<Vec<_>>();
    tokens
        .chunks(2)
        .filter(|c| c.len() == 2)
        .map(|c| (c[0].to_string(), c[1].to_string()))
        .collect()
}

/// Returns every value stored under `key` in a raw object string.
fn values_of(s: &str, key: &str) -> Vec<String> {
    object_kv_pairs(s)
        .into_iter()
        .filter(|(k, _)| k == key)
        .map(|(_, v)| v)
        .collect()
}

/// Path to a generated (gitignored) file in `test_gmds/`.
fn generated_path(name: &str) -> PathBuf {
    PathBuf::from(format!("test_gmds/generated_{name}"))
}

/// A non-default object config that is used when building objects in tests.
fn test_config() -> GDObjConfig {
    GDObjConfig::new()
        .with_pos(45.0, 75.0)
        .with_groups([3i16, 7])
}

/// A block (object ID 1) with the default config.
fn default_block() -> GDObject {
    GDObject::new(1, &GDObjConfig::default(), vec![])
}

/// A level with every field set to a non-default value.
fn sample_level() -> GDLevel {
    let mut level = GDLevel::new(GDVersion::GD22082);

    level.identity.id = 128;
    level.identity.name = "gdlib test level".into();
    level.identity.description = Some("A level used in gdlib's unit tests.".into());
    level.identity.creator = "gdlib".into();
    level.identity.user_id = 1234;
    level.identity.account_id = 5678;
    level.identity.level_version = 3;
    level.identity.level_revision = Some(2);
    level.identity.game_version = 22;
    level.identity.build_version = 7;
    level.identity.level_type = GDLevelType::Saved;
    level.identity.timely_id = 42;
    level.identity.original_id = Some(127);
    level.identity.password = Some(1_000_000);

    level.content.data = default_level_data();
    level.add_objects([
        GDObject::new(1, &test_config(), vec![]),
        GDObject::new(
            901,
            &GDObjConfig::new().with_pos(-15.0, 1005.5).with_angle(90.0),
            vec![],
        ),
    ]);
    level.content.object_count = 2;
    level.content.high_object_count = true;
    level.content.song_list = vec![10, 20, 30];
    level.content.sfx_list = Some(vec![1, 2]);
    level.content.official_song_id = Some(OfficialSong::Jumper);
    level.content.custom_song_id = Some(467339);
    level.content.length = Length::Long;
    level.content.two_player_mode = true;
    level.content.is_platformer = true;

    level.ratings.rating = DifficultyRating::Harder;
    level.ratings.rating_sum = 40;
    level.ratings.downloads = 1000;
    level.ratings.like_rating = -5;
    level.ratings.dislikes = 6;
    level.ratings.stars = 8;
    level.ratings.requested_stars = Some(9);
    level.ratings.feature_score = Some(11);
    level.ratings.epic_rating = EpicRating::Legendary;
    level.ratings.difficulty = 4;
    level.ratings.is_demon = true;
    level.ratings.demon_type = Some(DemonType::InsaneDemon);
    level.ratings.is_auto = true;

    level.coins.total_coins = 3;
    level.coins.required_coins = Some(2);
    level.coins.coins_verified = true;
    level.coins.coin1_acquired = true;
    level.coins.coin2_acquired = true;
    level.coins.coin3_acquired = true;

    level.player_stats.completions = 12;
    level.player_stats.attempts = 345;
    level.player_stats.normal_percentage = 67;
    level.player_stats.practice_percentage = 100;
    level.player_stats.jumps = 890;
    level.player_stats.clicks = 1011;
    level.player_stats.best_attempt_time = 1213;
    level.player_stats.mana_orb_percentage = 14;
    level.player_stats.leaderboard_percentage = 15;
    level.player_stats.verification_time = Some(2400);
    level.player_stats.best_time_ms = Some(16_000);
    level.player_stats.best_points = Some(17);
    level.player_stats.local_best_times = vec![16_000, 17_500];
    level.player_stats.local_best_points = vec![17, 3];
    level.player_stats.progress_diffs = vec![12, 40, 15];

    level.flags.is_editable = true;
    level.flags.verified = true;
    level.flags.uploaded = true;
    level.flags.is_playable = true;
    level.flags.is_unlocked = true;
    level.flags.unlisted = true;
    level.flags.has_been_modified = true;
    level.flags.has_ldm = true;
    level.flags.ldm_enabled = true;
    level.flags.is_gauntlet = true;
    level.flags.is_alt_game = true;
    level.flags.is_favourited = true;
    level.flags.unlimited_objects = true;
    level.flags.no_shake = true;

    level.editor_state.camera_x = 123.5;
    level.editor_state.camera_y = -45.25;
    level.editor_state.camera_zoom = 0.75;
    level.editor_state.build_tab_page = 2;
    level.editor_state.build_tab = 1;
    level.editor_state.build_tab_pages = smallvec![0, 1, 2, 3, 0, 0, 4, 0, 0, 0, 0, 5, 0, 6];
    level.editor_state.editor_layer = 4.0;

    level.meta.level_order = 1;
    level.meta.level_folder = 2;
    level.meta.seconds_editing = 3600;
    level.meta.seconds_editing_copies = 60;
    level.meta.level_size = 512;
    level.meta.capacity_001 = 1;
    level.meta.capacity_002 = 2;
    level.meta.capacity_003 = 3;
    level.meta.capacity_004 = 4;
    level.meta.capacity_string = Some("1_2_3_4".into());

    level.integrity.level_seed = Some(99);
    level.integrity.triggered_anti_cheat = true;
    level.integrity.vfd_chk = true;
    level.integrity.platformer_seed = Some(77);
    level.integrity.replay_data = Some(b"replaydata".to_vec());

    level.unknowns.k91 = Some("k91 value".into());
    level.unknowns.k92 = Some(92);
    level.unknowns.k101 = Some("0,0,0,0".into());
    level.unknowns.k106 = Some(106);
    level.unknowns.other = HashMap::from([
        ("k200".to_string(), Value::Integer(200.into())),
        ("kQ".to_string(), Value::String("unknown string".into())),
        ("someOtherKey".to_string(), Value::Real(1.5)),
    ]);

    level
}

/// A list with every field set to a non-default value.
fn sample_list() -> GDList {
    let mut level_a = GDLevel::default();
    level_a.identity.id = 1001;
    level_a.identity.name = "first".into();
    let mut level_b = GDLevel::default();
    level_b.identity.id = 1002;
    level_b.identity.name = "second".into();

    GDList {
        id: 55,
        name: "gdlib test list".into(),
        description: "List description".into(),
        creator: "gdlib".into(),
        difficulty: crate::cclocallevels::gdlevel::enums::GDListDifficulty::Insane,
        downloads: 300,
        is_uploaded: true,
        version: 4,
        list_type: crate::cclocallevels::gdlevel::enums::GDListType::Local,
        likes: 25,
        is_demon: true,
        stars: 10,
        featured: true,
        original: Some(54),
        list_revision: 2,
        account_id: 71,
        unlisted: true,
        favourited: true,
        order: 3,
        level_ids: vec![1001, 1002],
        levels: vec![level_a, level_b],
        upload_time: 1_700_000_000,
        update_time: Some(1_700_000_500),
        diamond_reward: 50,
        required_levels: 2,
        other_properties: HashMap::from([("k500".to_string(), Value::Integer(5.into()))]),
        ..Default::default()
    }
}

/// Asserts that two levels are equal. Level data is compared in its decrypted form,
/// since the same level data can be stored as either [`GDLevelState`](crate::cclocallevels::gdlevel::leveldata::GDLevelState) variant.
#[track_caller]
fn assert_levels_eq(actual: &GDLevel, expected: &GDLevel) {
    macro_rules! cmp_debug {
        ($($field:ident),*) => {$(
            assert_eq!(
                format!("{:?}", actual.$field),
                format!("{:?}", expected.$field),
                concat!("GDLevel.", stringify!($field), " differs")
            );
        )*};
    }
    cmp_debug!(
        identity,
        ratings,
        coins,
        player_stats,
        flags,
        editor_state,
        meta,
        integrity
    );

    let mut actual_content = actual.content.clone();
    let mut expected_content = expected.content.clone();
    actual_content.data = None;
    expected_content.data = None;
    assert_eq!(
        format!("{actual_content:?}"),
        format!("{expected_content:?}"),
        "GDLevel.content differs"
    );
    assert_eq!(
        actual.get_decrypted_data(),
        expected.get_decrypted_data(),
        "GDLevel.content.data differs"
    );

    assert_eq!(
        actual.unknowns.k91, expected.unknowns.k91,
        "GDLevel.unknowns.k91 differs"
    );
    assert_eq!(
        actual.unknowns.k92, expected.unknowns.k92,
        "GDLevel.unknowns.k92 differs"
    );
    assert_eq!(
        actual.unknowns.k101, expected.unknowns.k101,
        "GDLevel.unknowns.k101 differs"
    );
    assert_eq!(
        actual.unknowns.k106, expected.unknowns.k106,
        "GDLevel.unknowns.k106 differs"
    );
    assert_eq!(
        actual.unknowns.other, expected.unknowns.other,
        "GDLevel.unknowns.other differs"
    );
}

/// Asserts that two lists are equal.
#[track_caller]
fn assert_lists_eq(actual: &GDList, expected: &GDList) {
    macro_rules! cmp {
        ($($field:ident),*) => {$(
            assert_eq!(actual.$field, expected.$field, concat!("GDList.", stringify!($field), " differs"));
        )*};
    }
    cmp!(
        id,
        name,
        description,
        creator,
        difficulty,
        downloads,
        is_uploaded,
        version,
        list_type,
        likes,
        is_demon,
        stars,
        featured,
        original,
        list_revision,
        account_id,
        unlisted,
        favourited,
        order,
        level_ids,
        upload_time,
        update_time,
        diamond_reward,
        required_levels,
        kcek,
        other_properties
    );
    assert_eq!(
        actual.levels.len(),
        expected.levels.len(),
        "GDList.levels has a different length"
    );
    for (a, e) in actual.levels.iter().zip(&expected.levels) {
        assert_levels_eq(a, e);
    }
}
