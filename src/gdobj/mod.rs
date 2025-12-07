//! This module contains the GDObject struct, used for parsing to/from raw object strings
//! This module also contains the GDObjConfig struct for creating new GDObjects
use std::fmt::{Debug, Display, Write};

use crate::gdobj::lookup::get_property_type;
use itoa;
use smallvec::SmallVec;

pub mod ids;
pub mod lookup;
pub mod misc;
pub mod triggers;

pub mod animation_ids {
    #[repr(i32)]
    #[derive(Clone, Copy)]
    pub enum BigBeast {
        Bite = 0,
        Attack01 = 1,
        Attack01End = 2,
        Idle01 = 3,
    }
    #[repr(i32)]
    #[derive(Clone, Copy)]
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
    #[derive(Clone, Copy)]
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

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Copy)]
pub enum GDObjPropType {
    Int,
    Float,
    Text,
    Bool,
    Group,
    Item,
    Easing,
    ColourChannel,
    Unknown,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
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
#[derive(Debug, Clone, PartialEq)]
pub enum ColourChannel {
    Channel(i32),
    Background,
    Ground1,
    Ground2,
    Line,
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
#[derive(Debug, Clone, Copy, Default)]
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

#[derive(Debug, Clone)]
pub enum GDValue {
    Int(i32),
    Float(f64),
    Bool(bool),
    Group(i16),
    Item(i16),
    GroupList(smallvec::SmallVec<[i16; LIST_ALLOCSIZE]>),
    ProbabilitiesList(smallvec::SmallVec<[(i16, i32); LIST_ALLOCSIZE]>),
    Easing(MoveEasing),
    ColourChannel(ColourChannel),
    ZLayer(ZLayer),
    String(String), // fallback
}

impl GDValue {
    pub fn from(t: GDObjPropType, s: &str) -> Self {
        match t {
            GDObjPropType::Bool => Self::Bool(s == "1"),
            GDObjPropType::ColourChannel => {
                Self::ColourChannel(ColourChannel::from_i32(s.parse::<i32>().unwrap()))
            }
            GDObjPropType::Easing => Self::Easing(MoveEasing::from_i32(s.parse::<i32>().unwrap())),
            GDObjPropType::Float => Self::Float(s.parse::<f64>().unwrap()),
            GDObjPropType::Int => Self::Int(s.parse::<i32>().unwrap()),
            GDObjPropType::Group => Self::Group(s.parse::<i16>().unwrap()),
            GDObjPropType::Item => Self::Item(s.parse::<i16>().unwrap()),
            GDObjPropType::Text | GDObjPropType::Unknown => Self::String(s.to_owned()),
        }
    }

    #[inline(always)]
    pub fn from_group_list(g: Vec<i16>) -> Self {
        Self::GroupList(SmallVec::from_vec(g))
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

impl Display for GDValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i_buf = itoa::Buffer::new();
        let mut d_buf = dtoa::Buffer::new();

        match self {
            GDValue::Bool(b) => write!(
                f,
                "{}",
                match b {
                    true => "1",
                    false => "0",
                }
            ),
            GDValue::ColourChannel(v) => write!(f, "{}", i_buf.format(v.as_i32())),
            GDValue::Easing(v) => write!(f, "{}", i_buf.format(*v as i32)),
            GDValue::Float(v) => write!(f, "{}", d_buf.format(*v)),
            GDValue::Group(v) | GDValue::Item(v) => write!(f, "{}", i_buf.format(*v)),
            GDValue::GroupList(v) => write!(f, "{}", {
                let mut g_str = String::with_capacity(v.len() * 4);
                for (idx, g) in v.iter().enumerate() {
                    if idx != 0 {
                        g_str.push('.');
                    }
                    g_str.push_str(i_buf.format(*g));
                }
                g_str
            }),
            GDValue::ProbabilitiesList(v) => write!(f, "{}", {
                let mut g_str = String::with_capacity(v.len() * 8);
                for (idx, g) in v.iter().enumerate() {
                    if idx != 0 {
                        g_str.push('.');
                    }
                    g_str.push_str(i_buf.format(g.0));
                    g_str.push('.');
                    g_str.push_str(i_buf.format(g.1));
                }
                g_str
            }),
            GDValue::Int(v) => write!(f, "{}", i_buf.format(*v)),
            GDValue::String(v) => write!(f, "{v}"),
            GDValue::ZLayer(v) => write!(f, "{}", i_buf.format(*v as i32)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
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
    (6, "Waffle block no bottom"), // todo
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
    (1347, "Trigger Follow"),
    (1520, "Trigger Shake"),
    (1585, "Trigger Animate"),
    (1611, "Trigger Count"),
    (1615, "Counter"),
    (1616, "Trigger Stop"),
    (1812, "Trigger On death"),
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
    (2068, "Trigger Advanced random"),
    (2900, "Trigger rotate gameplay"),
    (3600, "Trigger End"),
    (3606, "BG speed config"),
    (3608, "Trigger Spawn particle"),
    (3609, "Trigger Instant collision"),
    (3612, "MG speed config"),
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

// // TODO: UPDATE THIS!!!!!
// pub const TRIGGER_OBJ_IDS: &[i32] = &[
//     22, 23, 24, 25, 26, 27, 28, 32, 33, 55, 56, 57, 58, 59, 31,
//     899, 901, 914, 1006, 1007, 1049, 1268, 1520, 1615, 1616, 1812,
//     1815, 1816, 1818, 1819, 1912, 1913, 1915, 1917, 1932, 1934, 1935,
//     2016, 2066, 3600, 3606, 3612, 3615, 3617, 3618, 3619, 3620, 3640,
//     3641, 3643, 3662,
// ];

/// Container for GD Object properties.
/// * `id`: The object's ID.
/// * `config`: General properties like position and scale.
/// * `properties`: Object-specific properties like target group for a move trigger
#[derive(Clone)]
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
                    .map(|g| format!("{g}"))
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
                1 => obj.id = val.parse().unwrap_or(0),
                2 => obj.config.pos.0 = val.parse().unwrap_or(0.0),
                3 => obj.config.pos.1 = val.parse().unwrap_or(0.0),
                6 => obj.config.angle = val.parse().unwrap_or(0.0),
                11 => obj.config.trigger_cfg.touchable = val.parse().unwrap_or(false),
                62 => obj.config.trigger_cfg.spawnable = val.parse().unwrap_or(false),
                87 => obj.config.trigger_cfg.multitriggerable = val.parse().unwrap_or(false),
                57 => {
                    obj.config.groups = val
                        .trim_matches('"')
                        .split(".")
                        .filter_map(|g| g.parse::<i16>().ok())
                        .collect()
                }
                128 => obj.config.scale.0 = val.parse().unwrap_or(1.0),
                129 => obj.config.scale.1 = val.parse().unwrap_or(1.0),
                20 => obj.config.editor_layers.0 = val.parse().unwrap_or(0),
                61 => obj.config.editor_layers.1 = val.parse().unwrap_or(0),
                21 => {
                    obj.config.colour_channels.0 = ColourChannel::from_i32(val.parse().unwrap_or(0))
                }
                22 => {
                    obj.config.colour_channels.1 = ColourChannel::from_i32(val.parse().unwrap_or(0))
                }
                24 => obj.config.z_layer = ZLayer::from_i32(val.parse().unwrap_or(0)),
                25 => obj.config.z_order = val.parse().unwrap_or(0),
                343 => obj.config.enter_effect_channel = val.parse().unwrap_or(0),
                446 => obj.config.material_id = val.parse().unwrap_or(0),
                64 => obj.config.attributes.dont_fade = val.parse().unwrap_or(false),
                67 => obj.config.attributes.dont_enter = val.parse().unwrap_or(false),
                116 => obj.config.attributes.no_effects = val.parse().unwrap_or(false),
                34 => obj.config.attributes.is_group_parent = val.parse().unwrap_or(false),
                279 => obj.config.attributes.is_area_parent = val.parse().unwrap_or(false),
                509 => obj.config.attributes.dont_boost_x = val.parse().unwrap_or(false),
                496 => obj.config.attributes.dont_boost_y = val.parse().unwrap_or(false),
                103 => obj.config.attributes.high_detail = val.parse().unwrap_or(false),
                121 => obj.config.attributes.no_touch = val.parse().unwrap_or(false),
                134 => obj.config.attributes.passable = val.parse().unwrap_or(false),
                135 => obj.config.attributes.hidden = val.parse().unwrap_or(false),
                136 => obj.config.attributes.non_stick_x = val.parse().unwrap_or(false),
                289 => obj.config.attributes.non_stick_y = val.parse().unwrap_or(false),
                495 => obj.config.attributes.extra_sticky = val.parse().unwrap_or(false),
                511 => obj.config.attributes.extended_collision = val.parse().unwrap_or(false),
                137 => obj.config.attributes.is_ice_block = val.parse().unwrap_or(false),
                193 => obj.config.attributes.grip_slope = val.parse().unwrap_or(false),
                96 => obj.config.attributes.no_glow = val.parse().unwrap_or(false),
                507 => obj.config.attributes.no_particles = val.parse().unwrap_or(false),
                356 => obj.config.attributes.scale_stick = val.parse().unwrap_or(false),
                372 => obj.config.attributes.no_audio_scale = val.parse().unwrap_or(false),
                284 => obj.config.attributes.single_ptouch = val.parse().unwrap_or(false),
                369 => obj.config.attributes.center_effect = val.parse().unwrap_or(false),
                117 => obj.config.attributes.reverse = val.parse().unwrap_or(false),
                534 => obj.config.material_control_id = val.parse().unwrap_or(0),
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

    /// TODO
    pub fn set_property(&mut self, p: u16, val: GDValue) {
        if let Some(v) = self.properties.iter_mut().find(|(k, _)| *k == p) {
            v.1 = val;
        } else {
            let new_idx = self.properties.partition_point(|(k, _)| k < &p);
            self.properties.insert(new_idx, (p, val));
        }
    }

    /// TODO
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
    /// assert_eq!(object_str, "1,1,155,1,2,0.0,3,0.0,64,1,67,1;");
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
    pub fn new(id: i32, config: GDObjConfig, properties: Vec<(u16, GDValue)>) -> Self {
        GDObject {
            id,
            config,
            properties,
        }
    }

    // TODO: GDValue enum for this
    pub fn get_property(&self, p: u16) -> Option<GDValue> {
        match p {
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
    pub groups: Vec<i16>,
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
        let mut properties = format!(
            ",2,{},3,{},6,{},128,{},129,{},11,{},62,{},87,{},20,{},61,{},21,{},22,{},24,{},25,{},343,{},446,{},64,{},67,{},116,{},34,{},279,{},509,{},496,{},103,{},121,{},134,{},135,{},136,{},289,{},495,{},511,{},137,{},193,{},96,{},507,{},356,{},372,{},284,{},369,{},117,{},534,{}",
            self.pos.0,
            self.pos.1,
            self.angle,
            self.scale.0,
            self.scale.1,
            self.trigger_cfg.touchable as u8,
            self.trigger_cfg.spawnable as u8,
            self.trigger_cfg.multitriggerable as u8,
            self.editor_layers.0,
            self.editor_layers.1,
            self.colour_channels.0.as_i32(),
            self.colour_channels.1.as_i32(),
            self.z_layer as i32,
            self.z_order,
            self.enter_effect_channel,
            self.material_id,
            self.attributes.dont_fade as u8,
            self.attributes.dont_enter as u8,
            self.attributes.no_effects as u8,
            self.attributes.is_group_parent as u8,
            self.attributes.is_area_parent as u8,
            self.attributes.dont_boost_x as u8,
            self.attributes.dont_boost_y as u8,
            self.attributes.high_detail as u8,
            self.attributes.no_touch as u8,
            self.attributes.passable as u8,
            self.attributes.hidden as u8,
            self.attributes.non_stick_x as u8,
            self.attributes.non_stick_y as u8,
            self.attributes.extra_sticky as u8,
            self.attributes.extended_collision as u8,
            self.attributes.is_ice_block as u8,
            self.attributes.grip_slope as u8,
            self.attributes.no_glow as u8,
            self.attributes.no_particles as u8,
            self.attributes.scale_stick as u8,
            self.attributes.no_audio_scale as u8,
            self.attributes.single_ptouch as u8,
            self.attributes.center_effect as u8,
            self.attributes.reverse as u8,
            self.material_control_id
        );

        if !self.groups.is_empty() {
            properties += "57";
            properties += &self
                .groups
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(".");
        };

        return properties;
    }

    /// Sets groups of this object
    #[inline(always)]
    pub fn groups<T: IntoIterator<Item = i16>>(mut self, groups: T) -> Self {
        self.groups = groups.into_iter().collect();
        self
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

    /// Alias for `new()`
    #[inline(always)]
    pub fn default() -> Self {
        Self::new()
    }
}
