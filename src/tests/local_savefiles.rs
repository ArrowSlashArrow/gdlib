//! Tests that need the savefiles of a local GD installation. None of these write to the savefiles.
use std::fs;

use plist::Value::{self, Array, Dictionary};

use super::{assert_levels_eq, assert_lists_eq};
use crate::{ccgamemanager::CCGameManager, cclocallevels::gdlevel::CCLocalLevels};

#[test]
fn read_cclocallevels() {
    let cclocallevels = CCLocalLevels::from_local().unwrap();
    println!("binary version: {:?}", cclocallevels.binary_version);
}

#[test]
fn cclocallevels_in_memory_roundtrip() {
    let mut cc = CCLocalLevels::from_local().unwrap();
    let parsed = CCLocalLevels::from_decrypted(cc.serialise_to_string()).unwrap();

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
fn every_local_level_decrypts() {
    let cc = CCLocalLevels::from_local().unwrap();
    for level in &cc.levels {
        if level.content.data.is_some() {
            assert!(
                level.get_decrypted_data().is_some(),
                "level data of \"{}\" could not be decrypted",
                level.identity.name
            );
        }
    }
}

#[test]
#[ignore]
fn print_list_info() {
    let cc = CCLocalLevels::from_local().unwrap();
    println!("{:#?}", cc.lists);
}

#[test]
#[ignore]
fn cc_game_manager_parse() {
    let gm = CCGameManager::from_local().unwrap();
    fs::write("ccgamemanager dump 2", format!("{gm:#?}")).unwrap();
    for (k, v) in gm.other_properties {
        println!(
            "{k}: {:?}",
            match v {
                Dictionary(d) => Value::String(format!("{{ length: {} }}", d.len())),
                Array(a) => Value::String(format!("[ length: {} ]", a.len())),
                v => v,
            }
        );
    }
}
