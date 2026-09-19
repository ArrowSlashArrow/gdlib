//! Constructors for all parameterized gameplay objects: pads, orbs, portals, and force blocks.

use crate::cclocallevels::{
    consts::*,
    gdobj::{
        ids::{objects::*, properties::*},
        structs::{GDValue, TeleportConfig},
        GDOrbType::{GreenDash, PinkDash},
        ObjectProperties, ToggleBlock,
    },
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
            (IS_INTERACTABLE, GDValue::Bool(true)),
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
                (IS_INTERACTABLE, GDValue::Bool(true)),
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

repr_t!(
    /// Type of force object.
    ///
    /// Casting a variant of this enum to an integer yields this object's ID
    strict ForceObject: i32 {
        Block = FORCE_BLOCK,
        Circle = FORCE_CIRCLE
    }
);

/// Configuration for an object that applies a force to the player when their hitboxes intersect.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ForceObjectConfig {
    /// What kind of force object to configure
    pub force_object: ForceObject,
    /// Force that is applied to player
    pub force: Force,
    /// Makes force direction based on the player's direction relative to the force block.
    /// Negative relative force pulls the player towards the block and positive relative force repels the player away from the block.
    pub is_relative: bool,
    /// Identifier for this specific force (can be 0). Blocks with the same force do not stack their force output.
    pub force_id: i32,
}

impl ObjectProperties for ForceObjectConfig {
    fn object_id(&self) -> i32 {
        self.force_object as i32
    }
    fn serialise(&self) -> Vec<(u16, GDValue)> {
        let mut properties = vec![
            (IS_INTERACTABLE, GDValue::Bool(true)),
            (FORCE_ID, GDValue::Int(self.force_id)),
            (RELATIVE_FORCE, GDValue::Bool(self.is_relative)),
        ];
        match self.force {
            Force::Scalar(s) => properties.push((FORCE_MAGNITUDE, GDValue::Float(s))),
            Force::Range { minimum, maximum } => properties.extend_from_slice(&[
                (FORCE_RANGE_MINIMUM, GDValue::Float(minimum)),
                (FORCE_RANGE_MAXIMUM, GDValue::Float(maximum)),
            ]),
        }

        properties
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Force that is applied to a player in a force object
pub enum Force {
    /// Applies a fixed amount of force in the direction that the force block is pointing - 0 degrees = up, 90 degrees = right, 180 degrees = down, -90 degrees = left
    Scalar(f64),
    /// Applies force based on player position.
    Range {
        /// @nodoc
        minimum: f64,
        /// @nodoc
        maximum: f64,
    },
}

// three different portals:
// 1. speed portal - just has `allow multi` (both classic and platformer)
// 2. gameplay portal - camera settings
// 3. teleportals (linked teleportals are one object)

repr_t!(
    /// Type of speed portal in GD. The variant corresponds to each speed portal by its color.
    ///
    /// Casting a variant of this enum to an integer yields its object ID.
    strict SpeedPortal: i32 {
        Yellow = YELLOW_SLOW_SPEED_PORTAL,
        Blue = BLUE_NORMAL_SPEED_PORTAL,
        Green = GREEN_FAST_SPEED_PORTAL,
        Pink = PINK_FAST_SPEED_PORTAL,
        Red = RED_FAST_SPEED_PORTAL
    }
);

impl SpeedPortal {
    /// Returns the speed that this portal will apply to the player in units per second.
    ///
    /// In classic mode, this is how fast the player will travel in the forward direction when this portal is hit.
    /// In platformer mode, this is the upper limit of the player's speed either forwards or backwards.
    pub fn get_speed(&self) -> f64 {
        match self {
            Self::Yellow => SPEED_05X,
            Self::Blue => SPEED_1X,
            Self::Green => SPEED_2X,
            Self::Pink => SPEED_3X,
            Self::Red => SPEED_4X,
        }
    }
}

/// Settings for a speed portal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpeedPortalConfig {
    /// What kind of speed portal this object is
    pub portal: SpeedPortal,
    /// Allow this portal to be hit multiple times
    pub multi_activate: bool,
}

impl ObjectProperties for SpeedPortalConfig {
    fn object_id(&self) -> i32 {
        self.portal as i32
    }

    fn serialise(&self) -> Vec<(u16, GDValue)> {
        vec![(MULTITRIGGERABLE, GDValue::Bool(self.multi_activate))]
    }

    fn from_object(obj: &crate::cclocallevels::gdobj::GDObject) -> Option<Self>
    where
        Self: Sized,
    {
        let kind = SpeedPortal::try_from(obj.id).ok()?;
        let multi_activate = match obj
            .get_property(MULTITRIGGERABLE)
            .unwrap_or(GDValue::Bool(false))
        {
            GDValue::Bool(b) => b,
            _ => false,
        };
        Some(Self {
            portal: kind,
            multi_activate,
        })
    }
}

repr_t!(
    /// All gameplay portals in GD: gravity portals, mirror portals, size portals, gamemode portals, dual portals, and teleportals.
    ///
    /// Casting a variant of this enum to an integer yields the object ID.
    strict GameplayPortal: i32 {
        BlueGravity = BLUE_GRAVITY_PORTAL,
        YellowGravity = YELLOW_GRAVITY_PORTAL,
        GreenGravity = GREEN_GRAVITY_PORTAL,
        BlueMirror = BLUE_MIRROR_PORTAL,
        OrangeMirror = ORANGE_MIRROR_PORTAL,
        NormalSize = GREEN_SIZE_PORTAL,
        SmallSize = PINK_SIZE_PORTAL,
        EnterDual = DUAL_PORTAL,
        ExitDual = EXIT_DUAL_PORTAL,
        CubeGamemode = CUBE_PORTAL,
        ShipGamemode = SHIP_PORTAL,
        BallGamemode = BALL_PORTAL,
        UFOGamemode = UFO_PORTAL,
        WaveGamemode = WAVE_PORTAL,
        RobotGamemode = ROBOT_PORTAL,
        SpiderGamemode = SPIDER_PORTAL,
        SwingGamemode = SWING_PORTAL,
        /// This teleportal uses a [`TeleportConfig`] to specify the object to teleport to. It is effectively the portal equivalent of `TeleportTrigger`.
        /// Unlike the blue linked teleportal, this object does not store an orange counterpart or need one at all.
        UnlinkedBlueTeleportal = UNLINKED_BLUE_TELEPORT_PORTAL,
        UnlinkedOrangeTeleportal = UNLINKED_ORANGE_TELEPORT_PORTAL,
        /// This variant represents the blue of the two linked portals, as that is the object that is available to the player to place in the level.
        /// When said object is placed, another object is created on the same y-position as the blue portal (object 749, [`LINKED_ORANGE_TELEPORT_PORTAL`]).
        ///
        /// Normally, the game tries to keep these two objects linked as much as possible - it won't let you delete just the orange portal or move it away from the blue one on the x-axis.
        /// If the blue portal is deleted, so is the orange one. Part of this effort is revealed in how the game serializes this object. For each pair of linked teleportals,
        /// the y-position of the orange portal relative to the blue one is stored *in this object* (specifically in property 54, [`LINKED_ORANGE_TELEPORTAL_YOFFSET`])
        ///
        /// **TL;DR** - This is only one of two linked teleportal objects (the blue one), however, this object also stores the position of the orange counterpart.
        /// You *DO NOT* need to use `LinkedOrangeTeleportal` to ensure that the teleportation works.
        ///
        /// Note: This object also supports `TeleportConfig`.
        LinkedTeleportals = LINKED_BLUE_TELEPORT_PORTAL,
        /// This object will not do anything by itself as it exists as an internal counterpart to `LinkedTeleportals`.
        /// This object is also not available to be placed in the editor and exists only when playing the level.
        LinkedOrangeTeleportal = LINKED_ORANGE_TELEPORT_PORTAL
    }
);

/// Configuration of a gameplay portal object - any of the portals [here](GameplayPortal).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GameplayPortalConfig {
    /// What kind of portal this is.
    pub portal: GameplayPortal,
    /// Whether this portal can be activated multiple times in classic mode. Off by default.
    pub multi_activate: bool,
    /// Whether this portal can be activated multiple times in platformer mode. On by default (false here since enabling this option would disable the default behaviour).
    pub no_plat_multi_activate: bool,
    /// Extra options for portals that have them.
    pub extra_config: PortalOptions,
}
/// Extra options for portals that have them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PortalOptions {
    /// For portals that don't have any options other than multi-activate - speed, mirror, size portals.
    None,
    /// For `UnlinkedBlueTeleportal` only.
    Teleportal(TeleportConfig),
    /// For `LinkedTeleportals` only. The second item specifies the y-position of the orange teleportal in units relative to the blue one (default = 100).
    LinkedTeleportal((TeleportConfig, f64)),
    /// For gameplay portals and dual portals.
    Camera(PortalCameraOptions),
}

/// Options for camera motion in gameplay portals
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PortalCameraOptions {
    /// Disables the default behaviour of the camera snapping onto the nearest grid space when a portal is activated.
    ///
    /// If this setting is left disabled, the camera will center on the nearest block on the y-axis when the portal is hit.
    pub disable_gridsnap: bool,
    /// Removes borders for some gamemodes like ball and spider
    pub free_mode: bool,
    /// Tuple of two items - (easing, padding)
    /// * Easing: smoothness of camera motion
    /// * Padding: how close the player has to be to the edge of the screen for the camera to move
    pub edit_camera_settings: Option<(i32, f64)>,
}

// 1,1933,2,2295,3,855,155,9,36,1                                      ;    nothing
// 1,1933,2,2355,3,855,155,9,36,1                                ,370,1;    + disable gridsnap
// 1,1933,2,2415,3,855,155,9,36,1,111,1                          ,370,1;    + free mode
// 1,1933,2,2475,3,855,155,9,36,1,111,1,112,1,113,26,114,0.306667,370,1;    + edit camera settings

impl PortalCameraOptions {
    /// Serializes this object to a list of GDValues
    pub fn to_properties(&self) -> Vec<(u16, GDValue)> {
        let mut properties = vec![
            (DISABLE_GRIDSNAP, GDValue::Bool(self.disable_gridsnap)),
            (PORTAL_FREE_MODE, GDValue::Bool(self.free_mode)),
        ];
        if let Some((easing, padding)) = self.edit_camera_settings {
            properties.extend_from_slice(&[
                (PORTAL_EDIT_CAMERA_SETTINGS, GDValue::Bool(true)),
                (PORTAL_CAMERA_EASING, GDValue::Int(easing)),
                (PORTAL_CAMERA_PADDING, GDValue::Float(padding)),
            ]);
        }
        properties
    }
}

impl ObjectProperties for GameplayPortalConfig {
    fn object_id(&self) -> i32 {
        self.portal as i32
    }
    fn serialise(&self) -> Vec<(u16, GDValue)> {
        let mut properties = vec![
            (MULTI_ACTIVATE, GDValue::Bool(self.multi_activate)),
            (
                NO_MULTIACTIVATE_PLATFORMER,
                GDValue::Bool(self.no_plat_multi_activate),
            ),
        ];

        match self.extra_config {
            PortalOptions::None => {}
            PortalOptions::Teleportal(tp) => properties.extend_from_slice(&tp.to_properties()),
            PortalOptions::LinkedTeleportal((tp, y_offset)) => {
                properties.extend_from_slice(&tp.to_properties());
                properties.push((LINKED_ORANGE_TELEPORTAL_YOFFSET, GDValue::Float(y_offset)))
            }
            PortalOptions::Camera(c) => properties.extend_from_slice(&c.to_properties()),
        }

        properties
    }
    /* TODO: implement from_object */
}
