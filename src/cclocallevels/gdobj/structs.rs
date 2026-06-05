//! This module contains all structs and enums for values that are present in GD objects.

use std::{fmt::Display, str::FromStr};

use anyhow::anyhow;
use smallvec::SmallVec;

use crate::repr_t;

const LIST_ALLOCSIZE: usize = 5;

macro_rules! parse {
    ($v:expr => $t:ty) => {
        $v.parse::<$t>().unwrap_or_default()
    };
}

pub mod animation_ids {
    #![allow(missing_docs)]

    /// Animations for the big beast (chomper)
    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum BigBeast {
        Bite = 0,
        Attack01 = 1,
        Attack01End = 2,
        Idle01 = 3,
    }

    /// Animations for the bat
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

    /// Animations for the spike ball
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
/// Enum for animation IDs
pub enum Anim {
    /// User-specified animation
    Other(i32),
    /// Built-ins for the big beast (chomper)
    BigBeast(animation_ids::BigBeast),
    /// Built-ins for the bat
    Bat(animation_ids::Bat),
    /// Built-ins for the spike ball
    Spikeball(animation_ids::Spikeball),
}

impl From<Anim> for i32 {
    fn from(value: Anim) -> i32 {
        match value {
            Anim::Bat(b) => b as i32,
            Anim::BigBeast(b) => b as i32,
            Anim::Spikeball(s) => s as i32,
            Anim::Other(i) => i,
        }
    }
}

/// In-level value container. Used in such triggers as item edit, item compare and item persisent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Item {
    Counter(i16),
    Timer(i16),
    Points,
    Attempts,
    MainTime,
}

impl Item {
    /// Returns this item's type
    pub fn get_type(&self) -> ItemType {
        match self {
            Self::Attempts => ItemType::Attempts,
            Self::Counter(_) => ItemType::Counter,
            Self::MainTime => ItemType::MainTime,
            Self::Points => ItemType::Points,
            Self::Timer(_) => ItemType::Timer,
        }
    }
    #[inline(always)]
    /// Returns this item's type as an i32
    pub fn get_type_as_i32(&self) -> i32 {
        self.get_type().to_num()
    }
    /// Returns this item's special mode if it has one
    pub fn as_special_mode(&self) -> Option<CounterMode> {
        match self {
            Self::Attempts => Some(CounterMode::Attempts),
            Self::MainTime => Some(CounterMode::MainTime),
            Self::Points => Some(CounterMode::Points),
            _ => None,
        }
    }
    #[inline(always)]
    /// Returns this item's special mode if it has one as an i32
    pub fn as_special_mode_i32(&self) -> i32 {
        self.as_special_mode().unwrap().to_num()
    }

    /// Returns this item's ID
    pub fn id(&self) -> i16 {
        match self {
            Self::Counter(c) => *c,
            Self::Timer(t) => *t,
            _ => 0,
        }
    }
}

repr_t!(
    /// Enum for counter types
    ItemType: i32 {
        Counter = 1,
        Timer = 2,
        Points = 3,
        MainTime = 4,
        Attempts = 5,
    }
);

repr_t!(
    /// Enum for counter modes
    CounterMode: i32 {
        Attempts = -3,
        Points = -2,
        MainTime = -1,
    }
);

/// Corresponding types for [`GDValue`]s.
#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
#[allow(missing_docs)]
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
    ProbabilitiesList,
    SpawnRemapsList,
    Toggle,
    Unknown,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(missing_docs)]
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

impl From<i32> for ZLayer {
    fn from(int: i32) -> Self {
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
#[allow(missing_docs)]
pub enum ColourChannel {
    Channel(i16),
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

impl From<i16> for ColourChannel {
    fn from(c: i16) -> Self {
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

impl From<ColourChannel> for i16 {
    fn from(value: ColourChannel) -> Self {
        match value {
            ColourChannel::Channel(n) => n,
            ColourChannel::Background => 1000,
            ColourChannel::Ground1 => 1001,
            ColourChannel::Ground2 => 1009,
            ColourChannel::Line => 1002,
            ColourChannel::Object => 1004,
            ColourChannel::ThreeDLine => 1003,
            ColourChannel::MiddleGround => 1013,
            ColourChannel::MiddleGround2 => 1014,
            ColourChannel::P1 => 1005,
            ColourChannel::P2 => 1006,
        }
    }
}

/// Enum for all of the move easings
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(missing_docs)]
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

impl From<i32> for MoveEasing {
    fn from(i: i32) -> Self {
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

/// Enum for all values represented by Geometry Dash.
/// All values are parsed according to their specified [`GDObjPropType`].
#[derive(Debug, Clone, PartialEq)]
pub enum GDValue {
    /// Any 32-bit signed integer. Fallback for ints.
    Int(i32),
    /// Any 16-bit signed integer.
    Short(i16),
    /// Any 64-bit signed float.
    Float(f64),
    /// Any boolean.
    Bool(bool),
    /// Alternative boolean form. It is serialised as -1 instead of 0 if false.
    Toggle(bool),
    /// Any group, which is represented by an `i16`.
    Group(i16),
    /// Any item ID, whcih is represented by an `i16`.
    Item(i16),
    /// A list of group IDs as i16, which is stored in a SmallVec.
    GroupList(smallvec::SmallVec<[i16; LIST_ALLOCSIZE]>),
    /// A list of probability pairs: (group id, relative chance). Used in the advanced random trigger
    ProbabilitiesList(smallvec::SmallVec<[(i16, i32); LIST_ALLOCSIZE]>),
    /// A list of spawn remap pairs: (old id, new id)
    SpawnRemapsList(smallvec::SmallVec<[(i16, i16); LIST_ALLOCSIZE]>),
    /// A [`MoveEasing`].
    Easing(MoveEasing),
    /// A [`ColourChannel`]. It may be any of the built in ones, or one with an ID in the range of \[1, 999]
    ColourChannel(ColourChannel),
    /// A [`ZLayer`].
    ZLayer(ZLayer),
    /// A list of [`Event`]s. Used in the event trigger.
    Events(Vec<Event>),
    /// A UTF-8 string. The fallback for any value that did not fit any of the aforementioned criteria.
    String(String), // fallback
}

impl GDValue {
    /// Converts input string to a variant of this enum based on the property type
    pub fn from(t: GDObjPropType, s: &str) -> Self {
        match t {
            GDObjPropType::Bool => Self::Bool(s == "1"),
            GDObjPropType::Toggle => Self::Toggle(s == "1"),
            GDObjPropType::ColourChannel => {
                Self::ColourChannel(ColourChannel::from(parse!(s => i16)))
            }
            GDObjPropType::Easing => Self::Easing(MoveEasing::from(parse!(s => i32))),
            GDObjPropType::Float => Self::Float(parse!(s => f64)),
            GDObjPropType::Int => Self::Int(parse!(s => i32)),
            GDObjPropType::EventsList => Self::Events(
                s.split('.')
                    .map(|i| Event::from(parse!(i => i32)))
                    .collect(),
            ),
            GDObjPropType::ProbabilitiesList => {
                let tuples = parse_sibling_items::<i16, i32>(s);
                Self::ProbabilitiesList(SmallVec::from_vec(tuples))
            }
            GDObjPropType::SpawnRemapsList => {
                let tuples = parse_sibling_items::<i16, i16>(s);
                Self::SpawnRemapsList(SmallVec::from_vec(tuples))
            }
            GDObjPropType::Group => Self::Group(parse!(s => i16)),
            GDObjPropType::Item => Self::Item(parse!(s => i16)),
            GDObjPropType::Text | GDObjPropType::Unknown => Self::String(s.to_owned()),
        }
    }

    #[inline(always)]
    /// Converts a vector of [`Group`]s to a [`GDValue`]
    pub fn from_group_list(g: Vec<Group>) -> Self {
        Self::GroupList(SmallVec::from_vec(g.iter().map(|&g| g.id()).collect()))
    }

    #[inline(always)]
    /// Converts a vector of parent [`Group`]s to a [`GDValue`]
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
    /// Converts a probabilities list to a [`GDValue`].
    pub fn from_prob_list(g: Vec<(i16, i32)>) -> Self {
        Self::ProbabilitiesList(SmallVec::from_vec(g))
    }

    #[inline(always)]
    /// Converts a spawn remaps list to a [`GDValue`].
    pub fn from_spawn_remaps(g: Vec<(i16, i16)>) -> Self {
        Self::SpawnRemapsList(SmallVec::from_vec(g))
    }

    #[inline(always)]
    /// Converts a raw colour channel value to a [`GDValue`].
    pub fn colour_channel(s: &str) -> Self {
        Self::ColourChannel(ColourChannel::from(s.parse().unwrap_or(0)))
    }

    #[inline(always)]
    /// Converts a raw zlayer value to a [`GDValue`].
    pub fn zlayer(s: &str) -> Self {
        Self::ZLayer(ZLayer::from(s.parse().unwrap_or(0)))
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

    // Vec<Into<int>>
    // specialization case for repr_t! enums
    ($vals:expr => $i_buf:expr) => {{
        let mut items_str = String::with_capacity($vals.len() * 4);
        for (idx, item) in $vals.iter().enumerate() {
            if idx != 0 {
                items_str.push('.');
            }
            items_str.push_str($i_buf.format(item.to_num()));
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
            GDValue::ColourChannel(v) => write!(f, "{}", i_buf.format(Into::<i16>::into(*v))),
            GDValue::Easing(v) => write!(f, "{}", i_buf.format(*v as i32)),
            GDValue::Float(v) => write!(f, "{}", d_buf.format(*v)),
            GDValue::Group(v) | GDValue::Item(v) => write!(f, "{}", i_buf.format(*v)),
            GDValue::GroupList(v) => write!(f, "{}", fmt_intlist!(v, i_buf)),
            GDValue::ProbabilitiesList(v) => write!(f, "{}", fmt_inttuples!(v, i_buf)),
            GDValue::SpawnRemapsList(v) => write!(f, "{}", fmt_inttuples!(v, i_buf)),
            GDValue::Int(v) => write!(f, "{}", i_buf.format(*v)),
            GDValue::Short(v) => write!(f, "{}", i_buf.format(*v)),
            GDValue::String(v) => write!(f, "{v}"),
            GDValue::ZLayer(v) => write!(f, "{}", i_buf.format(*v as i32)),
            GDValue::Events(evts) => write!(f, "{}", fmt_intlist!(evts => i_buf)),
        }
    }
}

repr_t!(
    #[allow(missing_docs)]
    /// Enum for all events that the event trigger can listen for
    Event: i32 {
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
);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(missing_docs)]
/// Extra ID 2 parameter in the event trigger
pub enum ExtraID2 {
    #[default]
    All = 0,
    P1 = 1,
    P2 = 2,
}

/// Enum for move targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum MoveTarget {
    // Targets this group's parent object
    Group(i16),
    Player1,
    Player2,
}

repr_t!(
    /// Enum for the GD gamemodes corresponding to their internal values
    #[allow(missing_docs)]
    strict Gamemode: i32 {
        Cube = 0,
        Ship = 1,
        Ball = 2,
        Ufo = 3,
        Wave = 4,
        Robot = 5,
        Spider = 6,
        Swing = 7,
    }
    default Cube
);

repr_t!(
    /// Enum for stop trigger modes
    StopMode: i32 {
        Stop = 0,
        Pause = 1,
        Resume = 2,
    }
);

repr_t!(
    /// Enum for item alignments
    ItemAlign: i32 {
        Center = 0,
        Left = 1,
        Right = 2,
    }
);

repr_t!(
    /// Enum for transition object enter/exit config
    TransitionMode: i32 {
        Both = 0,
        Enter = 1,
        Exit = 2,
    }
);

/// Enum for transition object type (from top, from bottom, etc.)
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(missing_docs)]
pub enum TransitionType {
    Fade = 22,
    FromBottom = 23,
    FromTop = 24,
    FromLeft = 25,
    FromRight = 26,
    ScaleIn = 27,
    ScaleOut = 28,
    Random = 55,
    AwayToLeft = 56,
    AwayToRight = 57,
    AwayFromMiddle = 58,
    TowardsMiddle = 59,
    #[default]
    None = 1915,
}

repr_t!(
    /// Enum for item operators
    Op: i32 {
        Set = 0,
        Add = 1,
        Sub = 2,
        Mul = 3,
        Div = 4,
    }
);

repr_t!(
    /// Enum for item comparison operators
    CompareOp: i32 {
        Equals = 0,
        Greater = 1,
        GreaterOrEquals = 2,
        Less = 3,
        LessOrEquals = 4,
        NotEquals = 5,
    }
);

/// Compare operand configuration specifier for the item control trigger
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CompareOperand {
    /// Base item whose value is being used for the comparsion
    pub operand_item: Item,
    /// Multiplier
    pub modifier: f64,
    /// Operator between the item's value and modifier. Can only be `Op::Mul` or `Op::Div`
    pub mod_op: Op,
    /// Forces a specific rounding on the resulting value: See [`RoundMode`]
    pub rounding: RoundMode,
    /// Forces a specific sign on the resulting value: See [`SignMode`]
    pub sign: SignMode,
}

impl CompareOperand {
    /// Constructor for an operand that is simply a number literal.  
    /// Useful for comparing an [`Item`] against a number
    pub fn number_literal(num: f64) -> Self {
        Self {
            operand_item: Item::Counter(0),
            modifier: num,
            mod_op: Op::Mul,
            rounding: RoundMode::None,
            sign: SignMode::None,
        }
    }
}

impl From<Item> for CompareOperand {
    fn from(value: Item) -> Self {
        Self {
            operand_item: value,
            modifier: 1.0,
            mod_op: Op::Mul,
            rounding: RoundMode::None,
            sign: SignMode::None,
        }
    }
}

repr_t!(
    /// Enum for item round modes
    strict RoundMode: i32 {
        /// Leave as-is
        None = 0,
        Nearest = 1,
        Floor = 2,
        Ceiling = 3,
    }
);

repr_t!(
    /// Enum for item sign modes
    strict SignMode: i32 {
        /// Leave as-is
        None = 0,
        Absolute = 1,
        Negative = 2,
    }
);

/// Enum for target player in gravity trigger
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetPlayer {
    /// Player 1
    Player1 = 138,
    /// Player 2
    Player2 = 200,
    /// Player that touched the gravity trigger
    PlayerTarget = 201,
}

/// Enum for move mode setting. See structs [`DefaultMove`], [`TargetMove`], and [`DirectionalMove`]
#[derive(Debug, Clone, PartialEq)]
pub enum MoveMode {
    /// Normal axis-based move mode
    Default(DefaultMove),
    /// Moves the group to the position of another group
    Targeting(TargetMove),
    /// Moves the group in the direction of another group
    Directional(DirectionalMove),
}

/// Enum for lock config: player or camera
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum MoveLock {
    Player,
    Camera,
}

repr_t!(
    /// Enum for relative UI reference position
    strict UIReferencePos: i32 {
        Auto = 1,
        Center = 2,
        Left = 3,
        Right = 4,
    }
);

/// Config struct for default movement
#[derive(Debug, Clone, PartialEq)]
pub struct DefaultMove {
    /// Units to move in x-axis. Used as multiplier of player/camera movement if `x_lock` is used
    pub dx: f64,
    /// Units to move in y-axis. Used as multiplier of player/camera movement if `y_lock` is used
    pub dy: f64,
    /// Optional lock on x movement which allows the object to move relative to either the player or the camera
    pub x_lock: Option<MoveLock>,
    /// Optional lock on y movement which allows the object to move relative to either the player or the camera
    pub y_lock: Option<MoveLock>,
}

/// Config struct for moving to a specific target.
#[derive(Debug, Clone, PartialEq)]
pub struct TargetMove {
    /// Group that will be moved to. Use `POS_PLAYER1` and `POS_PLAYER2` consts to specify moving to one of the players.
    pub target_group_id: MoveTarget,
    /// (Optional) The objects that represent the center of the group that is moving
    pub center_group_id: Option<i16>,
    /// Optional axis restriction. Use constants `MOVE_X_ONLY` and `MOVE_Y_ONLY` to specify axis.
    pub axis_only: Option<AxisOnlyMove>,
}

/// Optional axis lock for move triggers
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AxisOnlyMove {
    /// Locks to X-axis
    X = 1,
    /// Locks to Y-axis
    Y = 2,
}

/// Config struct for moving to a specific target.
#[derive(Debug, Clone, PartialEq)]
pub struct DirectionalMove {
    /// Group that will be moved to. Use `POS_PLAYER1` and `POS_PLAYER2` consts to specify moving to one of the players.
    pub target_group_id: MoveTarget,
    /// (Optional) The objects that represent the center of the group that is moving
    pub center_group_id: Option<i16>,
    /// Distance in units to move in the direction of the target objects.
    pub distance: i32,
}

repr_t!(
    /// Enum for starting speed in a startpos
    #[allow(missing_docs)]
    strict Speed: i32 {
        X0Point5 = 1,
        X1 = 0,
        X2 = 2,
        X3 = 3,
        X4 = 4,
    }
    default X1
);

/// Config struct for HSV colour settings
#[derive(Debug, Clone, PartialEq)]
pub struct HSVColour {
    /// Hue shift
    pub hue_shift: i32,
    /// Saturation multiplier
    pub saturation_mult: f64,
    /// Brightness multiplier
    pub brightness_mult: f64,
    /// Use static saturation scalar
    pub static_sat_scalar: bool,
    /// Use static brightness scalar
    pub static_bright_scalar: bool,
}

impl Display for HSVColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}a{}a{}a{}a{}",
            self.hue_shift,
            self.saturation_mult,
            self.brightness_mult,
            if self.static_sat_scalar { "1" } else { "0" },
            if self.static_bright_scalar { "1" } else { "0" }
        )
    }
}

macro_rules! set_value {
    ($iter:expr => $t:ty) => {
        match $iter.next() {
            Some(v) => match v.parse::<$t>() {
                Ok(v) => v,
                Err(_) => return None,
            },
            None => return None,
        }
    };
    ($iter:expr) => {
        match $iter.next() {
            Some(v) => match v.parse::<i32>() {
                Ok(v) => v != 0,
                Err(_) => return None,
            },
            None => return None,
        }
    };
}

impl HSVColour {
    /// Parses a string to this object
    pub fn parse(s: &str) -> Option<Self> {
        let mut vals_iter = s.split("a").into_iter();
        let mut new = Self {
            hue_shift: 0,
            saturation_mult: 1.0,
            brightness_mult: 1.0,
            static_bright_scalar: false,
            static_sat_scalar: false,
        };

        new.hue_shift = set_value!(vals_iter => i32);
        new.saturation_mult = set_value!(vals_iter => f64);
        new.brightness_mult = set_value!(vals_iter => f64);
        new.static_bright_scalar = set_value!(vals_iter);
        new.static_sat_scalar = set_value!(vals_iter);
        Some(new)
    }
}

/// Enum for target of pulse
#[derive(Debug, Clone, PartialEq)]
pub enum PulseTarget {
    /// Pulse for a group
    Group(PulseGroup),
    /// Pulse for a channel
    Channel(PulseChannel),
}

/// Config struct for channel pulses
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PulseChannel {
    /// Channel which is pulsed
    pub channel_id: i16,
}

/// Config struct for group pulses
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PulseGroup {
    /// Group that is being pulsed
    pub group_id: i16,
    /// Toggles pulsing the main colour of objects only
    pub main_colour_only: bool,
    /// Toggles pulsing the detail colour of object only
    pub detail_colour_only: bool,
}

/// Enum for pulse mode
pub enum PulseMode {
    /// Pulse with colour
    Colour(Colour),
    /// Pulse with HSV
    HSV(PulseHSV),
}

/// RGB colour tuple
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[allow(missing_docs)]
pub struct Colour {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Colour {
    /// Converts an RGB tuple to a [`Colour`]
    pub fn from_rgb(rgb: (u8, u8, u8)) -> Self {
        Self {
            red: rgb.0,
            green: rgb.1,
            blue: rgb.2,
        }
    }

    /// Parses a hex code (#123456) to a [`Colour`]
    pub fn from_hex<S: AsRef<str>>(hex_str: S) -> Result<Self, anyhow::Error> {
        let str = hex_str.as_ref();
        if str.len() != 7 || !str.starts_with('#') {
            return Err(anyhow!(
                "Hex code must start with a hashtag followed by a 6-digit RGB hex tuple."
            ));
        }

        let mut owned = str.to_string();
        owned.remove(0);

        let hex = i32::from_str_radix(&owned, 16)?;
        Ok(Self {
            red: (hex >> 16 & 0xFF) as u8,
            green: (hex >> 8 & 0xFF) as u8,
            blue: (hex & 0xFF) as u8,
        })
    }
}

/// Configuration struct for scaling of objects in the scale trigger
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScaleConfig {
    /// Scale of target on x-axis
    pub x_scale: f64,
    /// Scale of target on y-axis
    pub y_scale: f64,
    /// Divides the x scale by the existing x-axis scale value of the target
    pub div_by_value_x: bool,
    /// Divides the y scale by the existing y-axis scale value of the target
    pub div_by_value_y: bool,
    /// Makes the objects only move as if they were scaled, but not actually scale them
    pub only_move: bool,
    /// Bases scaling on the center object
    pub relative_scale: bool,
    /// Rotates the x and y axes too
    pub relative_rotation: bool,
}

/// Pulse with HSV mode configuration
#[derive(Debug, Clone, PartialEq)]
pub struct PulseHSV {
    /// HSV pulse specification
    pub hsv_config: HSVColour,
    /// Toggles using static HSV
    pub use_static_hsv: bool,
    /// Target of the pulse
    pub colour_id: ColourChannel,
}

/// Config struct for copy colour options
#[derive(Debug, Clone, PartialEq)]
pub struct CopyColourConfig {
    /// Original colour channel from which to copy colour
    pub original_ch: ColourChannel,
    /// HSV modifier for new colour
    pub hsv_config: HSVColour,
    /// Whether to apply legacy HSV transformation
    pub use_legacy_hsv: bool,
    /// Copy original colour's opacity
    pub copy_opacity: bool,
}

/// Enum for rotation configs
#[derive(Debug, Clone, PartialEq)]
pub enum RotationMode {
    /// Regular rotation based on a fixed degree amount
    Default(RotationNormal),
    /// Rotates the target objects to face a target group
    Aim(RotationAim),
    /// Follows a target object's rotation
    Follow(RotationAim),
}

/// Struct for specifying rotation settings in a rotate trigger
#[derive(Debug, Clone, PartialEq)]
pub struct RotationConfig {
    /// See [`RotationMode`]
    pub mode: RotationMode,
    /// Update location of aim group in real time]
    pub dynamic_mode: bool,
    /// Prevent target object from rotating around its center
    pub lock_object_rotation: bool,
}

/// Configuration struct for the time trigger
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeTriggerConfig {
    /// Starting time of target timer that will be set on activation of the trigger
    pub start_time: f64,
    /// Time at which to call the target group
    pub stop_time: f64,
    /// Whether or not to pause the timer once it reaches the stop time
    pub pause_when_reached: bool,
    /// Time multiplier for this timer
    pub time_mod: f64,
    /// Target timer ID
    pub timer_id: i16,
    /// Toggles ignoring global timewarp
    pub ignore_timewarp: bool,
    /// Starts this timer paused, which allows a time control trigger to un-pause it
    pub start_paused: bool,
    /// Only starts the timer if any of these are met:
    ///     1. Target timer is at 0.00
    ///     2. The `start_paused` option is on
    ///     3. The timer is not currently counting
    pub dont_override: bool,
}

/// Degree amount rotation specifier
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RotationNormal {
    /// Amount of degrees
    pub degrees: f64,
    /// Amount of full rotations. Specifying these is preferred to specifying an overly large number of degrees.
    pub x360: i32,
}

impl RotationNormal {
    /// Convert from degrees to this object.
    pub fn from_degrees(deg: f64) -> Self {
        Self {
            degrees: deg % 360.0,
            x360: (deg as i32 / 360),
        }
    }
}

/// Optional target of rotation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum RotationPlayerTarget {
    Player1,
    Player2,
}

/// Config struct for aim mode rotation
#[derive(Debug, Clone, PartialEq)]
pub struct RotationAim {
    /// Group around which to rotate
    pub aim_target: i16,
    /// Rotation offset of the rotating group
    pub rot_offset: f64,
    ///  Overrides aim_target if not None, uses either P1 or P2 as the target instead.
    pub player_target: Option<RotationPlayerTarget>,
}

/// Configuration descriptor for spawning particles in particle spawn trigger
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ParticleSpawnConfig {
    /// (x, y) tuple for offsets from their original spawn location.
    ///   Note: all particle objects spawn in the same position, regardless of their offsets within their group.
    pub position_offsets: Option<(i32, i32)>,
    /// (x, y) tuple for range of possible random positional variation.
    pub position_variation: Option<(i32, i32)>,
    /// (rotation, variation) tuple that describes the rotation of the particles + random offset range
    pub rotation_config: Option<(i32, i32)>,
    /// (scale, variation) tuple that describes the scale of the particles + random offset range
    pub scale_config: Option<(f64, f64)>,
    /// Makes all of the particles in the group be rotated in the same direction.
    pub match_rotation: bool,
}

/// Gameplay starting settings specification struct for the startpos trigger
#[derive(Debug, Default, Clone, Copy)]
pub struct StartposConfig {
    /// Starting speed of player
    pub start_speed: Speed,
    /// Starting gamemode; Default: Cube
    pub starting_gamemode: Gamemode,
    /// Starting as mini? Default: false
    pub starting_as_mini: bool,
    /// Start as dual? Default: false
    pub starting_as_dual: bool,
    /// Start as mirrored? Default: false
    pub starting_mirrored: bool,
    /// Reset camera? Default: false
    pub reset_camera: bool,
    /// Rotate gameplay? Default: false
    pub rotate_gameplay: bool,
    /// Reverse gameplay? Default: false
    pub reverse_gameplay: bool,
}

/// Configuration struct for collide triggers which specifies the colliders
/// that are checked for collisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColliderConfig {
    /// ID of first collision block
    pub collider1: i16,
    /// ID of second collision block
    pub collider2: i16,
    /// Whether to check for collision with player 1 instead of collider 1
    pub collide_player1: bool,
    /// Whether to check for collision with player 2 instead of collider 1.
    ///   Does not override collision checking with player 1 if `collide_player1` is also true.
    pub collide_player2: bool,
    /// Whether to check for collision between the two players instead of two collision blocks
    pub collide_both_players: bool,
}

impl ColliderConfig {
    /// Creates a new instance of this object from two collision block IDs
    pub fn two_colliders(collider1_id: i16, collider2_id: i16) -> Self {
        Self {
            collider1: collider1_id,
            collider2: collider2_id,
            collide_player1: false,
            collide_player2: false,
            collide_both_players: false,
        }
    }
}

/// Configuration struct for the primary colour operation in a colour trigger
#[derive(Debug, Clone, Copy)]
pub struct ColourTriggerConfig {
    /// (R, G, B) tuple of `u8`s
    pub colour: Colour,
    /// Channel whose colour will be changed
    pub channel: ColourChannel,
    /// Opacity of colour
    pub opacity: f64,
    /// Use blending?
    pub blending: bool,
    /// Use player colour 1 instead of the specified colour.
    pub use_player_col_1: bool,
    /// Use player colour 2 instead of the specified colour.
    pub use_player_col_2: bool,
}

#[repr(i32)]
#[allow(missing_docs)]
/// Enum for middle grounds
pub enum MiddleGround {
    None = 0,
    SeasweptMountains = 1,
    RockyMountains = 2,
    Clouds = 3,
}

#[repr(i32)]
/// Enum for an optional player target. Used in the touch trigger
pub enum OptionalPlayerTarget {
    /// Registers input from both players
    None = 0,
    /// Only registers input from player 1.
    Player1 = 1,
    /// Only registers input from player 2.
    Player2 = 2,
}

#[repr(i32)]
/// Enum for modes of activation in a touch trigger
pub enum TouchToggle {
    /// Alternates between activating and deactivating the target group
    None = 0,
    /// Activates target group only
    ToggleOn = 1,
    /// De-activates target group only
    ToggleOff = 2,
}

// helper function to parse strings of this formatting "k1.v1.k2.v2.etc.etc."
fn parse_sibling_items<T, S>(s: &str) -> Vec<(T, S)>
where
    T: Default + FromStr + Copy + Clone,
    S: Default + FromStr + Copy + Clone,
{
    let mut curr_group: T = T::default();
    let mut idx = 0;
    let mut tuples = vec![];
    s.split('.').for_each(|c| {
        match idx % 2 == 0 {
            true => {
                // at even idx, so this is a group
                curr_group = parse!(c => T)
            }
            false => {
                // at odd idx, so this is a chance
                tuples.push((curr_group, parse!(c => S)));
            }
        };
        idx += 1
    });
    tuples
}

/// Group ID container for regular and parent groups
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
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

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
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

impl PartialOrd for GroupType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Group {
    /// Returns this group's ID
    pub fn id(&self) -> i16 {
        match self {
            Self::Regular(id) => *id,
            Self::Parent(id) => *id,
        }
    }
    /// Returns this group's type
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
