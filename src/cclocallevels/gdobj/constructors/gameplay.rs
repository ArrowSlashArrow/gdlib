//! Constructors for parameterized gameplay objects: pads, orbs, portals and force blocks.
//!
//! | Object | Config struct | Selects the object with | Extra options |
//! |---|---|---|---|
//! | Pads | [`GDPad`] | [`GDPadType`] | None |
//! | Orbs | [`GDOrb`] | [`GDOrbType`] | [`OrbOptions`], including [`DashOrbSettings`] |
//! | Force blocks | [`ForceObjectConfig`] | [`ForceObject`] | [`Force`] |
//! | Portals | [`GameplayPortalConfig`] | [`GameplayPortal`] | [`PortalOptions`], including [`PortalCameraOptions`] |
//!
//! # Example
//!
//! ```
//! use gdlib::cclocallevels::gdobj::{GDObject, GDPad, GDPadType, ids::objects, meta::GDObjConfig};
//!
//! let config = GDObjConfig::new().with_pos(150.0, 105.0);
//! let pad = GDObject::from_config(&config, GDPad::new(GDPadType::Yellow));
//! assert_eq!(pad.id, objects::YELLOW_JUMP_PAD);
//! ```
//!
//! # Gotchas
//!
//! - **A toggle orb has one multi-activate setting.** [`GDOrb::multi_activate`] and the [`ToggleBlock::multi_activate`] inside [`OrbOptions::ToggleOrb`] are the same property. The orb's own field is the one that's written, and [`ObjectProperties::from_object`] copies it into both.
//! - **Portal options are inferred when converting back.** [`ObjectProperties::from_object`] picks [`PortalOptions`] from the portal type, and only returns [`PortalOptions::Camera`] if the object has camera properties.

use crate::cclocallevels::{
    consts::*,
    gdobj::{
        GDObject,
        GDOrbType::{GreenDash, PinkDash},
        ObjectProperties, ToggleBlock,
        ids::{objects::*, properties::*},
        structs::{GDValue, TeleportConfig},
    },
};
use crate::repr_t;

/// Reads a boolean property, treating anything other than `Bool(true)` as `false`.
fn flag(obj: &GDObject, p: u16) -> bool {
    matches!(obj.get_property(p), Some(GDValue::Bool(true)))
}

/// Reads a float property, or returns `default` if it is missing or has another type.
fn float(obj: &GDObject, p: u16, default: f32) -> f32 {
    match obj.get_property(p) {
        Some(GDValue::Float(f)) => f,
        _ => default,
    }
}

/// Reads an integer-like property, or returns 0 if it is missing or has another type.
fn int(obj: &GDObject, p: u16) -> i32 {
    match obj.get_property(p) {
        Some(GDValue::Int(i)) => i,
        Some(GDValue::Short(i) | GDValue::Group(i) | GDValue::Item(i)) => i as i32,
        _ => 0,
    }
}

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
        Blue = BLUE_GRAVITY_ORB,
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
        matches!(self, GreenDash | PinkDash)
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Configuration struct for orbs.
pub struct GDOrb {
    /// What type of orb this object is.
    pub otype: GDOrbType,
    /// Allows the orb to activate multiple times in normal mode
    pub multi_activate: bool,
    /// Restricts the orb from activating multiple times in platformer mode
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
            // the toggle block's multi-activate is the same property as the orb's own, which was written above.
            OrbOptions::ToggleOrb(ref t) => properties.extend(
                t.serialise()
                    .into_iter()
                    .filter(|(id, _)| *id != MULTI_ACTIVATE),
            ),
        };

        properties
    }

    fn from_object(obj: &GDObject) -> Option<Self> {
        let otype = GDOrbType::try_from(obj.id).ok()?;
        let multi_activate = flag(obj, MULTI_ACTIVATE);

        let extra_orb_config = match otype {
            GreenDash | PinkDash => {
                let defaults = DashOrbSettings::default();
                OrbOptions::DashOrb(DashOrbSettings {
                    speed: float(obj, DASH_ORB_SPEED, defaults.speed),
                    end_boost: float(obj, DASH_ORB_END_BOOST, defaults.end_boost),
                    max_duration: float(obj, DASH_ORB_MAXIMAL_DURATION, defaults.max_duration),
                    allow_collide: flag(obj, DASH_ORB_ALLOW_COLLIDE),
                    stop_slide: flag(obj, DASH_ORB_STOP_SLIDE),
                })
            }
            GDOrbType::Teleport => OrbOptions::Teleporb(TeleportConfig::from_object(obj)),
            GDOrbType::Toggle => OrbOptions::ToggleOrb(ToggleBlock {
                target_group: int(obj, TARGET_ITEM) as i16,
                activate_group: flag(obj, ACTIVATE_GROUP),
                claim_touch: flag(obj, CLAIM_TOUCH),
                multi_activate,
                spawn_only: flag(obj, SPAWN_ONLY),
            }),
            _ => OrbOptions::None,
        };

        Some(Self {
            otype,
            multi_activate,
            no_plat_multi_activate: flag(obj, NO_MULTIACTIVATE_PLATFORMER),
            extra_orb_config,
        })
    }
}

/// Settings for dash orbs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DashOrbSettings {
    /// Speed of the dash. Default is `1.0`
    pub speed: f32,
    /// Force applied to the player when the dash stops. Default is `1.0`
    pub end_boost: f32,
    /// Dash ends automatically after this many seconds have passed since the orb was activated. Set to `0.0` for infinite duration.
    pub max_duration: f32,
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

    fn from_object(obj: &GDObject) -> Option<Self> {
        let force_object = ForceObject::try_from(obj.id).ok()?;
        // a force object uses a range if either bound is set; otherwise it uses a fixed magnitude.
        let force =
            if obj.has_property(FORCE_RANGE_MINIMUM) || obj.has_property(FORCE_RANGE_MAXIMUM) {
                Force::Range {
                    minimum: float(obj, FORCE_RANGE_MINIMUM, 0.0),
                    maximum: float(obj, FORCE_RANGE_MAXIMUM, 0.0),
                }
            } else {
                Force::Scalar(float(obj, FORCE_MAGNITUDE, 0.0))
            };

        Some(Self {
            force_object,
            force,
            is_relative: flag(obj, RELATIVE_FORCE),
            force_id: int(obj, FORCE_ID),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Force that is applied to a player in a force object
pub enum Force {
    /// Applies a fixed amount of force in the direction that the force block is pointing - 0 degrees = up, 90 degrees = right, 180 degrees = down, -90 degrees = left
    Scalar(f32),
    /// Applies force based on player position.
    Range {
        /// @nodoc
        minimum: f32,
        /// @nodoc
        maximum: f32,
    },
}

// three different portals:
// 1. speed portal - just has `allow multi` (both classic and platformer)
// 2. gameplay portal - camera settings
// 3. teleportals (linked teleportals are one object)

repr_t!(
    /// Type of speed portal in GD. The variant corresponds to each speed portal by its color.
    ///
    /// **Note:** to create a speed portal with `multi activate` enabled, you must set this in the object's `GDObjConfig.trigger_cfg.multitriggerable` (see [`GDObjConfig::set_multitrigger`](crate::cclocallevels::gdobj::meta::GDObjConfig::set_multitrigger)).
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
    pub fn get_speed(&self) -> f32 {
        match self {
            Self::Yellow => SPEED_05X,
            Self::Blue => SPEED_1X,
            Self::Green => SPEED_2X,
            Self::Pink => SPEED_3X,
            Self::Red => SPEED_4X,
        }
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
        /// If the blue portal is deleted, so is the orange one. This is revealed in how the game serializes this object. For each pair of linked teleportals,
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
    LinkedTeleportal((TeleportConfig, f32)),
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
    pub edit_camera_settings: Option<(i32, f32)>,
}

impl PortalCameraOptions {
    /// Reads the camera options of a portal. Returns `None` if the object has none of the camera properties.
    pub fn from_object(obj: &GDObject) -> Option<Self> {
        if ![
            DISABLE_GRIDSNAP,
            PORTAL_FREE_MODE,
            PORTAL_EDIT_CAMERA_SETTINGS,
        ]
        .iter()
        .any(|p| obj.has_property(*p))
        {
            return None;
        }

        Some(Self {
            disable_gridsnap: flag(obj, DISABLE_GRIDSNAP),
            free_mode: flag(obj, PORTAL_FREE_MODE),
            edit_camera_settings: flag(obj, PORTAL_EDIT_CAMERA_SETTINGS).then(|| {
                (
                    int(obj, PORTAL_CAMERA_EASING),
                    float(obj, PORTAL_CAMERA_PADDING, 0.0),
                )
            }),
        })
    }

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

    fn from_object(obj: &GDObject) -> Option<Self> {
        use GameplayPortal::*;
        let portal = GameplayPortal::try_from(obj.id).ok()?;

        let extra_config = match portal {
            UnlinkedBlueTeleportal => PortalOptions::Teleportal(TeleportConfig::from_object(obj)),
            LinkedTeleportals => PortalOptions::LinkedTeleportal((
                TeleportConfig::from_object(obj),
                float(obj, LINKED_ORANGE_TELEPORTAL_YOFFSET, 100.0),
            )),
            CubeGamemode | ShipGamemode | BallGamemode | UFOGamemode | WaveGamemode
            | RobotGamemode | SpiderGamemode | SwingGamemode | EnterDual | ExitDual => {
                PortalCameraOptions::from_object(obj)
                    .map_or(PortalOptions::None, PortalOptions::Camera)
            }
            _ => PortalOptions::None,
        };

        Some(Self {
            portal,
            multi_activate: flag(obj, MULTI_ACTIVATE),
            no_plat_multi_activate: flag(obj, NO_MULTIACTIVATE_PLATFORMER),
            extra_config,
        })
    }
}

// checkpoint object (extra properties: 107, 11, 36, all 1)
// 1,2063,2,285,3,75,107,1,11,1,36,1                                     ;  base checkpoint
// 1,2063,2,285,3,75,107,1,11,1,36,1,                               138,1;  + player pos
// 1,2063,2,285,3,75,107,1,11,1,36,1,                       604,123,138,1;  + pre spawn id
// 1,2063,2,285,3,75,107,1,11,1,36,1,51,456,                604,123,138,1;  + spawn id
// 1,2063,2,285,3,75,107,1,11,1,36,1,51,456,        448,789,604,123,138,1;  + respawn id
// 1,2063,2,285,3,75,107,1,11,1,36,1,51,456,71,1122,448,789,604,123,138,1;  + target pos

/// Saves level state in platformer mode. Note that this object is touch-triggerable by default in the editor.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Checkpoint {
    /// Group spawned before this checkpoint is saved.
    ///
    /// Note: While this field is an integer, it is parsed as a float before being cast to an integer. This means that large precise values like 123456789 don't actually get parsed as they are, they are rounded slightly.
    pub pre_spawn_id: i32, // 604
    /// Group spawned once the checkpoint is saved
    pub spawn_id: i16, // 51
    /// Group spawned once the player respawns at this checkpoint.
    ///
    /// Note: While this field is an integer, it is parsed as a float before being cast to an integer. This means that large precise values like 123456789 don't actually get parsed as they are, they are rounded slightly.
    pub respawn_id: i32, // 448
    /// If not 0, the player respawns at this group ID. This group should only have one object; if it doesn't the player respawns at the location of the checkpoint instead.
    pub target_pos: i16, // 71
    /// Respawns where the player was when the checkpoint was touched. Overrides `target_pos`.
    pub player_pos: bool, // 138
}

impl ObjectProperties for Checkpoint {
    fn object_id(&self) -> i32 {
        CHECKPOINT
    }
    fn serialise(&self) -> Vec<(u16, GDValue)> {
        vec![
            (CHECKPOINT_UNKNOWN_107, GDValue::Int(1)),
            (IS_INTERACTABLE, GDValue::Bool(true)),
            (CONTROLLING_PLAYER_1, GDValue::Bool(self.player_pos)),
            (CHECKPOINT_RESPAWN_ID, GDValue::Int(self.respawn_id)),
            (CHECKPOINT_PRE_SPAWN_ID, GDValue::Int(self.pre_spawn_id)),
            (TARGET_ITEM, GDValue::Group(self.spawn_id)),
            (TARGET_ITEM_2, GDValue::Group(self.target_pos)),
        ]
    }
    fn from_object(obj: &GDObject) -> Option<Self>
    where
        Self: Sized,
    {
        if obj.id != CHECKPOINT {
            return None;
        }

        // 107 and 36 are always 1 and carry no information, so they aren't read back.
        Some(Self {
            pre_spawn_id: int(obj, CHECKPOINT_PRE_SPAWN_ID),
            spawn_id: int(obj, TARGET_ITEM) as i16,
            respawn_id: int(obj, CHECKPOINT_RESPAWN_ID),
            target_pos: int(obj, TARGET_ITEM_2) as i16,
            player_pos: flag(obj, CONTROLLING_PLAYER_1),
        })
    }
}

repr_t!(
    /// All collectible objects in GD - objects that do something when you "collect" them.
    GDCollectible: i32 {
        Key = KEY_COLLECTABLE,
        Heart = HEART_COLLECTABLE,
        Potion = POTION_COLLECTABLE,
        Skull = SKULL_COLLECTABLE,
        SmallCoin = SMALL_COIN_COLLECTABLE,
        Clock = CLOCK_COLLECTABLE,
        SmallPotionPixel = SMALL_POTION_PIXEL_COLLECTABLE,
        MediumPotionPixel = MEDIUM_POTION_PIXEL_COLLECTABLE,
        LargePotionPixel = LARGE_POTION_PIXEL_COLLECTABLE,
        DiagonalKeyPixel = DIAGONAL_KEY_PIXEL_COLLECTABLE,
        RoundKeyPixel = ROUND_KEY_PIXEL_COLLECTABLE,
        FlatKeyPixel = FLAT_KEY_PIXEL_COLLECTABLE,
        CoinPixel = COIN_PIXEL_COLLECTABLE,
        SmallCoinStackPixel = SMALL_COIN_STACK_PIXEL_COLLECTABLE,
        LargeCoinStackPixel = LARGE_COIN_STACK_PIXEL_COLLECTABLE,
        CandyPixel = CANDY_PIXEL_COLLECTABLE,
        MushroomPixel = MUSHROOM_PIXEL_COLLECTABLE,
        BonePixel = BONE_PIXEL_COLLECTABLE,
        SpherePixel = SPHERE_PIXEL_COLLECTABLE,
        IngotPixel = INGOT_PIXEL_COLLECTABLE,
        SquareGemPixel = SQUARE_GEM_PIXEL_COLLECTABLE,
        HexagonGemPixel = HEXAGON_GEM_PIXEL_COLLECTABLE,
        OctagonGemPixel = OCTAGON_GEM_PIXEL_COLLECTABLE,
        RectangleGemPixel = RECTANGLE_GEM_PIXEL_COLLECTABLE,
        PointyGemPixel = POINTY_GEM_PIXEL_COLLECTABLE,
        DiamondPixel = DIAMOND_PIXEL_COLLECTABLE,
        FishPixel = FISH_PIXEL_COLLECTABLE,
        RocksPixel = ROCKS_PIXEL_COLLECTABLE,
        WoodPixel = WOOD_PIXEL_COLLECTABLE,
        EggPixel = EGG_PIXEL_COLLECTABLE,
        LargeHeartPixel = LARGE_HEART_PIXEL_COLLECTABLE,
        SmallHeartPixel = SMALL_HEART_PIXEL_COLLECTABLE,
        MapPixel = MAP_PIXEL_COLLECTABLE,
        BookPixel = BOOK_PIXEL_COLLECTABLE,
        DevicePixel = DEVICE_PIXEL_COLLECTABLE,
        ComputerPixel = COMPUTER_PIXEL_COLLECTABLE,
        LargeSkullPixel = LARGE_SKULL_PIXEL_COLLECTABLE,
        SmallSkullPixel = SMALL_SKULL_PIXEL_COLLECTABLE,
        SpeechBubblePixel = SPEECH_BUBBLE_PIXEL_COLLECTABLE,
        BranchPixel = BRANCH_PIXEL_COLLECTABLE,
        ShardGemPixel = SHARD_GEM_PIXEL_COLLECTABLE,
        EyePixel = EYE_PIXEL_COLLECTABLE,
        EyeballPixel = EYEBALL_PIXEL_COLLECTABLE,
        UpArrowPixel = UP_ARROW_PIXEL_COLLECTABLE,
        DownArrowPixel = DOWN_ARROW_PIXEL_COLLECTABLE,
        LightningPixel = LIGHTNING_PIXEL_COLLECTABLE,
        NoSymbolPixel = NO_SYMBOL_PIXEL_COLLECTABLE,
        GearPixel = GEAR_PIXEL_COLLECTABLE,
        PlusPixel = PLUS_PIXEL_COLLECTABLE,
        PlusSymbolsPixel = PLUS_SYMBOLS_PIXEL_COLLECTABLE,
        PebblesPixel = PEBBLES_PIXEL_COLLECTABLE,
        PresentPixel = PRESENT_PIXEL_COLLECTABLE,
        ChestPixel = CHEST_PIXEL_COLLECTABLE,
        BagPixel = BAG_PIXEL_COLLECTABLE,
        BackpackPixel = BACKPACK_PIXEL_COLLECTABLE,
        BundlePixel = BUNDLE_PIXEL_COLLECTABLE,
        RingPixel = RING_PIXEL_COLLECTABLE,
        PatternedRingPixel = PATTERNED_RING_PIXEL_COLLECTABLE,
        GemRingPixel = GEM_RING_PIXEL_COLLECTABLE,
        LargeNecklacePixel = LARGE_NECKLACE_PIXEL_COLLECTABLE,
        SmallNecklacePixel = SMALL_NECKLACE_PIXEL_COLLECTABLE,
        ShoePixel = SHOE_PIXEL_COLLECTABLE,
        BootPixel = BOOT_PIXEL_COLLECTABLE,
        PointyBootPixel = POINTY_BOOT_PIXEL_COLLECTABLE,
        HatPixel = HAT_PIXEL_COLLECTABLE,
        PointyHatPixel = POINTY_HAT_PIXEL_COLLECTABLE,
        CurlyHatPixel = CURLY_HAT_PIXEL_COLLECTABLE,
        Helmet1Pixel = HELMET_1_PIXEL_COLLECTABLE,
        Helmet2Pixel = HELMET_2_PIXEL_COLLECTABLE,
        Helmet3Pixel = HELMET_3_PIXEL_COLLECTABLE,
        Mask1Pixel = MASK_1_PIXEL_COLLECTABLE,
        Mask2Pixel = MASK_2_PIXEL_COLLECTABLE,
        Mask3Pixel = MASK_3_PIXEL_COLLECTABLE,
        RoundMaskPixel = ROUND_MASK_PIXEL_COLLECTABLE,
        HornedRoundMaskPixel = HORNED_ROUND_MASK_PIXEL_COLLECTABLE,
        SmallLabCoatPixel = SMALL_LAB_COAT_PIXEL_COLLECTABLE,
        LargeLabCoatPixel = LARGE_LAB_COAT_PIXEL_COLLECTABLE,
        Armor1Pixel = ARMOR_1_PIXEL_COLLECTABLE,
        Armor2Pixel = ARMOR_2_PIXEL_COLLECTABLE,
        Armor3Pixel = ARMOR_3_PIXEL_COLLECTABLE,
        Armor4Pixel = ARMOR_4_PIXEL_COLLECTABLE,
        Armor5Pixel = ARMOR_5_PIXEL_COLLECTABLE,
        Armor6Pixel = ARMOR_6_PIXEL_COLLECTABLE,
        Shield1Pixel = SHIELD_1_PIXEL_COLLECTABLE,
        Shield2Pixel = SHIELD_2_PIXEL_COLLECTABLE,
        Shield3Pixel = SHIELD_3_PIXEL_COLLECTABLE,
        Shield4Pixel = SHIELD_4_PIXEL_COLLECTABLE,
        Shield5Pixel = SHIELD_5_PIXEL_COLLECTABLE,
        Shield6Pixel = SHIELD_6_PIXEL_COLLECTABLE,
        Shield7Pixel = SHIELD_7_PIXEL_COLLECTABLE,
        Shield8Pixel = SHIELD_8_PIXEL_COLLECTABLE,
        Shield9Pixel = SHIELD_9_PIXEL_COLLECTABLE,
        SwordPixel = SWORD_PIXEL_COLLECTABLE,
        BowPixel = BOW_PIXEL_COLLECTABLE,
        AxePixel = AXE_PIXEL_COLLECTABLE,
        SpearPixel = SPEAR_PIXEL_COLLECTABLE,
        LargeStaffPixel = LARGE_STAFF_PIXEL_COLLECTABLE,
        ShovelPixel = SHOVEL_PIXEL_COLLECTABLE,
        PickaxePixel = PICKAXE_PIXEL_COLLECTABLE,
        HammerPixel = HAMMER_PIXEL_COLLECTABLE,
        HoePixel = HOE_PIXEL_COLLECTABLE,
        SmallStaffPixel = SMALL_STAFF_PIXEL_COLLECTABLE,
        HookPixel = HOOK_PIXEL_COLLECTABLE,
        ExclamationPointPixel = EXCLAMATION_POINT_PIXEL_COLLECTABLE,
        QuestionMarkPixel = QUESTION_MARK_PIXEL_COLLECTABLE,
        PlusSignPixel = PLUS_SIGN_PIXEL_COLLECTABLE,
        MinusSignPixel = MINUS_SIGN_PIXEL_COLLECTABLE,
        EqualsSignPixel = EQUALS_SIGN_PIXEL_COLLECTABLE,
        TimesSignPixel = TIMES_SIGN_PIXEL_COLLECTABLE,
        DivisionSignPixel = DIVISION_SIGN_PIXEL_COLLECTABLE,
        Numeral0Pixel = NUMERAL_0_PIXEL_COLLECTABLE,
        Numeral1Pixel = NUMERAL_1_PIXEL_COLLECTABLE,
        Numeral2Pixel = NUMERAL_2_PIXEL_COLLECTABLE,
        Numeral3Pixel = NUMERAL_3_PIXEL_COLLECTABLE,
        Numeral4Pixel = NUMERAL_4_PIXEL_COLLECTABLE,
        Numeral5Pixel = NUMERAL_5_PIXEL_COLLECTABLE,
        Numeral6Pixel = NUMERAL_6_PIXEL_COLLECTABLE,
        Numeral7Pixel = NUMERAL_7_PIXEL_COLLECTABLE,
        Numeral8Pixel = NUMERAL_8_PIXEL_COLLECTABLE,
        Numeral9Pixel = NUMERAL_9_PIXEL_COLLECTABLE,
        AnimalSkullPixel = ANIMAL_SKULL_PIXEL_COLLECTABLE,
        CrackedSkullPixel = CRACKED_SKULL_PIXEL_COLLECTABLE,
        SmallBonePixel = SMALL_BONE_PIXEL_COLLECTABLE,
        LargeKeyPixel = LARGE_KEY_PIXEL_COLLECTABLE,
        KeyholeChestPixel = KEYHOLE_CHEST_PIXEL_COLLECTABLE,
        BreadPixel = BREAD_PIXEL_COLLECTABLE,
        SmallDiamondPixel = SMALL_DIAMOND_PIXEL_COLLECTABLE,
        CloakPixel = CLOAK_PIXEL_COLLECTABLE,
        ScrollPixel = SCROLL_PIXEL_COLLECTABLE,
        BannerPixel = BANNER_PIXEL_COLLECTABLE,
        BombPixel = BOMB_PIXEL_COLLECTABLE,
        MetalNuggetPixel = METAL_NUGGET_PIXEL_COLLECTABLE,
        SmallArrowPixel = SMALL_ARROW_PIXEL_COLLECTABLE,
        LargeArrowPixel = LARGE_ARROW_PIXEL_COLLECTABLE,
        CheesePixel = CHEESE_PIXEL_COLLECTABLE,
        ApplePixel = APPLE_PIXEL_COLLECTABLE,
        CarrotPixel = CARROT_PIXEL_COLLECTABLE,
        SteakPixel = STEAK_PIXEL_COLLECTABLE,
        FirePixel = FIRE_PIXEL_COLLECTABLE,
        WavePixel = WAVE_PIXEL_COLLECTABLE,
        SpikePixel = SPIKE_PIXEL_COLLECTABLE,
        CauldronPixel = CAULDRON_PIXEL_COLLECTABLE,
        FishingRodPixel = FISHING_ROD_PIXEL_COLLECTABLE,
        PointyBowPixel = POINTY_BOW_PIXEL_COLLECTABLE,
        FloppyDiskPixel = FLOPPY_DISK_PIXEL_COLLECTABLE,
    }
);

/// Object that does something when it is collected.
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(missing_docs)]
pub struct CollectibleObject {
    pub object: GDCollectible,
    /// Particle group that is spawned when the item is collected
    pub particle: i32,
    pub points: i32,
    /// Corresponds to the "Toggle trigger" option in a collectible.
    pub spawn_group: bool,
    pub group_id: i16,
    /// Toggles/spawns the spawn_group instead of toggling it off
    pub enable_group: bool,
    /// Sets this object to be a pickup item - a collectible that adds `self.points` to `self.item_id` when collected.
    pub pickup_item: bool,
    pub item_id: i16,
    /// Subtracts from `item_id` instead of adding to it (see `pickup_item`)
    pub subtract_count: bool,
    /// Disables collection animation
    pub no_animation: bool,
}

impl ObjectProperties for CollectibleObject {
    fn object_id(&self) -> i32 {
        self.object.to_num()
    }

    fn serialise(&self) -> Vec<(u16, GDValue)> {
        vec![
            (COLLECTIBLE_PARTICLE_GROUP, GDValue::Int(self.particle)),
            (COLLECTIBLE_POINTS, GDValue::Int(self.points)),
            (COLLECTIBLE_TOGGLE_TRIGGER, GDValue::Bool(self.spawn_group)),
            (TARGET_ITEM, GDValue::Group(self.group_id)),
            (ACTIVATE_GROUP, GDValue::Bool(self.enable_group)),
            (COLLECTIBLE_PICKUP_ITEM, GDValue::Bool(self.pickup_item)),
            (INPUT_ITEM_1, GDValue::Item(self.item_id)),
            (
                COLLECTIBLE_SUBTRACT_COUNT,
                GDValue::Bool(self.subtract_count),
            ),
            (COLLECTIBLE_NO_ANIMATION, GDValue::Bool(self.no_animation)),
        ]
    }

    fn from_object(obj: &GDObject) -> Option<Self>
    where
        Self: Sized,
    {
        let object = match GDCollectible::from(obj.id) {
            GDCollectible::Unrecognized(_) => return None,
            c => c,
        };

        Some(Self {
            object,
            particle: int(obj, COLLECTIBLE_PARTICLE_GROUP),
            points: int(obj, COLLECTIBLE_POINTS),
            spawn_group: flag(obj, COLLECTIBLE_TOGGLE_TRIGGER),
            group_id: int(obj, TARGET_ITEM) as i16,
            enable_group: flag(obj, ACTIVATE_GROUP),
            pickup_item: flag(obj, COLLECTIBLE_PICKUP_ITEM),
            item_id: int(obj, INPUT_ITEM_1) as i16,
            subtract_count: flag(obj, COLLECTIBLE_SUBTRACT_COUNT),
            no_animation: flag(obj, COLLECTIBLE_NO_ANIMATION),
        })
    }
}
