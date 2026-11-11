//! Tests for parsing raw strings and plist dictionaries into structs, including malformed input.
//!
//! Parsers in this crate should never panic on bad input. Tests named `*_does_not_panic` only
//! check exactly that; a panic inside one of them is a bug.
use std::collections::HashMap;

use plist::{Dictionary, Value};
use smallvec::{SmallVec, smallvec};

use crate::{
    ccgamemanager::{
        CCGameManager,
        achievements::GDAchievement,
        structs::{
            GDPlatformerUI, GDSearchFilterFlags, GDSearchFilters, Resolution, TextureQuality,
        },
    },
    cclocallevels::{
        gdlevel::{
            CCLocalLevels, GDLevel, PLIST_FOOTER, PLIST_HEADER,
            enums::{
                DemonType, DifficultyRating, EpicRating, GDLevelType, GDListDifficulty, GDListType,
                Length, OfficialSong,
            },
            leveldata::{
                ColourString, DEFAULT_LEVEL_HEADERS, GDLevelData, GDLevelHeaderKey,
                GDLevelHeaderValType, GDLevelHeaderValue, GDLevelState, GuidelineColour,
                GuidelineString, PlayerColour, parse_objects, parse_raw_level_headers,
            },
            version::GDVersion,
        },
        gdlist::GDList,
        gdobj::{
            GDObject,
            defaults::{default_object, default_object_string},
            ids::properties as p,
            meta::GDObjAttributes,
            structs::*,
        },
    },
    core::{
        b64_decode, b64_encode,
        io::{decompress, decrypt, encrypt_level_str, encrypt_savefile_str},
        structs::KCEKValue,
    },
};

// ---------------- GDValue::from ----------------

#[test]
fn gdvalue_from_primitives() {
    use GDObjPropType as T;
    let cases: &[(T, &str, GDValue)] = &[
        (T::Int, "42", GDValue::Int(42)),
        (T::Int, "-2147483648", GDValue::Int(i32::MIN)),
        (T::Float, "1.5", GDValue::Float(1.5)),
        (T::Float, "-0.25", GDValue::Float(-0.25)),
        (T::Float, "3", GDValue::Float(3.0)),
        (T::Double, "0.1", GDValue::Double(0.1)),
        (T::Bool, "1", GDValue::Bool(true)),
        (T::Bool, "0", GDValue::Bool(false)),
        (T::Group, "9999", GDValue::Group(9999)),
        (T::Item, "12", GDValue::Item(12)),
        (T::Text, "SGVsbG8=", GDValue::String("SGVsbG8=".into())),
        (T::Unknown, "anything", GDValue::String("anything".into())),
        (T::Unknown, "", GDValue::String(String::new())),
    ];
    for (t, s, expected) in cases {
        assert_eq!(GDValue::from(*t, s), *expected, "{t:?} {s:?}");
    }
}

#[test]
fn gdvalue_from_lists() {
    assert_eq!(
        GDValue::from(GDObjPropType::GroupList, "1.2.3"),
        GDValue::GroupList(smallvec![1, 2, 3])
    );
    assert_eq!(
        GDValue::from(GDObjPropType::ProbabilitiesList, "1.10.2.20"),
        GDValue::ProbabilitiesList(smallvec![(1, 10), (2, 20)])
    );
    assert_eq!(
        GDValue::from(GDObjPropType::SpawnRemapsList, "5.6.-7.8"),
        GDValue::SpawnRemapsList(smallvec![(5, 6), (-7, 8)])
    );
    assert_eq!(
        GDValue::from(GDObjPropType::EventsList, "19.76.64"),
        GDValue::Events(vec![
            Event::BallSwitch,
            Event::FallSpeedLow,
            Event::CheckpointRespawn
        ])
    );
}

#[test]
fn gdvalue_from_empty_lists() {
    assert_eq!(
        GDValue::from(GDObjPropType::GroupList, ""),
        GDValue::GroupList(SmallVec::new())
    );
    assert_eq!(
        GDValue::from(GDObjPropType::EventsList, ""),
        GDValue::Events(vec![])
    );
    assert_eq!(
        GDValue::from(GDObjPropType::ProbabilitiesList, ""),
        GDValue::ProbabilitiesList(SmallVec::new())
    );
    assert_eq!(
        GDValue::from(GDObjPropType::SpawnRemapsList, ""),
        GDValue::SpawnRemapsList(SmallVec::new())
    );
}

#[test]
fn gdvalue_from_pair_list_with_odd_length_drops_last_item() {
    assert_eq!(
        GDValue::from(GDObjPropType::ProbabilitiesList, "1.10.2"),
        GDValue::ProbabilitiesList(smallvec![(1, 10)])
    );
}

#[test]
fn gdvalue_from_unknown_event_is_preserved() {
    assert_eq!(
        GDValue::from(GDObjPropType::EventsList, "999"),
        GDValue::Events(vec![Event::Unrecognized(999)])
    );
}

#[test]
fn gdvalue_from_enums() {
    use GDObjPropType as T;
    let cases: &[(T, &str, GDValue)] = &[
        (T::Toggle, "-1", GDValue::Toggle(Toggle::Off)),
        (T::Toggle, "1", GDValue::Toggle(Toggle::On)),
        (
            T::ColourChannel,
            "1000",
            GDValue::ColourChannel(ColourChannel::Background),
        ),
        (
            T::ColourChannel,
            "7",
            GDValue::ColourChannel(ColourChannel::Channel(7)),
        ),
        (T::Easing, "18", GDValue::Easing(MoveEasing::BackOut)),
        (T::Speed, "1", GDValue::Speed(Speed::X0Point5)),
        (T::Gamemode, "6", GDValue::Gamemode(Gamemode::Spider)),
        (T::ItemEditOperator, "4", GDValue::ItemEditOperator(Op::Div)),
        (
            T::ItemCompareOperator,
            "3",
            GDValue::ItemCompareOperator(CompareOp::Less),
        ),
        (T::BPMSpeed, "2", GDValue::BPMSpeed(BPMSpeed::Fast)),
        (
            T::ItemEditRoundMode,
            "2",
            GDValue::ItemEditRoundMode(RoundMode::Floor),
        ),
        (
            T::ItemEditSignMode,
            "1",
            GDValue::ItemEditSignMode(SignMode::Absolute),
        ),
        (
            T::TouchToggle,
            "1",
            GDValue::TouchToggle(TouchToggle::ToggleOn),
        ),
        (T::AxisOnlyMove, "1", GDValue::AxisOnlyMove(AxisOnlyMove::X)),
        (
            T::ArrowDirection,
            "3",
            GDValue::ArrowDirection(ArrowDirection::Left),
        ),
        (
            T::PlayerTarget,
            "1",
            GDValue::PlayerTarget(OptionalPlayerTarget::Player1),
        ),
        (
            T::TransitionMode,
            "1",
            GDValue::TransitionMode(TransitionMode::Enter),
        ),
        (
            T::UIReferencePos,
            "6",
            GDValue::UIReferencePos(UIReferencePos::YCenter),
        ),
        (
            T::CounterMode,
            "-2",
            GDValue::CounterMode(CounterMode::Points),
        ),
        (
            T::CounterMode,
            "-9",
            GDValue::CounterMode(CounterMode::Unrecognized(-9)),
        ),
        (T::ItemType, "2", GDValue::ItemType(ItemType::Timer)),
        (T::StopMode, "1", GDValue::StopMode(StopMode::Pause)),
        (T::ItemAlign, "1", GDValue::ItemAlign(ItemAlign::Left)),
        (
            T::MiddleGround,
            "2",
            GDValue::MiddleGround(MiddleGround::RockyMountains),
        ),
        (T::ExtraID2, "1", GDValue::ExtraID2(ExtraID2::P1)),
        (
            T::TeleportGravity,
            "2",
            GDValue::TeleportGravity(TeleportGravity::Flipped),
        ),
    ];
    for (t, s, expected) in cases {
        assert_eq!(GDValue::from(*t, s), *expected, "{t:?} {s:?}");
    }
}

#[test]
fn gdvalue_from_malformed_input_uses_defaults() {
    use GDObjPropType as T;
    // documented in `GDValue::from`: malformed input is replaced by the type's default value
    let cases: &[(T, &str, GDValue)] = &[
        (T::Int, "abc", GDValue::Int(0)),
        (T::Int, "", GDValue::Int(0)),
        (T::Int, "99999999999", GDValue::Int(0)),
        (T::Float, "not a float", GDValue::Float(0.0)),
        (T::Double, "", GDValue::Double(0.0)),
        (T::Group, "40000", GDValue::Group(0)),
        (T::Item, "x", GDValue::Item(0)),
        (T::Speed, "99", GDValue::Speed(Speed::X1)),
        (T::Gamemode, "-1", GDValue::Gamemode(Gamemode::Cube)),
        (T::Easing, "19", GDValue::Easing(MoveEasing::None)),
        (T::Toggle, "2", GDValue::Toggle(Toggle::Unset)),
        (
            T::ItemEditOperator,
            "abc",
            GDValue::ItemEditOperator(Op::Set),
        ),
        (
            T::ArrowDirection,
            "0",
            GDValue::ArrowDirection(ArrowDirection::Right),
        ),
        (T::ItemType, "", GDValue::ItemType(ItemType::Counter)),
        (
            T::TeleportGravity,
            "7",
            GDValue::TeleportGravity(TeleportGravity::Unset),
        ),
    ];
    for (t, s, expected) in cases {
        assert_eq!(GDValue::from(*t, s), *expected, "{t:?} {s:?}");
    }
}

#[test]
fn gdvalue_from_malformed_list_items_do_not_panic() {
    let _ = GDValue::from(GDObjPropType::GroupList, "1..2.abc.");
    let _ = GDValue::from(GDObjPropType::ProbabilitiesList, "a.b.c.d");
    let _ = GDValue::from(GDObjPropType::SpawnRemapsList, "....");
    let _ = GDValue::from(GDObjPropType::EventsList, "x.y");
}

#[test]
fn gdvalue_to_easing() {
    assert_eq!(
        GDValue::Easing(MoveEasing::SineIn).to_easing(1.5),
        Some(Easing::from(MoveEasing::SineIn, 1.5))
    );
    assert_eq!(GDValue::Int(14).to_easing(1.5), None);
}

#[test]
fn gdvalue_helper_constructors() {
    assert_eq!(
        GDValue::colour_channel("1005"),
        GDValue::ColourChannel(ColourChannel::P1)
    );
    assert_eq!(
        GDValue::colour_channel("garbage"),
        GDValue::ColourChannel(ColourChannel::Channel(0))
    );
    assert_eq!(GDValue::zlayer("-3"), GDValue::ZLayer(ZLayer::B4));
    assert_eq!(GDValue::zlayer("bad"), GDValue::ZLayer(ZLayer::Default));
    assert_eq!(
        GDValue::from_group_list(&[3, 4]),
        GDValue::GroupList(smallvec![3, 4])
    );
    assert_eq!(
        GDValue::from_prob_list(vec![(1, 2)]),
        GDValue::ProbabilitiesList(smallvec![(1, 2)])
    );
    assert_eq!(
        GDValue::from_spawn_remaps(vec![(1, 2)]),
        GDValue::SpawnRemapsList(smallvec![(1, 2)])
    );
}

// ---------------- GDObject::parse_str ----------------

#[test]
fn object_minimal() {
    let obj = GDObject::parse_str("1,8,2,15,3,30;");
    assert_eq!(obj.id, 8);
    assert_eq!(obj.config.pos, (15.0, 30.0));
    assert!(obj.get_properties().is_empty());
}

#[test]
fn object_without_trailing_semicolon() {
    assert_eq!(
        GDObject::parse_str("1,8,2,15,3,30"),
        GDObject::parse_str("1,8,2,15,3,30;")
    );
}

#[test]
fn object_with_many_trailing_semicolons() {
    assert_eq!(
        GDObject::parse_str("1,8,2,15,3,30;;;"),
        GDObject::parse_str("1,8,2,15,3,30;")
    );
}

#[test]
fn object_config_fields() {
    let obj = GDObject::parse_str(
        "1,1,2,15.5,3,-30,6,45,11,1,62,1,87,1,57,3.1.2,128,2,129,0.5,20,4,61,5,21,1000,22,25,24,-1,25,-7,343,8,446,9,534,10,274,2;",
    );
    let c = &obj.config;
    assert_eq!(c.pos, (15.5, -30.0));
    assert_eq!(c.angle, 45.0);
    assert!(c.trigger_cfg.touchable);
    assert!(c.trigger_cfg.spawnable);
    assert!(c.trigger_cfg.multitriggerable);
    assert_eq!(c.groups, vec![1, 2, 3]);
    assert_eq!(c.pink_groups, vec![2]);
    assert_eq!(c.scale, (2.0, 0.5));
    assert_eq!(c.editor_layers, (4, 5));
    assert_eq!(
        c.colour_channels,
        (ColourChannel::Background, ColourChannel::Channel(25))
    );
    assert_eq!(c.z_layer, ZLayer::B3);
    assert_eq!(c.z_order, -7);
    assert_eq!(c.enter_effect_channel, 8);
    assert_eq!(c.material_id, 9);
    assert_eq!(c.control_id, 10);
    assert!(
        obj.get_properties().is_empty(),
        "{:?}",
        obj.get_properties()
    );
}

#[test]
fn object_every_attribute() {
    let ids = [
        (p::DONT_FADE, GDObjAttributes::dont_fade),
        (p::DONT_ENTER, GDObjAttributes::dont_enter),
        (p::NO_OBJECT_EFFECTS, GDObjAttributes::no_effects),
        (p::IS_GROUP_PARENT, GDObjAttributes::is_group_parent),
        (p::IS_AREA_PARENT, GDObjAttributes::is_area_parent),
        (p::DONT_BOOST_X, GDObjAttributes::dont_boost_x),
        (p::DONT_BOOST_Y, GDObjAttributes::dont_boost_y),
        (p::IS_HIGH_DETAIL, GDObjAttributes::high_detail),
        (p::NO_TOUCH, GDObjAttributes::no_touch),
        (p::PASSABLE, GDObjAttributes::passable),
        (p::HIDDEN, GDObjAttributes::hidden),
        (p::NONSTICK_X, GDObjAttributes::non_stick_x),
        (p::NONSTICK_Y, GDObjAttributes::non_stick_y),
        (p::EXTRA_STICKY, GDObjAttributes::extra_sticky),
        (
            p::HAS_EXTENDED_COLLISION,
            GDObjAttributes::extended_collision,
        ),
        (p::IS_ICE_BLOCK, GDObjAttributes::is_ice_block),
        (p::GRIP_SLOPE, GDObjAttributes::grip_slope),
        (p::NO_GLOW, GDObjAttributes::no_glow),
        (p::NO_PARTICLES, GDObjAttributes::no_particles),
        (p::SCALE_STICK, GDObjAttributes::scale_stick),
        (p::NO_AUDIO_SCALE, GDObjAttributes::no_audio_scale),
        (p::SINGLE_PLAYER_TOUCH, GDObjAttributes::single_ptouch),
        (p::CENTER_EFFECT, GDObjAttributes::center_effect),
        (p::REVERSES_GAMEPLAY, GDObjAttributes::reverse),
    ];
    for (id, flag) in ids {
        let obj = GDObject::parse_str(format!("1,1,2,0,3,0,{id},1;"));
        assert_eq!(obj.config.attributes, flag, "property {id}");
        assert!(obj.get_properties().is_empty(), "property {id}");

        let obj = GDObject::parse_str(format!("1,1,2,0,3,0,{id},0;"));
        assert_eq!(
            obj.config.attributes,
            GDObjAttributes::empty(),
            "property {id}"
        );
    }
}

#[test]
fn object_group_formats() {
    assert_eq!(GDObject::parse_str("1,1,57,5;").config.groups, vec![5]);
    assert_eq!(
        GDObject::parse_str("1,1,57,\"1.2\";").config.groups,
        vec![1, 2]
    );
    // unsorted and duplicate groups are sorted and deduplicated
    assert_eq!(
        GDObject::parse_str("1,1,57,9.3.3.1;").config.groups,
        vec![1, 3, 9]
    );
    // invalid group entries are ignored
    assert_eq!(
        GDObject::parse_str("1,1,57,1.abc.2;").config.groups,
        vec![1, 2]
    );
    assert!(GDObject::parse_str("1,1,57,;").config.groups.is_empty());
}

#[test]
fn object_pink_groups_do_not_leak_into_groups() {
    let obj = GDObject::parse_str("1,1,57,1.2,274,7;");
    assert_eq!(obj.config.groups, vec![1, 2]);
    assert_eq!(obj.config.pink_groups, vec![7]);
}

#[test]
fn object_extra_properties_use_property_table_types() {
    let obj = GDObject::parse_str("1,901,2,0,3,0,51,12,10,0.5,30,18,85,1.5,36,1;");
    assert_eq!(obj.get_property(51), Some(GDValue::Group(12)));
    assert_eq!(obj.get_property(10), Some(GDValue::Float(0.5)));
    assert_eq!(
        obj.get_property(30),
        Some(GDValue::Easing(MoveEasing::BackOut))
    );
    assert_eq!(obj.get_property(85), Some(GDValue::Float(1.5)));
    assert_eq!(obj.get_property(36), Some(GDValue::Bool(true)));
}

#[test]
fn object_properties_are_sorted() {
    let obj = GDObject::parse_str("1,901,500,1,36,1,51,3,10,2;");
    let ids = obj
        .get_properties()
        .iter()
        .map(|(id, _)| *id)
        .collect::<Vec<_>>();
    let mut sorted = ids.clone();
    sorted.sort();
    assert_eq!(ids, sorted);
}

#[test]
fn object_duplicate_property_last_value_wins() {
    let obj = GDObject::parse_str("1,901,51,1,51,2;");
    assert_eq!(obj.get_property(51), Some(GDValue::Group(2)));
    assert_eq!(obj.get_properties().len(), 1);
}

#[test]
fn object_untyped_property_is_kept_as_string() {
    let obj = GDObject::parse_str("1,1,60000,hello;");
    assert_eq!(
        obj.get_property(60000),
        Some(GDValue::String("hello".into()))
    );
}

#[test]
fn object_ka_property() {
    let obj = GDObject::parse_str("1,31,kA10,0,kA11,,kA27,1;");
    assert_eq!(obj.id, 31);
    assert!(obj.has_property(10_010));
    assert!(obj.has_property(10_011));
    assert!(obj.has_property(10_027));
}

#[test]
fn object_odd_number_of_tokens_ignores_dangling_key() {
    let obj = GDObject::parse_str("1,5,2,15,3");
    assert_eq!(obj.id, 5);
    assert_eq!(obj.config.pos, (15.0, 0.0));
}

#[test]
fn object_malformed_values_use_defaults() {
    let obj = GDObject::parse_str("1,abc,2,x,3,y,128,z;");
    assert_eq!(obj.config.pos, (0.0, 0.0));
    assert_eq!(obj.config.scale.0, 1.0);
}

#[test]
fn object_non_numeric_single_char_key_does_not_panic() {
    let _ = GDObject::parse_str("1,1,a,5;");
}

#[test]
fn object_non_ascii_key_does_not_panic() {
    let _ = GDObject::parse_str("1,1,é,5;");
    let _ = GDObject::parse_str("1,1,kAé,5;");
}

#[test]
fn object_huge_ka_key_does_not_panic() {
    let _ = GDObject::parse_str("1,1,kA60000,5;");
}

#[test]
fn object_unparseable_keys_are_kept_apart() {
    // two different unknown keys must not collapse into one property
    let obj = GDObject::parse_str("1,1,foo,1,bar,2;");
    assert_eq!(obj.get_properties().len(), 2, "{:?}", obj.get_properties());
}

#[test]
fn object_empty_string() {
    let obj = GDObject::parse_str("");
    assert!(obj.get_properties().is_empty());
}

#[test]
fn object_large_and_negative_numbers() {
    let obj = GDObject::parse_str("1,1,2,-1e6,3,123456.789,25,-2147483648;");
    assert_eq!(obj.config.pos.0, -1e6);
    assert_eq!(obj.config.pos.1, 123456.789);
    assert_eq!(obj.config.z_order, i32::MIN);
}

#[test]
fn parse_objects_skips_empty_segments() {
    let objs = parse_objects("1,1,2,0,3,0;;1,2,2,30,3,0;;");
    assert_eq!(objs.len(), 2);
    assert_eq!(objs[0].id, 1);
    assert_eq!(objs[1].id, 2);
    assert!(parse_objects("").is_empty());
    assert!(parse_objects(";;;").is_empty());
}

// ---------------- default objects ----------------

#[test]
fn default_object_strings_end_with_semicolon() {
    for id in 1..=4600 {
        let s = default_object_string(id);
        assert!(s.ends_with(';'), "default string for {id}: {s}");
    }
}

#[test]
fn default_objects_have_their_id() {
    for id in 1..=4600 {
        assert_eq!(default_object(id).id, id);
    }
}

#[test]
fn default_object_matches_default_object_string() {
    for id in 1..=4600 {
        assert_eq!(
            default_object(id),
            GDObject::parse_str(default_object_string(id)),
            "object {id}"
        );
    }
}

#[test]
fn default_object_unknown_id() {
    assert_eq!(default_object_string(-5), "1,-5,2,0,3,0;");
    assert_eq!(default_object(-5).id, -5);
}

// ---------------- sub-structs ----------------

#[test]
fn hsv_colour() {
    assert_eq!(
        HSVColour::parse("-30a0.5a1.25a1a0"),
        Some(HSVColour {
            hue_shift: -30,
            saturation_mult: 0.5,
            brightness_mult: 1.25,
            static_sat_scalar: true,
            static_bright_scalar: false,
        })
    );
    assert_eq!(
        HSVColour::parse("0a1a1a0a1"),
        Some(HSVColour {
            hue_shift: 0,
            saturation_mult: 1.0,
            brightness_mult: 1.0,
            static_sat_scalar: false,
            static_bright_scalar: true,
        })
    );
}

#[test]
fn hsv_colour_invalid() {
    for s in ["", "1a2", "1a2a3a4", "xa1a1a0a0", "0a1a1axa0", "0a1a1a0a"] {
        assert_eq!(HSVColour::parse(s), None, "{s:?}");
    }
}

#[test]
fn colour_from_hex() {
    assert_eq!(
        Colour::from_hex("#FF8000").unwrap(),
        Colour::from_rgb((255, 128, 0))
    );
    assert_eq!(
        Colour::from_hex("#00ff7f").unwrap(),
        Colour::from_rgb((0, 255, 127))
    );
    assert_eq!(Colour::from_hex("#000000").unwrap(), Colour::default());
}

#[test]
fn colour_from_hex_invalid() {
    for s in [
        "FF8000",
        "#FF80",
        "#FF800000",
        "#GGGGGG",
        "",
        "#",
        "#-12345",
        "#+12345",
    ] {
        assert!(Colour::from_hex(s).is_none(), "{s:?}");
    }
}

#[test]
fn easing_from_object() {
    let obj = GDObject::parse_str("1,901,30,4,85,0.5;");
    assert_eq!(
        Easing::from_object(&obj),
        Easing::from(MoveEasing::ElasticInOut, 0.5)
    );
    assert_eq!(
        Easing::from_object(&GDObject::parse_str("1,901;")),
        Easing::none()
    );
}

// ---------------- level header ----------------

#[test]
fn header_key_parse() {
    assert_eq!(GDLevelHeaderKey::parse("kA13"), GDLevelHeaderKey::A(13));
    assert_eq!(GDLevelHeaderKey::parse("kS38"), GDLevelHeaderKey::S(38));
    assert_eq!(GDLevelHeaderKey::parse("kA0"), GDLevelHeaderKey::A(0));
    for s in ["kA", "kS", "kAx", "kB5", "", "A13", "kA-1", "kA70000"] {
        assert_eq!(
            GDLevelHeaderKey::parse(s),
            GDLevelHeaderKey::Unknown(s.into()),
            "{s:?}"
        );
    }
}

#[test]
fn header_key_parse_non_ascii_does_not_panic() {
    let _ = GDLevelHeaderKey::parse("kAé");
    let _ = GDLevelHeaderKey::parse("é");
}

#[test]
fn header_key_ids() {
    assert_eq!(GDLevelHeaderKey::A(13).to_id(), Some(13));
    assert_eq!(GDLevelHeaderKey::S(38).to_id(), Some(1038));
    assert_eq!(GDLevelHeaderKey::Unknown("x".into()).to_id(), None);
    assert_eq!(GDLevelHeaderKey::from_id(13), GDLevelHeaderKey::A(13));
    assert_eq!(GDLevelHeaderKey::from_id(1038), GDLevelHeaderKey::S(38));
}

#[test]
fn header_key_large_s_key_does_not_overflow() {
    let _ = GDLevelHeaderKey::S(u16::MAX).to_id();
}

#[test]
fn header_values() {
    use GDLevelHeaderValType as T;
    let cases: &[(T, &str, GDLevelHeaderValue)] = &[
        (T::Int, "-4", GDLevelHeaderValue::Int(-4)),
        (T::Float, "0.5", GDLevelHeaderValue::Float(0.5)),
        (T::Bool, "1", GDLevelHeaderValue::Bool(true)),
        (T::Bool, "0", GDLevelHeaderValue::Bool(false)),
        (
            T::Gamemode,
            "7",
            GDLevelHeaderValue::Gamemode(Gamemode::Swing),
        ),
        (T::Speed, "4", GDLevelHeaderValue::Speed(Speed::X4)),
    ];
    for (t, s, expected) in cases {
        assert_eq!(
            GDLevelHeaderValue::parse(s, *t).as_ref(),
            Some(expected),
            "{t:?} {s:?}"
        );
    }
}

#[test]
fn header_values_invalid() {
    use GDLevelHeaderValType as T;
    let cases: &[(T, &str)] = &[
        (T::Int, "abc"),
        (T::Int, ""),
        (T::Float, "x"),
        (T::Bool, "yes"),
        (T::Gamemode, "8"),
        (T::Speed, "-1"),
    ];
    for (t, s) in cases {
        assert_eq!(GDLevelHeaderValue::parse(s, *t), None, "{t:?} {s:?}");
    }
}

#[test]
fn header_colour_string_value() {
    let v = GDLevelHeaderValue::parse(
        "1_40_2_125_3_255_6_1000|1_0_2_102_3_255_6_1001|",
        GDLevelHeaderValType::ColourString,
    )
    .unwrap();
    let GDLevelHeaderValue::ColourString(cs) = v else {
        panic!("not a colour string: {v:?}");
    };
    assert_eq!(cs.len(), 2);
    assert_eq!(cs[0].from, Colour::from_rgb((40, 125, 255)));
    assert_eq!(cs[0].colour_ch_idx, 1000);
    assert_eq!(cs[1].colour_ch_idx, 1001);
}

#[test]
fn header_colour_string_without_trailing_pipe_keeps_last_colour() {
    let v = GDLevelHeaderValue::parse(
        "1_40_2_125_3_255_6_1000|1_0_2_102_3_255_6_1001",
        GDLevelHeaderValType::ColourString,
    )
    .unwrap();
    let GDLevelHeaderValue::ColourString(cs) = v else {
        panic!("not a colour string: {v:?}");
    };
    assert_eq!(cs.len(), 2);
}

#[test]
fn header_empty_colour_string() {
    assert_eq!(
        GDLevelHeaderValue::parse("", GDLevelHeaderValType::ColourString),
        Some(GDLevelHeaderValue::ColourString(vec![]))
    );
}

#[test]
fn colour_string_segment() {
    let cs = ColourString::parse(
        "1_40_2_125_3_255_4_2_5_1_6_1000_7_0.5_8_1_9_3_10_10a1a0.5a0a1_11_1_12_2_13_3_14_0.75_15_0.25_16_2_17_1_18_1",
    )
    .unwrap();
    assert_eq!(cs.from, Colour::from_rgb((40, 125, 255)));
    assert_eq!(cs.player_colour, PlayerColour::Second);
    assert!(cs.blending);
    assert_eq!(cs.colour_ch_idx, 1000);
    assert_eq!(cs.from_opacity, 0.5);
    assert!(cs.opacity_toggled);
    assert_eq!(cs.inherited_col_ch_idx, 3);
    assert_eq!(cs.copied_hsv, HSVColour::parse("10a1a0.5a0a1"));
    assert_eq!(cs.to, Colour::from_rgb((1, 2, 3)));
    assert_eq!(cs.deltatime, 0.75);
    assert_eq!(cs.to_opacity, 0.25);
    assert_eq!(cs.duration, 2.0);
    assert!(cs.copy_opacity);
    assert!(cs.unknown_property18);
}

#[test]
fn colour_string_player_colours() {
    let pc = |v: &str| {
        ColourString::parse(&format!("4_{v}"))
            .unwrap()
            .player_colour
    };
    assert_eq!(pc("-1"), PlayerColour::None);
    assert_eq!(pc("0"), PlayerColour::None);
    assert_eq!(pc("1"), PlayerColour::First);
    assert_eq!(pc("2"), PlayerColour::Second);
}

#[test]
fn colour_string_invalid() {
    for s in ["x_1", "1_x", "10_notanhsv", "7_abc"] {
        assert_eq!(ColourString::parse(s), None, "{s:?}");
    }
}

#[test]
fn colour_string_out_of_range_rgb_does_not_wrap() {
    // a corrupted value such as 256 must not silently wrap around to 0.
    // Rejecting the segment or clamping the value are both fine.
    if let Some(cs) = ColourString::parse("1_256_2_300_3_-1") {
        assert_ne!(cs.from.red, 0);
        assert_ne!(cs.from.green, 44);
        assert_ne!(cs.from.blue, 255);
    }
}

#[test]
fn colour_string_unknown_keys_are_ignored() {
    assert_eq!(
        ColourString::parse("1_10_99_5"),
        Some(ColourString {
            from: Colour::from_rgb((10, 0, 0)),
            ..Default::default()
        })
    );
}

#[test]
fn guideline_string() {
    // kA14 is a flat list of `time~colour~` pairs
    assert_eq!(
        GuidelineString::parse("1.5~0.9~2~1~3.25~0~"),
        Some(GuidelineString {
            guidelines: vec![
                ("1.5".into(), GuidelineColour::Yellow),
                ("2".into(), GuidelineColour::Green),
                ("3.25".into(), GuidelineColour::Orange),
            ]
        })
    );
}

#[test]
fn guideline_string_empty() {
    assert_eq!(
        GuidelineString::parse(""),
        Some(GuidelineString { guidelines: vec![] })
    );
}

#[test]
fn guideline_colours() {
    assert_eq!(GuidelineColour::from_f32(0.9), GuidelineColour::Yellow);
    assert_eq!(GuidelineColour::from_f32(1.0), GuidelineColour::Green);
    assert_eq!(GuidelineColour::from_f32(0.0), GuidelineColour::Orange);
    assert_eq!(GuidelineColour::from_f32(0.8), GuidelineColour::Orange);
    assert_eq!(GuidelineColour::from_f32(0.5), GuidelineColour::Transparent);
    assert_eq!(
        GuidelineColour::from_f32(-1.0),
        GuidelineColour::Transparent
    );
}

#[test]
fn default_level_headers() {
    let h = parse_raw_level_headers(DEFAULT_LEVEL_HEADERS);
    let GDLevelHeaderValue::ColourString(colours) = &h[&GDLevelHeaderKey::S(38)] else {
        panic!("kS38 is not a colour string");
    };
    assert_eq!(colours.len(), 8);
    assert_eq!(
        colours.iter().map(|c| c.colour_ch_idx).collect::<Vec<_>>(),
        vec![1000, 1001, 1009, 1002, 1013, 1014, 1005, 1006]
    );
    assert_eq!(h[&GDLevelHeaderKey::A(13)], GDLevelHeaderValue::Float(0.0));
    assert_eq!(
        h[&GDLevelHeaderKey::A(2)],
        GDLevelHeaderValue::Gamemode(Gamemode::Cube)
    );
    assert_eq!(
        h[&GDLevelHeaderKey::A(4)],
        GDLevelHeaderValue::Speed(Speed::X1)
    );
    assert_eq!(
        h[&GDLevelHeaderKey::A(14)],
        GDLevelHeaderValue::GuidelineString(GuidelineString { guidelines: vec![] })
    );
    assert_eq!(h[&GDLevelHeaderKey::A(27)], GDLevelHeaderValue::Bool(true));
    assert_eq!(h[&GDLevelHeaderKey::S(39)], GDLevelHeaderValue::Int(0));
    // every key in the default header string is kept
    assert_eq!(h.len(), DEFAULT_LEVEL_HEADERS.split(',').count() / 2);
}

#[test]
fn level_headers_unknown_key_is_kept() {
    let h = parse_raw_level_headers("kA2,1,kZZ,5,kA999,3");
    assert_eq!(
        h.get(&GDLevelHeaderKey::Unknown("kZZ".into())),
        Some(&GDLevelHeaderValue::Int(5))
    );
    assert_eq!(
        h.get(&GDLevelHeaderKey::A(999)),
        Some(&GDLevelHeaderValue::Int(3))
    );
}

#[test]
fn level_headers_dangling_key() {
    let h = parse_raw_level_headers("kA2,1,kA3");
    assert_eq!(h.len(), 1);
}

#[test]
fn level_headers_empty() {
    assert!(parse_raw_level_headers("").is_empty());
}

#[test]
fn level_data_parse_invalid() {
    assert_eq!(GDLevelData::parse(""), None);
    assert_eq!(GDLevelData::parse("not encrypted data"), None);
}

#[test]
fn level_data_parse_headers_only() {
    let data = GDLevelData::parse(String::from_utf8(encrypt_level_str("kA2,1;")).unwrap()).unwrap();
    assert!(data.objects.is_empty());
    assert_eq!(
        data.headers.get(&GDLevelHeaderKey::A(2)),
        Some(&GDLevelHeaderValue::Gamemode(Gamemode::Ship))
    );
}

#[test]
fn level_data_parse_objects() {
    let enc = String::from_utf8(encrypt_level_str("kA2,0;1,1,2,15,3,15;1,8,2,45,3,15;")).unwrap();
    let data = GDLevelData::parse(enc).unwrap();
    assert_eq!(data.objects.len(), 2);
    assert_eq!(data.objects[1].id, 8);
}

// ---------------- GDLevel::from_dict ----------------

fn level_dict(entries: &[(&str, Value)]) -> Dictionary {
    entries
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect()
}

#[test]
fn level_empty_dict() {
    let level = GDLevel::from_dict(&Dictionary::new()).unwrap();
    assert_eq!(level.meta.kcek, KCEKValue::GJGameLevel);
}

#[test]
fn level_fields() {
    let d = level_dict(&[
        ("kCEK", Value::from(4)),
        ("k1", Value::from(55)),
        ("k2", Value::from("name")),
        ("k3", Value::from(b64_encode("description"))),
        ("k5", Value::from("creator")),
        ("k8", Value::from(6)),
        ("k9", Value::from(30)),
        ("k14", Value::Boolean(true)),
        ("k18", Value::from(400)),
        ("k21", Value::from(2)),
        ("k23", Value::from(5)),
        ("k34", Value::from("replay")),
        ("k45", Value::from(123456)),
        ("k50", Value::from(48)),
        ("k75", Value::from(3)),
        ("k76", Value::from(6)),
        ("k88", Value::from("1,2,3")),
        ("k104", Value::from("4,5")),
        ("k105", Value::from("6")),
        ("kI1", Value::Real(12.5)),
        ("kI2", Value::from(-3)),
        ("kI3", Value::Real(0.5)),
        ("kI4", Value::from(2)),
        ("kI5", Value::from(1)),
        (
            "kI6",
            Value::Dictionary(level_dict(&[
                ("0", Value::from("1")),
                ("3", Value::from("4")),
            ])),
        ),
        ("kI7", Value::Real(2.0)),
    ]);
    let level = GDLevel::from_dict(&d).unwrap();

    assert_eq!(level.identity.id, 55);
    assert_eq!(level.identity.name, "name");
    assert_eq!(level.identity.description.as_deref(), Some("description"));
    assert_eq!(level.identity.creator, "creator");
    assert_eq!(level.content.official_song_id, Some(OfficialSong::Jumper));
    assert_eq!(level.ratings.rating, DifficultyRating::Hard);
    assert!(level.flags.verified);
    assert_eq!(level.player_stats.attempts, 400);
    assert_eq!(level.identity.level_type, GDLevelType::Local);
    assert_eq!(level.content.length, Length::Platformer);
    assert_eq!(level.integrity.replay_data.as_deref(), Some(&b"replay"[..]));
    assert_eq!(level.content.custom_song_id, Some(123456));
    assert_eq!(level.identity.binary_version, GDVersion::GD22082);
    assert_eq!(level.ratings.epic_rating, EpicRating::Mythic);
    assert_eq!(level.ratings.demon_type, Some(DemonType::ExtremeDemon));
    assert_eq!(level.player_stats.progress_diffs, vec![1, 2, 3]);
    assert_eq!(level.content.song_list, vec![4, 5]);
    assert_eq!(level.content.sfx_list, Some(vec![6]));
    assert_eq!(level.editor_state.camera_x, 12.5);
    assert_eq!(level.editor_state.camera_y, -3.0);
    assert_eq!(level.editor_state.camera_zoom, 0.5);
    assert_eq!(level.editor_state.build_tab_page, 2);
    assert_eq!(level.editor_state.build_tab, 1);
    assert_eq!(
        level.editor_state.build_tab_pages,
        [(0i32, 1i32), (3, 4)]
            .iter()
            .map(|&(k, v)| (k, v))
            .collect::<HashMap<_, _>>()
    );
    assert_eq!(level.editor_state.editor_layer, 2);
    assert!(matches!(
        level.content.data,
        Some(GDLevelState::Decrypted(_))
    ));
}

#[test]
fn level_data_key_is_kept_encrypted() {
    let d = level_dict(&[("k4", Value::from("H4sIAAAAAAAAC"))]);
    let level = GDLevel::from_dict(&d).unwrap();
    assert!(matches!(
        level.content.data,
        Some(GDLevelState::Encrypted(_))
    ));
}

#[test]
fn level_unknown_enum_values_are_preserved() {
    let d = level_dict(&[
        ("k8", Value::from(500)),
        ("k9", Value::from(15)),
        ("k21", Value::from(9)),
        ("k23", Value::from(10)),
        ("k75", Value::from(8)),
        ("k76", Value::from(2)),
        ("k50", Value::from(9999)),
    ]);
    let level = GDLevel::from_dict(&d).unwrap();
    assert_eq!(
        level.content.official_song_id,
        Some(OfficialSong::Unrecognized(500))
    );
    assert_eq!(level.ratings.rating, DifficultyRating::Unrecognized(15));
    assert_eq!(level.identity.level_type, GDLevelType::Unrecognized(9));
    assert_eq!(level.content.length, Length::Unrecognized(10));
    assert_eq!(level.ratings.epic_rating, EpicRating::Unrecognized(8));
    assert_eq!(level.ratings.demon_type, Some(DemonType::Unrecognized(2)));
    assert_eq!(level.identity.binary_version, GDVersion::Unknown(9999));
}

#[test]
fn level_unknown_keys_are_kept() {
    let d = level_dict(&[
        ("k999", Value::from(1)),
        ("kI99", Value::from(2)),
        ("kX", Value::from("x")),
        ("notAKey", Value::Boolean(true)),
    ]);
    let level = GDLevel::from_dict(&d).unwrap();
    assert_eq!(level.unknowns.len(), 4);
    assert_eq!(level.unknowns["k999"], Value::from(1));
    assert_eq!(level.unknowns["kI99"], Value::from(2));
    assert_eq!(level.unknowns["kX"], Value::from("x"));
    assert_eq!(level.unknowns["notAKey"], Value::Boolean(true));
}

#[test]
fn level_wrongly_typed_value_returns_none() {
    for (k, v) in [
        ("k1", Value::from("not an int")),
        ("k2", Value::from(5)),
        ("k14", Value::from("true")),
        ("kI1", Value::from("1.5")),
        ("kCEK", Value::from("4")),
    ] {
        assert!(
            GDLevel::from_dict(&level_dict(&[(k, v.clone())])).is_some(),
            "{k} = {v:?}"
        );
    }
}

#[test]
fn level_empty_csv_is_an_empty_list() {
    let d = level_dict(&[("k104", Value::from("")), ("k88", Value::from(""))]);
    let level = GDLevel::from_dict(&d).expect("empty CSV should not fail the whole level");
    assert!(level.content.song_list.is_empty());
    assert!(level.player_stats.progress_diffs.is_empty());
}

#[test]
fn level_csv_with_non_string_value_does_not_panic() {
    let _ = GDLevel::from_dict(&level_dict(&[("k104", Value::from(5))]));
}

#[test]
fn level_invalid_build_tab_index_does_not_panic() {
    let pages = Value::Dictionary(level_dict(&[("abc", Value::from("1"))]));
    let _ = GDLevel::from_dict(&level_dict(&[("kI6", pages)]));
}

#[test]
fn level_non_string_build_tab_page_does_not_panic() {
    let pages = Value::Dictionary(level_dict(&[("0", Value::from(1))]));
    let _ = GDLevel::from_dict(&level_dict(&[("kI6", pages)]));
}

#[test]
fn level_huge_build_tab_index_does_not_panic() {
    let pages = Value::Dictionary(level_dict(&[(&usize::MAX.to_string(), Value::from("1"))]));
    let _ = GDLevel::from_dict(&level_dict(&[("kI6", pages)]));
}

#[test]
fn level_description_with_invalid_utf8_does_not_panic() {
    let d = level_dict(&[("k3", Value::from(b64_encode([0xff, 0xfe, 0x00])))]);
    let _ = GDLevel::from_dict(&d);
}

#[test]
fn level_description_with_invalid_base64_returns_none() {
    let d = level_dict(&[("k3", Value::from("!!! not base64 !!!"))]);
    assert!(GDLevel::from_dict(&d).is_none());
}

#[test]
fn level_from_plist_string() {
    let s = format!(
        "{PLIST_HEADER}<dict><k>kCEK</k><i>4</i><k>k2</k><s>From plist</s><k>k13</k><t /></dict>{PLIST_FOOTER}"
    );
    let level = GDLevel::from_plist_string(s).unwrap();
    assert_eq!(level.identity.name, "From plist");
    assert!(level.flags.is_editable);
}

#[test]
fn level_from_plist_string_invalid_xml() {
    assert!(GDLevel::from_plist_string("not xml at all").is_err());
}

#[test]
fn level_from_plist_string_non_dict_root_does_not_panic() {
    let s = format!("{PLIST_HEADER}<s>hello</s>{PLIST_FOOTER}");
    assert!(GDLevel::from_plist_string(s).is_err());
}

// ---------------- GDList ----------------

#[test]
fn list_fields() {
    let mut level = Dictionary::new();
    level.insert("k1".into(), Value::from(1001));
    level.insert("k2".into(), Value::from("level in list"));
    let mut levels = Dictionary::new();
    levels.insert("1001".into(), Value::Dictionary(level));

    let d = level_dict(&[
        ("kCEK", Value::from(12)),
        ("k1", Value::from(55)),
        ("k2", Value::from("list")),
        ("k3", Value::from(b64_encode("desc"))),
        ("k5", Value::from("me")),
        ("k7", Value::from(-1)),
        ("k15", Value::Boolean(true)),
        ("k21", Value::from(4)),
        ("k42", Value::from(7)),
        ("k96", Value::from("1001")),
        ("k97", Value::Dictionary(levels)),
        ("k99", Value::from(10)),
        ("k114", Value::from(1)),
    ]);
    let list = GDList::from_dictionary(&d).unwrap();
    assert_eq!(list.kcek, KCEKValue::GJLevelList);
    assert_eq!(list.id, 55);
    assert_eq!(list.name, "list");
    assert_eq!(list.description, "desc");
    assert_eq!(list.creator, "me");
    assert_eq!(list.difficulty, GDListDifficulty::NA);
    assert!(list.is_uploaded);
    assert_eq!(list.list_type, GDListType::Online);
    assert_eq!(list.original, Some(7));
    assert_eq!(list.level_ids, vec![1001]);
    assert_eq!(list.levels.len(), 1);
    assert_eq!(list.levels[0].identity.name, "level in list");
    assert_eq!(list.update_time, Some(10));
    assert_eq!(list.required_levels, 1);
}

#[test]
fn list_unknown_keys_are_kept() {
    let d = level_dict(&[("k500", Value::from(5)), ("other", Value::from("x"))]);
    let list = GDList::from_dictionary(&d).unwrap();
    assert_eq!(list.other_properties.len(), 2);
}

#[test]
fn list_keys_without_k_prefix_are_not_parsed_as_known_keys() {
    let d = level_dict(&[("x2", Value::from("not the name"))]);
    let list = GDList::from_dictionary(&d).unwrap();
    assert_eq!(list.name, "");
    assert_eq!(
        list.other_properties.get("x2"),
        Some(&Value::from("not the name"))
    );
}

#[test]
fn list_empty_key_does_not_panic() {
    let _ = GDList::from_dictionary(&level_dict(&[("", Value::from(1))]));
}

#[test]
fn list_non_integer_kcek_does_not_panic() {
    let _ = GDList::from_dictionary(&level_dict(&[("kCEK", Value::from("12"))]));
}

#[test]
fn list_non_dict_levels_does_not_panic() {
    let _ = GDList::from_dictionary(&level_dict(&[("k97", Value::from("oops"))]));
}

#[test]
fn list_invalid_level_does_not_panic() {
    let mut level = Dictionary::new();
    level.insert("k1".into(), Value::from("not an id"));
    let mut levels = Dictionary::new();
    levels.insert("1".into(), Value::Dictionary(level));
    let _ = GDList::from_dictionary(&level_dict(&[("k97", Value::Dictionary(levels))]));
}

// ---------------- CCLocalLevels ----------------

fn cclocallevels_string(body: &str) -> String {
    format!("{PLIST_HEADER}<dict>{body}</dict>{PLIST_FOOTER}")
}

#[test]
fn cclocallevels_minimal() {
    let s = cclocallevels_string(
        "<k>LLM_01</k><d><k>_isArr</k><t /><k>k_0</k><d><k>k2</k><s>lvl</s></d></d><k>LLM_02</k><i>48</i><k>LLM_03</k><d><k>_isArr</k><t /></d>",
    );
    let cc = CCLocalLevels::from_decrypted(s).unwrap();
    assert_eq!(cc.levels.len(), 1);
    assert_eq!(cc.levels[0].identity.name, "lvl");
    assert_eq!(cc.binary_version, GDVersion::GD22082);
    assert!(cc.lists.is_empty());
}

#[test]
fn cclocallevels_bad_header() {
    assert!(CCLocalLevels::from_decrypted("<plist></plist>".into()).is_err());
    assert!(CCLocalLevels::from_decrypted(String::new()).is_err());
}

#[test]
fn cclocallevels_missing_levels() {
    let s = cclocallevels_string("<k>LLM_02</k><i>48</i><k>LLM_03</k><d />");
    assert!(CCLocalLevels::from_decrypted(s).is_err());
}

#[test]
fn cclocallevels_non_integer_binary_version() {
    let s = cclocallevels_string("<k>LLM_01</k><d /><k>LLM_02</k><s>48</s><k>LLM_03</k><d />");
    assert!(CCLocalLevels::from_decrypted(s).is_err());
}

#[test]
fn cclocallevels_without_lists() {
    // older savefiles don't have LLM_03
    let s = cclocallevels_string("<k>LLM_01</k><d><k>_isArr</k><t /></d><k>LLM_02</k><i>35</i>");
    let cc = CCLocalLevels::from_decrypted(s).expect("LLM_03 should be optional");
    assert!(cc.lists.is_empty());
}

// ---------------- CCGameManager ----------------

fn gamemanager_string(body: &str) -> String {
    format!("{PLIST_HEADER}<dict>{body}</dict>{PLIST_FOOTER}")
}

#[test]
fn gamemanager_empty() {
    let gm = CCGameManager::from_raw_string(gamemanager_string("")).unwrap();
    assert!(gm.other_properties.is_empty());
}

#[test]
fn gamemanager_bad_header() {
    assert!(CCGameManager::from_raw_string("<plist />".into()).is_err());
}

#[test]
fn gamemanager_player_and_config_fields() {
    let gm = CCGameManager::from_raw_string(gamemanager_string(concat!(
        "<k>playerName</k><s>Player</s>",
        "<k>playerUDID</k><s>S123</s>",
        "<k>playerUserID</k><i>99</i>",
        "<k>playerFrame</k><i>12</i>",
        "<k>playerColor</k><i>3</i>",
        "<k>playerGlow</k><t />",
        "<k>bootups</k><i>50</i>",
        "<k>bgVolume</k><r>0.5</r>",
        "<k>sfxVolume</k><r>0.25</r>",
        "<k>practicePosX</k><r>10.5</r>",
        "<k>practicePosY</k><r>-20</r>",
        "<k>customFPSTarget</k><r>240</r>",
        "<k>timeOffset</k><i>-15</i>",
        "<k>showProgressBar</k><t />",
        "<k>resolution</k><i>8</i>",
        "<k>texQuality</k><i>3</i>",
        "<k>GJA_001</k><s>acc</s>",
        "<k>GJA_002</k><s>plain</s>",
        "<k>GJA_003</k><i>71</i>",
        "<k>GLM_11</k><i>1234</i>",
        "<k>secretNumber</k><i>42</i>",
    )))
    .unwrap();

    assert_eq!(gm.player_info.username, "Player");
    assert_eq!(gm.player_info.udid, "S123");
    assert_eq!(gm.player_info.user_id, 99);
    assert_eq!(gm.player_info.icon_cube, 12);
    assert_eq!(gm.player_info.player_col1, 3);
    assert!(gm.player_info.using_glow);
    assert_eq!(gm.stats.bootups, 50);
    assert_eq!(gm.config.bgm_volume, 0.5);
    assert_eq!(gm.config.sfx_volume, 0.25);
    assert_eq!(gm.config.practice_ui_pos, (10.5, -20.0));
    assert_eq!(gm.config.fps_target, 240.0);
    assert_eq!(gm.config.music_offset, -15);
    assert!(gm.config.show_progress_bar);
    assert_eq!(gm.config.resolution, Resolution::R1280x720);
    assert_eq!(gm.config.texture_quality, TextureQuality::High);
    assert_eq!(gm.account.username, "acc");
    assert_eq!(gm.account.plaintext_password.as_deref(), Some("plain"));
    assert_eq!(gm.account.account_id, 71);
    assert_eq!(gm.temp_state.current_daily_level, 1234);
    assert_eq!(gm.config.secret_number, 42);
    assert!(gm.other_properties.is_empty(), "{:?}", gm.other_properties);
}

#[test]
fn gamemanager_unknown_resolution_falls_back_to_default() {
    let gm = CCGameManager::from_raw_string(gamemanager_string(
        "<k>resolution</k><i>500</i><k>texQuality</k><i>9</i>",
    ))
    .expect("unknown resolution should not fail the whole savefile");
    assert_eq!(gm.config.resolution, Resolution::default());
    assert_eq!(gm.config.texture_quality, TextureQuality::default());
}

#[test]
fn gamemanager_dpads() {
    let gm = CCGameManager::from_raw_string(gamemanager_string(concat!(
        "<k>dpad01</k><s>100,80,1.5,200,10.5,20.5,1,0.25,30,0,1</s>",
        "<k>dpad_layout</k><s>1,2,1,255,0,0,0,0,0,0,0</s>",
    )))
    .unwrap();
    let d = &gm.config.dpads[0];
    assert_eq!(d.width, 100);
    assert_eq!(d.height, 80);
    assert_eq!(d.scale, 1.5);
    assert_eq!(d.opacity, 200);
    assert_eq!(d.pos, (10.5, 20.5));
    assert!(d.mode_b);
    assert_eq!(d.deadzone, 0.25);
    assert_eq!(d.radius, 30.0);
    assert!(!d.snap);
    assert!(d.split);
    assert_eq!(gm.config.dpad_layout.as_ref().unwrap().opacity, 255);
    // dpad_layout is parsed, so it must not also end up in the unknown properties
    assert!(!gm.other_properties.contains_key("dpad_layout"));
}

#[test]
fn gamemanager_level_lists() {
    let gm = CCGameManager::from_raw_string(gamemanager_string(concat!(
        "<k>GLM_03</k><d><k>1</k><d><k>k1</k><i>1</i></d><k>2</k><d><k>k1</k><i>2</i></d></d>",
        "<k>GLM_10</k><d><k>5</k><d><k>k1</k><i>50</i></d></d>",
    )))
    .unwrap();
    assert_eq!(gm.stats.online_levels_played.len(), 2);
    assert_eq!(gm.stats.completed_dailies.len(), 1);
    assert_eq!(gm.stats.completed_dailies[&5].identity.id, 50);
}

#[test]
fn gamemanager_id_lists() {
    let gm = CCGameManager::from_raw_string(gamemanager_string(concat!(
        "<k>GLM_06</k><d><k>71</k><s>1</s><k>72</k><s>1</s></d>",
        "<k>GLM_14</k><d><k>1000</k><s>1</s></d>",
    )))
    .unwrap();
    let mut following = gm.account.following_creators.clone();
    following.sort();
    assert_eq!(following, vec![71, 72]);
    assert_eq!(gm.account.reported_levels, vec![1000]);
}

#[test]
fn gamemanager_folder_names_are_sorted() {
    let gm = CCGameManager::from_raw_string(gamemanager_string(
        "<k>GLM_19</k><d><k>3</k><s>c</s><k>1</k><s>a</s><k>2</k><s>b</s></d>",
    ))
    .unwrap();
    assert_eq!(
        gm.config.local_levels_foldernames,
        vec![(1, "a".into()), (2, "b".into()), (3, "c".into())]
    );
}

#[test]
fn gamemanager_glm12() {
    let gm = CCGameManager::from_raw_string(gamemanager_string(
        "<k>GLM_12</k><d><k>likes_1_2_3_4</k><s>1</s></d>",
    ))
    .unwrap();
    assert_eq!(gm.config.glm12_unknown, vec![[1, 2, 3, 4]]);
}

#[test]
fn gamemanager_custom_objects() {
    let gm = CCGameManager::from_raw_string(gamemanager_string(
        "<k>customObjectDict</k><d><k>-1</k><s>1,1,2,0,3,0;1,8,2,30,3,0;</s></d>",
    ))
    .unwrap();
    assert_eq!(gm.config.custom_objects.len(), 1);
    assert_eq!(gm.config.custom_objects[0].0, -1);
    assert_eq!(gm.config.custom_objects[0].1.len(), 2);
}

#[test]
fn gamemanager_achievements() {
    let gm = CCGameManager::from_raw_string(gamemanager_string(
        "<k>reportedAchievements</k><d><k>geometry.ach.level01b</k><s>100</s><k>modded.ach.1</k><s>50</s></d>",
    ))
    .unwrap();
    assert_eq!(
        gm.stats.get_vanilla_achievement(GDAchievement::Level01b),
        100
    );
    assert_eq!(gm.stats.get_achievement_by_ident("modded.ach.1"), Some(50));
}

#[test]
fn gamemanager_search_filters() {
    let gm = CCGameManager::from_raw_string(gamemanager_string(
        "<k>GLM_08</k><d><k>Diff1</k><s>1</s><k>star_filter</k><s>1</s><k>Len0</k><s>0</s><k>song_filter</k><s>5</s></d>",
    ))
    .unwrap();
    let f = &gm.config.search_filters;
    assert_eq!(
        f.boolean_flags,
        GDSearchFilterFlags::DifficultyAuto | GDSearchFilterFlags::HasStars
    );
    assert_eq!(f.song, Some(5));
}

#[test]
fn gamemanager_unknown_keys_are_kept() {
    let gm = CCGameManager::from_raw_string(gamemanager_string(
        "<k>someUnknownKey</k><i>5</i><k>GLM_20</k><d />",
    ))
    .unwrap();
    assert_eq!(
        gm.other_properties.get("someUnknownKey"),
        Some(&Value::from(5))
    );
    assert!(gm.other_properties.contains_key("GLM_20"));
}

macro_rules! gamemanager_no_panic_tests {
    ($($name:ident => $body:expr),* $(,)?) => {$(
        #[test]
        fn $name() {
            let _ = CCGameManager::from_raw_string(gamemanager_string($body));
        }
    )*};
}

gamemanager_no_panic_tests! {
    gamemanager_non_string_dpad_does_not_panic => "<k>dpad01</k><i>5</i>",
    gamemanager_malformed_dpad_does_not_panic => "<k>dpad01</k><s>a,b,c</s>",
    gamemanager_non_string_dpad_layout_does_not_panic => "<k>dpad_layout</k><i>5</i>",
    gamemanager_non_numeric_id_list_key_does_not_panic => "<k>GLM_06</k><d><k>notanumber</k><s>1</s></d>",
    gamemanager_short_glm12_key_does_not_panic => "<k>GLM_12</k><d><k>likes_1</k><s>1</s></d>",
    gamemanager_non_numeric_folder_index_does_not_panic => "<k>GLM_19</k><d><k>x</k><s>a</s></d>",
    gamemanager_non_string_folder_name_does_not_panic => "<k>GLM_19</k><d><k>1</k><i>1</i></d>",
    gamemanager_non_numeric_achievement_does_not_panic => "<k>reportedAchievements</k><d><k>geometry.ach.level01b</k><s>lots</s></d>",
    gamemanager_huge_achievement_progress_does_not_panic => "<k>reportedAchievements</k><d><k>geometry.ach.level01b</k><s>70000</s></d>",
    gamemanager_non_numeric_custom_object_key_does_not_panic => "<k>customObjectDict</k><d><k>x</k><s>1,1;</s></d>",
    gamemanager_non_dict_completed_daily_does_not_panic => "<k>GLM_10</k><d><k>1</k><s>not a level</s></d>",
    gamemanager_non_numeric_completed_daily_key_does_not_panic => "<k>GLM_10</k><d><k>x</k><d /></d>",
    gamemanager_non_dict_level_in_level_list_does_not_panic => "<k>GLM_03</k><d><k>1</k><s>not a level</s></d>",
}

#[test]
fn gamemanager_non_dict_root_does_not_panic() {
    let _ = CCGameManager::from_raw_string(format!("{PLIST_HEADER}<s>oops</s>{PLIST_FOOTER}"));
}

#[test]
fn search_filters_from_dict() {
    let d = level_dict(&[
        ("Diff7", Value::from("1")),
        ("Len5", Value::from("1")),
        ("legendary_filter", Value::from("1")),
        ("epic_filter", Value::from("0")),
        ("unknown_filter", Value::from("1")),
    ]);
    let f = GDSearchFilters::from_dict(&d).unwrap();
    assert_eq!(
        f.boolean_flags,
        GDSearchFilterFlags::DifficultyDemon
            | GDSearchFilterFlags::Platformer
            | GDSearchFilterFlags::RatingLegendary
    );
    assert_eq!(f.song, None);
    assert_eq!(f.other.get("unknown_filter"), Some(&Value::from("1")));
}

#[test]
fn search_filters_invalid() {
    assert!(GDSearchFilters::from_dict(&level_dict(&[("Diff1", Value::from(1))])).is_none());
    assert!(
        GDSearchFilters::from_dict(&level_dict(&[("song_filter", Value::from("abc"))])).is_none()
    );
}

#[test]
fn platformer_ui_partial_input() {
    let ui = GDPlatformerUI::parse("10,20");
    assert_eq!(ui.width, 10);
    assert_eq!(ui.height, 20);
    assert_eq!(ui.scale, 0.0);
}

#[test]
fn platformer_ui_too_many_fields_does_not_panic() {
    let _ = GDPlatformerUI::parse("1,2,3,4,5,6,7,8,9,10,11,12,13");
}

#[test]
fn platformer_ui_malformed_field_does_not_panic() {
    let _ = GDPlatformerUI::parse("wide,tall");
    let _ = GDPlatformerUI::parse("");
}

// ---------------- core::io / core ----------------

#[test]
fn b64_decode_url_safe() {
    assert_eq!(b64_decode("-_8=").unwrap(), vec![0xfb, 0xff]);
    assert_eq!(b64_decode("aGVsbG8=").unwrap(), b"hello");
}

#[test]
fn b64_decode_invalid() {
    assert!(b64_decode("not base64!").is_err());
    assert!(b64_decode("+/8=").is_err());
}

#[test]
fn decompress_invalid_input_returns_err() {
    assert!(decompress(b"not base64 at all!".to_vec()).is_err());
}

#[test]
fn decompress_short_input_does_not_panic() {
    let _ = decompress(b"H4sI".to_vec());
    let _ = decompress(Vec::new());
}

#[test]
fn decompress_ignores_null_bytes() {
    let mut enc = encrypt_level_str("kA2,0;");
    enc.push(0);
    enc.insert(5, 0);
    assert_eq!(decompress(enc).unwrap(), b"kA2,0;");
}

#[test]
fn decrypt_savefile() {
    let raw = format!("{PLIST_HEADER}<dict />{PLIST_FOOTER}");
    assert_eq!(decrypt(encrypt_savefile_str(&raw)).unwrap(), raw.as_bytes());
}

#[test]
fn decrypt_garbage_does_not_panic() {
    let _ = decrypt(vec![1, 2, 3, 4, 5]);
    let _ = decrypt(Vec::new());
}
