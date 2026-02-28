//! This module contains the GDObject struct, used for parsing to/from raw object strings
//! This module also contains the GDObjConfig struct for creating new GDObjects
use std::fmt::{Debug, Display, Write};

use crate::gdobj::{
    ids::properties::{
        CENTER_EFFECT, DONT_BOOST_X, DONT_BOOST_Y, DONT_ENTER, DONT_FADE, EDITOR_LAYER_1,
        EDITOR_LAYER_2, ENTER_EFFECT_CHANNEL, EXTRA_STICKY, GRIP_SLOPE, GROUPS,
        HAS_EXTENDED_COLLISION, HIDDEN, IS_AREA_PARENT, IS_GROUP_PARENT, IS_HIGH_DETAIL,
        IS_ICE_BLOCK, MATERIAL_CONTROL_ID, MULTITRIGGERABLE, NO_AUDIO_SCALE, NO_END_EFFECTS,
        NO_GLOW, NO_OBJECT_EFFECTS, NO_PARTICLES, NO_TOUCH, NONSTICK_X, NONSTICK_Y, OBJECT_COLOUR,
        OBJECT_ID, OBJECT_MATERIAL, PARENT_GROUPS, PASSABLE, REVERSES_GAMEPLAY, ROTATION,
        SCALE_STICK, SECONDARY_COLOUR, SINGLE_PLAYER_TOUCH, SPAWN_TRIGGERABLE, TOUCH_TRIGGERABLE,
        X_POS, X_SCALE, Y_POS, Y_SCALE, Z_LAYER, Z_ORDER,
    },
    lookup::get_property_type,
};
use itoa;
use smallvec::SmallVec;

pub mod ids;
pub mod lookup;
pub mod misc;
pub mod triggers;

pub mod animation_ids {
    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum BigBeast {
        Bite = 0,
        Attack01 = 1,
        Attack01End = 2,
        Idle01 = 3,
    }
    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Bat {
        Idle01 = 0,
        Idle02 = 1,
        Idle03 = 2,
        Attack01 = 3,
        Attack02 = 4,
        Attack02End = 5,
        Sleep = 6,
        SleepLoop = 7,
        SleepEnd = 8,
        Attack02Loop = 9,
    }
    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Spikeball {
        Idle01 = 0,
        Idle02 = 1,
        ToAttack01 = 2,
        Attack01 = 3,
        Attack02 = 4,
        ToAttack03 = 5,
        Attack03 = 6,
        Idle03 = 7,
        FromAttack03 = 8,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Anim {
    Other(i32),
    BigBeast(animation_ids::BigBeast),
    Bat(animation_ids::Bat),
    Spikeball(animation_ids::Spikeball),
}

impl Anim {
    pub fn as_i32(&self) -> i32 {
        match self {
            Self::Bat(b) => *b as i32,
            Self::BigBeast(b) => *b as i32,
            Self::Spikeball(s) => *s as i32,
            Self::Other(i) => *i,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
    Counter(i16),
    Timer(i16),
    Points,
    Attempts,
    MainTime,
}

impl Item {
    pub fn get_type(&self) -> ItemType {
        match self {
            Self::Attempts => ItemType::Attempts,
            Self::Counter(_) => ItemType::Counter,
            Self::MainTime => ItemType::MainTime,
            Self::Points => ItemType::Points,
            Self::Timer(_) => ItemType::Timer,
        }
    }
    pub fn get_type_as_i32(&self) -> i32 {
        self.get_type() as i32
    }
    pub fn as_special_mode(&self) -> Option<CounterMode> {
        match self {
            Self::Attempts => Some(CounterMode::Attempts),
            Self::MainTime => Some(CounterMode::MainTime),
            Self::Points => Some(CounterMode::Points),
            _ => None,
        }
    }
    pub fn as_special_mode_i32(&self) -> i32 {
        self.as_special_mode().unwrap() as i32
    }

    pub fn id(&self) -> i16 {
        match self {
            Self::Counter(c) => *c,
            Self::Timer(t) => *t,
            _ => 0,
        }
    }
}

/// Enum for counter types
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemType {
    Counter = 1,
    Timer = 2,
    Points = 3,
    MainTime = 4,
    Attempts = 5,
}

/// Enum for counter modes
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CounterMode {
    Attempts = -3,
    Points = -2,
    MainTime = -1,
}

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum GDObjPropType {
    Int,
    Float,
    Text,
    Bool,
    Group,
    Item,
    Easing,
    EventsList,
    ColourChannel,
    Toggle,
    Unknown,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ZLayer {
    B5 = -5,
    B4 = -3,
    B3 = -1,
    B2 = 1,
    B1 = 3,
    #[default]
    Default = 0,
    T1 = 5,
    T2 = 7,
    T3 = 9,
    T4 = 11,
}

impl ZLayer {
    pub fn from_i32(int: i32) -> Self {
        match int {
            -5 => Self::B5,
            -3 => Self::B4,
            -1 => Self::B3,
            1 => Self::B2,
            3 => Self::B1,
            5 => Self::T1,
            7 => Self::T2,
            9 => Self::T3,
            11 => Self::T4,
            _ => Self::Default,
        }
    }
}

/// Enum for colour channels and their IDs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ColourChannel {
    Channel(i32),
    Background,
    Ground1,
    Ground2,
    Line,
    #[default]
    Object,
    ThreeDLine,
    MiddleGround,
    MiddleGround2,
    P1,
    P2,
}

impl ColourChannel {
    pub fn as_i32(&self) -> i32 {
        match self {
            Self::Background => 1000,
            Self::Channel(n) => *n,
            Self::Ground1 => 1001,
            Self::Ground2 => 1009,
            Self::Line => 1002,
            Self::Object => 1004,
            Self::ThreeDLine => 1003,
            Self::MiddleGround => 1013,
            Self::MiddleGround2 => 1014,
            Self::P1 => 1005,
            Self::P2 => 1006,
        }
    }

    pub fn from_i32(c: i32) -> Self {
        match c {
            1000 => Self::Background,
            1001 => Self::Ground1,
            1009 => Self::Ground2,
            1002 => Self::Line,
            1004 => Self::Object,
            1003 => Self::ThreeDLine,
            1013 => Self::MiddleGround,
            1014 => Self::MiddleGround2,
            1005 => Self::P1,
            1006 => Self::P2,
            n => Self::Channel(n),
        }
    }
}

/// Enum for all the move easings
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MoveEasing {
    #[default]
    None = 0,
    EaseInOut = 1,
    EaseIn = 2,
    EaseOut = 3,
    ElasticInOut = 4,
    ElasticIn = 5,
    ElasticOut = 6,
    BounceInOut = 7,
    BounceIn = 8,
    BounceOut = 9,
    ExponentialInOut = 10,
    ExponentialIn = 11,
    ExponentialOut = 12,
    SineInOut = 13,
    SineIn = 14,
    SineOut = 15,
    BackInOut = 16,
    BackIn = 17,
    BackOut = 18,
}

impl MoveEasing {
    pub fn from_i32(i: i32) -> Self {
        match i {
            1 => Self::EaseInOut,
            2 => Self::EaseIn,
            3 => Self::EaseOut,
            4 => Self::ElasticInOut,
            5 => Self::ElasticIn,
            6 => Self::ElasticOut,
            7 => Self::BounceInOut,
            8 => Self::BounceIn,
            9 => Self::BounceOut,
            10 => Self::ExponentialInOut,
            11 => Self::ExponentialIn,
            12 => Self::ExponentialOut,
            13 => Self::SineInOut,
            14 => Self::SineIn,
            15 => Self::SineOut,
            16 => Self::BackInOut,
            17 => Self::BackIn,
            18 => Self::BackOut,
            _ => Self::None,
        }
    }
}

const LIST_ALLOCSIZE: usize = 5;

#[derive(Debug, Clone, PartialEq)]
pub enum GDValue {
    Int(i32),
    Float(f64),
    Bool(bool),
    Toggle(bool),
    Group(i16),
    Item(i16),
    GroupList(smallvec::SmallVec<[i16; LIST_ALLOCSIZE]>),
    ProbabilitiesList(smallvec::SmallVec<[(i16, i32); LIST_ALLOCSIZE]>),
    Easing(MoveEasing),
    ColourChannel(ColourChannel),
    ZLayer(ZLayer),
    Events(Vec<Event>),
    String(String), // fallback
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    // zamn!! that's a lot of events
    TinyLanding = 1,
    FeatherLanding = 2,
    SoftLanding = 3,
    NormalLanding = 4,
    HardLanding = 5,
    HitHead = 6,
    OrbTouched = 7,
    OrbActivated = 8,
    PadActivated = 9,
    GravityInverted = 10,
    GravityRestored = 11,
    NormalJump = 12,
    RobotBoostStart = 13,
    RobotBoostStop = 14,
    UFOJump = 15,
    ShipBoostStart = 16,
    ShipBoostEnd = 17,
    SpiderTeleport = 18,
    BallSwitch = 19,
    SwingSwitch = 20,
    WavePush = 21,
    WaveRelease = 22,
    DashStart = 23,
    DashStop = 24,
    Teleported = 25,
    PortalNormal = 26,
    PortalShip = 27,
    PortalBall = 28,
    PortalUFO = 29,
    PortalWave = 30,
    PortalRobot = 31,
    PortalSpider = 32,
    PortalSwing = 33,
    YellowOrb = 34,
    PinkOrb = 35,
    RedOrb = 36,
    GravityOrb = 37,
    GreenOrb = 38,
    DropOrb = 39,
    CustomOrb = 40,
    DashOrb = 41,
    GravityDashOrb = 42,
    SpiderOrb = 43,
    TeleportOrb = 44,
    YellowPad = 45,
    PinkPad = 46,
    RedPad = 47,
    GravityPad = 48,
    SpiderPad = 49,
    PortalGravityFlip = 50,
    PortalGravityNormal = 51,
    PortalGravityInvert = 52,
    PoratlFlip = 53,
    PortalUnflip = 54,
    PortalNormalScale = 55,
    PortalMiniScale = 56,
    PortalDualOn = 57,
    PortalDualOff = 58,
    PortalTeleport = 59,
    Checkpoint = 60,
    DestroyBlock = 61,
    UserCoin = 62,
    PickupItem = 63,
    FallLow = 65,
    FallMed = 66,
    FallHigh = 67,
    FallVHigh = 68,
    JumpPush = 69,
    JumpRelease = 70,
    LeftPush = 71,
    LeftRelease = 72,
    RightPush = 73,
    RightRelease = 74,
    PlayerReversed = 75,
    CheckpointRespawn = 64, // <- intentionally placed here, the ordering follows that in gd.
    FallSpeedLow = 76,
    FallSpeedMed = 77,
    FallSpeedHigh = 78,
}

impl Event {
    /// Converts the event ID to the variant of the [`Event`] enum. Default to TinyLanding.
    pub fn from_i32(i: i32) -> Self {
        match i {
            1 => Self::TinyLanding,
            2 => Self::FeatherLanding,
            3 => Self::SoftLanding,
            4 => Self::NormalLanding,
            5 => Self::HardLanding,
            6 => Self::HitHead,
            7 => Self::OrbTouched,
            8 => Self::OrbActivated,
            9 => Self::PadActivated,
            10 => Self::GravityInverted,
            11 => Self::GravityRestored,
            12 => Self::NormalJump,
            13 => Self::RobotBoostStart,
            14 => Self::RobotBoostStop,
            15 => Self::UFOJump,
            16 => Self::ShipBoostStart,
            17 => Self::ShipBoostEnd,
            18 => Self::SpiderTeleport,
            19 => Self::BallSwitch,
            20 => Self::SwingSwitch,
            21 => Self::WavePush,
            22 => Self::WaveRelease,
            23 => Self::DashStart,
            24 => Self::DashStop,
            25 => Self::Teleported,
            26 => Self::PortalNormal,
            27 => Self::PortalShip,
            28 => Self::PortalBall,
            29 => Self::PortalUFO,
            30 => Self::PortalWave,
            31 => Self::PortalRobot,
            32 => Self::PortalSpider,
            33 => Self::PortalSwing,
            34 => Self::YellowOrb,
            35 => Self::PinkOrb,
            36 => Self::RedOrb,
            37 => Self::GravityOrb,
            38 => Self::GreenOrb,
            39 => Self::DropOrb,
            40 => Self::CustomOrb,
            41 => Self::DashOrb,
            42 => Self::GravityDashOrb,
            43 => Self::SpiderOrb,
            44 => Self::TeleportOrb,
            45 => Self::YellowPad,
            46 => Self::PinkPad,
            47 => Self::RedPad,
            48 => Self::GravityPad,
            49 => Self::SpiderPad,
            50 => Self::PortalGravityFlip,
            51 => Self::PortalGravityNormal,
            52 => Self::PortalGravityInvert,
            53 => Self::PoratlFlip,
            54 => Self::PortalUnflip,
            55 => Self::PortalNormalScale,
            56 => Self::PortalMiniScale,
            57 => Self::PortalDualOn,
            58 => Self::PortalDualOff,
            59 => Self::PortalTeleport,
            60 => Self::Checkpoint,
            61 => Self::DestroyBlock,
            62 => Self::UserCoin,
            63 => Self::PickupItem,
            65 => Self::FallLow,
            66 => Self::FallMed,
            67 => Self::FallHigh,
            68 => Self::FallVHigh,
            69 => Self::JumpPush,
            70 => Self::JumpRelease,
            71 => Self::LeftPush,
            72 => Self::LeftRelease,
            73 => Self::RightPush,
            74 => Self::RightRelease,
            75 => Self::PlayerReversed,
            64 => Self::CheckpointRespawn,
            76 => Self::FallSpeedLow,
            77 => Self::FallSpeedMed,
            // this will be the default because i said so
            // it has event id 78
            _ => Self::FallSpeedHigh,
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ExtraID2 {
    #[default]
    All = 0,
    P1 = 1,
    P2 = 2,
}

// for debug purposes

// fn parse_with_err_handle<T>(s: &str, p: u16) -> T
// where
//     T: FromStr + Default + Display,
//     <T as FromStr>::Err: Debug,
// {
//     match s.parse::<T>() {
//         Ok(n) => n,
//         Err(e) => {
//             println!(
//                 "Error with parsing property {p} with value {s}, type {} ({e:?})",
//                 type_name::<T>()
//             );
//             T::default()
//         }
//     }
// }

macro_rules! parse {
    ($v:expr => $t:ty) => {
        $v.parse::<$t>().unwrap_or_default()
    };
}

impl GDValue {
    pub fn from(t: GDObjPropType, s: &str) -> Self {
        match t {
            GDObjPropType::Bool => Self::Bool(s == "1"),
            GDObjPropType::Toggle => Self::Toggle(s == "1"),
            GDObjPropType::ColourChannel => Self::ColourChannel(ColourChannel::from_i32(
                s.parse::<i32>().unwrap_or_default(),
            )),
            GDObjPropType::Easing => {
                Self::Easing(MoveEasing::from_i32(s.parse::<i32>().unwrap_or_default()))
            }
            GDObjPropType::Float => Self::Float(s.parse::<f64>().unwrap_or_default()),
            GDObjPropType::Int => Self::Int(s.parse::<i32>().unwrap_or_default()),
            GDObjPropType::EventsList => Self::Events(
                s.split('.')
                    .into_iter()
                    .map(|i| Event::from_i32(parse!(i => i32)))
                    .collect(),
            ),
            GDObjPropType::Group => Self::Group(s.parse::<i16>().unwrap_or_default()),
            GDObjPropType::Item => Self::Item(s.parse::<i16>().unwrap_or_default()),
            GDObjPropType::Text | GDObjPropType::Unknown => Self::String(s.to_owned()),
        }
    }

    #[inline(always)]
    pub fn from_group_list(g: Vec<Group>) -> Self {
        Self::GroupList(SmallVec::from_vec(g.iter().map(|&g| g.id()).collect()))
    }

    #[inline(always)]
    pub fn parents_group_list(g: Vec<Group>) -> Self {
        Self::GroupList(SmallVec::from_vec(
            g.iter()
                .filter_map(|g| match g {
                    Group::Parent(p) => Some(*p),
                    Group::Regular(_) => None,
                })
                .collect(),
        ))
    }

    #[inline(always)]
    pub fn from_prob_list(g: Vec<(i16, i32)>) -> Self {
        Self::ProbabilitiesList(SmallVec::from_vec(g))
    }

    #[inline(always)]
    pub fn colour_channel(s: &str) -> Self {
        Self::ColourChannel(ColourChannel::from_i32(s.parse().unwrap_or(0)))
    }

    #[inline(always)]
    pub fn zlayer(s: &str) -> Self {
        Self::ZLayer(ZLayer::from_i32(s.parse().unwrap_or(0)))
    }
}

macro_rules! fmt_intlist {
    // Vec<int>
    ($vals:expr, $i_buf:expr) => {{
        let mut items_str = String::with_capacity($vals.len() * 4);
        for (idx, item) in $vals.iter().enumerate() {
            if idx != 0 {
                items_str.push('.');
            }
            items_str.push_str($i_buf.format(*item as i32));
        }
        items_str
    }};
}

macro_rules! fmt_inttuples {
    // Vec<(int, int)>
    ($vals:expr, $i_buf:expr) => {{
        let mut items_str = String::with_capacity($vals.len() * 8);
        for (idx, item) in $vals.iter().enumerate() {
            if idx != 0 {
                items_str.push('.');
            }
            items_str.push_str($i_buf.format(item.0));
            items_str.push('.');
            items_str.push_str($i_buf.format(item.1));
        }
        items_str
    }};
}

impl Display for GDValue {
    // also the serialisation
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i_buf = itoa::Buffer::new();
        let mut d_buf = dtoa::Buffer::new();

        match self {
            GDValue::Bool(b) => write!(f, "{}", if *b { '1' } else { '0' }),
            GDValue::Toggle(b) => write!(
                f,
                "{}",
                match b {
                    true => "1",
                    false => "-1",
                }
            ),
            GDValue::ColourChannel(v) => write!(f, "{}", i_buf.format(v.as_i32())),
            GDValue::Easing(v) => write!(f, "{}", i_buf.format(*v as i32)),
            GDValue::Float(v) => write!(f, "{}", d_buf.format(*v)),
            GDValue::Group(v) | GDValue::Item(v) => write!(f, "{}", i_buf.format(*v)),
            GDValue::GroupList(v) => write!(f, "{}", fmt_intlist!(v, i_buf)),
            GDValue::ProbabilitiesList(v) => write!(f, "{}", fmt_inttuples!(v, i_buf)),
            GDValue::Int(v) => write!(f, "{}", i_buf.format(*v)),
            GDValue::String(v) => write!(f, "{v}"),
            GDValue::ZLayer(v) => write!(f, "{}", i_buf.format(*v as i32)),
            GDValue::Events(evts) => write!(f, "{}", fmt_intlist!(evts, i_buf)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct GDObjProperty {
    pub name: u16,
    pub desc: &'static str,
    pub arg_type: GDObjPropType,
}

// Map of all object ids to names: (id, name)
const OBJECT_NAMES: &[(i32, &str)] = &[
    (1, "Default block"),
    (2, "Waffle block floor"),
    (3, "Waffle block corner"),
    (4, "Waffle block inner corner"),
    (5, "Waffle block filler"),
    (6, "Waffle block no bottom"),
    (7, "Waffle block straight"),
    (8, "Spike"),
    (9, "Ground spikes"),
    (10, "Normal gravity portal"),
    (11, "Flipped gravity portal"),
    (12, "Cube portal"),
    (13, "Ship portal"),
    (15, "Pulse pole tall"),
    (16, "Pulse pole medium"),
    (17, "Pulse pole short"),
    (18, "Transparent spikes huge"),
    (19, "Transparent spikes big"),
    (20, "Transparent spikes medium"),
    (21, "Transparent spikes small"),
    (22, "No block transition object"),
    (23, "Blocks from top transition object"),
    (24, "Blocks from bottom transition object"),
    (25, "Blocks from left transition object"),
    (26, "Blocks from right transition object"),
    (27, "Scale in transition object"),
    (28, "Scale out transition object"),
    // 29 + 30: mystery colour triggers
    (31, "Start pos"),
    (32, "Enable player trail"),
    (33, "Disable player trail"),
    (34, "Solid startpos"),
    (35, "Yellow pad"),
    (36, "Yellow orb"),
    (39, "Small spike"),
    (40, "Half block default"),
    (41, "Chain tall"),
    (45, "Mirror portal reverse"),
    (46, "Mirror portal normal"),
    (47, "Ball portal"),
    (48, "Transparent clouds big"),
    (49, "Transparent clouds small"),
    (50, "Pulse circle"),
    (51, "Pulse ring"),
    (52, "Pulse heart"),
    (53, "Pulse diamond"),
    (54, "Pulse star"),
    (55, "Random direction transition object"),
    (56, "Away to left transition object"),
    (57, "Away to right transition object"),
    (58, "Away from middle transition object"),
    (59, "Away to middle transition object"),
    (60, "Pulse music note"),
    (61, "Ground spikes wavy"),
    (62, "Wavy block floor"),
    (67, "Blue pad"),
    (83, "Waffle block"),
    (84, "Blue orb"),
    (88, "Buzzsaw big"),
    (89, "Buzzsaw medium"),
    (98, "Buzzsaw small"),
    (99, "Size portal normal"),
    (101, "Size portal small"),
    (111, "UFO portal"),
    (140, "Pink pad"),
    (141, "Pink orb"),
    (200, "Speed portal 0.5x"),
    (201, "Speed portal 1x"),
    (202, "Speed portal 2x"),
    (203, "Speed portal 3x"),
    (286, "Dual portal double"),
    (287, "Dual portal single"),
    (899, "Trigger Colour"),
    (901, "Trigger Move"),
    (914, "Text object"),
    (1006, "Trigger Pulse"),
    (1007, "Trigger Alpha"),
    (1049, "Trigger Toggle"),
    (1268, "Trigger Spawn"),
    (1346, "Trigger Rotation"),
    (1347, "Trigger Follow"),
    (1520, "Trigger Shake"),
    (1585, "Trigger Animate"),
    (1611, "Trigger Count"),
    (1615, "Counter"),
    (1616, "Trigger Stop"),
    (1812, "Trigger On death"),
    (1812, "Trigger follow player y"),
    (1815, "Trigger Collision"),
    (1816, "Collision block"),
    (1818, "BG effect on"),
    (1819, "BG effect off"),
    (1912, "Trigger Random"),
    (1913, "Trigger Camera zoom"),
    (1915, "Don't fade + don't enter transition object"),
    (1917, "Trigger Reverse gameplay"),
    (1932, "Trigger Player control"),
    (1934, "Trigger Song"),
    (1935, "Trigger Time warp"),
    (2016, "Camera guide"),
    (2066, "Trigger Gravity"),
    (2067, "Trigger Scale"),
    (2068, "Trigger Advanced random"),
    (2900, "Trigger rotate gameplay"),
    (2900, "Trigger Middleground config"),
    (3600, "Trigger End"),
    (3604, "Trigger Event"),
    (3606, "BG speed config"),
    (3608, "Trigger Spawn particle"),
    (3609, "Trigger Instant collision"),
    (3612, "MG speed config"),
    (3613, "UI config"),
    (3614, "Trigger Time"),
    (3615, "Trigger Time event"),
    (3617, "Trigger Time control"),
    (3618, "Trigger Reset group"),
    (3619, "Trigger Item edit"),
    (3620, "Trigger Item compare"),
    (3640, "Collision state block"),
    (3641, "Trigger Persistent item"),
    (3643, "Toggle block"),
    (3662, "Trigger Link visible"),
];

/// Container for GD Object properties.
/// * `id`: The object's ID.
/// * `config`: General properties like position and scale.
/// * `properties`: Object-specific properties like target group for a move trigger
#[derive(Clone, PartialEq)]
pub struct GDObject {
    pub id: i32,
    pub config: GDObjConfig,
    pub properties: Vec<(u16, GDValue)>,
}

impl Display for GDObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let group_str = match self.config.groups.len() > 0 {
            true => &format!(
                " with groups: {}",
                self.config
                    .groups
                    .iter()
                    .map(|g| format!("{}", g.id()))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            false => "",
        };

        let mut trigger_conf_str = String::new();
        if self.config.trigger_cfg.spawnable || self.config.trigger_cfg.touchable {
            if self.config.trigger_cfg.multitriggerable {
                trigger_conf_str += "Multi"
            }
            if self.config.trigger_cfg.touchable {
                trigger_conf_str += "touchable "
            } else if self.config.trigger_cfg.spawnable {
                trigger_conf_str += "spawnable "
            }
        }

        write!(
            f,
            "{trigger_conf_str}{} @ ({}, {}) scaled to ({}, {}){} angled to {}°",
            self.name(),
            self.config.pos.0,
            self.config.pos.1,
            self.config.scale.0,
            self.config.scale.1,
            group_str,
            self.config.angle
        )
    }
}

impl Debug for GDObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut property_str = String::with_capacity(self.properties.len() * 32);

        for (property, value) in self.properties.iter() {
            let desc = lookup::PROPERTY_TABLE.get(property).map(|p| p.0);
            if let Some(d) = desc {
                write!(property_str, "\n    - {d}: {value:?}")
            } else {
                write!(property_str, "\n    - {property}: {value:?}")
            }
            .unwrap();
        }

        write!(
            f,
            "{} with properties:{property_str}",
            <Self as ToString>::to_string(self),
        )
    }
}

impl GDObject {
    /// Parses raw object string to GDObject
    ///
    /// Example:
    /// ```
    /// use gdlib::gdobj::{GDObject, GDObjConfig, GDObjProperties};
    ///
    /// let obj = GDObject::parse_str("1,1,2,0,3,0;");
    /// assert_eq!(obj, GDObject::new(1, GDObjConfig::default(), GDObjProperties::new()));
    /// ```
    pub fn parse_str(s: &str) -> GDObject {
        let mut obj = GDObject {
            id: 1,
            config: GDObjConfig::default(),
            properties: vec![],
        };

        let mut iter = s.trim_end_matches(';').split(",");
        while let (Some(idx), Some(val)) = (iter.next(), iter.next()) {
            let idx_u16 = match idx.parse::<u16>() {
                Ok(n) => n,
                Err(_) => match idx[2..].parse::<u16>() {
                    Ok(n) => n + 10_000,
                    Err(_) => 65535,
                },
            };

            match idx_u16 {
                OBJECT_ID => obj.id = val.parse().unwrap_or(0),
                X_POS => obj.config.pos.0 = val.parse().unwrap_or(0.0),
                Y_POS => obj.config.pos.1 = val.parse().unwrap_or(0.0),
                ROTATION => obj.config.angle = val.parse().unwrap_or(0.0),
                TOUCH_TRIGGERABLE => {
                    obj.config.trigger_cfg.touchable = val.parse().unwrap_or(false)
                }
                SPAWN_TRIGGERABLE => {
                    obj.config.trigger_cfg.spawnable = val.parse().unwrap_or(false)
                }
                MULTITRIGGERABLE => {
                    obj.config.trigger_cfg.multitriggerable = val.parse().unwrap_or(false)
                }
                GROUPS => {
                    obj.config.add_groups(
                        val.trim_matches('"')
                            .split(".")
                            .filter_map(|g| g.parse::<i16>().ok())
                            .map(|id| Group::Regular(id))
                            .collect::<Vec<Group>>(),
                    );
                }
                X_SCALE => obj.config.scale.0 = val.parse().unwrap_or(1.0),
                Y_SCALE => obj.config.scale.1 = val.parse().unwrap_or(1.0),
                EDITOR_LAYER_1 => obj.config.editor_layers.0 = val.parse().unwrap_or(0),
                EDITOR_LAYER_2 => obj.config.editor_layers.1 = val.parse().unwrap_or(0),
                OBJECT_COLOUR => {
                    obj.config.colour_channels.0 = ColourChannel::from_i32(val.parse().unwrap_or(0))
                }
                SECONDARY_COLOUR => {
                    obj.config.colour_channels.1 = ColourChannel::from_i32(val.parse().unwrap_or(0))
                }
                Z_LAYER => obj.config.z_layer = ZLayer::from_i32(val.parse().unwrap_or(0)),
                Z_ORDER => obj.config.z_order = val.parse().unwrap_or(0),
                ENTER_EFFECT_CHANNEL => obj.config.enter_effect_channel = val.parse().unwrap_or(0),
                OBJECT_MATERIAL => obj.config.material_id = val.parse().unwrap_or(0),
                DONT_FADE => obj.config.attributes.dont_fade = val.parse().unwrap_or(false),
                DONT_ENTER => obj.config.attributes.dont_enter = val.parse().unwrap_or(false),
                NO_OBJECT_EFFECTS => {
                    obj.config.attributes.no_effects = val.parse().unwrap_or(false)
                }
                IS_GROUP_PARENT => {
                    obj.config.attributes.is_group_parent = val.parse().unwrap_or(false)
                }
                IS_AREA_PARENT => {
                    obj.config.attributes.is_area_parent = val.parse().unwrap_or(false)
                }
                DONT_BOOST_X => obj.config.attributes.dont_boost_x = val.parse().unwrap_or(false),
                DONT_BOOST_Y => obj.config.attributes.dont_boost_y = val.parse().unwrap_or(false),
                IS_HIGH_DETAIL => obj.config.attributes.high_detail = val.parse().unwrap_or(false),
                NO_TOUCH => obj.config.attributes.no_touch = val.parse().unwrap_or(false),
                PASSABLE => obj.config.attributes.passable = val.parse().unwrap_or(false),
                HIDDEN => obj.config.attributes.hidden = val.parse().unwrap_or(false),
                NONSTICK_X => obj.config.attributes.non_stick_x = val.parse().unwrap_or(false),
                NONSTICK_Y => obj.config.attributes.non_stick_y = val.parse().unwrap_or(false),
                EXTRA_STICKY => obj.config.attributes.extra_sticky = val.parse().unwrap_or(false),
                HAS_EXTENDED_COLLISION => {
                    obj.config.attributes.extended_collision = val.parse().unwrap_or(false)
                }
                IS_ICE_BLOCK => obj.config.attributes.is_ice_block = val.parse().unwrap_or(false),
                GRIP_SLOPE => obj.config.attributes.grip_slope = val.parse().unwrap_or(false),
                NO_GLOW => obj.config.attributes.no_glow = val.parse().unwrap_or(false),
                NO_PARTICLES => obj.config.attributes.no_particles = val.parse().unwrap_or(false),
                SCALE_STICK => obj.config.attributes.scale_stick = val.parse().unwrap_or(false),
                NO_AUDIO_SCALE => {
                    obj.config.attributes.no_audio_scale = val.parse().unwrap_or(false)
                }
                SINGLE_PLAYER_TOUCH => {
                    obj.config.attributes.single_ptouch = val.parse().unwrap_or(false)
                }
                CENTER_EFFECT => obj.config.attributes.center_effect = val.parse().unwrap_or(false),
                REVERSES_GAMEPLAY => obj.config.attributes.reverse = val.parse().unwrap_or(false),
                MATERIAL_CONTROL_ID => obj.config.material_control_id = val.parse().unwrap_or(0),
                PARENT_GROUPS => {
                    // add groups method handles deduping
                    obj.config.add_groups(
                        val.trim_matches('"')
                            .split(".")
                            .filter_map(|g| g.parse::<i16>().ok())
                            .map(|id| Group::Parent(id))
                            .collect::<Vec<Group>>(),
                    );
                }
                n => obj.set_property_raw(n, val),
            }
        }

        // obj.properties.sort_by(|a, b| a.0.cmp(&b.0));

        return obj;
    }

    fn set_property_raw(&mut self, p: u16, value: &str) {
        self.set_property(
            p,
            GDValue::from(
                get_property_type(p).unwrap_or(GDObjPropType::Unknown),
                value,
            ),
        );
    }

    /// Sets the prpoerty ID to the value, and craetes it if it doesn't exist
    pub fn set_property(&mut self, p: u16, val: GDValue) {
        if let Some(v) = self.properties.iter_mut().find(|(k, _)| *k == p) {
            v.1 = val;
        } else {
            let new_idx = self.properties.partition_point(|(k, _)| k < &p);
            self.properties.insert(new_idx, (p, val));
        }
    }

    /// Removes the property from this object's property map by its ID.
    pub fn del_property(&mut self, p: u16) {
        if let Ok(idx) = self.properties.binary_search_by_key(&p, |t| t.0) {
            self.properties.remove(idx);
        }
    }

    /// Returns this object as a property string
    ///
    /// Example:
    /// ```
    /// use gdlib::gdobj::{GDObject, GDObjConfig, GDObjProperties};
    ///
    /// let object_str = GDObject::new(1, GDObjConfig::default(), GDObjProperties::new()).to_string();
    /// assert_eq!(object_str, "1,1,2,0.0,3,0.0,64,1,67,1;");
    /// ```
    pub fn to_string(&self) -> String {
        let mut properties_string = String::with_capacity(self.properties.len() * 8);
        for (idx, val) in self.properties.iter() {
            let (pref, id) = if *idx < 10_000 {
                ("", *idx)
            } else {
                ("kA", idx - 10_000) // also need to add a "kA" prepend
            };

            write!(properties_string, ",{pref}{id},{val}").unwrap();
        }
        let config_str = self.config.to_string();

        let raw_str = format!("1,{}{config_str}{properties_string}", self.id);
        return raw_str.replace("\"", "") + ";";
    }

    pub fn name(&self) -> String {
        OBJECT_NAMES
            .iter()
            .find(|&o| o.0 == self.id)
            .unwrap_or(&(0, format!("Object {}", self.id).as_str()))
            .1
            .to_string()
    }

    /// Creates a new GDObject from ID, config, and extra proerties
    #[inline(always)]
    pub fn new(id: i32, config: &GDObjConfig, properties: Vec<(u16, GDValue)>) -> Self {
        GDObject {
            id,
            config: config.clone(),
            properties,
        }
    }

    pub fn get_property(&self, p: u16) -> Option<GDValue> {
        match p {
            // one of the most fascinating matches of all time
            1 => Some(GDValue::Int(self.id)),
            2 => Some(GDValue::Float(self.config.pos.0)),
            3 => Some(GDValue::Float(self.config.pos.1)),
            6 => Some(GDValue::Float(self.config.angle)),
            11 => Some(GDValue::Bool(self.config.trigger_cfg.touchable)),
            57 => Some(GDValue::from_group_list(self.config.groups.clone())),
            62 => Some(GDValue::Bool(self.config.trigger_cfg.spawnable)),
            87 => Some(GDValue::Bool(self.config.trigger_cfg.multitriggerable)),
            128 => Some(GDValue::Float(self.config.scale.0)),
            129 => Some(GDValue::Float(self.config.scale.1)),
            20 => Some(GDValue::Int(self.config.editor_layers.0)),
            61 => Some(GDValue::Int(self.config.editor_layers.1)),
            21 => Some(GDValue::Int(self.config.colour_channels.0.as_i32())),
            22 => Some(GDValue::Int(self.config.colour_channels.1.as_i32())),
            24 => Some(GDValue::ZLayer(self.config.z_layer)),
            25 => Some(GDValue::Int(self.config.z_order)),
            343 => Some(GDValue::Int(self.config.enter_effect_channel)),
            446 => Some(GDValue::Int(self.config.material_id)),
            64 => Some(GDValue::Bool(self.config.attributes.dont_fade)),
            67 => Some(GDValue::Bool(self.config.attributes.dont_enter)),
            116 => Some(GDValue::Bool(self.config.attributes.no_effects)),
            34 => Some(GDValue::Bool(self.config.attributes.is_group_parent)),
            279 => Some(GDValue::Bool(self.config.attributes.is_area_parent)),
            509 => Some(GDValue::Bool(self.config.attributes.dont_boost_x)),
            496 => Some(GDValue::Bool(self.config.attributes.dont_boost_y)),
            103 => Some(GDValue::Bool(self.config.attributes.high_detail)),
            121 => Some(GDValue::Bool(self.config.attributes.no_touch)),
            134 => Some(GDValue::Bool(self.config.attributes.passable)),
            135 => Some(GDValue::Bool(self.config.attributes.hidden)),
            136 => Some(GDValue::Bool(self.config.attributes.non_stick_x)),
            289 => Some(GDValue::Bool(self.config.attributes.non_stick_y)),
            495 => Some(GDValue::Bool(self.config.attributes.extra_sticky)),
            511 => Some(GDValue::Bool(self.config.attributes.extended_collision)),
            137 => Some(GDValue::Bool(self.config.attributes.is_ice_block)),
            193 => Some(GDValue::Bool(self.config.attributes.grip_slope)),
            96 => Some(GDValue::Bool(self.config.attributes.no_glow)),
            507 => Some(GDValue::Bool(self.config.attributes.no_particles)),
            356 => Some(GDValue::Bool(self.config.attributes.scale_stick)),
            372 => Some(GDValue::Bool(self.config.attributes.no_audio_scale)),
            284 => Some(GDValue::Bool(self.config.attributes.single_ptouch)),
            369 => Some(GDValue::Bool(self.config.attributes.center_effect)),
            117 => Some(GDValue::Bool(self.config.attributes.reverse)),
            534 => Some(GDValue::Int(self.config.material_control_id)),
            _ => self
                .properties
                .iter()
                .find(|pair| pair.0 == p)
                .map(|p| p.1.clone()),
        }
    }

    pub fn set_config(&mut self, config: GDObjConfig) {
        self.config = config;
    }
}

/// Trigger config, used for defining general properties of a trigger object:
/// * is touch triggerable?
/// * is spawn triggerable?
/// * is multitriggerable?
#[derive(Clone, Debug, PartialEq)]
pub struct TriggerConfig {
    pub touchable: bool,
    pub spawnable: bool,
    pub multitriggerable: bool,
}

/// Group ID container for regular and parent groups
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
pub enum Group {
    Regular(i16),
    Parent(i16),
}

impl Ord for Group {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // check ids first
        // check the types only if equal
        match self.id().cmp(&other.id()) {
            std::cmp::Ordering::Equal => self.get_type().cmp(&other.get_type()),
            o => o,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
/// Group type enum
pub enum GroupType {
    Regular,
    Parent,
}

impl Ord for GroupType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            std::cmp::Ordering::Equal
        } else if *self == Self::Regular {
            // other is parent, so is less
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    }
}

impl Group {
    pub fn id(&self) -> i16 {
        match self {
            Self::Regular(id) => *id,
            Self::Parent(id) => *id,
        }
    }
    pub fn get_type(&self) -> GroupType {
        match self {
            Group::Parent(_) => GroupType::Parent,
            Group::Regular(_) => GroupType::Regular,
        }
    }
}

impl From<i16> for Group {
    fn from(value: i16) -> Self {
        Self::Regular(value)
    }
}

/// Object config, used for defining general properties of an object:
/// * position
/// * scale
/// * rotation angle
/// * groups
/// * trigger_cfg
#[derive(Clone, Debug, PartialEq)]
pub struct GDObjConfig {
    pub pos: (f64, f64),
    pub scale: (f64, f64),
    pub angle: f64,
    pub groups: Vec<Group>,
    pub trigger_cfg: TriggerConfig,
    pub z_order: i32,
    pub z_layer: ZLayer,
    pub editor_layers: (i32, i32),
    pub colour_channels: (ColourChannel, ColourChannel),
    pub enter_effect_channel: i32,
    pub material_id: i32,
    pub material_control_id: i32,
    pub attributes: GDObjAttributes,
}

impl GDObjConfig {
    /// Constructor with default properties:
    /// * position: 0, 0
    /// * scale: 1.0, 1.0
    /// * angle: 0.0,
    /// * groups: none
    /// * not touch triggerable
    /// * not spawn triggerable
    /// * not multi triggerable
    #[inline(always)]
    pub fn default() -> Self {
        GDObjConfig {
            pos: (0.0, 0.0),
            scale: (1.0, 1.0),
            angle: 0.0,
            groups: vec![],
            trigger_cfg: TriggerConfig {
                touchable: false,
                spawnable: false,
                multitriggerable: false,
            },
            z_layer: ZLayer::T1,
            z_order: 0,
            editor_layers: (0, 0),
            colour_channels: (ColourChannel::Object, ColourChannel::Channel(1)),
            enter_effect_channel: 0,
            material_id: 0,
            material_control_id: 0,
            attributes: GDObjAttributes::new(),
        }
    }

    /// Alias for default
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Converts this config to a properties hashmap
    pub fn to_string(&self) -> String {
        let mut properties = String::with_capacity(64);
        let _ = write!(
            properties,
            ",2,{},3,{}{}",
            self.pos.0,
            self.pos.1,
            self.attributes.get_property_str()
        );

        // bools
        serialise_bools(
            &[
                ("11", self.trigger_cfg.touchable),
                ("62", self.trigger_cfg.spawnable),
                ("87", self.trigger_cfg.multitriggerable),
            ],
            &mut properties,
        );

        // f64
        serialise_fields(
            &[
                ("6", self.angle, 0.0),
                ("128", self.scale.0, 1.0),
                ("129", self.scale.1, 1.0),
            ],
            &mut properties,
        );

        // i32
        serialise_fields(
            &[
                ("20", self.editor_layers.0, 0),
                ("61", self.editor_layers.1, 0),
                (
                    "21",
                    self.colour_channels.0.as_i32(),
                    ColourChannel::Object.as_i32(),
                ),
                ("22", self.colour_channels.1.as_i32(), 1),
                ("24", self.z_layer as i32, ZLayer::T1 as i32),
                ("25", self.z_order, 0),
                ("343", self.enter_effect_channel, 0),
                ("446", self.material_id, 0),
                ("534", self.material_control_id, 0),
            ],
            &mut properties,
        );

        if !self.groups.is_empty() {
            properties.push_str(",57,");
            let group_str = &self
                .groups
                .iter()
                .map(|g| g.id().to_string())
                .collect::<Vec<String>>()
                .join(".");
            properties.push_str(group_str);
        };

        return properties;
    }

    fn dedup_groups(&mut self) {
        // sort beforehand
        self.groups.sort_by(|a, b| a.cmp(&b));
        self.groups.dedup_by(|a, b| a.id() == b.id());
    }

    /// Sets groups of this object
    #[inline(always)]
    pub fn groups<T: IntoIterator<Item = I>, I: Into<Group>>(mut self, groups: T) -> Self {
        self.groups = groups.into_iter().map(|g| g.into()).collect();
        self.dedup_groups();
        self
    }
    /// Adds groups to this object's groups
    #[inline(always)]
    pub fn add_groups<T: AsRef<[Group]>>(&mut self, groups: T) {
        self.groups.extend_from_slice(groups.as_ref());
        self.dedup_groups();
    }
    /// Adds group to this object's groups
    #[inline(always)]
    pub fn add_group(&mut self, group: Group) {
        self.groups.push(group);
        self.dedup_groups();
    }
    /// Removes this group from this object's groups
    #[inline(always)]
    pub fn remove_group(&mut self, group: Group) {
        if let Some(idx) = self.groups.iter().position(|&g| g == group) {
            self.groups.swap_remove(idx);
        }
    }
    /// Sets x position of this object
    #[inline(always)]
    pub fn x(mut self, x: f64) -> Self {
        self.pos.0 = x;
        self
    }
    /// Sets y position of this object
    #[inline(always)]
    pub fn y(mut self, y: f64) -> Self {
        self.pos.1 = y;
        self
    }

    /// Applies a translation to this object's position
    pub fn translate(&mut self, x: f64, y: f64) {
        self.pos.0 += x;
        self.pos.1 += y;
    }

    /// Sets x and y position of this object
    #[inline(always)]
    pub fn pos(mut self, x: f64, y: f64) -> Self {
        self.pos = (x, y);
        self
    }
    /// Sets x scale of this object
    #[inline(always)]
    pub fn xscale(mut self, xscale: f64) -> Self {
        self.scale.0 = xscale;
        self
    }
    /// Sets y scale of this object
    #[inline(always)]
    pub fn yscale(mut self, yscale: f64) -> Self {
        self.scale.1 = yscale;
        self
    }
    /// Sets x and y scale of this object
    #[inline(always)]
    pub fn scale(mut self, x: f64, y: f64) -> Self {
        self.scale = (x, y);
        self
    }
    /// Sets rotation angle of this object
    #[inline(always)]
    pub fn angle(mut self, angle: f64) -> Self {
        self.angle = angle;
        self
    }
    /// Makes this object touch triggerable
    #[inline(always)]
    pub fn touchable(mut self, touchable: bool) -> Self {
        self.trigger_cfg.touchable = touchable;
        self
    }
    /// Makes this object spawn triggerable
    #[inline(always)]
    pub fn spawnable(mut self, spawnable: bool) -> Self {
        self.trigger_cfg.spawnable = spawnable;
        self
    }
    /// Makes this object multi-triggerable
    #[inline(always)]
    pub fn multitrigger(mut self, multi: bool) -> Self {
        self.trigger_cfg.multitriggerable = multi;
        self
    }
    /// Sets this object's base colour channel
    #[inline(always)]
    pub fn set_base_colour(mut self, channel: ColourChannel) -> Self {
        self.colour_channels.0 = channel;
        self
    }
    /// Sets this object's detail colour channel
    #[inline(always)]
    pub fn set_detail_colour(mut self, channel: ColourChannel) -> Self {
        self.colour_channels.1 = channel;
        self
    }
    /// Sets this object's Z-layer
    #[inline(always)]
    pub fn set_z_layer(mut self, z: ZLayer) -> Self {
        self.z_layer = z;
        self
    }
    /// Sets this object's Z-order
    #[inline(always)]
    pub fn set_z_order(mut self, z: i32) -> Self {
        self.z_order = z;
        self
    }
    /// Sets editor layer 1 of this object
    #[inline(always)]
    pub fn editor_layer_1(mut self, l: i32) -> Self {
        self.editor_layers.0 = l;
        self
    }
    /// Sets editor layer 2 of this object
    #[inline(always)]
    pub fn editor_layer_2(mut self, l: i32) -> Self {
        self.editor_layers.1 = l;
        self
    }
    /// Sets this object's material id
    #[inline(always)]
    pub fn set_material_id(mut self, material_id: i32) -> Self {
        self.material_id = material_id;
        self
    }
    /// Sets this object's enter effect channel
    #[inline(always)]
    pub fn set_enter_channel(mut self, channel: i32) -> Self {
        self.enter_effect_channel = channel;
        self
    }

    ////////////////////// ATTRIBUTES DOWN HERE

    /// Enables `dont_fade` on this object.
    #[inline(always)]
    pub fn dont_fade(mut self, toggle: bool) -> Self {
        self.attributes.dont_fade = toggle;
        self
    }

    /// Enables `dont_enter` on this object.
    #[inline(always)]
    pub fn dont_enter(mut self, toggle: bool) -> Self {
        self.attributes.dont_enter = toggle;
        self
    }

    /// Enables `no_effects` on this object.
    #[inline(always)]
    pub fn no_effects(mut self, toggle: bool) -> Self {
        self.attributes.no_effects = toggle;
        self
    }

    /// Enables `is_group_parent` on this object.
    #[inline(always)]
    pub fn is_group_parent(mut self, toggle: bool) -> Self {
        self.attributes.is_group_parent = toggle;
        self
    }

    /// Enables `is_area_parent` on this object.
    #[inline(always)]
    pub fn is_area_parent(mut self, toggle: bool) -> Self {
        self.attributes.is_area_parent = toggle;
        self
    }

    /// Enables `dont_boost_x` on this object.
    #[inline(always)]
    pub fn dont_boost_x(mut self, toggle: bool) -> Self {
        self.attributes.dont_boost_x = toggle;
        self
    }

    /// Enables `dont_boost_y` on this object.
    #[inline(always)]
    pub fn dont_boost_y(mut self, toggle: bool) -> Self {
        self.attributes.dont_boost_y = toggle;
        self
    }

    /// Enables `high_detail` on this object.
    #[inline(always)]
    pub fn high_detail(mut self, toggle: bool) -> Self {
        self.attributes.high_detail = toggle;
        self
    }

    /// Enables `no_touch` on this object.
    #[inline(always)]
    pub fn no_touch(mut self, toggle: bool) -> Self {
        self.attributes.no_touch = toggle;
        self
    }

    /// Enables `passable` on this object.
    #[inline(always)]
    pub fn passable(mut self, toggle: bool) -> Self {
        self.attributes.passable = toggle;
        self
    }

    /// Enables `hidden` on this object.
    #[inline(always)]
    pub fn hidden(mut self, toggle: bool) -> Self {
        self.attributes.hidden = toggle;
        self
    }

    /// Enables `non_stick_x` on this object.
    #[inline(always)]
    pub fn non_stick_x(mut self, toggle: bool) -> Self {
        self.attributes.non_stick_x = toggle;
        self
    }

    /// Enables `non_stick_y` on this object.
    #[inline(always)]
    pub fn non_stick_y(mut self, toggle: bool) -> Self {
        self.attributes.non_stick_y = toggle;
        self
    }

    /// Enables `extra_sticky` on this object.
    #[inline(always)]
    pub fn extra_sticky(mut self, toggle: bool) -> Self {
        self.attributes.extra_sticky = toggle;
        self
    }

    /// Enables `extended_collision` on this object.
    #[inline(always)]
    pub fn extended_collision(mut self, toggle: bool) -> Self {
        self.attributes.extended_collision = toggle;
        self
    }

    /// Enables `is_ice_block` on this object.
    #[inline(always)]
    pub fn is_ice_block(mut self, toggle: bool) -> Self {
        self.attributes.is_ice_block = toggle;
        self
    }

    /// Enables `grip_slope` on this object.
    #[inline(always)]
    pub fn grip_slope(mut self, toggle: bool) -> Self {
        self.attributes.grip_slope = toggle;
        self
    }

    /// Enables `no_glow` on this object.
    #[inline(always)]
    pub fn no_glow(mut self, toggle: bool) -> Self {
        self.attributes.no_glow = toggle;
        self
    }

    /// Enables `no_particles` on this object.
    #[inline(always)]
    pub fn no_particles(mut self, toggle: bool) -> Self {
        self.attributes.no_particles = toggle;
        self
    }

    /// Enables `scale_stick` on this object.
    #[inline(always)]
    pub fn scale_stick(mut self, toggle: bool) -> Self {
        self.attributes.scale_stick = toggle;
        self
    }

    /// Enables `no_audio_scale` on this object.
    #[inline(always)]
    pub fn no_audio_scale(mut self, toggle: bool) -> Self {
        self.attributes.no_audio_scale = toggle;
        self
    }

    /// Enables `single_ptouch` on this object.
    /// If enabled, this object will ignore the second player's input
    /// if both player inputs are pressed on the same tick.
    #[inline(always)]
    pub fn single_ptouch(mut self, toggle: bool) -> Self {
        self.attributes.single_ptouch = toggle;
        self
    }

    /// Enables `center_effect` on this object.
    #[inline(always)]
    pub fn center_effect(mut self, toggle: bool) -> Self {
        self.attributes.center_effect = toggle;
        self
    }

    /// Enables `reverse` on this object.
    #[inline(always)]
    pub fn reverse(mut self, toggle: bool) -> Self {
        self.attributes.reverse = toggle;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GDObjAttributes {
    pub dont_fade: bool,
    pub dont_enter: bool,
    pub no_effects: bool,
    pub is_group_parent: bool,
    pub is_area_parent: bool,
    pub dont_boost_x: bool,
    pub dont_boost_y: bool,
    pub high_detail: bool,
    pub no_touch: bool,
    pub passable: bool,
    pub hidden: bool,
    pub non_stick_x: bool,
    pub non_stick_y: bool,
    pub extra_sticky: bool,
    pub extended_collision: bool,
    pub is_ice_block: bool,
    pub grip_slope: bool,
    pub no_glow: bool,
    pub no_particles: bool,
    pub scale_stick: bool,
    pub no_audio_scale: bool,
    pub single_ptouch: bool,
    pub center_effect: bool,
    pub reverse: bool,
}

impl GDObjAttributes {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            dont_fade: false,
            dont_enter: false,
            no_effects: false,
            is_group_parent: false,
            is_area_parent: false,
            dont_boost_x: false,
            dont_boost_y: false,
            high_detail: false,
            no_touch: false,
            passable: false,
            hidden: false,
            non_stick_x: false,
            non_stick_y: false,
            extra_sticky: false,
            extended_collision: false,
            is_ice_block: false,
            grip_slope: false,
            no_glow: false,
            no_particles: false,
            scale_stick: false,
            no_audio_scale: false,
            center_effect: false,
            single_ptouch: false,
            reverse: false,
        }
    }

    pub fn get_property_str(&self) -> String {
        let fields = [
            (DONT_FADE, self.dont_fade),
            (DONT_ENTER, self.dont_enter),
            (NO_OBJECT_EFFECTS, self.no_effects),
            (IS_GROUP_PARENT, self.is_group_parent),
            (IS_AREA_PARENT, self.is_area_parent),
            (DONT_BOOST_X, self.dont_boost_x),
            (DONT_BOOST_Y, self.dont_boost_y),
            (IS_HIGH_DETAIL, self.high_detail),
            (NO_TOUCH, self.no_touch),
            (PASSABLE, self.passable),
            (HIDDEN, self.hidden),
            (NONSTICK_X, self.non_stick_x),
            (NONSTICK_Y, self.non_stick_y),
            (EXTRA_STICKY, self.extra_sticky),
            (HAS_EXTENDED_COLLISION, self.extended_collision),
            (IS_ICE_BLOCK, self.is_ice_block),
            (GRIP_SLOPE, self.grip_slope),
            (NO_GLOW, self.no_glow),
            (NO_PARTICLES, self.no_particles),
            (SCALE_STICK, self.scale_stick),
            (NO_AUDIO_SCALE, self.no_audio_scale),
            (SINGLE_PLAYER_TOUCH, self.single_ptouch),
            (CENTER_EFFECT, self.center_effect),
            (REVERSES_GAMEPLAY, self.reverse),
        ];
        let mut properties_str = String::with_capacity(6 * fields.len());

        for (id, val) in fields {
            if val {
                let _ = write!(properties_str, ",{id},1");
            }
        }
        properties_str
    }

    /// Alias for `new()`
    #[inline(always)]
    pub fn default() -> Self {
        Self::new()
    }
}

fn serialise_fields<T: PartialEq + Display>(fields: &[(&str, T, T)], buf: &mut String) {
    for (id, field, default) in fields {
        if field != default {
            let _ = write!(buf, ",{id},{field}");
        }
    }
}

/// Function is separate from [`serialise_fields`] to optimise boolean serialising
fn serialise_bools(fields: &[(&str, bool)], buf: &mut String) {
    for (id, field) in fields {
        if *field {
            let _ = write!(buf, ",{id},1");
        }
    }
}
