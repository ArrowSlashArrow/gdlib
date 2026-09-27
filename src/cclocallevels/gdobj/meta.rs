//! This module contains all object metadata descriptors.

use std::fmt::{Debug, Display, Write};

use bitflags::bitflags;

use crate::cclocallevels::gdobj::{
    ids::properties::*,
    structs::{ColourChannel, Group, GroupType, ZLayer},
};
/// Object config, used for defining general properties of an object
#[derive(Clone, Debug, PartialEq)]
#[must_use]
pub struct GDObjConfig {
    /// Position of this object
    pub pos: (f64, f64),
    /// Scale of this object
    pub scale: (f64, f64),
    /// Angle of rotation
    pub angle: f64,
    /// Groups (both parents and regular)
    pub groups: Vec<Group>,
    /// Trigger activation config
    pub trigger_cfg: TriggerConfig,
    /// Z order of this object
    pub z_order: i32,
    /// Z layer of this object
    pub z_layer: ZLayer,
    /// Editor layers of this object
    pub editor_layers: (i16, i16),
    /// Main and detail colour channels respectively
    pub colour_channels: (ColourChannel, ColourChannel),
    /// Enter effect channel
    pub enter_effect_channel: i16,
    /// Material ID
    pub material_id: i16,
    /// Control ID
    pub control_id: i16,
    /// Common attributes
    pub attributes: GDObjAttributes,
}

impl Default for GDObjConfig {
    fn default() -> Self {
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
            control_id: 0,
            attributes: GDObjAttributes::new(),
        }
    }
}

use paste::paste;

macro_rules! config_fn {
    ($fn_ident:ident ($t:ty) => $param:ident $(. $subparam:tt)?) => {
        paste! {
            #[inline]
            #[doc = concat!("Sets this object's ", stringify!($fn_ident), " field.")]
            pub fn [< set _ $fn_ident >](&mut self, $fn_ident: $t) {
                self.$param $(. $subparam)? = $fn_ident;
            }

            #[inline]
            #[doc = concat!("Sets this object's ", stringify!($fn_ident), " field and returns this object.")]
            ///
            /// This function enables object builder syntax.
            pub fn [< with _ $fn_ident >](mut self, $fn_ident: $t) -> Self {
                self.$param $(. $subparam)? = $fn_ident;
                self
            }
        }
    };

    (#[$customdoc:meta] $fn_ident:ident ($t:ty) => $param:ident $(. $subparam:tt)?) => {


        paste! {
            #[inline]
            #[$customdoc]
            pub fn [< set _ $fn_ident >](&mut self, $fn_ident: $t) {
                self.$param $(. $subparam)? = $fn_ident;
            }

            #[inline]
            #[$customdoc]
            ///
            /// This function enables object builder syntax.
            pub fn [< with _ $fn_ident >](mut self, $fn_ident: $t) -> Self {
                self.$param $(. $subparam)? = $fn_ident;
                self
            }
        }
    };
}

impl GDObjConfig {
    /// Alias for default
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Serialises this config struct to a string
    #[must_use]
    pub fn serialise_to_string(&self) -> String {
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

        // i16
        serialise_fields(
            &[
                ("20", self.editor_layers.0, 0),
                ("61", self.editor_layers.1, 0),
                (
                    "21",
                    self.colour_channels.0.into(),
                    ColourChannel::Object.into(),
                ),
                ("22", self.colour_channels.1.into(), 1),
                ("24", self.z_layer as i16, ZLayer::T1 as i16),
                ("343", self.enter_effect_channel, 0),
                ("446", self.material_id, 0),
                ("534", self.control_id, 0),
            ],
            &mut properties,
        );

        serialise_fields(&[("25", self.z_order, 0)], &mut properties);

        if !self.groups.is_empty() {
            properties.push_str(",57,");
            let mut i_buf = itoa::Buffer::new();
            for (idx, group) in self.groups.iter().enumerate() {
                if idx != 0 {
                    properties.push('.');
                }
                properties.push_str(i_buf.format(group.id()));
            }
        }

        properties
    }

    fn dedup_groups(&mut self) {
        self.groups.sort();
        self.groups.dedup_by(|a, b| a.id() == b.id());
    }

    /// Sets groups of this object.
    ///
    /// This function allows for object builder syntax
    #[inline]
    pub fn with_groups<T: IntoIterator<Item = I>, I: Into<Group>>(mut self, groups: T) -> Self {
        self.groups = groups.into_iter().map(std::convert::Into::into).collect();
        self.dedup_groups();
        self
    }

    /// Sets groups of this object.
    #[inline]
    pub fn set_groups<T: IntoIterator<Item = I>, I: Into<Group>>(mut self, groups: T) {
        self.groups = groups.into_iter().map(std::convert::Into::into).collect();
        self.dedup_groups();
    }
    /// Adds groups to this object's groups
    #[inline]
    pub fn add_groups<T: AsRef<[Group]>>(&mut self, groups: T) {
        self.groups.extend_from_slice(groups.as_ref());
        self.dedup_groups();
    }
    /// Adds group to this object's groups
    #[inline]
    pub fn add_group(&mut self, group: Group) {
        self.groups.push(group);
        self.dedup_groups();
    }
    /// Removes this group from this object's groups
    #[inline]
    pub fn remove_group(&mut self, group: Group) {
        if let Some(idx) = self.groups.iter().position(|&g| g == group) {
            self.groups.swap_remove(idx);
        }
    }
    /// Clears all groups from this object
    #[inline]
    pub fn clear_groups(&mut self) {
        self.groups.clear();
    }

    config_fn!(
        /// Sets this object's y-position.
        x(f64) => pos.0
    );
    config_fn!(
        /// Sets this object's y-position.
        y(f64) => pos.1
    );

    /// Applies a translation to this object's position
    #[inline]
    pub fn translate(mut self, x: f64, y: f64) -> Self {
        self.pos.0 += x;
        self.pos.1 += y;
        self
    }

    /// Sets x and y position of this object
    #[inline]
    pub fn set_pos(mut self, x: f64, y: f64) {
        self.pos = (x, y);
    }
    /// Sets x and y position of this object
    ///
    /// This function enables object builder syntax.
    #[inline]
    pub fn with_pos(mut self, x: f64, y: f64) -> Self {
        self.pos = (x, y);
        self
    }
    config_fn!(xscale(f64) => scale.0);
    config_fn!(yscale(f64) => scale.1);
    /// Sets x and y scale of this object.
    #[inline]
    pub fn set_scale(mut self, x: f64, y: f64) {
        self.scale = (x, y);
    }
    /// Sets x and y scale of this object.
    ///
    /// This function enables object builder syntax.
    #[inline]
    pub fn with_scale(mut self, x: f64, y: f64) -> Self {
        self.scale = (x, y);
        self
    }
    config_fn!(angle(f64) => angle);
    config_fn!(
        /// Makes this object touch triggerable. This option is used primarily in triggers but also in things such as collectibles and orbs. This field is automatically set when such an object is placed in-game.
        touchable(bool) => trigger_cfg.touchable
    );
    config_fn!(
        /// Makes this object spawn triggerable. This option is used primarily in triggers.
        spawnable(bool) => trigger_cfg.spawnable
    );
    config_fn!(
        /// Makes this object multi-triggerable. This option is used in any object that can actiavte more than once. This includes pads, portals, orbs, and triggers. This field is automatically set when such an object is placed in-game.
        multitrigger(bool) => trigger_cfg.multitriggerable
    );
    config_fn!(base_colour(ColourChannel) => colour_channels.0);
    config_fn!(detail_colour(ColourChannel) => colour_channels.1);
    config_fn!(
        /// Sets this object's Z-layer
        z_layer(ZLayer) => z_layer
    );
    config_fn!(
        /// Sets this object's Z-order
        z_order(i32) => z_order
    );
    config_fn!(control_id(i16) => control_id);
    config_fn!(enter_channel(i16) => enter_effect_channel);
    config_fn!(material_id(i16) => material_id);
    config_fn!(editor_layer_1(i16) => editor_layers.0);
    config_fn!(editor_layer_2(i16) => editor_layers.1);

    /// Gets the value of a set attribute flag.  
    /// All flags are false by default.
    #[inline]
    #[must_use]
    pub fn get_attributes(&self, flag: GDObjAttributes) -> bool {
        self.attributes.contains(flag)
    }

    /// Sets the attribute of the specified flag.
    #[inline]
    pub fn set_attributes(mut self, flag: GDObjAttributes, toggle: bool) {
        self.attributes.set(flag, toggle);
    }

    /// Sets the attribute(s) of the specified flag. Function is useable in builder syntax.
    #[inline]
    pub fn with_attributes(mut self, flag: GDObjAttributes, toggle: bool) -> Self {
        self.attributes.set(flag, toggle);
        self
    }
}

bitflags! {
    #[allow(missing_docs)]
    #[derive(Debug, Copy, Clone, PartialEq, Default, Eq, Hash)]
    #[must_use]
    /// Common boolean attributes of a GDObject.
    pub struct GDObjAttributes: u32 {
        /// @nodoc
        const dont_fade          = 1;
        /// @nodoc
        const dont_enter         = 1 << 1;
        /// @nodoc
        const no_effects         = 1 << 2;
        /// @nodoc
        const is_group_parent    = 1 << 3;
        /// @nodoc
        const is_area_parent     = 1 << 4;
        /// @nodoc
        const dont_boost_x       = 1 << 5;
        /// @nodoc
        const dont_boost_y       = 1 << 6;
        /// @nodoc
        const high_detail        = 1 << 7;
        /// @nodoc
        const no_touch           = 1 << 8;
        /// @nodoc
        const passable           = 1 << 9;
        /// @nodoc
        const hidden             = 1 << 10;
        /// @nodoc
        const non_stick_x        = 1 << 11;
        /// @nodoc
        const non_stick_y        = 1 << 12;
        /// @nodoc
        const extra_sticky       = 1 << 13;
        /// @nodoc
        const extended_collision = 1 << 14;
        /// @nodoc
        const is_ice_block       = 1 << 15;
        /// @nodoc
        const grip_slope         = 1 << 16;
        /// @nodoc
        const no_glow            = 1 << 17;
        /// @nodoc
        const no_particles       = 1 << 18;
        /// @nodoc
        const scale_stick        = 1 << 19;
        /// @nodoc
        const no_audio_scale     = 1 << 20;
        /// @nodoc
        const single_ptouch      = 1 << 21;
        /// @nodoc
        const center_effect      = 1 << 22;
        /// @nodoc
        const reverse            = 1 << 23;
    }
}

const GDOBJ_ATTR_FIELDS: &[(u16, GDObjAttributes)] = &[
    (DONT_FADE, GDObjAttributes::dont_fade),
    (DONT_ENTER, GDObjAttributes::dont_enter),
    (NO_OBJECT_EFFECTS, GDObjAttributes::no_effects),
    (IS_GROUP_PARENT, GDObjAttributes::is_group_parent),
    (IS_AREA_PARENT, GDObjAttributes::is_area_parent),
    (DONT_BOOST_X, GDObjAttributes::dont_boost_x),
    (DONT_BOOST_Y, GDObjAttributes::dont_boost_y),
    (IS_HIGH_DETAIL, GDObjAttributes::high_detail),
    (NO_TOUCH, GDObjAttributes::no_touch),
    (PASSABLE, GDObjAttributes::passable),
    (HIDDEN, GDObjAttributes::hidden),
    (NONSTICK_X, GDObjAttributes::non_stick_x),
    (NONSTICK_Y, GDObjAttributes::non_stick_y),
    (EXTRA_STICKY, GDObjAttributes::extra_sticky),
    (HAS_EXTENDED_COLLISION, GDObjAttributes::extended_collision),
    (IS_ICE_BLOCK, GDObjAttributes::is_ice_block),
    (GRIP_SLOPE, GDObjAttributes::grip_slope),
    (NO_GLOW, GDObjAttributes::no_glow),
    (NO_PARTICLES, GDObjAttributes::no_particles),
    (SCALE_STICK, GDObjAttributes::scale_stick),
    (NO_AUDIO_SCALE, GDObjAttributes::no_audio_scale),
    (SINGLE_PLAYER_TOUCH, GDObjAttributes::single_ptouch),
    (CENTER_EFFECT, GDObjAttributes::center_effect),
    (REVERSES_GAMEPLAY, GDObjAttributes::reverse),
];

const GDOBJ_ATTR_PROPSTR_ALLOCSIZE: usize = GDOBJ_ATTR_FIELDS.len() * 6;

impl GDObjAttributes {
    #[inline]
    /// Makes a default instance of this object
    pub fn new() -> Self {
        Self::default()
    }

    /// Serialises this object to a string
    #[inline]
    #[must_use]
    pub fn get_property_str(&self) -> String {
        let mut properties_str = String::with_capacity(GDOBJ_ATTR_PROPSTR_ALLOCSIZE);

        for (id, flag) in GDOBJ_ATTR_FIELDS {
            if self.contains(*flag) {
                let _ = write!(properties_str, ",{id},1");
            }
        }
        properties_str
    }
}

/// Trigger config, used for defining general properties of a trigger object
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TriggerConfig {
    /// is touch triggerable?
    pub touchable: bool,
    /// is spawn triggerable?
    pub spawnable: bool,
    /// is multitriggerable?
    pub multitriggerable: bool,
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

// for sorting a list of groups
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
