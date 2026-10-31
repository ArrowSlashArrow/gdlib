//! Tests for turning structs into raw strings and plist dictionaries.
use plist::{Dictionary, Value};
use smallvec::smallvec;

use super::{default_block, object_kv_pairs, sample_level, sample_list, values_of};
use crate::{
    cclocallevels::{
        gdlevel::{
            CCLocalLevels, GDLevel, PLIST_FOOTER, PLIST_HEADER,
            leveldata::{
                ColourString, GDLevelData, GDLevelHeaderKey, GDLevelHeaderValue, GuidelineColour,
                GuidelineString, PlayerColour,
            },
            version::GDVersion,
        },
        gdobj::{
            GDObject,
            constructors::text,
            gameplay::PortalCameraOptions,
            ids::{objects as o, properties as p},
            meta::{GDObjAttributes, GDObjConfig},
            serialise_objects,
            structs::*,
        },
    },
    core::{
        b64_decode, b64_encode,
        crypto::generate_gjp2_hexdigest,
        io::{encrypt_level_str, encrypt_savefile_str, stringify_xml},
        proper_plist_tags,
    },
};

// ---------------- GDValue ----------------

#[test]
fn gdvalue_scalars() {
    let cases: &[(GDValue, &str)] = &[
        (GDValue::Int(0), "0"),
        (GDValue::Int(-17), "-17"),
        (GDValue::Int(i32::MAX), "2147483647"),
        (GDValue::Int(i32::MIN), "-2147483648"),
        (GDValue::Short(i16::MIN), "-32768"),
        (GDValue::Short(i16::MAX), "32767"),
        (GDValue::Group(9999), "9999"),
        (GDValue::Item(-1), "-1"),
        (GDValue::Bool(true), "1"),
        (GDValue::Bool(false), "0"),
        (GDValue::Float(1.5), "1.5"),
        (GDValue::Float(-0.25), "-0.25"),
        (GDValue::Float(0.1), "0.1"),
        (GDValue::Double(0.1), "0.1"),
        (GDValue::Double(-1234.5678), "-1234.5678"),
        (GDValue::String("hello".into()), "hello"),
        (GDValue::String(String::new()), ""),
    ];
    for (value, expected) in cases {
        assert_eq!(value.to_string(), *expected, "{value:?}");
    }
}

#[test]
fn gdvalue_toggle() {
    assert_eq!(GDValue::Toggle(Toggle::On).to_string(), "1");
    assert_eq!(GDValue::Toggle(Toggle::Unset).to_string(), "0");
    assert_eq!(GDValue::Toggle(Toggle::Off).to_string(), "-1");
}

#[test]
fn gdvalue_colour_channels() {
    let cases = [
        (ColourChannel::Channel(1), "1"),
        (ColourChannel::Channel(999), "999"),
        (ColourChannel::Background, "1000"),
        (ColourChannel::Ground1, "1001"),
        (ColourChannel::Line, "1002"),
        (ColourChannel::ThreeDLine, "1003"),
        (ColourChannel::Object, "1004"),
        (ColourChannel::P1, "1005"),
        (ColourChannel::P2, "1006"),
        (ColourChannel::Ground2, "1009"),
        (ColourChannel::MiddleGround, "1013"),
        (ColourChannel::MiddleGround2, "1014"),
    ];
    for (ch, expected) in cases {
        assert_eq!(GDValue::ColourChannel(ch).to_string(), expected, "{ch:?}");
    }
}

#[test]
fn gdvalue_lists() {
    assert_eq!(GDValue::GroupList(smallvec![1, 2, 3]).to_string(), "1.2.3");
    assert_eq!(GDValue::GroupList(smallvec![42]).to_string(), "42");
    assert_eq!(GDValue::GroupList(smallvec![]).to_string(), "");
    // more groups than the inline capacity of the SmallVec
    assert_eq!(
        GDValue::GroupList(smallvec![1, 2, 3, 4, 5, 6, 7, 8]).to_string(),
        "1.2.3.4.5.6.7.8"
    );

    assert_eq!(
        GDValue::ProbabilitiesList(smallvec![(1, 10), (2, 20), (3, 30)]).to_string(),
        "1.10.2.20.3.30"
    );
    assert_eq!(GDValue::ProbabilitiesList(smallvec![]).to_string(), "");
    assert_eq!(
        GDValue::SpawnRemapsList(smallvec![(1, 2), (-3, 40000)]).to_string(),
        "1.2.-3.40000"
    );
    assert_eq!(GDValue::SpawnRemapsList(smallvec![]).to_string(), "");

    assert_eq!(
        GDValue::Events(vec![Event::BallSwitch, Event::FallSpeedLow]).to_string(),
        "19.76"
    );
    assert_eq!(
        GDValue::Events(vec![Event::CheckpointRespawn]).to_string(),
        "64"
    );
    assert_eq!(
        GDValue::Events(vec![Event::Unrecognized(1234)]).to_string(),
        "1234"
    );
    assert_eq!(GDValue::Events(vec![]).to_string(), "");
}

#[test]
fn gdvalue_enums() {
    let cases: &[(GDValue, &str)] = &[
        (GDValue::Easing(MoveEasing::None), "0"),
        (GDValue::Easing(MoveEasing::BackOut), "18"),
        (GDValue::ZLayer(ZLayer::B5), "-5"),
        (GDValue::ZLayer(ZLayer::Default), "0"),
        (GDValue::ZLayer(ZLayer::T4), "11"),
        (GDValue::Speed(Speed::X0Point5), "1"),
        (GDValue::Speed(Speed::X1), "0"),
        (GDValue::Speed(Speed::X4), "4"),
        (GDValue::Gamemode(Gamemode::Swing), "7"),
        (GDValue::ItemEditOperator(Op::Div), "4"),
        (GDValue::ItemCompareOperator(CompareOp::NotEquals), "5"),
        (GDValue::BPMSpeed(BPMSpeed::Slow), "1"),
        (GDValue::ItemEditRoundMode(RoundMode::Ceiling), "3"),
        (GDValue::ItemEditSignMode(SignMode::Negative), "2"),
        (GDValue::TouchToggle(TouchToggle::ToggleOff), "2"),
        (GDValue::AxisOnlyMove(AxisOnlyMove::Y), "2"),
        (GDValue::ArrowDirection(ArrowDirection::Up), "1"),
        (GDValue::PlayerTarget(OptionalPlayerTarget::Player2), "2"),
        (GDValue::TransitionMode(TransitionMode::Exit), "2"),
        (GDValue::UIReferencePos(UIReferencePos::Top), "8"),
        (GDValue::CounterMode(CounterMode::Attempts), "-3"),
        (GDValue::CounterMode(CounterMode::Unrecognized(-50)), "-50"),
        (GDValue::ItemType(ItemType::Attempts), "5"),
        (GDValue::ItemType(ItemType::Unrecognized(77)), "77"),
        (GDValue::StopMode(StopMode::Resume), "2"),
        (GDValue::ExtraID2(ExtraID2::P2), "2"),
        (GDValue::ItemAlign(ItemAlign::Right), "2"),
        (GDValue::TeleportGravity(TeleportGravity::Toggle), "3"),
        (GDValue::MiddleGround(MiddleGround::Clouds), "3"),
        (GDValue::MiddleGround(MiddleGround::Unrecognized(12)), "12"),
    ];
    for (value, expected) in cases {
        assert_eq!(value.to_string(), *expected, "{value:?}");
    }
}

// ---------------- GDObjAttributes / GDObjConfig ----------------

#[test]
fn attributes_empty() {
    assert_eq!(GDObjAttributes::new().get_property_str(), "");
}

#[test]
fn attributes_each_flag_maps_to_its_property() {
    let cases = [
        (GDObjAttributes::dont_fade, p::DONT_FADE),
        (GDObjAttributes::dont_enter, p::DONT_ENTER),
        (GDObjAttributes::no_effects, p::NO_OBJECT_EFFECTS),
        (GDObjAttributes::is_group_parent, p::IS_GROUP_PARENT),
        (GDObjAttributes::is_area_parent, p::IS_AREA_PARENT),
        (GDObjAttributes::dont_boost_x, p::DONT_BOOST_X),
        (GDObjAttributes::dont_boost_y, p::DONT_BOOST_Y),
        (GDObjAttributes::high_detail, p::IS_HIGH_DETAIL),
        (GDObjAttributes::no_touch, p::NO_TOUCH),
        (GDObjAttributes::passable, p::PASSABLE),
        (GDObjAttributes::hidden, p::HIDDEN),
        (GDObjAttributes::non_stick_x, p::NONSTICK_X),
        (GDObjAttributes::non_stick_y, p::NONSTICK_Y),
        (GDObjAttributes::extra_sticky, p::EXTRA_STICKY),
        (
            GDObjAttributes::extended_collision,
            p::HAS_EXTENDED_COLLISION,
        ),
        (GDObjAttributes::is_ice_block, p::IS_ICE_BLOCK),
        (GDObjAttributes::grip_slope, p::GRIP_SLOPE),
        (GDObjAttributes::no_glow, p::NO_GLOW),
        (GDObjAttributes::no_particles, p::NO_PARTICLES),
        (GDObjAttributes::scale_stick, p::SCALE_STICK),
        (GDObjAttributes::no_audio_scale, p::NO_AUDIO_SCALE),
        (GDObjAttributes::single_ptouch, p::SINGLE_PLAYER_TOUCH),
        (GDObjAttributes::center_effect, p::CENTER_EFFECT),
        (GDObjAttributes::reverse, p::REVERSES_GAMEPLAY),
    ];
    for (flag, id) in cases {
        assert_eq!(flag.get_property_str(), format!(",{id},1"), "{flag:?}");
    }
}

#[test]
fn attributes_all_flags() {
    let s = GDObjAttributes::all().get_property_str();
    let pairs = object_kv_pairs(s.trim_start_matches(','));
    assert_eq!(pairs.len(), 24);
    assert!(pairs.iter().all(|(_, v)| v == "1"));
}

#[test]
fn config_default() {
    assert_eq!(GDObjConfig::default().serialise_to_string(), ",2,0,3,0");
}

#[test]
fn config_position_is_always_written() {
    let s = GDObjConfig::new()
        .with_pos(-15.5, 1e3)
        .serialise_to_string();
    assert_eq!(values_of(&s, "2"), vec!["-15.5"]);
    assert_eq!(values_of(&s, "3"), vec!["1000"]);
}

#[test]
fn config_every_field() {
    let cfg = GDObjConfig::new()
        .with_pos(15.0, 30.0)
        .with_scale(2.0, 0.5)
        .with_angle(45.0)
        .with_touchable(true)
        .with_spawnable(true)
        .with_multitrigger(true)
        .with_editor_layer_1(3)
        .with_editor_layer_2(-4)
        .with_base_colour(ColourChannel::Background)
        .with_detail_colour(ColourChannel::Channel(25))
        .with_z_layer(ZLayer::B3)
        .with_z_order(-7)
        .with_enter_channel(5)
        .with_material_id(6)
        .with_control_id(8)
        .with_groups([10i16, 2, 3])
        .with_attributes(GDObjAttributes::hidden | GDObjAttributes::no_touch, true);
    let s = format!("1,1{}", cfg.serialise_to_string());

    for (key, expected) in [
        ("2", "15"),
        ("3", "30"),
        ("128", "2"),
        ("129", "0.5"),
        ("6", "45"),
        ("11", "1"),
        ("62", "1"),
        ("87", "1"),
        ("20", "3"),
        ("61", "-4"),
        ("21", "1000"),
        ("22", "25"),
        ("24", "-1"),
        ("25", "-7"),
        ("343", "5"),
        ("446", "6"),
        ("534", "8"),
        ("57", "2.3.10"),
        ("121", "1"),
        ("135", "1"),
    ] {
        assert_eq!(values_of(&s, key), vec![expected], "property {key} in {s}");
    }
}

#[test]
fn config_default_values_are_omitted() {
    let s = GDObjConfig::new().serialise_to_string();
    for key in [
        "6", "11", "20", "21", "22", "24", "25", "57", "61", "62", "87", "128", "129", "274",
        "343", "446", "534",
    ] {
        assert!(values_of(&s, key).is_empty(), "property {key} in {s}");
    }
}

#[test]
fn config_pink_groups_are_written_to_their_own_property() {
    let mut cfg = GDObjConfig::new().with_groups([1i16, 2, 3]);
    cfg.pink_groups = vec![2];
    let s = format!("1,1{}", cfg.serialise_to_string());
    assert_eq!(values_of(&s, "57"), vec!["1.2.3"]);
    assert_eq!(values_of(&s, "274"), vec!["2"]);
}

#[test]
fn config_only_pink_groups() {
    let mut cfg = GDObjConfig::new();
    cfg.pink_groups = vec![4, 5];
    let s = format!("1,1{}", cfg.serialise_to_string());
    assert_eq!(values_of(&s, "274"), vec!["4.5"]);
    assert!(values_of(&s, "57").is_empty());
}

#[test]
fn config_each_property_written_once() {
    let mut cfg = GDObjConfig::new()
        .with_groups([1i16, 2])
        .with_attributes(GDObjAttributes::all(), true)
        .with_multitrigger(true);
    cfg.pink_groups = vec![1];
    let s = format!("1,1{}", cfg.serialise_to_string());
    let mut keys = object_kv_pairs(&s)
        .into_iter()
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    let len = keys.len();
    keys.sort();
    keys.dedup();
    assert_eq!(keys.len(), len, "duplicate keys in {s}");
}

// ---------------- GDObject ----------------

#[test]
fn object_minimal() {
    assert_eq!(default_block().serialise_to_string(), "1,1,2,0,3,0;");
}

#[test]
fn object_starts_with_id_and_ends_with_semicolon() {
    let obj = GDObject::new(
        1611,
        &GDObjConfig::new().with_pos(1.0, 2.0),
        vec![(36, GDValue::Bool(true)), (51, GDValue::Group(5))],
    );
    let s = obj.serialise_to_string();
    assert!(s.starts_with("1,1611,"), "{s}");
    assert!(s.ends_with(';'), "{s}");
    assert_eq!(s.matches(';').count(), 1, "{s}");
    assert_eq!(values_of(&s, "36"), vec!["1"]);
    assert_eq!(values_of(&s, "51"), vec!["5"]);
}

#[test]
fn object_ka_properties() {
    let obj = GDObject::new(
        31,
        &GDObjConfig::default(),
        vec![
            (10_011, GDValue::String(String::new())),
            (10_027, GDValue::Int(1)),
        ],
    );
    let s = obj.serialise_to_string();
    assert_eq!(values_of(&s, "kA11"), vec![""]);
    assert_eq!(values_of(&s, "kA27"), vec!["1"]);
    assert!(values_of(&s, "10011").is_empty());
}

#[test]
fn object_ka_property_zero() {
    let obj = GDObject::new(31, &GDObjConfig::default(), vec![(10_000, GDValue::Int(3))]);
    assert_eq!(values_of(&obj.serialise_to_string(), "kA0"), vec!["3"]);
}

#[test]
fn object_property_that_belongs_to_config_is_not_duplicated() {
    // property 87 belongs to `GDObjConfig.trigger_cfg`. If it also gets stored in the property list,
    // it must not be written twice.
    let obj = GDObject::new(
        1,
        &GDObjConfig::new().with_multitrigger(true),
        vec![(p::MULTITRIGGERABLE, GDValue::Bool(false))],
    );
    let s = obj.serialise_to_string();
    assert_eq!(values_of(&s, "87").len(), 1, "{s}");
}

#[test]
fn objects_are_concatenated() {
    let a = default_block();
    let b = GDObject::new(2, &GDObjConfig::new().with_pos(30.0, 0.0), vec![]);
    assert_eq!(
        serialise_objects([a.clone(), b.clone()]),
        format!("{}{}", a.serialise_to_string(), b.serialise_to_string())
    );
    assert_eq!(serialise_objects(Vec::<GDObject>::new()), "");
}

#[test]
fn text_object() {
    let obj = text(&GDObjConfig::default(), "Hello, world!", 3);
    let s = obj.serialise_to_string();
    assert!(s.starts_with(&format!("1,{},", o::TEXT)));
    let encoded = values_of(&s, "31");
    assert_eq!(encoded.len(), 1);
    // the comma in the text must not leak into the object string
    assert_eq!(b64_decode(&encoded[0]).unwrap(), b"Hello, world!");
    assert_eq!(values_of(&s, "488"), vec!["3"]);
}

#[test]
fn text_object_unicode() {
    let obj = text(&GDObjConfig::default(), "日本語 ✓", 0);
    let encoded = values_of(&obj.serialise_to_string(), "31");
    assert_eq!(
        String::from_utf8(b64_decode(&encoded[0]).unwrap()).unwrap(),
        "日本語 ✓"
    );
}

// ---------------- sub-structs ----------------

#[test]
fn hsv_colour() {
    let hsv = HSVColour {
        hue_shift: -30,
        saturation_mult: 0.5,
        brightness_mult: 1.25,
        static_sat_scalar: true,
        static_bright_scalar: false,
    };
    assert_eq!(hsv.to_string(), "-30a0.5a1.25a1a0");

    let hsv = HSVColour {
        hue_shift: 0,
        saturation_mult: 1.0,
        brightness_mult: 1.0,
        static_sat_scalar: false,
        static_bright_scalar: true,
    };
    assert_eq!(hsv.to_string(), "0a1a1a0a1");
}

#[test]
fn colour_to_properties() {
    let c = Colour::from_rgb((255, 128, 0));
    assert_eq!(
        c.to_properties(),
        vec![
            (p::RED, GDValue::Int(255)),
            (p::GREEN, GDValue::Int(128)),
            (p::BLUE, GDValue::Int(0)),
        ]
    );
}

#[test]
fn easing_serialise() {
    let e = Easing::from(MoveEasing::ElasticOut, 1.5);
    assert_eq!(
        e.serialise(),
        [
            (p::MOVE_EASING, GDValue::Easing(MoveEasing::ElasticOut)),
            (p::EASING_RATE, GDValue::Float(1.5)),
        ]
    );
}

#[test]
fn teleport_config_without_force() {
    let tp = TeleportConfig {
        target_group_id: 12,
        gravity: TeleportGravity::Flipped,
        ..Default::default()
    };
    let props = tp.to_properties();
    assert!(props.contains(&(p::TARGET_ITEM, GDValue::Group(12))));
    assert!(props.contains(&(
        p::TELEPORT_EXIT_GRAVITY,
        GDValue::TeleportGravity(TeleportGravity::Flipped)
    )));
    assert!(!props.iter().any(
        |(id, _)| *id == p::TELEPORT_USE_STATIC_FORCE || *id == p::TELEPORT_USE_REDIRECT_FORCE
    ));
}

#[test]
fn teleport_config_static_force() {
    let tp = TeleportConfig {
        force: TeleportForce::Static {
            force: 2.5,
            is_additive: true,
        },
        ..Default::default()
    };
    let props = tp.to_properties();
    assert!(props.contains(&(p::TELEPORT_USE_STATIC_FORCE, GDValue::Bool(true))));
    assert!(props.contains(&(p::TELEPORT_STATIC_FORCE_VALUE, GDValue::Float(2.5))));
    assert!(props.contains(&(p::TELEPORT_STATIC_FORCE_IS_ADDITIVE, GDValue::Bool(true))));
}

#[test]
fn teleport_config_redirect_force() {
    let tp = TeleportConfig {
        force: TeleportForce::Redirect {
            min: 1.0,
            max: 3.5,
            modifier: 0.75,
        },
        ..Default::default()
    };
    let props = tp.to_properties();
    assert!(props.contains(&(p::TELEPORT_USE_REDIRECT_FORCE, GDValue::Bool(true))));
    assert!(props.contains(&(p::TELEPORT_REDIRECT_FORCE_MINIMUM, GDValue::Float(1.0))));
    assert!(props.contains(&(p::TELEPORT_REDIRECT_FORCE_MAXIMUM, GDValue::Float(3.5))));
    assert!(props.contains(&(p::TELEPORT_REDIRECT_FORCE_MOD, GDValue::Float(0.75))));
}

#[test]
fn portal_camera_options() {
    let opts = PortalCameraOptions {
        disable_gridsnap: true,
        free_mode: false,
        edit_camera_settings: None,
    };
    assert_eq!(
        opts.to_properties(),
        vec![
            (p::DISABLE_GRIDSNAP, GDValue::Bool(true)),
            (p::PORTAL_FREE_MODE, GDValue::Bool(false)),
        ]
    );

    let opts = PortalCameraOptions {
        disable_gridsnap: false,
        free_mode: true,
        edit_camera_settings: Some((26, 0.5)),
    };
    let props = opts.to_properties();
    assert!(props.contains(&(p::PORTAL_EDIT_CAMERA_SETTINGS, GDValue::Bool(true))));
    assert!(props.contains(&(p::PORTAL_CAMERA_EASING, GDValue::Int(26))));
    assert!(props.contains(&(p::PORTAL_CAMERA_PADDING, GDValue::Float(0.5))));
}

// ---------------- level header ----------------

#[test]
fn header_keys() {
    assert_eq!(GDLevelHeaderKey::A(13).to_string(), "kA13");
    assert_eq!(GDLevelHeaderKey::S(38).to_string(), "kS38");
    assert_eq!(GDLevelHeaderKey::A(0).to_string(), "kA0");
    assert_eq!(GDLevelHeaderKey::Unknown("kZZ".into()).to_string(), "kZZ");
}

#[test]
fn header_values() {
    assert_eq!(GDLevelHeaderValue::Int(-4).to_string(), "-4");
    assert_eq!(GDLevelHeaderValue::Float(0.5).to_string(), "0.5");
    assert_eq!(GDLevelHeaderValue::Bool(true).to_string(), "1");
    assert_eq!(GDLevelHeaderValue::Bool(false).to_string(), "0");
    assert_eq!(
        GDLevelHeaderValue::Gamemode(Gamemode::Wave).to_string(),
        "4"
    );
    assert_eq!(GDLevelHeaderValue::Speed(Speed::X0Point5).to_string(), "1");
    assert_eq!(GDLevelHeaderValue::ColourString(vec![]).to_string(), "");
    assert_eq!(
        GDLevelHeaderValue::GuidelineString(GuidelineString { guidelines: vec![] }).to_string(),
        ""
    );
}

#[test]
fn colour_string_default() {
    // properties 1, 2, 3, 8, 11, 12, 13, 18 are always present. 4 is present since its default is -1
    assert_eq!(
        ColourString::default().to_string(),
        "1_0_2_0_3_0_4_-1_8_0_11_0_12_0_13_0_18_0"
    );
}

#[test]
fn colour_string_full() {
    let cs = ColourString {
        from: Colour::from_rgb((40, 125, 255)),
        to: Colour::from_rgb((1, 2, 3)),
        player_colour: PlayerColour::Second,
        blending: true,
        colour_ch_idx: 1000,
        from_opacity: 0.5,
        to_opacity: 0.25,
        opacity_toggled: true,
        inherited_col_ch_idx: 3,
        copied_hsv: Some(HSVColour {
            hue_shift: 10,
            saturation_mult: 1.0,
            brightness_mult: 0.5,
            static_sat_scalar: false,
            static_bright_scalar: true,
        }),
        deltatime: 0.75,
        duration: 2.0,
        copy_opacity: true,
        unknown_property18: true,
    };
    assert_eq!(
        cs.to_string(),
        "1_40_2_125_3_255_4_2_5_1_6_1000_7_0.5_8_1_9_3_11_1_12_2_13_3_14_0.75_15_0.25_16_2_17_1_18_1_10_10a1a0.5a0a1"
    );
}

#[test]
fn colour_string_header_value_has_trailing_pipes() {
    let v =
        GDLevelHeaderValue::ColourString(vec![ColourString::default(), ColourString::default()]);
    let s = v.to_string();
    assert_eq!(s.matches('|').count(), 2);
    assert!(s.ends_with('|'));
}

#[test]
fn guideline_string() {
    // kA14 is a flat list of `time~colour~` pairs
    let g = GuidelineString {
        guidelines: vec![
            ("1.5".into(), GuidelineColour::Yellow),
            ("2".into(), GuidelineColour::Green),
            ("3.25".into(), GuidelineColour::Orange),
        ],
    };
    assert_eq!(g.to_string(), "1.5~0.9~2~1~3.25~0");
}

#[test]
fn level_headers_ks38_first() {
    let data = GDLevelData {
        headers: [
            (
                GDLevelHeaderKey::A(2),
                GDLevelHeaderValue::Gamemode(Gamemode::Ship),
            ),
            (
                GDLevelHeaderKey::S(38),
                GDLevelHeaderValue::ColourString(vec![ColourString::default()]),
            ),
            (GDLevelHeaderKey::A(13), GDLevelHeaderValue::Float(0.5)),
        ]
        .into(),
        objects: vec![],
    };
    let s = data.serialise_level_headers();
    assert!(s.starts_with("kS38,"), "{s}");
    assert!(s.ends_with(';'), "{s}");
    let kv = object_kv_pairs(&s);
    assert!(kv.contains(&("kA2".into(), "1".into())), "{s}");
    assert!(kv.contains(&("kA13".into(), "0.5".into())), "{s}");
}

#[test]
fn level_headers_only_ks38_has_no_dangling_comma() {
    let data = GDLevelData {
        headers: [(
            GDLevelHeaderKey::S(38),
            GDLevelHeaderValue::ColourString(vec![]),
        )]
        .into(),
        objects: vec![],
    };
    let s = data.serialise_level_headers();
    assert!(!s.contains(",;"), "{s}");
}

#[test]
fn level_headers_empty() {
    let data = GDLevelData {
        headers: Default::default(),
        objects: vec![],
    };
    assert_eq!(data.serialise_level_headers(), ";");
}

#[test]
fn level_headers_are_deterministic() {
    // each default level builds its own header map, so the iteration order of the map can differ
    let headers = || {
        GDLevel::default()
            .get_decrypted_data()
            .unwrap()
            .serialise_level_headers()
    };
    let first = headers();
    for _ in 0..10 {
        assert_eq!(headers(), first);
    }
}

#[test]
fn level_data_raw_string() {
    let data = GDLevelData {
        headers: [(
            GDLevelHeaderKey::A(2),
            GDLevelHeaderValue::Gamemode(Gamemode::Ball),
        )]
        .into(),
        objects: vec![
            default_block(),
            GDObject::new(2, &GDObjConfig::new().with_pos(30.0, 15.0), vec![]),
        ],
    };
    assert_eq!(
        data.serialise_to_raw_string(),
        "kA2,2;1,1,2,0,3,0;1,2,2,30,3,15;"
    );
}

#[test]
fn level_data_encrypted_string_has_gd_header() {
    let data = GDLevel::default().get_decrypted_data().unwrap();
    assert!(data.serialise_to_string().starts_with("H4sIAAAAAAAAC"));
}

// ---------------- GDLevel / GDList dictionaries ----------------

#[test]
fn level_default_dict() {
    let d = GDLevel::default().to_dict();
    assert_eq!(d.get("kCEK"), Some(&Value::from(4)));
    assert_eq!(d.get("k13"), Some(&Value::Boolean(true)));
    assert_eq!(d.get("k21"), Some(&Value::from(2))); // local level
    assert!(d.contains_key("k4"));
    assert!(d.contains_key("k50"));
    assert!(d.contains_key("kI6"));
    // defaults are omitted
    for key in ["k1", "k2", "k3", "k14", "k18", "k42", "k45", "k105"] {
        assert!(!d.contains_key(key), "{key} should be omitted");
    }
}

#[test]
fn level_dict_value_types() {
    let level = sample_level();
    let d = level.to_dict();

    assert_eq!(d.get("k1"), Some(&Value::from(128)));
    assert_eq!(d.get("k2"), Some(&Value::from("gdlib test level")));
    assert_eq!(
        d.get("k3"),
        Some(&Value::from(b64_encode(
            "A level used in gdlib's unit tests."
        )))
    );
    assert_eq!(d.get("k5"), Some(&Value::from("gdlib")));
    assert_eq!(d.get("k8"), Some(&Value::from(6)));
    assert_eq!(d.get("k9"), Some(&Value::from(40)));
    assert_eq!(d.get("k21"), Some(&Value::from(3)));
    assert_eq!(d.get("k23"), Some(&Value::from(3)));
    assert_eq!(d.get("k25"), Some(&Value::Boolean(true)));
    assert_eq!(d.get("k34"), Some(&Value::from("replaydata")));
    assert_eq!(d.get("k50"), Some(&Value::from(48)));
    assert_eq!(d.get("k75"), Some(&Value::from(2)));
    assert_eq!(d.get("k76"), Some(&Value::from(5)));
    assert_eq!(d.get("k88"), Some(&Value::from("12,40,15")));
    assert_eq!(d.get("k104"), Some(&Value::from("10,20,30")));
    assert_eq!(d.get("k105"), Some(&Value::from("1,2")));
    assert_eq!(d.get("k109"), Some(&Value::from("16000,17500")));
    assert_eq!(d.get("k110"), Some(&Value::from("17,3")));
    assert_eq!(d.get("kI1"), Some(&Value::from(123.5f32)));
    assert_eq!(d.get("kI4"), Some(&Value::from(2)));
    assert_eq!(d.get("k200"), Some(&Value::Integer(200.into())));
    assert_eq!(d.get("kQ"), Some(&Value::from("unknown string")));

    let ki6 = d.get("kI6").unwrap().as_dictionary().unwrap();
    assert_eq!(ki6.len(), 14);
    assert_eq!(ki6.get("1"), Some(&Value::from("1")));
    assert_eq!(ki6.get("13"), Some(&Value::from("6")));
}

#[test]
fn level_unknown_binary_version_is_preserved() {
    let level = GDLevel::new(GDVersion::Unknown(9000));
    assert_eq!(level.to_dict().get("k50"), Some(&Value::from(9000)));
}

#[test]
fn level_binary_version_without_known_id_is_not_written_as_zero() {
    // see `GDVersion::to_binary_version_id`: 2.207 has no known binary version.
    // Writing 0 makes GD treat the level as a 1.0 level and wipe its data.
    let level = GDLevel::new(GDVersion::GD2207);
    assert_ne!(level.to_dict().get("k50"), Some(&Value::from(0)));
}

#[test]
fn level_serialise_to_string_is_gmd() {
    let s = GDLevel::default().serialise_to_string();
    assert!(s.starts_with(PLIST_HEADER), "{s}");
    assert!(s.ends_with(PLIST_FOOTER), "{s}");
    assert!(s.contains("<k>kCEK</k><i>4</i>"), "{s}");
}

#[test]
fn level_display() {
    let mut level = GDLevel::default();
    level.identity.name = "Name".into();
    level.identity.creator = "Creator".into();
    assert_eq!(
        level.to_string(),
        "\"Name\" (<No description>) by Creator; 0 Objects"
    );

    level.identity.description = Some("desc".into());
    level.add_object(default_block());
    assert_eq!(level.to_string(), "\"Name\" (desc) by Creator; 1 Objects");

    level.content.data = None;
    assert_eq!(level.to_string(), "\"Name\" (desc) by Creator; Empty");
}

#[test]
fn list_default_dict() {
    let d = crate::cclocallevels::gdlist::GDList::default().to_dict();
    assert_eq!(d.get("kCEK"), Some(&Value::from(12)));
    assert_eq!(d.get("k97"), Some(&Value::Dictionary(Dictionary::new())));
    assert!(!d.contains_key("k1"));
    assert!(!d.contains_key("k3"));
}

#[test]
fn list_dict_value_types() {
    let d = sample_list().to_dict();
    assert_eq!(d.get("k1"), Some(&Value::from(55)));
    assert_eq!(d.get("k2"), Some(&Value::from("gdlib test list")));
    assert_eq!(
        d.get("k3"),
        Some(&Value::from(b64_encode("List description")))
    );
    assert_eq!(d.get("k7"), Some(&Value::from(5)));
    assert_eq!(d.get("k21"), Some(&Value::from(2)));
    assert_eq!(d.get("k42"), Some(&Value::from(54)));
    assert_eq!(d.get("k96"), Some(&Value::from("1001,1002")));
    assert_eq!(d.get("k99"), Some(&Value::from(1_700_000_500)));
    assert_eq!(d.get("k500"), Some(&Value::from(5)));

    let levels = d.get("k97").unwrap().as_dictionary().unwrap();
    assert_eq!(levels.keys().collect::<Vec<_>>(), vec!["1001", "1002"]);
}

#[test]
fn cclocallevels_string() {
    let mut cc = CCLocalLevels {
        levels: vec![GDLevel::default()],
        binary_version: GDVersion::GD22082,
        lists: vec![],
    };
    let s = cc.serialise_to_string();
    assert!(s.starts_with(PLIST_HEADER));
    assert!(s.ends_with(PLIST_FOOTER));
    assert!(s.contains("<k>LLM_01</k>"));
    assert!(s.contains("<k>LLM_02</k><i>48</i>"));
    assert!(s.contains("<k>LLM_03</k>"));
    assert!(s.contains("<k>_isArr</k><t />"));
    assert!(s.contains("<k>k_0</k>"));
}

// ---------------- core::io / core ----------------

#[test]
fn stringify_xml_empty() {
    assert_eq!(stringify_xml(&Dictionary::new(), false), "<d />");
}

#[test]
fn stringify_xml_value_types() {
    let mut inner = Dictionary::new();
    inner.insert("x".into(), Value::from(1));
    let mut d = Dictionary::new();
    d.insert("s".into(), Value::from("str"));
    d.insert("i".into(), Value::from(-5));
    d.insert("t".into(), Value::Boolean(true));
    d.insert("f".into(), Value::Boolean(false));
    d.insert("r".into(), Value::Real(1.5));
    d.insert("d".into(), Value::Dictionary(inner));
    d.insert("e".into(), Value::Dictionary(Dictionary::new()));

    assert_eq!(
        stringify_xml(&d, true),
        "<dict><k>s</k><s>str</s><k>i</k><i>-5</i><k>t</k><t /><k>f</k><f /><k>r</k><r>1.5</r><k>d</k><d><k>x</k><i>1</i></d><k>e</k><d /></dict>"
    );
    assert!(stringify_xml(&d, false).starts_with("<d>"));
    assert!(stringify_xml(&d, false).ends_with("</d>"));
}

#[test]
fn stringify_xml_escapes_special_characters() {
    let mut d = Dictionary::new();
    d.insert("a&b".into(), Value::from("<tag> & \"quotes\""));
    let s = stringify_xml(&d, false);
    assert!(!s.contains("a&b"), "{s}");
    assert!(!s.contains("<tag>"), "{s}");
    assert!(s.contains("&amp;"), "{s}");
    assert!(s.contains("&lt;tag&gt;"), "{s}");
}

#[test]
fn proper_plist_tags_converts_every_tag() {
    let s = "<d><k>a</k><s>b</s><k>c</k><i>1</i><k>e</k><r>1.5</r><k>t</k><t /><k>t2</k><t/><k>f</k><f /><k>f2</k><f/><k>g</k><d /></d>";
    assert_eq!(
        proper_plist_tags(s.into()).unwrap(),
        "<dict><key>a</key><string>b</string><key>c</key><integer>1</integer><key>e</key><real>1.5</real><key>t</key><true /><key>t2</key><true/><key>f</key><false /><key>f2</key><false/><key>g</key><dict /></dict>"
    );
}

#[test]
fn proper_plist_tags_leaves_other_text_alone() {
    assert_eq!(
        proper_plist_tags("plain text <x>".into()).unwrap(),
        "plain text <x>"
    );
}

#[test]
fn b64_encode_is_url_safe() {
    assert_eq!(b64_encode([0xfb, 0xff]), "-_8=");
    assert_eq!(b64_encode(""), "");
    assert_eq!(b64_encode("hello"), "aGVsbG8=");
}

#[test]
fn encrypted_level_string_header() {
    let enc = encrypt_level_str("kA2,0;1,1,2,0,3,0;");
    assert!(enc.starts_with(b"H4sIAAAAAAAAC"));
    assert!(enc.iter().all(|c| c.is_ascii()));
}

#[test]
fn encrypted_level_string_is_valid_gzip() {
    use std::io::Read;
    let raw = "kS38,1_0;1,1,2,15,3,15;";
    let enc = encrypt_level_str(raw);
    let gzip = b64_decode(&enc).unwrap();

    let mut out = String::new();
    flate2::read::GzDecoder::new(&gzip[..])
        .read_to_string(&mut out)
        .unwrap();
    assert_eq!(out, raw);

    // gzip trailer: CRC32 (4 bytes) followed by ISIZE (4 bytes), both little endian
    let trailer = &gzip[gzip.len() - 8..];
    assert_eq!(&trailer[..4], crc32fast::hash(raw.as_bytes()).to_le_bytes());
    assert_eq!(&trailer[4..], (raw.len() as u32).to_le_bytes());
}

#[test]
fn encrypted_savefile_is_xored_level_string() {
    let raw = "<?xml version=\"1.0\"?>";
    let level = encrypt_level_str(raw);
    let save = encrypt_savefile_str(raw);
    assert_eq!(level.len(), save.len());
    assert!(level.iter().zip(&save).all(|(a, b)| a ^ 11 == *b));
}

#[test]
fn gjp2() {
    assert_eq!(
        generate_gjp2_hexdigest("password".into()),
        "b35ab22e8265f5904100b253e29511f129ca1ce8"
    );
    assert_eq!(
        generate_gjp2_hexdigest(String::new()),
        "97e14c941814de32c2a6ad640d242ac6cebd21b0"
    );
}
