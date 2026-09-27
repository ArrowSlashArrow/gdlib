//! This module contains constructors for parameterized objects that have intrinsic properties other than those in GDObjConfig or GDObjAttributes.
pub mod gameplay;
pub mod triggers;

use crate::{
    cclocallevels::gdobj::{
        GDObjConfig, GDObject, GDValue, ObjectProperties,
        ids::{
            objects::*,
            properties::{BASE64ENCODED_TEXT, CUSTOM_ROTATION_AMOUNT, KERNING, ROTATION_DISABLED},
        },
    },
    core::b64_encode,
    repr_t,
};

/// Returns a text object
/// # Arguments
/// `config`: Object config
/// `text`: Text in the objecty
/// `kerning`: Spacing between chars. Default is 0
#[inline]
pub fn text<T: AsRef<str>>(config: &GDObjConfig, text: T, kerning: i32) -> GDObject {
    GDObject::new(
        TEXT,
        config,
        vec![
            (
                BASE64ENCODED_TEXT,
                GDValue::String(b64_encode(text.as_ref())),
            ),
            (KERNING, GDValue::Int(kerning)),
        ],
    )
}

repr_t!(
    /// All objects that spin - i.e. have a parameter to control constant rotation, which can be in one of three modes:
    ///     - Default: use the default rotation
    ///     - Custom: set a custom rotation in degrees per second
    ///     - Disable: equivalent to 0 degrees/second of rotation. Object is stationary is a result.
    ///
    /// Casting a variant of this enum returns the object's ID.
    strict SpinningObject: i32 {
        LargeSawBlade = LARGE_SAW_BLADE,
        MediumSawBlade = MEDIUM_SAW_BLADE,
        SmallSawBlade = SMALL_SAW_BLADE,
        LargeSpikeBlade = LARGE_SPIKE_BLADE,
        MediumSpikeBlade = MEDIUM_SPIKE_BLADE,
        SmallSpikeBlade = SMALL_SPIKE_BLADE,
        LargeGearBlade = LARGE_GEAR_BLADE,
        MediumGearBlade = MEDIUM_GEAR_BLADE,
        SmallGearBlade = SMALL_GEAR_BLADE,
        LargeOutlineBlade = LARGE_OUTLINE_BLADE,
        MediumOutlineBlade = MEDIUM_OUTLINE_BLADE,
        SmallOutlineBlade = SMALL_OUTLINE_BLADE,
        LargeInvisibleBlade = LARGE_INVISIBLE_BLADE,
        MediumInvisibleBlade = MEDIUM_INVISIBLE_BLADE,
        SmallInvisibleBlade = SMALL_INVISIBLE_BLADE,
        LargeColoredGear = LARGE_COLORED_GEAR,
        MediumColoredGear = MEDIUM_COLORED_GEAR,
        SmallColoredGear = SMALL_COLORED_GEAR,
        LargeScytheBlade = LARGE_SCYTHE_BLADE,
        SmallScytheBlade = SMALL_SCYTHE_BLADE,
        LargeBlade = LARGE_BLADE,
        MediumBlade = MEDIUM_BLADE,
        SmallBlade = SMALL_BLADE,
        LargeDecorativeGear = LARGE_DECORATIVE_GEAR,
        MediumDecorativeGear = MEDIUM_DECORATIVE_GEAR,
        SmallDecorativeGear = SMALL_DECORATIVE_GEAR,
        VerySmallDecorativeGear = VERY_SMALL_DECORATIVE_GEAR,
        LargeWheel = LARGE_WHEEL,
        MediumWheel = MEDIUM_WHEEL,
        SmallWheel = SMALL_WHEEL,
        LargeSpikeWheel = LARGE_SPIKE_WHEEL,
        MediumSpikeWheel = MEDIUM_SPIKE_WHEEL,
        SmallSpikeWheel = SMALL_SPIKE_WHEEL,
        LargeCartwheel = LARGE_CARTWHEEL,
        MediumCartwheel = MEDIUM_CARTWHEEL,
        SmallCartwheel = SMALL_CARTWHEEL,
        LargeRoundCloud = LARGE_ROUND_CLOUD,
        MediumRoundCloud = MEDIUM_ROUND_CLOUD,
        SmallRoundCloud = SMALL_ROUND_CLOUD,
        LargeArm = LARGE_ROTATING_ARM,
        MediumArm = MEDIUM_ROTATING_ARM,
        SmallArm = SMALL_ROTATING_ARM,
        VerySmallArm = VERY_SMALL_ROTATING_ARM,
        LargeJointlessArm = LARGE_JOINTLESS_ARM,
        MediumJointlessArm = MEDIUM_JOINTLESS_ARM,
        SmallJointlessArm = SMALL_JOINTLESS_ARM,
        VerySmallJointlessArm = VERY_SMALL_JOINTLESS_ARM,
        LargeParticle = LARGE_ROTATING_PARTICLE,
        MediumParticle = MEDIUM_ROTATING_PARTICLE,
        SmallParticle = SMALL_ROTATING_PARTICLE,
        VerySmallParticle = VERY_SMALL_ROTATING_PARTICLE,
        LargeHexagon = LARGE_ROTATING_HEXAGON,
        MediumHexagon = MEDIUM_ROTATING_HEXAGON,
        SmallHexagon = SMALL_ROTATING_HEXAGON,
        LargeSplitCircle = LARGE_SPLIT_CIRCLE,
        MediumSplitCircle = MEDIUM_SPLIT_CIRCLE,
        SmallSplitCircle = SMALL_SPLIT_CIRCLE,
        VerySmallSplitCircle = VERY_SMALL_SPLIT_CIRCLE,
        LargeShine = LARGE_ROTATING_SHINE,
        MediumShine = MEDIUM_ROTATING_SHINE,
        SmallShine = SMALL_ROTATING_SHINE,
        PulsingRingsOneDot = ROTATING_PULSING_RINGS_WITH_ONE_DOT,
        PulsingRingsTwoDots = ROTATING_PULSING_RINGS_WITH_TWO_DOTS,
        PulsingRingsFourDots = ROTATING_PULSING_RINGS_WITH_FOUR_DOTS,
        LargeSwirl = LARGE_SWIRL,
        MediumSwirl = MEDIUM_SWIRL,
        SmallSwirl = SMALL_SWIRL,
        VerySmallSwirl = VERY_SMALL_SWIRL,
        TriangleSwirl = TRIANGLE_SWIRL,
        LargeHollowQuarterCircle = LARGE_ROTATING_HOLLOW_QUARTER_CIRCLE,
        SmallHollowQuarterCircle = SMALL_ROTATING_HOLLOW_QUARTER_CIRCLE,
        LargeQuarterCircle = LARGE_ROTATING_QUARTER_CIRCLE,
        SmallQuarterCircle = SMALL_ROTATING_QUARTER_CIRCLE,
    }
);

/// Configuration for any object that naturally spins
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpinningObjectConfig {
    /// The specific [`SpinningObject`]
    pub object: SpinningObject,
    /// Describes how the object will spin
    pub rotation: RotationConfig,
}

impl ObjectProperties for SpinningObjectConfig {
    fn object_id(&self) -> i32 {
        self.object as i32
    }
    fn serialise(&self) -> Vec<(u16, GDValue)> {
        match self.rotation {
            RotationConfig::Default => vec![],
            RotationConfig::Custom(deg) => vec![(CUSTOM_ROTATION_AMOUNT, GDValue::Int(deg))],
            RotationConfig::Disabled => vec![(ROTATION_DISABLED, GDValue::Bool(true))],
        }
    }
    fn from_object(obj: &GDObject) -> Option<Self>
    where
        Self: Sized,
    {
        let spinner = SpinningObject::try_from(obj.id).ok()?;
        let spin = if let Some(deg) = obj.get_property(CUSTOM_ROTATION_AMOUNT)
            && let GDValue::Int(i) = deg
        {
            RotationConfig::Custom(i)
        } else if let Some(no_rotation) = obj.get_property(ROTATION_DISABLED)
            && no_rotation == GDValue::Bool(true)
        {
            RotationConfig::Disabled
        } else {
            RotationConfig::Default
        };
        Some(SpinningObjectConfig {
            object: spinner,
            rotation: spin,
        })
    }
}

/// Describes how a [`SpinningObject`] with rotate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RotationConfig {
    /// Default rotation (randomly assigned by GD)
    #[default]
    Default,
    /// Custom amount of rotation in degrees per second. Positive rotation is clockwise, negative rotation is counterclockwise.
    ///
    /// To set a rotation of 0 degrees / second, use `RotationConfig::Default`.
    Custom(i32),
    /// Prevents the object from rotating naturally.
    Disabled,
}

// default - 1,395,2,2505,3,315;        (no change)
// custom - 1,395,2,2505,3,315,97,125;  (+ 97, deg)
// disabled - 1,395,2,2505,3,315,98,1;  (+ 98, tru)
