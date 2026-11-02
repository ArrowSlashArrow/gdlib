//! Round-trip tests: serialising and then deserialising a value (or the other way around) must give back the original.
use std::collections::HashMap;

use plist::{Dictionary, Value};
use smallvec::{SmallVec, smallvec};

use super::{
    CONFIG_PROPERTY_IDS, assert_levels_eq, assert_lists_eq, default_block, sample_level,
    sample_list,
};
use crate::{
    cclocallevels::{
        gdlevel::{
            CCLocalLevels, GDLevel,
            enums::{DemonType, OfficialSong},
            leveldata::{
                ColourString, DEFAULT_LEVEL_HEADERS, GDEncryptedLevelData, GDLevelData,
                GDLevelHeaderKey, GDLevelHeaderValType, GDLevelHeaderValue, GDLevelState,
                GuidelineColour, GuidelineString, PlayerColour, parse_raw_level_headers,
            },
            version::GDVersion,
        },
        gdlist::GDList,
        gdobj::{
            GDObject,
            meta::{GDObjAttributes, GDObjConfig},
            structs::*,
        },
        properties::PROPERTY_TABLE,
    },
    core::{
        b64_decode, b64_encode,
        io::{decompress, decrypt, encrypt_level_str, encrypt_savefile_str, stringify_xml},
        proper_plist_tags,
    },
};

// ---------------- GDValue ----------------

/// Property type that a `GDValue` variant is parsed from. `None` for variants that no property type maps to.
fn prop_type_of(v: &GDValue) -> Option<GDObjPropType> {
    use GDObjPropType as T;
    Some(match v {
        GDValue::Int(_) => T::Int,
        GDValue::Float(_) => T::Float,
        GDValue::Double(_) => T::Double,
        GDValue::Bool(_) => T::Bool,
        GDValue::Toggle(_) => T::Toggle,
        GDValue::Group(_) => T::Group,
        GDValue::Item(_) => T::Item,
        GDValue::GroupList(_) => T::GroupList,
        GDValue::ProbabilitiesList(_) => T::ProbabilitiesList,
        GDValue::SpawnRemapsList(_) => T::SpawnRemapsList,
        GDValue::Easing(_) => T::Easing,
        GDValue::ColourChannel(_) => T::ColourChannel,
        GDValue::Events(_) => T::EventsList,
        GDValue::Speed(_) => T::Speed,
        GDValue::Gamemode(_) => T::Gamemode,
        GDValue::ItemEditOperator(_) => T::ItemEditOperator,
        GDValue::ItemCompareOperator(_) => T::ItemCompareOperator,
        GDValue::BPMSpeed(_) => T::BPMSpeed,
        GDValue::ItemEditRoundMode(_) => T::ItemEditRoundMode,
        GDValue::ItemEditSignMode(_) => T::ItemEditSignMode,
        GDValue::TouchToggle(_) => T::TouchToggle,
        GDValue::AxisOnlyMove(_) => T::AxisOnlyMove,
        GDValue::ArrowDirection(_) => T::ArrowDirection,
        GDValue::PlayerTarget(_) => T::PlayerTarget,
        GDValue::TransitionMode(_) => T::TransitionMode,
        GDValue::UIReferencePos(_) => T::UIReferencePos,
        GDValue::CounterMode(_) => T::CounterMode,
        GDValue::ItemType(_) => T::ItemType,
        GDValue::StopMode(_) => T::StopMode,
        GDValue::ExtraID2(_) => T::ExtraID2,
        GDValue::MiddleGround(_) => T::MiddleGround,
        GDValue::ItemAlign(_) => T::ItemAlign,
        GDValue::TeleportGravity(_) => T::TeleportGravity,
        GDValue::String(_) => T::Text,
        GDValue::Short(_) | GDValue::ZLayer(_) => return None,
    })
}

/// A non-default value for every property type.
fn sample_value(t: GDObjPropType) -> GDValue {
    use GDObjPropType as T;
    match t {
        T::Int => GDValue::Int(-12345),
        T::Float => GDValue::Float(12.75),
        T::Double => GDValue::Double(0.125),
        T::Text | T::Unknown => GDValue::String("SGVsbG8=".into()),
        T::Bool => GDValue::Bool(true),
        T::Group => GDValue::Group(9999),
        T::GroupList => GDValue::GroupList(smallvec![1, 20, 300]),
        T::Item => GDValue::Item(42),
        T::Easing => GDValue::Easing(MoveEasing::BounceOut),
        T::EventsList => GDValue::Events(vec![Event::DashStart, Event::FallSpeedHigh]),
        T::ColourChannel => GDValue::ColourChannel(ColourChannel::MiddleGround2),
        T::ProbabilitiesList => GDValue::ProbabilitiesList(smallvec![(1, 25), (2, 75)]),
        T::SpawnRemapsList => GDValue::SpawnRemapsList(smallvec![(5, 6), (7, 8)]),
        T::Toggle => GDValue::Toggle(Toggle::Off),
        T::Speed => GDValue::Speed(Speed::X3),
        T::Gamemode => GDValue::Gamemode(Gamemode::Robot),
        T::ItemEditOperator => GDValue::ItemEditOperator(Op::Mul),
        T::ItemCompareOperator => GDValue::ItemCompareOperator(CompareOp::GreaterOrEquals),
        T::BPMSpeed => GDValue::BPMSpeed(BPMSpeed::Faster),
        T::ItemEditRoundMode => GDValue::ItemEditRoundMode(RoundMode::Nearest),
        T::ItemEditSignMode => GDValue::ItemEditSignMode(SignMode::Negative),
        T::TouchToggle => GDValue::TouchToggle(TouchToggle::ToggleOn),
        T::AxisOnlyMove => GDValue::AxisOnlyMove(AxisOnlyMove::Y),
        T::ArrowDirection => GDValue::ArrowDirection(ArrowDirection::Down),
        T::PlayerTarget => GDValue::PlayerTarget(OptionalPlayerTarget::Player2),
        T::TransitionMode => GDValue::TransitionMode(TransitionMode::Exit),
        T::UIReferencePos => GDValue::UIReferencePos(UIReferencePos::Bottom),
        T::CounterMode => GDValue::CounterMode(CounterMode::MainTime),
        T::ItemType => GDValue::ItemType(ItemType::Points),
        T::StopMode => GDValue::StopMode(StopMode::Pause),
        T::ItemAlign => GDValue::ItemAlign(ItemAlign::Left),
        T::MiddleGround => GDValue::MiddleGround(MiddleGround::SeasweptMountains),
        T::TeleportGravity => GDValue::TeleportGravity(TeleportGravity::Normal),
        T::ExtraID2 => GDValue::ExtraID2(ExtraID2::P1),
    }
}

#[track_caller]
fn assert_gdvalue_roundtrip(v: GDValue) {
    let t = prop_type_of(&v).expect("value has no property type");
    let s = v.to_string();
    assert_eq!(GDValue::from(t, &s), v, "serialised as {s:?}");
}

#[test]
fn gdvalue_scalars() {
    for v in [
        GDValue::Int(0),
        GDValue::Int(-1),
        GDValue::Int(i32::MAX),
        GDValue::Int(i32::MIN),
        GDValue::Group(0),
        GDValue::Group(i16::MAX),
        GDValue::Group(-1),
        GDValue::Item(i16::MIN),
        GDValue::Bool(true),
        GDValue::Bool(false),
        GDValue::String("plain".into()),
        GDValue::String(String::new()),
    ] {
        assert_gdvalue_roundtrip(v);
    }
}

#[test]
fn gdvalue_floats() {
    for f in [
        0.0f32,
        -0.0,
        1.0,
        -1.0,
        0.1,
        1.0 / 3.0,
        12345.678,
        -98765.43,
        1e-7,
        1e10,
        f32::MAX,
        f32::MIN,
        f32::MIN_POSITIVE,
        f32::EPSILON,
        f32::INFINITY,
        f32::NEG_INFINITY,
    ] {
        assert_gdvalue_roundtrip(GDValue::Float(f));
    }
    for f in [
        0.0f64,
        0.1,
        1.0 / 3.0,
        -1e-12,
        1e300,
        f64::MAX,
        f64::MIN_POSITIVE,
    ] {
        assert_gdvalue_roundtrip(GDValue::Double(f));
    }
}

#[test]
fn gdvalue_nan_stays_nan() {
    let s = GDValue::Float(f32::NAN).to_string();
    let GDValue::Float(f) = GDValue::from(GDObjPropType::Float, &s) else {
        panic!("not a float");
    };
    assert!(f.is_nan(), "{s:?} parsed as {f}");
}

#[test]
fn gdvalue_lists() {
    for v in [
        GDValue::GroupList(smallvec![1]),
        GDValue::GroupList(smallvec![1, 2, 3]),
        GDValue::GroupList((1..=50).collect()),
        GDValue::GroupList(smallvec![-5, 0, i16::MAX]),
        GDValue::GroupList(SmallVec::new()),
        GDValue::ProbabilitiesList(smallvec![(1, 10)]),
        GDValue::ProbabilitiesList((1..=20).map(|i| (i, i as i32 * 100)).collect()),
        GDValue::ProbabilitiesList(smallvec![(5, 0), (-1, -1)]),
        GDValue::ProbabilitiesList(SmallVec::new()),
        GDValue::SpawnRemapsList(smallvec![(1, 2), (3, 4)]),
        GDValue::SpawnRemapsList(smallvec![(i32::MIN, i32::MAX)]),
        GDValue::SpawnRemapsList(SmallVec::new()),
        GDValue::Events(vec![Event::TinyLanding]),
        GDValue::Events((1..=78).map(Event::from).collect()),
        GDValue::Events(vec![Event::Unrecognized(500), Event::Unrecognized(-3)]),
        GDValue::Events(vec![]),
    ] {
        assert_gdvalue_roundtrip(v);
    }
}

#[test]
fn gdvalue_colour_channels() {
    for i in (-10..=1100).chain([i16::MIN, i16::MAX]) {
        let ch = ColourChannel::from(i);
        assert_gdvalue_roundtrip(GDValue::ColourChannel(ch));
    }
}

#[test]
fn gdvalue_strict_enums() {
    // every valid numeric value for each strict enum
    macro_rules! check {
        ($($variant:ident($enum:ty)),* $(,)?) => {$(
            let mut seen = 0;
            for i in -10..=2000 {
                if let Ok(e) = <$enum>::try_from(i) {
                    seen += 1;
                    assert_gdvalue_roundtrip(GDValue::$variant(e));
                }
            }
            assert!(seen > 0, stringify!($enum));
        )*};
    }
    check!(
        Toggle(Toggle),
        Easing(MoveEasing),
        Speed(Speed),
        Gamemode(Gamemode),
        ItemEditOperator(Op),
        ItemCompareOperator(CompareOp),
        BPMSpeed(BPMSpeed),
        ItemEditRoundMode(RoundMode),
        ItemEditSignMode(SignMode),
        TouchToggle(TouchToggle),
        AxisOnlyMove(AxisOnlyMove),
        ArrowDirection(ArrowDirection),
        PlayerTarget(OptionalPlayerTarget),
        TransitionMode(TransitionMode),
        UIReferencePos(UIReferencePos),
        StopMode(StopMode),
        ExtraID2(ExtraID2),
        ItemAlign(ItemAlign),
        TeleportGravity(TeleportGravity),
    );
}

#[test]
fn gdvalue_lenient_enums() {
    // these enums keep unrecognized values, so any integer must survive
    for i in [
        -100,
        -3,
        -2,
        -1,
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        100,
        i32::MIN,
        i32::MAX,
    ] {
        assert_gdvalue_roundtrip(GDValue::CounterMode(CounterMode::from(i)));
        assert_gdvalue_roundtrip(GDValue::ItemType(ItemType::from(i)));
        assert_gdvalue_roundtrip(GDValue::MiddleGround(MiddleGround::from(i)));
    }
}

#[test]
fn gdvalue_item_type_counter_default() {
    // documented on `ItemType::Counter`: unset item types are interpreted as counters
    assert_eq!(
        GDValue::from(GDObjPropType::ItemType, ""),
        GDValue::ItemType(ItemType::Counter)
    );
}

// ---------------- GDObject ----------------

/// Asserts that `parse(serialise(parse(s))) == parse(s)`.
#[track_caller]
fn assert_object_string_roundtrip(s: &str) {
    let parsed = GDObject::parse_str(s);
    let reserialised = parsed.serialise_to_string();
    assert_eq!(
        GDObject::parse_str(&reserialised),
        parsed,
        "\noriginal:     {s}\nreserialised: {reserialised}"
    );
}

#[test]
fn object_strings_from_real_levels() {
    for s in [
        "1,1,2,727.603,3,18.7869,64,1,67,1,155,1,128,0.072,129,0.072;",
        "1,8,2,525,3,825,96,1,121,1,61,1001,57,5953.6,64,1,67,1,155,1,21,300;",
        "1,1914,2,-15,3,435,96,1,61,1001,57,6,64,1,67,1,155,2,36,1,85,2,71,59,213,1;",
        "1,2015,2,45,3,465,96,1,61,1001,57,9.6,64,1,67,1,155,2,36,1,10,2,30,1,85,2,68,0.5;",
        "1,2068,2,75,3,75,155,1,11,1,87,1,36,1,152,1.10.2.20.3.15.4.5.5.25.6.50;",
        "1,914,2,195,3,195,155,3,31,MQ==;",
        "1,1913,2,15,3,105,155,1,170,1,36,1,371,0.4,10,1,30,1,85,2;",
        "1,1934,2,-15,3,135,155,1,13,1,36,1,392,10003154,406,1,413,1,421,1,422,0.5,10,0.5;",
        "1,29,2,15,3,285,155,1,36,1,7,180,8,0,9,24,10,2,35,1,23,1000;",
        "1,153,2,277.5,3,131,5,1,57,1,155,2,6,90;",
        "1,3600,2,2925,3,795,36,1,51,321,461,1,487,1;",
        "1,31,2,0,3,0,kA10,0,kA11,,kA22,0,kA27,1;",
    ] {
        assert_object_string_roundtrip(s);
    }
}

#[test]
fn object_with_every_config_field() {
    let mut cfg = GDObjConfig::new()
        .with_pos(-15.5, 1005.25)
        .with_scale(2.0, -0.5)
        .with_angle(-45.5)
        .with_touchable(true)
        .with_spawnable(true)
        .with_multitrigger(true)
        .with_editor_layer_1(3)
        .with_editor_layer_2(-4)
        .with_base_colour(ColourChannel::P1)
        .with_detail_colour(ColourChannel::Channel(999))
        .with_z_layer(ZLayer::T3)
        .with_z_order(-100)
        .with_enter_channel(5)
        .with_material_id(6)
        .with_control_id(7)
        .with_groups([9999i16, 1, 50])
        .with_attributes(GDObjAttributes::all(), true);
    cfg.pink_groups = vec![50];
    let obj = GDObject::new(1, &cfg, vec![]);
    let parsed = GDObject::parse_str(obj.serialise_to_string());
    assert_eq!(parsed.config, obj.config);
}

#[test]
fn object_with_each_attribute() {
    for bit in 0..24 {
        let flag = GDObjAttributes::from_bits(1 << bit).unwrap();
        let obj = GDObject::new(1, &GDObjConfig::new().with_attributes(flag, true), vec![]);
        let parsed = GDObject::parse_str(obj.serialise_to_string());
        assert_eq!(parsed.config.attributes, flag, "{flag:?}");
    }
}

#[test]
fn object_with_each_z_layer() {
    for z in [
        ZLayer::B5,
        ZLayer::B4,
        ZLayer::B3,
        ZLayer::B2,
        ZLayer::B1,
        ZLayer::Default,
        ZLayer::T1,
        ZLayer::T2,
        ZLayer::T3,
        ZLayer::T4,
    ] {
        let obj = GDObject::new(1, &GDObjConfig::new().with_z_layer(z), vec![]);
        let parsed = GDObject::parse_str(obj.serialise_to_string());
        assert_eq!(parsed.config.z_layer, z);
    }
}

#[test]
fn object_with_pink_groups_only() {
    let mut cfg = GDObjConfig::new();
    cfg.pink_groups = vec![4, 5];
    let obj = GDObject::new(1, &cfg, vec![]);
    let parsed = GDObject::parse_str(obj.serialise_to_string());
    assert_eq!(parsed.config, cfg);
}

#[test]
fn object_with_every_typed_property() {
    // put a sample value into every non-config property in the property table,
    // serialise the object and check that every value is parsed back.
    for (id, (name, t)) in PROPERTY_TABLE.entries() {
        if CONFIG_PROPERTY_IDS.contains(id) {
            continue;
        }
        let value = sample_value(*t);
        let obj = GDObject::new(901, &GDObjConfig::default(), vec![(*id, value.clone())]);
        let s = obj.serialise_to_string();
        assert_eq!(
            GDObject::parse_str(&s).get_property(*id),
            Some(value),
            "property {id} ({name}) serialised as {s}"
        );
    }
}

#[test]
fn object_with_ka_properties() {
    let obj = GDObject::new(
        31,
        &GDObjConfig::default(),
        vec![
            (10_000, GDValue::String("0".into())),
            (10_011, GDValue::String(String::new())),
            (10_048, GDValue::String("1".into())),
        ],
    );
    assert_eq!(GDObject::parse_str(obj.serialise_to_string()), obj);
}

#[test]
fn object_with_large_property_id() {
    // ids at and above `KA_PROPERTY_OFFSET` are written as kA keys. The highest ones must not overflow.
    let obj = GDObject::new(
        1,
        &GDObjConfig::default(),
        vec![(u16::MAX - 1, GDValue::String("x".into()))],
    );
    assert_eq!(GDObject::parse_str(obj.serialise_to_string()), obj);
}

// ---------------- sub-structs ----------------

#[test]
fn hsv_colour() {
    for hsv in [
        HSVColour {
            hue_shift: 0,
            saturation_mult: 1.0,
            brightness_mult: 1.0,
            static_sat_scalar: false,
            static_bright_scalar: false,
        },
        HSVColour {
            hue_shift: -180,
            saturation_mult: 0.0,
            brightness_mult: 2.0,
            static_sat_scalar: true,
            static_bright_scalar: false,
        },
        HSVColour {
            hue_shift: 180,
            saturation_mult: 0.333,
            brightness_mult: -0.5,
            static_sat_scalar: false,
            static_bright_scalar: true,
        },
        HSVColour {
            hue_shift: 15,
            saturation_mult: 1.5,
            brightness_mult: 0.25,
            static_sat_scalar: true,
            static_bright_scalar: true,
        },
    ] {
        assert_eq!(
            HSVColour::parse(&hsv.to_string()),
            Some(hsv.clone()),
            "{hsv}"
        );
    }
}

#[test]
fn hsv_colour_string() {
    for s in ["0a1a1a0a0", "10a1a1a1a0", "10a1a1a0a1", "-30a0.5a1.25a1a1"] {
        assert_eq!(HSVColour::parse(s).unwrap().to_string(), s);
    }
}

fn full_colour_string() -> ColourString {
    ColourString {
        from: Colour::from_rgb((40, 125, 255)),
        to: Colour::from_rgb((1, 2, 3)),
        player_colour: PlayerColour::First,
        blending: true,
        colour_ch_idx: 1013,
        from_opacity: 0.5,
        to_opacity: 0.25,
        opacity_toggled: true,
        inherited_col_ch_idx: 3,
        copied_hsv: HSVColour::parse("10a0.5a1.5a1a0"),
        deltatime: 0.75,
        duration: 2.5,
        copy_opacity: true,
        unknown_property18: true,
    }
}

#[test]
fn colour_string() {
    for cs in [
        ColourString::default(),
        full_colour_string(),
        ColourString {
            player_colour: PlayerColour::Second,
            from_opacity: 1.0,
            ..Default::default()
        },
    ] {
        assert_eq!(
            ColourString::parse(&cs.to_string()),
            Some(cs.clone()),
            "{}",
            cs.to_string()
        );
    }
}

#[test]
fn colour_strings_from_default_level() {
    let segments = DEFAULT_LEVEL_HEADERS
        .strip_prefix("kS38,")
        .unwrap()
        .split(",kA13")
        .next()
        .unwrap();
    for segment in segments.split('|').filter(|s| !s.is_empty()) {
        let cs = ColourString::parse(segment).unwrap();
        assert_eq!(ColourString::parse(&cs.to_string()), Some(cs), "{segment}");
    }
}

#[test]
fn guideline_string() {
    let g = GuidelineString {
        guidelines: vec![
            ("0.5".into(), GuidelineColour::Orange),
            ("1.25".into(), GuidelineColour::Yellow),
            ("10".into(), GuidelineColour::Green),
        ],
    };
    assert_eq!(
        GuidelineString::parse(&g.to_string()),
        Some(g.clone()),
        "{}",
        g.to_string()
    );
}

// ---------------- level header ----------------

#[test]
fn header_key_string() {
    for k in [
        GDLevelHeaderKey::A(0),
        GDLevelHeaderKey::A(13),
        GDLevelHeaderKey::A(u16::MAX),
        GDLevelHeaderKey::S(38),
        GDLevelHeaderKey::S(0),
        GDLevelHeaderKey::Unknown("kZZ".into()),
        GDLevelHeaderKey::Unknown("weird_key".into()),
    ] {
        assert_eq!(GDLevelHeaderKey::parse(&k.to_string()), k);
    }
}

#[test]
fn header_key_to_id_from_id() {
    // guaranteed by the documentation of `from_id`
    for id in 0..=u16::MAX {
        assert_eq!(GDLevelHeaderKey::from_id(id).to_id(), Some(id), "id {id}");
    }
}

#[test]
fn header_key_from_id_to_id() {
    for k in (0..1000)
        .map(GDLevelHeaderKey::A)
        .chain((0..=1000).map(GDLevelHeaderKey::S))
    {
        assert_eq!(GDLevelHeaderKey::from_id(k.to_id().unwrap()), k, "{k:?}");
    }
}

#[test]
fn header_values() {
    use GDLevelHeaderValType as T;
    let cases = [
        (T::Int, GDLevelHeaderValue::Int(-500)),
        (T::Int, GDLevelHeaderValue::Int(i32::MAX)),
        (T::Float, GDLevelHeaderValue::Float(0.125)),
        (T::Float, GDLevelHeaderValue::Float(-3.0)),
        (T::Bool, GDLevelHeaderValue::Bool(true)),
        (T::Bool, GDLevelHeaderValue::Bool(false)),
        (T::Gamemode, GDLevelHeaderValue::Gamemode(Gamemode::Swing)),
        (T::Speed, GDLevelHeaderValue::Speed(Speed::X0Point5)),
        (
            T::ColourString,
            GDLevelHeaderValue::ColourString(vec![ColourString::default(), full_colour_string()]),
        ),
        (T::ColourString, GDLevelHeaderValue::ColourString(vec![])),
        (
            T::GuidelineString,
            GDLevelHeaderValue::GuidelineString(GuidelineString {
                guidelines: vec![("2".into(), GuidelineColour::Green)],
            }),
        ),
    ];
    for (t, v) in cases {
        assert_eq!(
            GDLevelHeaderValue::parse(&v.to_string(), t),
            Some(v.clone()),
            "{v}"
        );
    }
}

#[test]
fn default_level_headers() {
    let headers = parse_raw_level_headers(DEFAULT_LEVEL_HEADERS);
    let data = GDLevelData {
        headers: headers.clone(),
        objects: vec![],
    };
    let serialised = data.serialise_level_headers();
    assert_eq!(
        parse_raw_level_headers(serialised.trim_end_matches(';')),
        headers
    );
}

#[test]
fn level_headers_with_unknown_keys() {
    let headers = parse_raw_level_headers("kA2,1,kZZ,5,kA999,3,kS100,7");
    let data = GDLevelData {
        headers: headers.clone(),
        objects: vec![],
    };
    let serialised = data.serialise_level_headers();
    assert_eq!(
        parse_raw_level_headers(serialised.trim_end_matches(';')),
        headers
    );
}

// ---------------- level data ----------------

fn sample_level_data() -> GDLevelData {
    let mut data = GDLevel::default().get_decrypted_data().unwrap();
    data.objects = vec![
        default_block(),
        GDObject::new(
            901,
            &GDObjConfig::new()
                .with_pos(45.0, 75.0)
                .with_groups([1i16, 2])
                .with_spawnable(true)
                .with_multitrigger(true),
            vec![
                (51, GDValue::Group(5)),
                (10, GDValue::Float(0.5)),
                (30, GDValue::Easing(MoveEasing::BackOut)),
                (85, GDValue::Float(1.5)),
            ],
        ),
        GDObject::parse_str("1,914,2,195,3,195,31,SGVsbG8sIHdvcmxkIQ==;"),
    ];
    data
}

#[test]
fn level_data_encrypted() {
    let data = sample_level_data();
    assert_eq!(GDLevelData::parse(data.serialise_to_string()), Some(data));
}

#[test]
fn level_data_without_objects() {
    let data = GDLevel::default().get_decrypted_data().unwrap();
    assert_eq!(GDLevelData::parse(data.serialise_to_string()), Some(data));
}

#[test]
fn level_data_without_headers() {
    let data = GDLevelData {
        headers: HashMap::new(),
        objects: vec![default_block()],
    };
    assert_eq!(GDLevelData::parse(data.serialise_to_string()), Some(data));
}

#[test]
fn level_data_many_objects() {
    let mut data = GDLevel::default().get_decrypted_data().unwrap();
    data.objects = (0..5000)
        .map(|i| {
            GDObject::new(
                1 + (i % 100),
                &GDObjConfig::new()
                    .with_pos(i as f32 * 30.0, (i % 7) as f32 * 30.0)
                    .with_groups([(i % 9999 + 1) as i16]),
                vec![],
            )
        })
        .collect();
    assert_eq!(GDLevelData::parse(data.serialise_to_string()), Some(data));
}

// ---------------- encryption ----------------

#[test]
fn level_string_compression() {
    for s in [
        String::new(),
        "a".into(),
        "kS38,1_40_2_125;1,1,2,15,3,15;".into(),
        "unicode: 日本語 ✓ é".into(),
        "x".repeat(1 << 20),
        (0..10_000).map(|i| format!("1,{i},2,{i},3,0;")).collect(),
    ] {
        assert_eq!(
            decompress(encrypt_level_str(&s)).unwrap(),
            s.as_bytes(),
            "length {}",
            s.len()
        );
    }
}

#[test]
fn savefile_encryption() {
    for s in [
        String::new(),
        "<?xml version=\"1.0\"?><plist />".into(),
        "unicode: 日本語 ✓ é".into(),
        "y".repeat(100_000),
    ] {
        assert_eq!(decrypt(encrypt_savefile_str(&s)).unwrap(), s.as_bytes());
    }
}

#[test]
fn base64() {
    for bytes in [
        &b""[..],
        b"a",
        b"ab",
        b"abc",
        &[0, 255, 254, 253, 0xfb, 0xff],
        "日本語".as_bytes(),
    ] {
        assert_eq!(b64_decode(b64_encode(bytes)).unwrap(), bytes);
    }
}

// ---------------- plist ----------------

fn reparse_xml(d: &Dictionary) -> Dictionary {
    let s = stringify_xml(d, true);
    let s = proper_plist_tags(format!("<plist version=\"1.0\">{s}</plist>")).unwrap();
    Value::from_reader_xml(s.as_bytes())
        .unwrap_or_else(|e| panic!("{e}: {s}"))
        .into_dictionary()
        .unwrap()
}

#[test]
fn stringified_xml() {
    let mut inner = Dictionary::new();
    inner.insert("n".into(), Value::from(1));
    let mut d = Dictionary::new();
    d.insert("s".into(), Value::from("str"));
    d.insert("i".into(), Value::from(-5));
    d.insert("t".into(), Value::Boolean(true));
    d.insert("f".into(), Value::Boolean(false));
    d.insert("r".into(), Value::Real(1.5));
    d.insert("d".into(), Value::Dictionary(inner));
    d.insert("e".into(), Value::Dictionary(Dictionary::new()));
    assert_eq!(reparse_xml(&d), d);
}

#[test]
fn stringified_xml_with_special_characters() {
    let mut d = Dictionary::new();
    d.insert(
        "key & <key>".into(),
        Value::from("a < b && c > d \"quoted\" 'single'"),
    );
    assert_eq!(reparse_xml(&d), d);
}

#[test]
fn stringified_xml_with_whole_real() {
    // a real with no fractional part must stay a real
    let mut d = Dictionary::new();
    d.insert("r".into(), Value::Real(2.0));
    assert_eq!(reparse_xml(&d), d);
}

// ---------------- GDLevel ----------------

#[test]
fn level_default_dict() {
    let level = GDLevel::default();
    assert_levels_eq(&GDLevel::from_dict(&level.to_dict()).unwrap(), &level);
}

#[test]
fn level_sample_dict() {
    let level = sample_level();
    assert_levels_eq(&GDLevel::from_dict(&level.to_dict()).unwrap(), &level);
}

#[test]
fn level_sample_gmd_string() {
    let level = sample_level();
    let parsed = GDLevel::from_plist_string(level.serialise_to_string()).unwrap();
    assert_levels_eq(&parsed, &level);
}

#[test]
fn level_string_is_stable() {
    // serialising a parsed level must give back the same level string
    let level = sample_level();
    let once = GDLevel::from_plist_string(level.serialise_to_string()).unwrap();
    let twice = GDLevel::from_plist_string(once.serialise_to_string()).unwrap();
    assert_levels_eq(&twice, &once);
}

#[test]
fn level_with_xml_special_characters() {
    let mut level = GDLevel::default();
    level.identity.name = "Tom & Jerry <3".into();
    level.identity.creator = "a>b".into();
    let parsed = GDLevel::from_plist_string(level.serialise_to_string()).unwrap();
    assert_levels_eq(&parsed, &level);
}

#[test]
fn level_with_unicode_text() {
    let mut level = GDLevel::default();
    level.identity.name = "Niveau élevé 日本".into();
    level.identity.description = Some("Описание ✓".into());
    let parsed = GDLevel::from_plist_string(level.serialise_to_string()).unwrap();
    assert_levels_eq(&parsed, &level);
}

#[test]
fn level_with_empty_description() {
    let mut level = GDLevel::default();
    level.identity.description = Some(String::new());
    assert_levels_eq(&GDLevel::from_dict(&level.to_dict()).unwrap(), &level);
}

#[test]
fn level_with_empty_sfx_list() {
    let mut level = GDLevel::default();
    level.content.sfx_list = Some(vec![]);
    let parsed = GDLevel::from_dict(&level.to_dict()).expect("level failed to parse");
    assert_levels_eq(&parsed, &level);
}

#[test]
fn level_with_single_entry_lists() {
    let mut level = GDLevel::default();
    level.content.song_list = vec![1];
    level.content.sfx_list = Some(vec![2]);
    level.player_stats.progress_diffs = vec![3];
    level.player_stats.local_best_times = vec![4];
    level.player_stats.local_best_points = vec![5];
    assert_levels_eq(&GDLevel::from_dict(&level.to_dict()).unwrap(), &level);
}

#[test]
fn level_with_extreme_integers() {
    let mut level = GDLevel::default();
    level.identity.id = i32::MAX;
    level.ratings.downloads = i32::MIN;
    level.ratings.like_rating = -1;
    level.identity.password = Some(0);
    level.integrity.level_seed = Some(i32::MIN);
    level.content.song_list = vec![i32::MIN, 0, i32::MAX];
    assert_levels_eq(&GDLevel::from_dict(&level.to_dict()).unwrap(), &level);
}

#[test]
fn level_with_optional_zero_values() {
    // `Some(0)` must not be confused with `None`
    let mut level = GDLevel::default();
    level.ratings.requested_stars = Some(0);
    level.coins.required_coins = Some(0);
    level.content.custom_song_id = Some(0);
    level.content.official_song_id = Some(OfficialSong::StereoMadness); // 0
    level.ratings.demon_type = Some(DemonType::HardDemon); // 0
    assert_levels_eq(&GDLevel::from_dict(&level.to_dict()).unwrap(), &level);
}

#[test]
fn level_with_unknown_binary_version() {
    let level = GDLevel::new(GDVersion::Unknown(1234));
    assert_levels_eq(&GDLevel::from_dict(&level.to_dict()).unwrap(), &level);
}

#[test]
fn level_with_short_build_tab_pages() {
    let mut level = GDLevel::default();
    level.editor_state.build_tab_pages = {
        let mut m = HashMap::new();
        m.insert(0, 5);
        m
    };
    assert_levels_eq(&GDLevel::from_dict(&level.to_dict()).unwrap(), &level);
}

#[test]
fn level_with_non_ascii_replay_data() {
    let mut level = GDLevel::default();
    level.integrity.replay_data = Some(vec![0x00, 0xff, 0x80, b'a']);
    assert_levels_eq(&GDLevel::from_dict(&level.to_dict()).unwrap(), &level);
}

#[test]
fn level_without_level_data() {
    let mut level = GDLevel::default();
    level.content.data = None;
    let parsed = GDLevel::from_dict(&level.to_dict()).unwrap();
    assert!(parsed.content.data.is_none());
}

#[test]
fn level_encrypted_data_is_passed_through() {
    let mut level = GDLevel::default();
    let data = sample_level_data().serialise_to_string();
    level.content.data = Some(GDLevelState::Encrypted(GDEncryptedLevelData {
        data: data.clone(),
    }));
    let parsed = GDLevel::from_dict(&level.to_dict()).unwrap();
    assert_eq!(
        parsed.content.data,
        Some(GDLevelState::Encrypted(GDEncryptedLevelData { data }))
    );
}

#[test]
fn level_with_unknown_values_of_every_plist_type() {
    let mut level = GDLevel::default();
    let mut nested = Dictionary::new();
    nested.insert("a".into(), Value::from(1));
    level.unknowns.other = HashMap::from([
        ("u_int".to_string(), Value::from(5)),
        ("u_str".to_string(), Value::from("s")),
        ("u_real".to_string(), Value::Real(0.5)),
        ("u_true".to_string(), Value::Boolean(true)),
        ("u_false".to_string(), Value::Boolean(false)),
        ("u_dict".to_string(), Value::Dictionary(nested)),
    ]);
    let parsed = GDLevel::from_plist_string(level.serialise_to_string()).unwrap();
    assert_levels_eq(&parsed, &level);
}

// ---------------- GDList ----------------

#[test]
fn list_default_dict() {
    let list = GDList::default();
    assert_lists_eq(&GDList::from_dictionary(&list.to_dict()).unwrap(), &list);
}

#[test]
fn list_sample_dict() {
    let list = sample_list();
    assert_lists_eq(&GDList::from_dictionary(&list.to_dict()).unwrap(), &list);
}

#[test]
fn list_with_unicode_description() {
    let mut list = GDList::default();
    list.description = "Описание ✓ 日本".into();
    list.name = "Liste élevée".into();
    assert_lists_eq(&GDList::from_dictionary(&list.to_dict()).unwrap(), &list);
}

#[test]
fn list_with_xml_special_characters() {
    let mut list = sample_list();
    list.name = "Levels & <stuff>".into();
    let s = stringify_xml(&list.to_dict(), true);
    let s = proper_plist_tags(format!("<plist version=\"1.0\">{s}</plist>")).unwrap();
    let d = Value::from_reader_xml(s.as_bytes())
        .expect("list name was not escaped")
        .into_dictionary()
        .unwrap();
    assert_lists_eq(&GDList::from_dictionary(&d).unwrap(), &list);
}

// ---------------- CCLocalLevels ----------------

#[test]
fn cclocallevels_string() {
    let mut second = GDLevel::default();
    second.identity.name = "second".into();
    let mut cc = CCLocalLevels {
        levels: vec![sample_level(), second],
        binary_version: GDVersion::GD22082,
        lists: vec![sample_list(), GDList::default()],
    };
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
fn cclocallevels_keeps_level_order_past_ten_levels() {
    // levels are stored as k_0, k_1, ..., k_10, k_11. The order must not become k_0, k_1, k_10, k_11, k_2, ...
    let mut cc = CCLocalLevels {
        levels: (0..15)
            .map(|i| {
                let mut l = GDLevel::default();
                l.identity.name = format!("level {i}");
                l
            })
            .collect(),
        binary_version: GDVersion::GD22082,
        lists: vec![],
    };
    let parsed = CCLocalLevels::from_decrypted(cc.serialise_to_string()).unwrap();
    let names = |c: &CCLocalLevels| {
        c.levels
            .iter()
            .map(|l| l.identity.name.clone())
            .collect::<Vec<_>>()
    };
    assert_eq!(names(&parsed), names(&cc));
}

#[test]
fn cclocallevels_empty() {
    let mut cc = CCLocalLevels {
        levels: vec![],
        binary_version: GDVersion::GD22082,
        lists: vec![],
    };
    let parsed = CCLocalLevels::from_decrypted(cc.serialise_to_string()).unwrap();
    assert!(parsed.levels.is_empty());
    assert!(parsed.lists.is_empty());
}
