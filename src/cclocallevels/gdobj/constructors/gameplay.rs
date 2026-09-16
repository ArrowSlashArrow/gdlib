//! Constructors for all parameterized gameplay objects: pads, orbs, portals, force blocks, and the checkpoint.

use crate::cclocallevels::gdobj::ids::objects::{
    BLACK_DROP_ORB, BLUE_GRAVITY_PAD, GREEN_DASH_ORB, GREEN_GRAVITY_ORB, PINK_GRAVITY_DASH_ORB,
    PINK_JUMP_ORB, PINK_JUMP_PAD, RED_JUMP_ORB, RED_JUMP_PAD, SPIDER_ORB, SPIDER_PAD, TELEPORT_ORB,
    TOGGLE_ORB, YELLOW_JUMP_ORB, YELLOW_JUMP_PAD,
};
use crate::cclocallevels::gdobj::ids::properties::{
    DASH_ORB_ALLOW_COLLIDE, DASH_ORB_END_BOOST, DASH_ORB_MAXIMAL_DURATION, DASH_ORB_SPEED,
    DASH_ORB_STOP_SLIDE, MULTI_ACTIVATE, NO_MULTIACTIVATE_PLATFORMER,
};
use crate::cclocallevels::gdobj::structs::{GDValue, TeleportConfig};
use crate::cclocallevels::gdobj::{
    GDOrbType::{GreenDash, PinkDash},
    ObjectProperties, ToggleBlock,
};
use crate::repr_t;

repr_t!(
    /// All types of pads in the game. Casting this type to an integer yields the corresponding variant's object ID.
    strict GDPadType: i32 {
        Yellow = YELLOW_JUMP_PAD,
        Red = RED_JUMP_PAD,
        Pink = PINK_JUMP_PAD,
        Blue = BLUE_GRAVITY_PAD,
        Spider = SPIDER_PAD,
    }
);

/// Configuration struct for pad objects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GDPad {
    /// What type of pad this object is.
    pub ptype: GDPadType,
    /// Allows the pad to activate multiple times in normal mode
    pub multi_activate: bool,
    /// Restricts the pad from activating multiple times in platformer mode
    pub no_plat_multi_activate: bool,
}

impl GDPad {
    /// Creates an instance of this object with both other multi-activate parameters set to their default value.
    pub fn new(ptype: GDPadType) -> Self {
        Self {
            ptype,
            multi_activate: false,
            no_plat_multi_activate: false,
        }
    }
}

impl ObjectProperties for GDPad {
    fn object_id(&self) -> i32 {
        self.ptype as i32
    }

    fn serialise(&self) -> Vec<(u16, crate::cclocallevels::gdobj::structs::GDValue)> {
        vec![
            (MULTI_ACTIVATE, GDValue::Bool(self.multi_activate)),
            (
                NO_MULTIACTIVATE_PLATFORMER,
                GDValue::Bool(self.no_plat_multi_activate),
            ),
        ]
    }

    fn from_object(obj: &crate::cclocallevels::gdobj::GDObject) -> Option<Self>
    where
        Self: Sized,
    {
        let ptype = match GDPadType::try_from(obj.id) {
            Ok(n) => n,
            Err(_) => return None,
        };

        let multi_activate = match obj
            .get_property(MULTI_ACTIVATE)
            .unwrap_or(GDValue::Bool(false))
        {
            GDValue::Bool(b) => b,
            _ => false,
        };
        let no_plat_multi_activate = match obj
            .get_property(NO_MULTIACTIVATE_PLATFORMER)
            .unwrap_or(GDValue::Bool(false))
        {
            GDValue::Bool(b) => b,
            _ => false,
        };
        Some(Self {
            ptype,
            multi_activate,
            no_plat_multi_activate,
        })
    }
}

repr_t!(
    /// All types of orbs in the game. Casting this type to an integer yields the corresponding variant's object ID.
    strict GDOrbType: i32 {
        Yellow = YELLOW_JUMP_ORB,
        Pink = PINK_JUMP_ORB,
        Blue = BLUE_GRAVITY_PAD,
        Red = RED_JUMP_ORB,
        Green = GREEN_GRAVITY_ORB,
        Black = BLACK_DROP_ORB,
        GreenDash = GREEN_DASH_ORB,
        PinkDash = PINK_GRAVITY_DASH_ORB,
        Spider = SPIDER_ORB,
        Teleport = TELEPORT_ORB,
        Toggle = TOGGLE_ORB
    }
);

impl GDOrbType {
    /// Returns true if this type of orb is a dash orb.
    pub fn is_dash_orb(&self) -> bool {
        match self {
            GreenDash | PinkDash => true,
            _ => false,
        }
    }
}

/// Configuration struct for orbs.
pub struct GDOrb {
    /// What type of pad this object is.
    pub otype: GDOrbType,
    /// Allows the pad to activate multiple times in normal mode
    pub multi_activate: bool,
    /// Restricts the pad from activating multiple times in platformer mode
    pub no_plat_multi_activate: bool,
    /// Extra configuration for orbs that have it.
    pub extra_orb_config: OrbOptions,
}

/// This field is required when using `.serialise()` if this orb has extra config options; otherwise, leave as `None`.
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum OrbOptions {
    /// Regular orbs require no extra config
    None,
    DashOrb(DashOrbSettings),
    /// [`ToggleBlock`] has the same exact configuration, use it to configure a toggle orb.
    ToggleOrb(ToggleBlock),
    Teleporb(TeleportConfig),
}

impl ObjectProperties for GDOrb {
    fn object_id(&self) -> i32 {
        self.otype as i32
    }

    fn serialise(&self) -> Vec<(u16, GDValue)> {
        let mut properties = vec![
            (MULTI_ACTIVATE, GDValue::Bool(self.multi_activate)),
            (
                NO_MULTIACTIVATE_PLATFORMER,
                GDValue::Bool(self.no_plat_multi_activate),
            ),
        ];

        match self.extra_orb_config {
            OrbOptions::None => {}
            OrbOptions::DashOrb(d) => properties.extend_from_slice(&[
                (DASH_ORB_SPEED, GDValue::Float(d.speed)),
                (DASH_ORB_ALLOW_COLLIDE, GDValue::Bool(d.allow_collide)),
                (DASH_ORB_END_BOOST, GDValue::Float(d.end_boost)),
                (DASH_ORB_STOP_SLIDE, GDValue::Bool(d.stop_slide)),
                (DASH_ORB_MAXIMAL_DURATION, GDValue::Float(d.max_duration)),
            ]),
            OrbOptions::Teleporb(tp) => properties.extend_from_slice(&tp.to_properties()),
            OrbOptions::ToggleOrb(ref t) => properties.extend_from_slice(&t.serialise()),
        };

        properties
    }

    /* TODO: implement from_object */
}

/// Settings for dash orbs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DashOrbSettings {
    /// Speed of the dash. Default is `1.0`
    pub speed: f64,
    /// Force applied to the player when the dash stops. Default is `1.0`
    pub end_boost: f64,
    /// Dash ends automatically after this many seconds have passed since the orb was activated. Set to `0.0` for infinite duration.
    pub max_duration: f64,
    /// Makes the dash not end when the player collides with a hitbox.
    pub allow_collide: bool,
    // TODO: clarify
    /// Limits the momentum after the dash.
    pub stop_slide: bool,
}

impl Default for DashOrbSettings {
    fn default() -> Self {
        Self {
            speed: 1.0,
            end_boost: 1.0,
            max_duration: 0.0,
            allow_collide: false,
            stop_slide: false,
        }
    }
}
