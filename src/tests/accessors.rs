//! Tests for getters, setters and builder methods on objects, object configs, levels and level data.
use super::{default_block, test_config};
use crate::cclocallevels::{
    gdlevel::{
        CCLocalLevels, GDLevel,
        leveldata::{GDEncryptedLevelData, GDLevelData, GDLevelState},
        version::GDVersion,
    },
    gdobj::{
        GDObject,
        ids::{metadata::GROUP_PROPERTY_IDS, properties as p},
        meta::{GDObjAttributes, GDObjConfig},
        structs::*,
    },
};

// ---------------- GDObject ----------------

#[test]
fn new_sorts_properties() {
    let obj = GDObject::new(
        1,
        &GDObjConfig::default(),
        vec![
            (500, GDValue::Int(1)),
            (36, GDValue::Bool(true)),
            (51, GDValue::Group(3)),
        ],
    );
    assert_eq!(
        obj.properties.iter().map(|(id, _)| *id).collect::<Vec<_>>(),
        vec![36, 51, 500]
    );
    assert_eq!(obj.get_property(500), Some(GDValue::Int(1)));
    assert_eq!(obj.get_property(36), Some(GDValue::Bool(true)));
    assert_eq!(obj.get_property(51), Some(GDValue::Group(3)));
}

#[test]
fn new_deduplicates_properties_with_the_same_id() {
    let obj = GDObject::new(
        1,
        &GDObjConfig::default(),
        vec![(36, GDValue::Bool(true)), (36, GDValue::Bool(false))],
    );
    assert_eq!(obj.properties.len(), 1, "{:?}", obj.properties);
}

#[test]
fn set_property_inserts_in_order() {
    let mut obj = default_block();
    for id in [500, 10, 300, 36, 10_005] {
        obj.set_property(id, GDValue::Int(id as i32));
    }
    let ids = obj.properties.iter().map(|(id, _)| *id).collect::<Vec<_>>();
    assert_eq!(ids, vec![10, 36, 300, 500, 10_005]);
    for id in ids {
        assert_eq!(obj.get_property(id), Some(GDValue::Int(id as i32)));
    }
}

#[test]
fn set_property_overwrites() {
    let mut obj = default_block();
    obj.set_property(51, GDValue::Group(1));
    obj.set_property(51, GDValue::Group(2));
    assert_eq!(obj.properties.len(), 1);
    assert_eq!(obj.get_property(51), Some(GDValue::Group(2)));
}

#[test]
fn set_config_property_updates_config() {
    // config-owned properties must go into the config, otherwise they would be serialised twice
    let mut obj = default_block();
    obj.set_property(p::X_POS, GDValue::Float(30.0));
    assert_eq!(obj.config.pos.0, 30.0);
    assert_eq!(obj.get_property(p::X_POS), Some(GDValue::Float(30.0)));
}

#[test]
fn del_property() {
    let mut obj = default_block();
    obj.set_property(51, GDValue::Group(1));
    obj.set_property(71, GDValue::Group(2));
    obj.del_property(51);
    assert!(!obj.has_property(51));
    assert_eq!(obj.get_property(51), None);
    assert_eq!(obj.get_property(71), Some(GDValue::Group(2)));
    // deleting a property that doesn't exist is a no-op
    obj.del_property(9999);
    assert_eq!(obj.properties.len(), 1);
}

#[test]
fn get_property_reads_config() {
    let cfg = GDObjConfig::new()
        .with_pos(15.0, 30.0)
        .with_angle(90.0)
        .with_scale(2.0, 3.0)
        .with_touchable(true)
        .with_spawnable(true)
        .with_multitrigger(true)
        .with_groups([1i16, 2])
        .with_editor_layer_1(4)
        .with_editor_layer_2(5)
        .with_base_colour(ColourChannel::Background)
        .with_detail_colour(ColourChannel::Channel(3))
        .with_z_layer(ZLayer::T2)
        .with_z_order(9)
        .with_enter_channel(6)
        .with_material_id(7)
        .with_control_id(8)
        .with_attributes(GDObjAttributes::hidden, true);
    let obj = GDObject::new(8, &cfg, vec![]);

    assert_eq!(obj.get_property(p::OBJECT_ID), Some(GDValue::Int(8)));
    assert_eq!(obj.get_property(p::X_POS), Some(GDValue::Float(15.0)));
    assert_eq!(obj.get_property(p::Y_POS), Some(GDValue::Float(30.0)));
    assert_eq!(obj.get_property(p::ROTATION), Some(GDValue::Float(90.0)));
    assert_eq!(obj.get_property(p::X_SCALE), Some(GDValue::Float(2.0)));
    assert_eq!(obj.get_property(p::Y_SCALE), Some(GDValue::Float(3.0)));
    assert_eq!(
        obj.get_property(p::TOUCH_TRIGGERABLE),
        Some(GDValue::Bool(true))
    );
    assert_eq!(
        obj.get_property(p::SPAWN_TRIGGERABLE),
        Some(GDValue::Bool(true))
    );
    assert_eq!(
        obj.get_property(p::MULTITRIGGERABLE),
        Some(GDValue::Bool(true))
    );
    assert_eq!(
        obj.get_property(p::GROUPS),
        Some(GDValue::from_group_list(&[1, 2]))
    );
    assert_eq!(
        obj.get_property(p::OBJECT_COLOUR),
        Some(GDValue::ColourChannel(ColourChannel::Background))
    );
    assert_eq!(
        obj.get_property(p::SECONDARY_COLOUR),
        Some(GDValue::ColourChannel(ColourChannel::Channel(3)))
    );
    assert_eq!(obj.get_property(p::Z_ORDER), Some(GDValue::Int(9)));
    assert_eq!(obj.get_property(p::HIDDEN), Some(GDValue::Bool(true)));
    assert_eq!(obj.get_property(p::NO_TOUCH), Some(GDValue::Bool(false)));
}

#[test]
fn get_property_config_values_match_property_table_types() {
    // `get_property` must return the same variant that parsing the property would produce
    let obj = GDObject::parse_str("1,1,2,0,3,0,20,4,24,5,61,6,343,7,446,8,534,9;");
    for id in [
        p::EDITOR_LAYER_1,
        p::EDITOR_LAYER_2,
        p::Z_LAYER,
        p::ENTER_EFFECT_CHANNEL,
        p::OBJECT_MATERIAL,
        p::MATERIAL_CONTROL_ID,
    ] {
        let value = obj.get_property(id).unwrap();
        let t = crate::cclocallevels::properties::get_obj_property_type(id)
            .unwrap_or(GDObjPropType::Unknown);
        assert_eq!(GDValue::from(t, &value.to_string()), value, "property {id}");
    }
}

#[test]
fn get_property_parent_groups() {
    let mut cfg = GDObjConfig::new().with_groups([1i16, 2]);
    cfg.pink_groups = vec![2];
    let obj = GDObject::new(1, &cfg, vec![]);
    assert_eq!(
        obj.get_property(p::PARENT_GROUPS),
        Some(GDValue::from_group_list(&[2]))
    );
}

#[test]
fn has_property_implies_get_property() {
    // documented on `has_property`
    let mut objects = vec![
        default_block(),
        GDObject::parse_str("1,901,2,15,3,15,51,3,10,0.5,30,1,85,2,64,1,121,1,10010,0;"),
        GDObject::new(
            1,
            &GDObjConfig::new().with_attributes(GDObjAttributes::all(), true),
            vec![],
        ),
    ];
    let mut with_pink = GDObjConfig::new();
    with_pink.pink_groups = vec![5];
    objects.push(GDObject::new(1, &with_pink, vec![]));

    for obj in objects {
        for id in 0..=u16::MAX {
            if obj.has_property(id) {
                assert!(obj.get_property(id).is_some(), "property {id} on {obj:?}");
            }
        }
    }
}

#[test]
fn has_property_for_config_and_attributes() {
    let obj = default_block();
    assert!(obj.has_property(p::OBJECT_ID));
    assert!(obj.has_property(p::X_POS));
    assert!(!obj.has_property(p::HIDDEN));
    let obj = GDObject::new(
        1,
        &GDObjConfig::new().with_attributes(GDObjAttributes::hidden, true),
        vec![],
    );
    assert!(obj.has_property(p::HIDDEN));
}

#[test]
fn set_config() {
    let mut obj = default_block();
    obj.set_config(test_config());
    assert_eq!(obj.config, test_config());
}

#[test]
fn names() {
    assert_eq!(
        GDObject::new(-5, &GDObjConfig::default(), vec![]).get_name(),
        "Object -5"
    );
    assert_ne!(default_block().get_name(), "Object 1");
}

#[test]
fn display() {
    let obj = GDObject::new(
        -5,
        &GDObjConfig::new()
            .with_pos(15.0, 30.0)
            .with_scale(2.0, 0.5)
            .with_angle(90.0)
            .with_groups([1i16, 2]),
        vec![],
    );
    assert_eq!(
        obj.to_string(),
        "Object -5 @ (15, 30) scaled to (2, 0.5) with groups: 1, 2 angled to 90°"
    );
}

#[test]
fn display_trigger_config() {
    let obj = GDObject::new(
        -5, // dummy id
        &GDObjConfig::new()
            .with_touchable(true)
            .with_multitrigger(true),
        vec![],
    );
    assert!(obj.to_string().starts_with("Multitouchable Object -5"));
    let obj = GDObject::new(-5, &GDObjConfig::new().with_spawnable(true), vec![]);
    assert!(obj.to_string().starts_with("spawnable Object -5"));
}

#[test]
fn debug_lists_properties() {
    let obj = GDObject::new(-5, &GDObjConfig::default(), vec![(60000, GDValue::Int(3))]);
    let s = format!("{obj:?}");
    assert!(s.contains("60000: Int(3)"), "{s}");
}

#[test]
fn default_from_id() {
    let obj = GDObject::default_from_id(1);
    assert_eq!(obj.id, 1);
}

// ---------------- GDObjConfig ----------------

#[test]
fn config_default_values() {
    let cfg = GDObjConfig::default();
    assert_eq!(cfg, GDObjConfig::new());
    assert_eq!(cfg.pos, (0.0, 0.0));
    assert_eq!(cfg.scale, (1.0, 1.0));
    assert_eq!(
        cfg.colour_channels,
        (ColourChannel::Object, ColourChannel::Channel(1))
    );
    assert!(cfg.groups.is_empty());
    assert!(cfg.pink_groups.is_empty());
    assert_eq!(cfg.attributes, GDObjAttributes::empty());
}

#[test]
fn config_setters_and_builders_agree() {
    let built = GDObjConfig::new()
        .with_x(1.0)
        .with_y(2.0)
        .with_xscale(3.0)
        .with_yscale(4.0)
        .with_angle(5.0)
        .with_touchable(true)
        .with_spawnable(true)
        .with_multitrigger(true)
        .with_base_colour(ColourChannel::Line)
        .with_detail_colour(ColourChannel::P2)
        .with_z_layer(ZLayer::B1)
        .with_z_order(6)
        .with_control_id(7)
        .with_enter_channel(8)
        .with_material_id(9)
        .with_editor_layer_1(10)
        .with_editor_layer_2(11);

    let mut set = GDObjConfig::new();
    set.set_x(1.0);
    set.set_y(2.0);
    set.set_xscale(3.0);
    set.set_yscale(4.0);
    set.set_angle(5.0);
    set.set_touchable(true);
    set.set_spawnable(true);
    set.set_multitrigger(true);
    set.set_base_colour(ColourChannel::Line);
    set.set_detail_colour(ColourChannel::P2);
    set.set_z_layer(ZLayer::B1);
    set.set_z_order(6);
    set.set_control_id(7);
    set.set_enter_channel(8);
    set.set_material_id(9);
    set.set_editor_layer_1(10);
    set.set_editor_layer_2(11);

    assert_eq!(built, set);
    assert_eq!(built.pos, (1.0, 2.0));
    assert_eq!(built.scale, (3.0, 4.0));
    assert_eq!(built.angle, 5.0);
    assert_eq!(
        built.colour_channels,
        (ColourChannel::Line, ColourChannel::P2)
    );
    assert_eq!(built.z_layer, ZLayer::B1);
    assert_eq!(built.z_order, 6);
    assert_eq!(built.control_id, 7);
    assert_eq!(built.enter_effect_channel, 8);
    assert_eq!(built.material_id, 9);
    assert_eq!(built.editor_layers, (10, 11));
}

#[test]
fn config_position_helpers() {
    let mut cfg = GDObjConfig::new().with_pos(1.0, 2.0).translate(10.0, -5.0);
    assert_eq!(cfg.pos, (11.0, -3.0));
    cfg.set_pos(4.0, 5.0);
    assert_eq!(cfg.pos, (4.0, 5.0));
    cfg.set_scale(0.5, 0.25);
    assert_eq!(cfg.scale, (0.5, 0.25));
    assert_eq!(GDObjConfig::new().with_scale(2.0, 3.0).scale, (2.0, 3.0));
}

#[test]
fn config_groups_are_sorted_and_deduplicated() {
    let mut cfg = GDObjConfig::new().with_groups([5i16, 1, 5, 3]);
    assert_eq!(cfg.groups, vec![1, 3, 5]);
    cfg.set_groups([9i16, 9, 2]);
    assert_eq!(cfg.groups, vec![2, 9]);
    cfg.add_groups([1, 9, 4]);
    assert_eq!(cfg.groups, vec![1, 2, 4, 9]);
    cfg.add_group(3);
    assert_eq!(cfg.groups, vec![1, 2, 3, 4, 9]);
    cfg.add_group(3);
    assert_eq!(cfg.groups, vec![1, 2, 3, 4, 9]);
}

#[test]
fn config_remove_group_keeps_order() {
    let mut cfg = GDObjConfig::new().with_groups([1i16, 2, 3, 4, 5]);
    cfg.remove_group(2);
    assert_eq!(cfg.groups, vec![1, 3, 4, 5]);
    cfg.remove_group(42); // not present
    assert_eq!(cfg.groups, vec![1, 3, 4, 5]);
}

#[test]
fn config_clear_groups() {
    let mut cfg = GDObjConfig::new().with_groups([1i16, 2]);
    cfg.clear_groups();
    assert!(cfg.groups.is_empty());
}

#[test]
fn config_pink_groups() {
    let cfg = GDObjConfig::new()
        .with_groups([1i16, 2])
        .with_pink_groups([2i16]);
    assert_eq!(cfg.groups, vec![1, 2]);
    assert_eq!(cfg.pink_groups, vec![2]);
}

#[test]
fn config_set_pink_groups() {
    let mut cfg = GDObjConfig::new().with_groups([1i16, 2, 3]);
    cfg.set_pink_groups([3i16, 1, 3]);
    assert_eq!(cfg.groups, vec![1, 2, 3]);
    assert_eq!(cfg.pink_groups, vec![1, 3]);
}

#[test]
fn config_add_pink_groups() {
    let mut cfg = GDObjConfig::new().with_groups([1i16, 2, 3]);
    cfg.add_pink_groups([3, 1]);
    cfg.add_pink_group(2);
    assert_eq!(cfg.groups, vec![1, 2, 3]);
    assert_eq!(cfg.pink_groups, vec![1, 2, 3]);
}

#[test]
fn config_remove_pink_group() {
    let mut cfg = GDObjConfig::new().with_groups([1i16, 2, 3]);
    cfg.pink_groups = vec![1, 2, 3];
    cfg.remove_pink_group(2);
    assert_eq!(cfg.groups, vec![1, 2, 3]);
    assert_eq!(cfg.pink_groups, vec![1, 3]);
}

#[test]
fn config_attributes() {
    let mut cfg = GDObjConfig::new()
        .with_attributes(GDObjAttributes::hidden | GDObjAttributes::no_glow, true);
    assert!(cfg.get_attributes(GDObjAttributes::hidden));
    assert!(cfg.get_attributes(GDObjAttributes::no_glow));
    assert!(!cfg.get_attributes(GDObjAttributes::passable));
    cfg.set_attributes(GDObjAttributes::hidden, false);
    assert!(!cfg.get_attributes(GDObjAttributes::hidden));
    assert!(cfg.get_attributes(GDObjAttributes::no_glow));
    assert_eq!(GDObjAttributes::new(), GDObjAttributes::empty());
}

// ---------------- GDLevel ----------------

#[test]
fn new_level_sets_binary_version() {
    assert_eq!(
        GDLevel::new(GDVersion::GD22082).identity.binary_version,
        GDVersion::GD22082
    );
}

#[test]
fn default_level_can_take_objects() {
    let mut level = GDLevel::default();
    level.add_object(default_block());
    level.add_objects([default_block(), default_block()]);
    assert_eq!(level.get_objects_ref().unwrap().len(), 3);
}

#[test]
fn add_object_without_data_creates_data() {
    let mut level = GDLevel::default();
    level.content.data = None;
    level.add_object(default_block());
    assert_eq!(level.get_objects_ref().map(Vec::len), Some(1));

    let mut level = GDLevel::default();
    level.content.data = None;
    level.add_objects([default_block(), default_block()]);
    assert_eq!(level.get_objects_ref().map(Vec::len), Some(2));
}

fn encrypted_level() -> GDLevel {
    let mut level = GDLevel::default();
    level.add_object(default_block());
    let data = level.get_decrypted_data().unwrap().serialise_to_string();
    level.content.data = Some(GDLevelState::Encrypted(GDEncryptedLevelData { data }));
    level
}

#[test]
fn add_object_to_encrypted_level_does_nothing() {
    // documented on `add_object`
    let mut level = encrypted_level();
    let before = level.content.data.clone();
    level.add_object(default_block());
    level.add_objects([default_block()]);
    assert_eq!(level.content.data, before);
    assert!(level.get_objects_ref().is_none());
    assert!(level.get_objects_mut().is_none());
    assert!(level.get_objects_clone().is_none());
}

#[test]
fn decrypt_level_data() {
    let mut level = encrypted_level();
    level.decrypt_level_data().unwrap();
    assert!(matches!(
        level.content.data,
        Some(GDLevelState::Decrypted(_))
    ));
    assert_eq!(level.get_objects_ref().unwrap().len(), 1);
    // decrypting again is a no-op
    level.decrypt_level_data().unwrap();
    assert_eq!(level.get_objects_ref().unwrap().len(), 1);
}

#[test]
fn decrypt_corrupt_level_data_returns_err() {
    let mut level = GDLevel::default();
    level.content.data = Some(GDLevelState::Encrypted(GDEncryptedLevelData {
        data: "corrupt".into(),
    }));
    assert!(level.decrypt_level_data().is_err());
}

#[test]
fn get_decrypted_data_ref_on_corrupt_data_does_not_panic() {
    let mut level = GDLevel::default();
    level.content.data = Some(GDLevelState::Encrypted(GDEncryptedLevelData {
        data: "corrupt".into(),
    }));
    assert!(level.get_decrypted_data_ref().is_none());
}

#[test]
fn get_decrypted_data_ref_decrypts() {
    let mut level = encrypted_level();
    let data = level.get_decrypted_data_ref().unwrap();
    data.objects.push(default_block());
    assert_eq!(level.get_objects_ref().unwrap().len(), 2);
}

#[test]
fn get_decrypted_data_does_not_mutate() {
    let level = encrypted_level();
    assert_eq!(level.get_decrypted_data().unwrap().objects.len(), 1);
    assert!(matches!(
        level.content.data,
        Some(GDLevelState::Encrypted(_))
    ));
}

#[test]
fn get_decrypted_data_without_data() {
    let mut level = GDLevel::default();
    level.content.data = None;
    assert!(level.get_decrypted_data().is_none());
    assert!(level.get_raw_decrypted_data().is_none());
    assert!(level.get_objects_ref().is_none());
}

#[test]
fn get_raw_decrypted_data() {
    let level = encrypted_level();
    let raw = level.get_raw_decrypted_data().unwrap();
    assert!(raw.ends_with("1,1,2,0,3,0;"), "{raw}");
    // only encrypted data can be returned raw
    assert!(GDLevel::default().get_raw_decrypted_data().is_none());
}

#[test]
fn objects_mut_and_clone() {
    let mut level = GDLevel::default();
    level.add_object(default_block());
    level.get_objects_mut().unwrap()[0].id = 8;
    assert_eq!(level.get_objects_clone().unwrap()[0].id, 8);
}

#[test]
fn add_level_inserts_at_front() {
    let mut cc = CCLocalLevels {
        levels: vec![],
        binary_version: GDVersion::GD22082,
        lists: vec![],
    };
    for name in ["a", "b", "c"] {
        let mut level = GDLevel::default();
        level.identity.name = name.into();
        cc.add_level(level);
    }
    assert_eq!(
        cc.levels
            .iter()
            .map(|l| l.identity.name.as_str())
            .collect::<Vec<_>>(),
        vec!["c", "b", "a"]
    );
}

// ---------------- GDLevelData ----------------

fn level_data(objects: Vec<GDObject>) -> GDLevelData {
    GDLevelData {
        headers: Default::default(),
        objects,
    }
}

#[test]
fn used_groups() {
    let data = level_data(vec![
        GDObject::new(1, &GDObjConfig::new().with_groups([3i16, 1]), vec![]),
        GDObject::new(1, &GDObjConfig::new().with_groups([1i16, 9999]), vec![]),
        default_block(),
    ]);
    assert_eq!(data.get_used_groups(), vec![1, 3, 9999]);
    assert!(level_data(vec![]).get_used_groups().is_empty());
}

#[test]
fn unused_groups() {
    let data = level_data(vec![GDObject::new(
        1,
        &GDObjConfig::new().with_groups([1i16, 5, 9999]),
        vec![],
    )]);
    let unused = data.get_unused_groups();
    assert_eq!(unused.len(), 9999 - 3);
    assert!(!unused.contains(&1));
    assert!(!unused.contains(&5));
    assert!(!unused.contains(&9999));
    assert!(unused.contains(&2));
    assert_eq!(level_data(vec![]).get_unused_groups().len(), 9999);
}

#[test]
fn unused_groups_ignores_out_of_range_groups() {
    let data = level_data(vec![GDObject::new(
        1,
        &GDObjConfig::new().with_groups([0i16, -5, 10_000, i16::MAX]),
        vec![],
    )]);
    assert_eq!(data.get_unused_groups().len(), 9999);
}

#[test]
fn argument_groups() {
    let data = level_data(vec![
        GDObject::new(
            901,
            &GDObjConfig::new().with_groups([100i16]),
            vec![
                (p::TARGET_ITEM, GDValue::Group(5)),
                (p::TARGET_ITEM_2, GDValue::Group(3)),
            ],
        ),
        GDObject::new(
            1268,
            &GDObjConfig::default(),
            vec![(p::TARGET_ITEM, GDValue::Group(5))],
        ),
    ]);
    let groups = data.get_argument_groups();
    for g in [3, 5] {
        assert!(groups.contains(&g), "{groups:?}");
    }
    // sorted and deduplicated
    let mut sorted = groups.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(groups, sorted);
    assert!(level_data(vec![]).get_argument_groups().is_empty());
}

#[test]
fn group_property_ids_include_target_items() {
    assert!(GROUP_PROPERTY_IDS.contains(&p::TARGET_ITEM));
    assert!(GROUP_PROPERTY_IDS.contains(&p::TARGET_ITEM_2));
}
