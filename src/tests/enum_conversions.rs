//! Tests for numeric <-> enum conversions: `repr_t!` enums, object type enums, `GDVersion`, `ColourChannel`, and friends.
//!
//! These conversions are what every serialiser and deserialiser in the crate is built on.
use crate::{
    ccgamemanager::{
        achievements::{GDAchievement, MAX_ACHIEVEMENT_INDEX},
        structs::{GDStatistics, Resolution, TextureQuality},
    },
    cclocallevels::{
        gdlevel::{
            enums::*,
            leveldata::{GuidelineColour, PlayerColour},
            version::GDVersion,
        },
        gdobj::{
            constructors::SpinningObject,
            gameplay::{ForceObject, GDOrbType, GDPadType, GameplayPortal, SpeedPortal},
            structs::*,
        },
    },
    core::structs::KCEKValue,
};

/// Checks a `strict` repr_t enum: every value it accepts converts back to the same number,
/// and values outside of its range are returned as the error.
macro_rules! check_strict {
    ($enum:ty, $range:expr, expected_variants: $count:expr) => {{
        let mut accepted = 0;
        for i in $range {
            match <$enum>::try_from(i) {
                Ok(v) => {
                    accepted += 1;
                    assert_eq!(v as i32, i, "{} {v:?}", stringify!($enum));
                }
                Err(e) => assert_eq!(e, i, stringify!($enum)),
            }
        }
        assert_eq!(accepted, $count, "number of {} variants", stringify!($enum));
    }};
}

/// Checks a non-strict repr_t enum: every number survives `from` -> `to_num`, recognised or not.
macro_rules! check_lenient {
    ($enum:ident, $range:expr, expected_variants: $count:expr) => {{
        let mut recognised = 0;
        for i in $range {
            let v = <$enum>::from(i);
            assert_eq!(v.to_num(), i, "{} {v:?}", stringify!($enum));
            if !matches!(v, $enum::Unrecognized(_)) {
                recognised += 1;
            }
        }
        for i in [i32::MIN, i32::MAX] {
            assert_eq!(<$enum>::from(i).to_num(), i, stringify!($enum));
        }
        assert_eq!(
            recognised,
            $count,
            "number of {} variants",
            stringify!($enum)
        );
    }};
}

#[test]
fn strict_object_value_enums() {
    check_strict!(MoveEasing, -5..=30, expected_variants: 19);
    check_strict!(ExtraID2, -5..=10, expected_variants: 3);
    check_strict!(Gamemode, -5..=10, expected_variants: 8);
    check_strict!(StopMode, -5..=10, expected_variants: 3);
    check_strict!(ItemAlign, -5..=10, expected_variants: 3);
    check_strict!(TransitionMode, -5..=10, expected_variants: 3);
    check_strict!(TransitionType, -5..=2000, expected_variants: 13);
    check_strict!(Op, -5..=10, expected_variants: 5);
    check_strict!(CompareOp, -5..=10, expected_variants: 6);
    check_strict!(RoundMode, -5..=10, expected_variants: 4);
    check_strict!(SignMode, -5..=10, expected_variants: 3);
    check_strict!(UIReferencePos, -5..=10, expected_variants: 8);
    check_strict!(AxisOnlyMove, -5..=10, expected_variants: 3);
    check_strict!(Speed, -5..=10, expected_variants: 5);
    check_strict!(OptionalPlayerTarget, -5..=10, expected_variants: 3);
    check_strict!(TouchToggle, -5..=10, expected_variants: 3);
    check_strict!(BPMSpeed, -5..=10, expected_variants: 5);
    check_strict!(PickupTriggerMode, -5..=10, expected_variants: 3);
    check_strict!(InstantCountComparison, -5..=10, expected_variants: 3);
    check_strict!(ArrowDirection, -5..=10, expected_variants: 4);
    check_strict!(Toggle, -5..=10, expected_variants: 3);
    check_strict!(TeleportGravity, -5..=10, expected_variants: 4);
}

#[test]
fn strict_savefile_enums() {
    check_strict!(TextureQuality, -5..=10, expected_variants: 4);
    check_strict!(Resolution, -5..=40, expected_variants: 27);
}

#[test]
fn object_type_enums() {
    check_strict!(GDPadType, -5..=5000, expected_variants: 5);
    check_strict!(GDOrbType, -5..=5000, expected_variants: 11);
    check_strict!(ForceObject, -5..=5000, expected_variants: 2);
    check_strict!(SpeedPortal, -5..=5000, expected_variants: 5);
    check_strict!(GameplayPortal, -5..=5000, expected_variants: 21);
    check_strict!(SpinningObject, -5..=5000, expected_variants: 73);
}

#[test]
fn lenient_enums() {
    check_lenient!(ItemType, -10..=20, expected_variants: 5);
    check_lenient!(CounterMode, -10..=20, expected_variants: 4);
    check_lenient!(Event, -10..=100, expected_variants: 78);
    check_lenient!(MiddleGround, -10..=20, expected_variants: 4);
    check_lenient!(KCEKValue, -10..=20, expected_variants: 8);
    check_lenient!(GDListType, -10..=20, expected_variants: 4);
    check_lenient!(GDListDifficulty, -10..=20, expected_variants: 12);
    check_lenient!(EpicRating, -10..=20, expected_variants: 4);
    check_lenient!(GDLevelType, -10..=20, expected_variants: 4);
    check_lenient!(DemonType, -10..=20, expected_variants: 6);
    check_lenient!(Length, -10..=20, expected_variants: 6);
    check_lenient!(OfficialSong, -10..=100, expected_variants: 41);
    check_lenient!(FeatureRating, -10..=20, expected_variants: 6);
    check_lenient!(DifficultyRating, -10..=100, expected_variants: 6);
}

#[test]
fn enum_defaults() {
    assert_eq!(ItemType::default(), ItemType::Counter);
    assert_eq!(CounterMode::default(), CounterMode::None);
    assert_eq!(MoveEasing::default(), MoveEasing::None);
    assert_eq!(ExtraID2::default(), ExtraID2::All);
    assert_eq!(Gamemode::default(), Gamemode::Cube);
    assert_eq!(StopMode::default(), StopMode::Stop);
    assert_eq!(ItemAlign::default(), ItemAlign::Center);
    assert_eq!(TransitionMode::default(), TransitionMode::Both);
    assert_eq!(TransitionType::default(), TransitionType::None);
    assert_eq!(Op::default(), Op::Set);
    assert_eq!(CompareOp::default(), CompareOp::Equals);
    assert_eq!(RoundMode::default(), RoundMode::None);
    assert_eq!(SignMode::default(), SignMode::None);
    assert_eq!(UIReferencePos::default(), UIReferencePos::XAuto);
    assert_eq!(AxisOnlyMove::default(), AxisOnlyMove::Unset);
    assert_eq!(Speed::default(), Speed::X1);
    assert_eq!(MiddleGround::default(), MiddleGround::None);
    assert_eq!(OptionalPlayerTarget::default(), OptionalPlayerTarget::None);
    assert_eq!(TouchToggle::default(), TouchToggle::None);
    assert_eq!(BPMSpeed::default(), BPMSpeed::Normal);
    assert_eq!(PickupTriggerMode::default(), PickupTriggerMode::Add);
    assert_eq!(
        InstantCountComparison::default(),
        InstantCountComparison::Equals
    );
    assert_eq!(ArrowDirection::default(), ArrowDirection::Right);
    assert_eq!(Toggle::default(), Toggle::Unset);
    assert_eq!(TeleportGravity::default(), TeleportGravity::Unset);
    assert_eq!(GDListType::default(), GDListType::None);
    assert_eq!(GDListDifficulty::default(), GDListDifficulty::NA);
    assert_eq!(EpicRating::default(), EpicRating::None);
    assert_eq!(GDLevelType::default(), GDLevelType::Local);
    assert_eq!(Length::default(), Length::Tiny);
    assert_eq!(FeatureRating::default(), FeatureRating::Unrated);
    assert_eq!(DifficultyRating::default(), DifficultyRating::None);
    assert_eq!(TextureQuality::default(), TextureQuality::Auto);
    assert_eq!(Resolution::default(), Resolution::R1920x1080);
    assert_eq!(ZLayer::default(), ZLayer::Default);
    assert_eq!(ColourChannel::default(), ColourChannel::Object);
    assert_eq!(GDVersion::default(), GDVersion::GD1000);
    assert_eq!(PlayerColour::default(), PlayerColour::None);
}

#[test]
fn event_order_matches_gd() {
    // `CheckpointRespawn` is declared after `PlayerReversed`, but its value is 64
    assert_eq!(Event::CheckpointRespawn.to_num(), 64);
    assert_eq!(Event::from(64), Event::CheckpointRespawn);
    assert_eq!(Event::PlayerReversed.to_num(), 75);
    assert_eq!(Event::FallSpeedLow.to_num(), 76);
}

#[test]
fn achievements() {
    for i in 1..=MAX_ACHIEVEMENT_INDEX as i32 {
        let a = GDAchievement::try_from(i).unwrap_or_else(|_| panic!("no achievement {i}"));
        assert_eq!(a as i32, i);
        assert_eq!(GDAchievement::parse_str(a.to_ident()), Some(a), "{a:?}");
    }
    assert!(GDAchievement::try_from(0).is_err());
    assert!(GDAchievement::try_from(MAX_ACHIEVEMENT_INDEX as i32 + 1).is_err());
    assert_eq!(GDAchievement::parse_str("geometry.ach.doesnotexist"), None);
}

#[test]
fn achievement_idents_are_unique() {
    let mut idents = (1..=MAX_ACHIEVEMENT_INDEX as i32)
        .map(|i| GDAchievement::try_from(i).unwrap().to_ident().to_string())
        .collect::<Vec<_>>();
    idents.sort();
    idents.dedup();
    assert_eq!(idents.len(), MAX_ACHIEVEMENT_INDEX);
}

#[test]
fn achievement_storage() {
    let mut stats = GDStatistics::default();
    for i in 1..=MAX_ACHIEVEMENT_INDEX as i32 {
        let a = GDAchievement::try_from(i).unwrap();
        stats.set_vanilla_achievement(a, i as u16);
    }
    for i in 1..=MAX_ACHIEVEMENT_INDEX as i32 {
        let a = GDAchievement::try_from(i).unwrap();
        assert_eq!(stats.get_vanilla_achievement(a), i as u16);
        assert_eq!(stats.get_achievement_by_ident(a.to_ident()), Some(i as u16));
    }
}

#[test]
fn custom_achievements() {
    let mut stats = GDStatistics::default();
    assert_eq!(stats.get_achievement_by_ident("mod.ach.custom"), None);
    stats.set_achievement_by_ident("mod.ach.custom", 10_000);
    assert_eq!(
        stats.get_achievement_by_ident("mod.ach.custom"),
        Some(10_000)
    );
    stats.set_achievement_by_ident("geometry.ach.level01b", 100);
    assert_eq!(stats.get_vanilla_achievement(GDAchievement::Level01b), 100);
    assert!(
        !stats
            .custom_achievements
            .contains_key("geometry.ach.level01b")
    );
}

#[test]
fn colour_channels() {
    for i in i16::MIN..=i16::MAX {
        assert_eq!(i16::from(ColourChannel::from(i)), i);
    }
    for (ch, id) in [
        (ColourChannel::Background, 1000),
        (ColourChannel::Ground1, 1001),
        (ColourChannel::Line, 1002),
        (ColourChannel::ThreeDLine, 1003),
        (ColourChannel::Object, 1004),
        (ColourChannel::P1, 1005),
        (ColourChannel::P2, 1006),
        (ColourChannel::Ground2, 1009),
        (ColourChannel::MiddleGround, 1013),
        (ColourChannel::MiddleGround2, 1014),
    ] {
        assert_eq!(ColourChannel::from(id), ch);
        assert_eq!(i16::from(ch), id);
    }
}

#[test]
fn z_layers() {
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
        assert_eq!(ZLayer::from(z as i32), z);
    }
}

#[test]
fn gd_versions() {
    let all = [
        GDVersion::GD1000,
        GDVersion::GD1010,
        GDVersion::GD1020,
        GDVersion::GD1100,
        GDVersion::GD1110,
        GDVersion::GD1200,
        GDVersion::GD1210,
        GDVersion::GD1220,
        GDVersion::GD1300,
        GDVersion::GD1400,
        GDVersion::GD1410,
        GDVersion::GD1500,
        GDVersion::GD1510,
        GDVersion::GD1600,
        GDVersion::GD1700,
        GDVersion::GD1710,
        GDVersion::GD1800,
        GDVersion::GD1810,
        GDVersion::GD1811,
        GDVersion::GD1900,
        GDVersion::GD1910,
        GDVersion::GD1920,
        GDVersion::GD1930,
        GDVersion::GD2000,
        GDVersion::GD2001,
        GDVersion::GD2010,
        GDVersion::GD2011,
        GDVersion::GD2100,
        GDVersion::GD2110,
        GDVersion::GD2111,
        GDVersion::GD2113,
        GDVersion::GD2200,
        GDVersion::GD2201,
        GDVersion::GD2202,
        GDVersion::GD2203,
        GDVersion::GD2204,
        GDVersion::GD2205,
        GDVersion::GD2206,
        GDVersion::GD2207,
        GDVersion::GD22071,
        GDVersion::GD22072,
        GDVersion::GD22073,
        GDVersion::GD22074,
        GDVersion::GD2208,
        GDVersion::GD22081,
        GDVersion::GD22082,
        GDVersion::GD2209,
        GDVersion::GD22091,
    ];
    for v in all {
        // every known binary version maps back onto a version with the same binary version
        if let Some(bv) = v.to_binary_version_id() {
            assert_eq!(
                GDVersion::from_binary_version(bv).to_binary_version_id(),
                Some(bv),
                "{v:?}"
            );
            assert!(
                !matches!(GDVersion::from_binary_version(bv), GDVersion::Unknown(_)),
                "{v:?}"
            );
        }
        // every released version has a game version
        assert!(
            v.to_game_version_id().is_some(),
            "{v:?} has no game version id"
        );
    }
}

#[test]
fn gd_version_from_binary_version_keeps_game_version() {
    // documented on `from_binary_version`
    for bv in 0..=60 {
        let v = GDVersion::from_binary_version(bv);
        if !matches!(v, GDVersion::Unknown(_)) {
            assert!(v.to_game_version_id().is_some(), "{v:?}");
        }
    }
}

#[test]
fn gd_version_unknown() {
    for bv in [-1, 3, 8, 50, 52, 1000, i32::MAX] {
        assert_eq!(GDVersion::from_binary_version(bv), GDVersion::Unknown(bv));
        assert_eq!(GDVersion::Unknown(bv).to_binary_version_id(), Some(bv));
    }
}

#[test]
fn gd_version_examples() {
    assert_eq!(GDVersion::GD22082.to_binary_version_id(), Some(48));
    assert_eq!(GDVersion::GD22082.to_game_version_id(), Some(22));
    assert_eq!(GDVersion::from_binary_version(47), GDVersion::GD22081);
    assert_eq!(GDVersion::GD2207.to_binary_version_id(), None);
    assert_eq!(GDVersion::GD2113.to_manifest_version(), None);
    assert_eq!(GDVersion::GD2111.to_manifest_version(), Some(33));
}

#[test]
fn items() {
    for item in [
        Item::Counter(0),
        Item::Counter(5),
        Item::Counter(-1),
        Item::Timer(9999),
        Item::Points,
        Item::Attempts,
        Item::MainTime,
    ] {
        assert_eq!(
            Item::from_id_type(item.id(), item.get_type()),
            item,
            "{item:?}"
        );
        assert_eq!(item.get_type_as_i32(), item.get_type().to_num());
        if let Some(mode) = item.as_special_mode() {
            assert_eq!(Item::from_counter_mode(mode), item);
            assert_eq!(item.as_special_mode_i32(), Some(mode.to_num()));
        }
    }
    assert_eq!(Item::Counter(3).as_special_mode(), None);
    assert_eq!(Item::Timer(3).as_special_mode(), None);
    assert_eq!(Item::Points.id(), 0);
    assert_eq!(Item::from_counter_mode(CounterMode::None), Item::Counter(0));
    assert_eq!(
        Item::from_counter_mode(CounterMode::Unrecognized(-50)),
        Item::Counter(0)
    );
    assert_eq!(
        Item::from_id_type(4, ItemType::Unrecognized(77)),
        Item::Counter(0)
    );
}

#[test]
fn animations() {
    assert_eq!(Anim::Other(42).to_num(), 42);
    assert_eq!(Anim::Bat(animation_ids::Bat::Attack02Loop).to_num(), 9);
    assert_eq!(Anim::BigBeast(animation_ids::BigBeast::Idle01).to_num(), 3);
    assert_eq!(
        Anim::Spikeball(animation_ids::Spikeball::FromAttack03).to_num(),
        8
    );
    assert_eq!(Anim::from(7), Anim::Other(7));
    assert_eq!(Anim::default(), Anim::Other(0));
}

#[test]
fn easings() {
    assert_eq!(
        <Easing as From<MoveEasing>>::from(MoveEasing::BounceIn).easing_rate,
        DEFAULT_EASING_RATE
    );
    assert_eq!(
        MoveEasing::from(Easing::from(MoveEasing::SineOut, 0.5)),
        MoveEasing::SineOut
    );
    assert_eq!(Easing::none(), Easing::from(MoveEasing::None, 0.0));
}

#[test]
fn target_player_ids() {
    assert_eq!(TargetPlayer::Player1 as u16, 138);
    assert_eq!(TargetPlayer::Player2 as u16, 200);
    assert_eq!(TargetPlayer::PlayerTarget as u16, 201);
}

#[test]
fn guideline_colours() {
    for c in [
        GuidelineColour::Orange,
        GuidelineColour::Yellow,
        GuidelineColour::Green,
    ] {
        assert_eq!(GuidelineColour::from_f32(c.to_float()), c);
    }
    assert_eq!(
        GuidelineColour::from_f32(GuidelineColour::Transparent.to_float()),
        GuidelineColour::Transparent
    );
}

#[test]
fn official_song_ids() {
    assert_eq!(OfficialSong::StayInsideMe.to_client_id(), None);
    assert_eq!(OfficialSong::StereoMadness.to_client_id(), Some(1));
    assert_eq!(OfficialSong::HexagonForce.to_client_id(), Some(16));
    assert_eq!(OfficialSong::Explorers.to_client_id(), Some(23));
    assert_eq!(OfficialSong::PowerTrip.to_client_id(), Some(4003));
    assert_eq!(OfficialSong::Unrecognized(500).to_client_id(), None);
    assert_eq!(OfficialSong::Fingerdash.to_server_id(), 20);

    // every recognised song except the practice song has a unique client id
    let mut ids = (0..=39)
        .map(|i| {
            OfficialSong::from(i)
                .to_client_id()
                .expect("missing client id")
        })
        .collect::<Vec<_>>();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), 40);
}

#[test]
fn orb_types() {
    assert!(GDOrbType::GreenDash.is_dash_orb());
    assert!(GDOrbType::PinkDash.is_dash_orb());
    assert!(!GDOrbType::Yellow.is_dash_orb());
    assert!(!GDOrbType::Teleport.is_dash_orb());
}

#[test]
fn speed_portal_speeds_increase() {
    let speeds = [
        SpeedPortal::Yellow,
        SpeedPortal::Blue,
        SpeedPortal::Green,
        SpeedPortal::Pink,
        SpeedPortal::Red,
    ]
    .map(|s| s.get_speed());
    assert!(speeds.windows(2).all(|w| w[0] < w[1]), "{speeds:?}");
}
