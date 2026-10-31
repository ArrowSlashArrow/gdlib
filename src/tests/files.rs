//! Tests that read and write the level and list files in `test_gmds/`.
//!
//! Files written by these tests are named `test_gmds/generated_*` and are gitignored.
use std::fs;

use super::{
    assert_levels_eq, assert_lists_eq, benchmark, generated_path, sample_level, sample_list,
};
use crate::{
    cclocallevels::{
        gdlevel::{
            CCLocalLevels, GDLevel,
            leveldata::{GDLevelData, GDLevelHeaderKey, GDLevelHeaderValue},
            version::GDVersion,
        },
        gdlist::GDList,
        gdobj::{
            self, GDObject,
            constructors::triggers::{
                AdvancedRandomTrigger, EventTrigger, ItemCompareTrigger, MoveTrigger,
            },
            meta::{GDObjAttributes, GDObjConfig},
            structs::{
                ColourChannel, CompareOp, CompareOperand, DefaultMove, Easing, Event, ExtraID2,
                Item, MoveEasing, MoveMode, Op, RoundMode, SignMode, ZLayer,
            },
        },
    },
    core::io::{decompress, decrypt_file},
};

const TEST_LEVELS: &[&str] = &[
    "test_gmds/level.gmd",
    "test_gmds/empty test level.gmd",
    "test_gmds/advrand test.gmd",
    "test_gmds/Chompstep.gmd",
    "test_gmds/All Object IDs.gmd",
    "test_gmds/big.gmd",
];

// ---------------- reading ----------------

#[test]
fn read_objs() {
    let level = GDLevel::from_gmd("test_gmds/All Object IDs.gmd").unwrap();
    let data = level.get_decrypted_data().unwrap();

    for (idx, obj) in data.objects.iter().enumerate() {
        println!("{idx}: {obj:?}");
    }
}

#[test]
fn level_display_test() {
    let level = GDLevel::from_gmd("test_gmds/big.gmd").unwrap();
    println!("GDLevel info: {level}");
    println!(
        "Unused groups: {:?}",
        level.get_decrypted_data().unwrap().get_unused_groups()
    );
    println!(
        "Used groups: {:?}",
        level.get_decrypted_data().unwrap().get_used_groups()
    );
}

#[test]
fn every_test_level_parses() {
    for path in TEST_LEVELS {
        let level = GDLevel::from_gmd(path).unwrap_or_else(|e| panic!("{path}: {e}"));
        assert!(
            level.get_decrypted_data().is_some(),
            "{path}: level data could not be decrypted"
        );
    }
}

#[test]
fn every_test_level_has_level_headers() {
    for path in TEST_LEVELS {
        let data = GDLevel::from_gmd(path)
            .unwrap()
            .get_decrypted_data()
            .unwrap();
        assert!(
            matches!(
                data.headers.get(&GDLevelHeaderKey::S(38)),
                Some(GDLevelHeaderValue::ColourString(_))
            ),
            "{path}: missing kS38"
        );
    }
}

#[test]
fn every_object_in_test_levels_is_reserialised_identically() {
    // parsing the serialised version of a parsed object must give back the same object
    for path in TEST_LEVELS {
        let data = GDLevel::from_gmd(path)
            .unwrap()
            .get_decrypted_data()
            .unwrap();
        for obj in &data.objects {
            let s = obj.serialise_to_string();
            assert_eq!(&GDObject::parse_str(&s), obj, "{path}: {s}");
        }
    }
}

#[test]
fn every_raw_object_in_test_levels_keeps_its_properties() {
    // every key in the raw object string must still be there after reserialising
    for path in TEST_LEVELS {
        let level = GDLevel::from_gmd(path).unwrap();
        let raw = level
            .get_raw_decrypted_data()
            .unwrap_or_else(|| panic!("{path}: no raw data"));
        for raw_obj in raw.split(';').skip(1).filter(|o| o.len() > 1) {
            let reserialised = GDObject::parse_str(raw_obj).serialise_to_string();
            let reserialised_keys = super::object_kv_pairs(&reserialised)
                .into_iter()
                .map(|(k, _)| k)
                .collect::<Vec<_>>();
            for (k, v) in super::object_kv_pairs(raw_obj) {
                if v == "0" || reserialised_keys.contains(&k) {
                    continue;
                }
                // config properties that are set to their default value are not written
                if matches!(
                    (k.as_str(), v.as_str()),
                    ("128" | "129", "1") | ("21", "1004") | ("22", "1") | ("24", "5")
                ) {
                    continue;
                }
                panic!(
                    "{path}: key {k} ({v}) lost\noriginal:     {raw_obj}\nreserialised: {reserialised}"
                );
            }
        }
    }
}

#[test]
fn test_list_parses() {
    let list = GDList::from_gmdl("test_gmds/Summer Vacation.gmdl").unwrap();
    assert!(!list.name.is_empty());
    assert_eq!(list.levels.len(), list.level_ids.len());
}

#[test]
fn parse_gdlvl() {
    let mut gdl = GDLevel::from_gdlvl("test_gmds/Jesse Pinkman 2.gdlvl").unwrap();
    gdl.identity.name = String::from("Jesse Pinkman 3");
    gdl.identity.level_version = 123;
    println!("{gdl:#?}");
    gdl.export_to_gdlvl(generated_path("Jesse Pinkman 3.gdlvl"))
        .unwrap();
}

#[test]
fn missing_files_return_err() {
    assert!(GDLevel::from_gmd("test_gmds/does not exist.gmd").is_err());
    assert!(GDLevel::from_gdlvl("test_gmds/does not exist.gdlvl").is_err());
    assert!(GDList::from_gmdl("test_gmds/does not exist.gmdl").is_err());
}

#[test]
fn gmd_as_gdlvl_returns_err() {
    assert!(GDLevel::from_gdlvl("test_gmds/level.gmd").is_err());
}

#[test]
fn gdlvl_as_gmd_returns_err() {
    assert!(GDLevel::from_gmd("test_gmds/Jesse Pinkman 2.gdlvl").is_err());
}

#[test]
fn non_utf8_gmd_returns_err() {
    let path = generated_path("non_utf8.gmd");
    fs::write(&path, [0xff, 0xfe, 0x00, 0x80]).unwrap();
    assert!(GDLevel::from_gmd(&path).is_err());
}

#[test]
fn non_dict_gmd_returns_err() {
    let path = generated_path("non_dict.gmd");
    fs::write(
        &path,
        "<?xml version=\"1.0\"?><plist version=\"1.0\" gjver=\"2.0\"><s>not a level</s></plist>",
    )
    .unwrap();
    assert!(GDLevel::from_gmd(&path).is_err());
}

#[test]
fn non_dict_gmdl_returns_err() {
    let path = generated_path("non_dict.gmdl");
    fs::write(
        &path,
        "<?xml version=\"1.0\"?><plist version=\"1.0\" gjver=\"2.0\"><s>not a list</s></plist>",
    )
    .unwrap();
    assert!(GDList::from_gmdl(&path).is_err());
}

// ---------------- round trips ----------------

#[test]
fn every_test_level_gmd_roundtrip() {
    for (idx, path) in TEST_LEVELS.iter().enumerate() {
        let level = GDLevel::from_gmd(path).unwrap();
        let out = generated_path(&format!("roundtrip_{idx}.gmd"));
        level.export_to_gmd(&out).unwrap();
        let reparsed = GDLevel::from_gmd(&out).unwrap_or_else(|e| panic!("{path}: {e}"));
        assert_levels_eq(&reparsed, &level);
    }
}

#[test]
fn every_test_level_data_roundtrip() {
    for path in TEST_LEVELS {
        let data = GDLevel::from_gmd(path)
            .unwrap()
            .get_decrypted_data()
            .unwrap();
        assert_eq!(
            GDLevelData::parse(data.serialise_to_string()).as_ref(),
            Some(&data),
            "{path}"
        );
    }
}

#[test]
fn gdlvl_roundtrip() {
    let level = GDLevel::from_gdlvl("test_gmds/Jesse Pinkman 2.gdlvl").unwrap();
    let out = generated_path("roundtrip.gdlvl");
    level.export_to_gdlvl(&out).unwrap();
    assert_levels_eq(&GDLevel::from_gdlvl(&out).unwrap(), &level);
}

#[test]
fn gmd_to_gdlvl_roundtrip() {
    let level = GDLevel::from_gmd("test_gmds/level.gmd").unwrap();
    let out = generated_path("level_as.gdlvl");
    level.export_to_gdlvl(&out).unwrap();
    assert_levels_eq(&GDLevel::from_gdlvl(&out).unwrap(), &level);
}

#[test]
fn gdlvl_file_is_gzip() {
    use std::io::Read;
    let out = generated_path("gzip_check.gdlvl");
    GDLevel::default().export_to_gdlvl(&out).unwrap();
    let raw = decompress(fs::read(&out).unwrap()).unwrap();
    assert!(String::from_utf8(raw).unwrap().starts_with("<?xml"));

    let gzip = crate::core::b64_decode(fs::read(&out).unwrap()).unwrap();
    let mut s = String::new();
    flate2::read::GzDecoder::new(&gzip[..])
        .read_to_string(&mut s)
        .unwrap();
    assert!(s.starts_with("<?xml"));
}

#[test]
fn format_to_gdlvl_matches_export() {
    let level = sample_level();
    let out = generated_path("format_check.gdlvl");
    level.export_to_gdlvl(&out).unwrap();
    assert_eq!(
        GDLevel::format_to_gdlvl(&level.stringified()),
        fs::read_to_string(&out).unwrap()
    );
}

#[test]
fn sample_level_gmd_roundtrip() {
    let level = sample_level();
    let out = generated_path("sample_level.gmd");
    level.export_to_gmd(&out).unwrap();
    assert_levels_eq(&GDLevel::from_gmd(&out).unwrap(), &level);
}

#[test]
fn sample_level_gdlvl_roundtrip() {
    let level = sample_level();
    let out = generated_path("sample_level.gdlvl");
    level.export_to_gdlvl(&out).unwrap();
    assert_levels_eq(&GDLevel::from_gdlvl(&out).unwrap(), &level);
}

#[test]
fn unicode_level_gmd_roundtrip() {
    let mut level = GDLevel::default();
    level.identity.name = "Niveau élevé 日本".into();
    level.identity.creator = "Ünïcödé".into();
    let out = generated_path("unicode.gmd");
    level.export_to_gmd(&out).unwrap();
    assert_levels_eq(&GDLevel::from_gmd(&out).unwrap(), &level);
}

#[test]
fn test_list_gmdl_roundtrip() {
    let list = GDList::from_gmdl("test_gmds/Summer Vacation.gmdl").unwrap();
    let out = generated_path("roundtrip.gmdl");
    list.export_to_gmdl(&out).unwrap();
    assert_lists_eq(&GDList::from_gmdl(&out).unwrap(), &list);
}

#[test]
fn sample_list_gmdl_roundtrip() {
    let list = sample_list();
    let out = generated_path("sample_list.gmdl");
    list.export_to_gmdl(&out).unwrap();
    assert_lists_eq(&GDList::from_gmdl(&out).unwrap(), &list);
}

#[test]
fn cclocallevels_file_roundtrip() {
    let mut cc = CCLocalLevels {
        levels: vec![sample_level(), GDLevel::default()],
        binary_version: GDVersion::GD22082,
        lists: vec![sample_list()],
    };
    let out = generated_path("CCLocalLevels.dat");
    cc.export_to_file(out.clone()).unwrap();
    let parsed = CCLocalLevels::from_decrypted(decrypt_file(out).unwrap()).unwrap();

    assert_eq!(parsed.binary_version, cc.binary_version);
    assert_eq!(parsed.levels.len(), cc.levels.len());
    for (a, e) in parsed.levels.iter().zip(&cc.levels) {
        assert_levels_eq(a, e);
    }
    assert_eq!(parsed.lists.len(), cc.lists.len());
    for (a, e) in parsed.lists.iter().zip(&cc.lists) {
        assert_lists_eq(a, e);
    }
}

#[test]
fn cclocallevels_file_keeps_non_ascii_text() {
    let mut level = GDLevel::default();
    level.identity.name = "Niveau élevé 日本".into();
    let mut cc = CCLocalLevels {
        levels: vec![level],
        binary_version: GDVersion::GD22082,
        lists: vec![],
    };
    let out = generated_path("CCLocalLevels_unicode.dat");
    cc.export_to_file(out.clone()).unwrap();
    let parsed = CCLocalLevels::from_decrypted(decrypt_file(out).unwrap()).unwrap();
    assert_eq!(parsed.levels[0].identity.name, "Niveau élevé 日本");
}

#[test]
fn decrypt_missing_file_returns_err() {
    assert!(decrypt_file(generated_path("does_not_exist.dat")).is_err());
}

// ---------------- generated levels for checking in-game ----------------

#[test]
fn move_constructor() {
    let mut level = GDLevel::default();
    level.identity.name = "move trigger t3st".into();
    level.identity.creator = "gdlib".into();
    level.add_object(GDObject::from_config(
        &GDObjConfig::default().with_pos(45.0, 45.0),
        MoveTrigger {
            move_config: MoveMode::Default(DefaultMove {
                dx: 45.0,
                dy: 54.0,
                x_lock: None,
                y_lock: None,
            }),
            time: 17.38,
            target_group: 679,
            silent: false,
            dynamic: true,
            easing: Easing::from(MoveEasing::BackOut, 1.50),
        },
    ));

    level
        .export_to_gmd(generated_path("move_trigger_test.gmd"))
        .unwrap();
}

#[test]
fn obj_properties() {
    let config = GDObjConfig::new()
        .with_editor_layer_1(4)
        .with_attributes(
            GDObjAttributes::dont_fade | GDObjAttributes::extra_sticky | GDObjAttributes::no_glow,
            true,
        )
        .with_groups([2i16, 3, 1738])
        .with_z_layer(ZLayer::B3)
        .with_base_colour(ColourChannel::Background);

    let block = GDObject::new(1, &config, vec![]);
    let mut level = GDLevel::default();
    level.add_object(block);

    level
        .export_to_gmd(generated_path("properties.gmd"))
        .unwrap();
}

#[test]
fn adv_random() {
    let mut level = GDLevel::default();
    level.add_object(GDObject::from_config(
        &GDObjConfig::default().with_pos(45.0, 45.0),
        AdvancedRandomTrigger {
            probabilities: vec![(50, 10), (60, 20), (70, 5), (80, 25), (90, 2)],
        },
    ));
    let _ = level.export_to_gmd(generated_path("adv_random.gmd"));
}

#[test]
fn event_trigger_test() {
    let mut level = GDLevel::default();
    let cfg = GDObjConfig::new().with_pos(45.0, 45.0);
    level.add_object(GDObject::from_config(
        &cfg,
        EventTrigger {
            target_group: 123,
            events: vec![Event::BallSwitch, Event::FallSpeedLow],
            extra_id: 0,
            extra_id2: ExtraID2::All,
        },
    ));
}

#[test]
fn item_compare() {
    benchmark("create level", || {
        let mut level = GDLevel::new(GDVersion::GD22082);

        level.add_object(GDObject::from_config(
            &GDObjConfig::new().with_pos(45.0, 45.0),
            ItemCompareTrigger {
                true_id: 11,
                false_id: 22,
                lhs: CompareOperand {
                    operand_item: Item::Counter(1),
                    modifier: 1.0,
                    mod_op: Op::Mul,
                    rounding: RoundMode::None,
                    sign: SignMode::None,
                },
                rhs: CompareOperand {
                    operand_item: Item::Counter(0),
                    modifier: 2.0,
                    mod_op: Op::Mul,
                    rounding: RoundMode::None,
                    sign: SignMode::None,
                },
                compare_op: CompareOp::LessOrEquals,
                tolerance: 0.0,
            },
        ));
        level
            .export_to_gmd(generated_path("itemcompare.gmd"))
            .unwrap();
    });
}
