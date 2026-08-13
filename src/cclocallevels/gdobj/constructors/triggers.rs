//! This file contains constructors for trigger objects.
//! # ⚠️ Warning
//! **This file is incomplete. More triggers will be added in the future.**

use crate::cclocallevels::gdobj::{
    Event, GDObjConfig, GDObject, GDValue, MoveEasing, ObjectProperties,
    ids::{objects::*, properties::*},
    object_descriptor,
    structs::*,
};

/// Returns a move trigger object
///
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `move_config`: Details for the movement of the target group. See [`MoveMode`] struct
/// * `time`: Move time of group.
/// * `target_group`: Group that is moving.
/// * `silent`: Skips collision checking with the player(s) in the path of its motion. Useful for reducing lag. Collision blocks are unaffected.
/// * `dynamic`: Updates location of the target group in real time for target/directional move modes.
/// * `easing`: Optional easing and easing rate (default: 2) tuple.
pub fn move_trigger(
    config: &GDObjConfig,
    move_config: MoveMode,
    time: f64,
    target_group: i16,
    silent: bool,
    dynamic: bool,
    easing: Option<(MoveEasing, f64)>,
) -> GDObject {
    // aim: target group 2
    let mut properties = vec![
        (TARGET_ITEM, GDValue::Group(target_group)),
        (DURATION_GROUP_TRIGGER_CHANCE, GDValue::Float(time)),
        (SMALL_STEP, GDValue::Bool(true)),
        (DYNAMIC_MOVE, GDValue::Bool(dynamic)),
        (SILENT_MOVE, GDValue::Bool(silent)),
    ];

    add_easing(&mut properties, easing);

    match move_config {
        MoveMode::Default(config) => {
            if let Some(lock) = config.x_lock {
                properties.push((
                    match lock {
                        MoveLock::Player => FOLLOW_PLAYERS_X_MOVEMENT,
                        MoveLock::Camera => FOLLOW_CAMERAS_X_MOVEMENT,
                    },
                    GDValue::Int(1),
                ));
                properties.push((X_MOVEMENT_MULTIPLIER, GDValue::Float(config.dx)));
            } else {
                properties.push((MOVE_UNITS_X, GDValue::Int(config.dx as i32)));
            }

            if let Some(lock) = config.y_lock {
                properties.push((
                    match lock {
                        MoveLock::Player => FOLLOW_PLAYERS_Y_MOVEMENT,
                        MoveLock::Camera => FOLLOW_CAMERAS_Y_MOVEMENT,
                    },
                    GDValue::Int(1),
                ));
                properties.push((Y_MOVEMENT_MULTIPLIER, GDValue::Float(config.dy)));
            } else {
                properties.push((MOVE_UNITS_Y, GDValue::Int(config.dy as i32)));
            }
        }
        MoveMode::Targeting(config) => {
            properties.push((TARGET_MOVE_MODE, GDValue::Int(1)));
            if let Some(id) = config.center_group_id {
                properties.push((CENTER_GROUP_ID, GDValue::Group(id)));
            }

            if let Some(axis) = config.axis_only {
                properties.push((TARGET_MOVE_MODE_AXIS_LOCK, GDValue::Int(axis as i32)));
            }

            match config.target_group_id {
                MoveTarget::Player1 => properties.push((CONTROLLING_PLAYER_1, GDValue::Int(1))),
                MoveTarget::Player2 => properties.push((CONTROLLING_PLAYER_2, GDValue::Int(1))),
                MoveTarget::Group(id) => properties.push((TARGET_ITEM_2, GDValue::Group(id))),
            };
        }
        MoveMode::Directional(config) => {
            if let Some(id) = config.center_group_id {
                properties.push((CENTER_GROUP_ID, GDValue::Group(id)));
            }

            match config.target_group_id {
                MoveTarget::Player1 => properties.push((CONTROLLING_PLAYER_1, GDValue::Int(1))),
                MoveTarget::Player2 => properties.push((CONTROLLING_PLAYER_2, GDValue::Int(1))),
                MoveTarget::Group(id) => properties.push((TARGET_ITEM_2, GDValue::Group(id))),
            };

            properties.push((DIRECTIONAL_MOVE_MODE, GDValue::Int(1)));
            properties.push((DIRECTIONAL_MODE_DISTANCE, GDValue::Int(config.distance)));
        }
    }

    GDObject::new(TRIGGER_MOVE, config, properties)
}

/// Returns a start pos object
///
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `gameplay_settings`: Gameplay options for startpos
/// * `target_order`: Target order (of what, I don't know); Default: 0
/// * `target_channel`: Target channel (once again, I don't know); Default: 0
/// * `disabled`: Disabled startpos? Default: false
pub fn start_pos(
    config: &GDObjConfig,
    gameplay_settings: StartposConfig,
    target_order: i32,
    target_channel: i32,
    disabled: bool,
) -> GDObject {
    GDObject::new(
        TRIGGER_START_POS,
        config,
        vec![
            (
                STARTING_SPEED,
                GDValue::Int(gameplay_settings.start_speed as i32),
            ),
            (
                STARTING_GAMEMODE,
                GDValue::Int(gameplay_settings.starting_gamemode as i32),
            ),
            (
                STARTING_IN_MINI_MODE,
                GDValue::Bool(gameplay_settings.starting_as_mini),
            ),
            (
                STARTING_IN_DUAL_MODE,
                GDValue::Bool(gameplay_settings.starting_as_dual),
            ),
            (IS_DISABLED, GDValue::Bool(disabled)),
            (
                STARTING_IN_MIRROR_MODE,
                GDValue::Bool(gameplay_settings.starting_mirrored),
            ),
            (
                ROTATE_GAMEPLAY,
                GDValue::Bool(gameplay_settings.rotate_gameplay),
            ),
            (
                REVERSE_GAMEPLAY,
                GDValue::Bool(gameplay_settings.reverse_gameplay),
            ),
            (TARGET_ORDER, GDValue::Int(target_order)),
            (TARGET_CHANNEL, GDValue::Int(target_channel)),
            (RESET_CAMERA, GDValue::Bool(gameplay_settings.reset_camera)),
            (10010, GDValue::Int(0)),
            (10011, GDValue::String(String::new())),
            (10022, GDValue::Int(0)),
            (10023, GDValue::Int(0)),
            (10024, GDValue::Int(0)),
            (10027, GDValue::Int(1)),
            (10031, GDValue::Int(1)),
            (10032, GDValue::Int(1)),
            (10033, GDValue::Int(1)),
            (10034, GDValue::Int(1)),
            (10036, GDValue::Int(0)),
            (10037, GDValue::Int(1)),
            (10038, GDValue::Int(1)),
            (10039, GDValue::Int(1)),
            (10040, GDValue::Int(1)),
            (10041, GDValue::Int(1)),
            (10042, GDValue::Int(1)),
            (10043, GDValue::Int(0)),
            (10044, GDValue::Int(0)),
            (10045, GDValue::Int(1)),
            (10046, GDValue::Int(0)),
            (10009, GDValue::Int(1)),
        ],
    )
}

/// Returns a colour trigger
///
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `fade_time`: Time to fade into the colour
/// * `copy_colour`: Optional [`CopyColourConfig`]
pub fn colour_trigger(
    config: &GDObjConfig,
    colour_cfg: ColourTriggerConfig,
    fade_time: f64,
    copy_colour: Option<CopyColourConfig>,
) -> GDObject {
    let mut properties = vec![
        (RED, GDValue::Int(colour_cfg.colour.red as i32)),
        (GREEN, GDValue::Int(colour_cfg.colour.green as i32)),
        (BLUE, GDValue::Int(colour_cfg.colour.blue as i32)),
        (DURATION_GROUP_TRIGGER_CHANCE, GDValue::Float(fade_time)),
        (
            USING_PLAYER_COLOUR_1,
            GDValue::Bool(colour_cfg.use_player_col_1),
        ),
        (
            USING_PLAYER_COLOUR_2,
            GDValue::Bool(colour_cfg.use_player_col_2),
        ),
        (COLOUR_CHANNEL, GDValue::Short(colour_cfg.channel.into())),
        (OPACITY, GDValue::Float(colour_cfg.opacity)),
        (BLENDING_ENABLED, GDValue::Bool(colour_cfg.blending)),
    ];

    if let Some(config) = copy_colour {
        let cfg_string = config.hsv_config.to_string();
        if !config.use_legacy_hsv {
            properties.push((NO_LEGACY_HSV, GDValue::Bool(true)));
        }

        properties.push((COPY_OPACITY, GDValue::Bool(config.copy_opacity)));
        properties.push((COPY_COLOUR_SPECS, GDValue::String(cfg_string)));
        properties.push((
            COPY_COLOUR_FROM_CHANNEL,
            GDValue::ColourChannel(config.original_ch),
        ));
    }

    GDObject::new(TRIGGER_COLOUR, config, properties)
}

/// Returns a pulse trigger
///
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `pulse_fade_in_time`: fade-in time of the pulse in seconds  
/// * `pulse_hold_time`: gold time of the pulse in seconds  
/// * `pulse_fade_out_time`: fade-out time of the pulse in seconds  
/// * `exclusive_pulse`: disable all other pulses of the same ID when this trigger is activated
/// * `pulse_target`: Target group/channel of pulse. See [`PulseTarget`]
/// * `pulse_mode`: Colour settings of this pulse. See [`PulseMode`]
pub fn pulse_trigger(
    config: &GDObjConfig,
    pulse_fade_in_time: f64,
    pulse_hold_time: f64,
    pulse_fade_out_time: f64,
    exclusive_pulse: bool,
    pulse_target: &PulseTarget,
    pulse_mode: PulseMode,
) -> GDObject {
    let mut properties = vec![
        (PULSE_FADE_IN_TIME, GDValue::Float(pulse_fade_in_time)),
        (PULSE_HOLD_TIME, GDValue::Float(pulse_hold_time)),
        (PULSE_FADE_OUT_TIME, GDValue::Float(pulse_fade_out_time)),
        (EXCLUSIVE_PULSE_MODE, GDValue::Bool(exclusive_pulse)),
    ];
    match pulse_target {
        PulseTarget::Channel(c) => properties.push((TARGET_ITEM, GDValue::Group(c.channel_id))),
        PulseTarget::Group(g) => {
            properties.extend_from_slice(&[
                (
                    PULSE_DETAIL_COLOUR_ONLY,
                    GDValue::Bool(g.detail_colour_only),
                ),
                (PULSE_MAIN_COLOUR_ONLY, GDValue::Bool(g.main_colour_only)),
                (PULSE_GROUP, GDValue::Group(g.group_id)),
            ]);
        }
    }

    match pulse_mode {
        PulseMode::Colour(c) => {
            properties.extend_from_slice(&[
                (RED, GDValue::Int(c.red as i32)),
                (GREEN, GDValue::Int(c.green as i32)),
                (BLUE, GDValue::Int(c.blue as i32)),
            ]);
        }
        PulseMode::HSV(h) => {
            properties.extend_from_slice(&[
                (NO_LEGACY_HSV, GDValue::Bool(h.use_static_hsv)),
                (COPY_COLOUR_SPECS, GDValue::String(h.hsv_config.to_string())),
                (
                    COPY_COLOUR_FROM_CHANNEL,
                    GDValue::ColourChannel(h.colour_id),
                ),
            ]);
        }
    }
    GDObject::new(TRIGGER_PULSE, config, properties)
}

object_descriptor!(
    /// Stop trigger
    StopTrigger: TRIGGER_STOP => {
        /// Target group to stop/pause/resume
        target_group: i16 => Group TARGET_ITEM,
        /// Stop mode (see [`StopMode`] struct)
        stop_mode: StopMode => to_i32 STOP_MODE,
        /// Only stops certain triggers within a group if enabled.
        use_control_id: bool => Bool USE_CONTROL_ID
    }
);

object_descriptor!(
    /// Alpha trigger
    AlphaTrigger: TRIGGER_ALPHA => {
        /// Target group to stop/pause/resume
        target_group: i16 => Group TARGET_ITEM,
        /// Opacity to set group at
        opacity: f64 => Float OPACITY,
        /// Time to fade to the opacity
        fade_time: f64 => Float DURATION_GROUP_TRIGGER_CHANCE
    }
);

object_descriptor!(
    /// Toggle trigger
    ToggleTrigger: TRIGGER_TOGGLE => {
        /// Target group to stop/pause/resume
        target_group: i16 => Group TARGET_ITEM,
        /// Active group instead of deactivating?
        activate_group: bool => Bool ACTIVATE_GROUP
    }
);

/// Sets a transition mode for objects on an edge of the screen.
pub struct TransitionTrigger {
    /// Type of transition
    pub transition: TransitionType,
    /// Which objects the transition applies to
    pub mode: TransitionMode,
    /// Transition only applies to objects on this channel. Channel 0 applies to all objects.
    pub channel: i32,
}

impl ObjectProperties for TransitionTrigger {
    fn serialise(&self) -> Vec<(u16, GDValue)> {
        vec![
            (
                ENTEREXIT_TRANSITION_CONFIG,
                GDValue::Int(self.mode.to_num()),
            ),
            (TARGET_TRANSITION_CHANNEL, GDValue::Int(self.channel)),
        ]
    }
    fn object_id(&self) -> i32 {
        self.transition as i32
    }
}

// misc stuff

object_descriptor!(
    /// This object ensures that if any one of the objects in the group is visible, then all are loaded
    LinkVisibleTrigger: TRIGGER_LINK_VISIBLE => {
        /// group that is linked visibly
        target_group: i16 => Group TARGET_ITEM
    }
);

object_descriptor!(
    /// Timewarp trigger
    TimewarpTrigger: TRIGGER_TIME_WARP => {
        /// How much to speed up/slow down time by. 1.0 is the default
        time_scale: f64 => Float TIMEWARP_AMOUNT
    }
);

// these have no params so we don't need a struct for them

/// Returns a reverse gameplay trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
#[inline]
pub fn reverse_gameplay(config: &GDObjConfig) -> GDObject {
    GDObject::new(TRIGGER_REVERSE_GAMEPLAY, config, vec![])
}

/// Returns a trigger that shows the player
/// # Arguments
/// * `config`: General object options, such as position and scale
#[inline]
pub fn show_player(config: &GDObjConfig) -> GDObject {
    GDObject::new(TRIGGER_SHOW_PLAYER, config, vec![])
}

/// Returns a trigger that hides the player
/// # Arguments
/// * `config`: General object options, such as position and scale
#[inline]
pub fn hide_player(config: &GDObjConfig) -> GDObject {
    GDObject::new(TRIGGER_HIDE_PLAYER, config, vec![])
}

/// Returns a trigger that shows the player trail
/// # Arguments
/// * `config`: General object options, such as position and scale
#[inline]
pub fn show_player_trail(config: &GDObjConfig) -> GDObject {
    GDObject::new(TRIGGER_ENABLE_PLAYER_TRAIL, config, vec![])
}

/// Returns a trigger that hides the player trail
/// # Arguments
/// * `config`: General object options, such as position and scale\
#[inline]
pub fn hide_player_trail(config: &GDObjConfig) -> GDObject {
    GDObject::new(TRIGGER_DISABLE_PLAYER_TRAIL, config, vec![])
}

/// Returns a trigger that enables the background effect
/// # Arguments
/// * `config`: General object options, such as position and scale
#[inline]
pub fn bg_effect_on(config: &GDObjConfig) -> GDObject {
    GDObject::new(TRIGGER_BG_EFFECT_ON, config, vec![])
}

/// Returns a trigger that disables the background effect
/// # Arguments
/// * `config`: General object options, such as position and scale
#[inline]
pub fn bg_effect_off(config: &GDObjConfig) -> GDObject {
    GDObject::new(TRIGGER_BG_EFFECT_OFF, config, vec![])
}

object_descriptor!(
    /// Group reset trigger
    GroupResetTrigger: TRIGGER_RESET_GROUP => {
        /// group that is to be reset
        target_group: i16 => Group TARGET_ITEM
    }
);

object_descriptor!(
    /// Shake trigger
    ShakeTrigger: TRIGGER_SHAKE => {
        /// Strength of shake
        strength: i32 => Int SHAKE_STRENGTH,
        /// Interval in seconds between each shake
        interval: f64 => Float SHAKE_INTERVAL,
        /// Total duration of shaking
        duration: f64 => Float DURATION_GROUP_TRIGGER_CHANCE
    }
);

object_descriptor!(
    /// Background speed trigger
    BGSpeedTrigger: TRIGGER_BG_SPEED_CONFIG => {
        /// X-axis speed of BG in terms of player speed. Default is 0.3
        mod_x: f64 => Float X_MOVEMENT_MULTIPLIER,
        /// Y-axis speed of BG in terms of player speed. Default is 0.5
        mod_y: f64 => Float Y_MOVEMENT_MULTIPLIER
    }
);

object_descriptor!(
    /// Middleground speed trigger
    MGSpeedTrigger: TRIGGER_MG_SPEED_CONFIG => {
        /// X-axis speed of MG in terms of player speed. Default is 0.3
        mod_x: f64 => Float X_MOVEMENT_MULTIPLIER,
        /// Y-axis speed of MG in terms of player speed. Default is 0.5
        mod_y: f64 => Float Y_MOVEMENT_MULTIPLIER
    }
);

object_descriptor!(
    /// Controls what the player can and can't do. Useful for suppressing player input.
    PlayerControlTrigger: TRIGGER_PLAYER_CONTROL => {
        /// Enables these controls for player 1
        p1: bool => Bool CONTROLLING_PLAYER_1,
        /// Enables these controls for player 2
        p2: bool => Bool CONTROLLING_PLAYER_2,
        /// Cancel's the player's current jump
        stop_jump: bool => Bool STOP_PLAYER_JUMP,
        /// Stops the player from moving
        stop_move: bool => Bool STOP_PLAYER_MOVEMENT,
        /// Stops the player's rotation
        stop_rotation: bool => Bool STOP_PLAYER_ROTATION,
        /// Stops the player from sliding after a force
        stop_slide: bool => Bool STOP_PLAYER_SLIDING
    }
);

/// Sets the gravity intensity.
pub struct GravityTrigger {
    /// Intensity of gravity. 0 = no gravity, <1 = decreased gravity, 1 = normal gravity >1 = boosted gravity
    pub gravity: f64,
    /// Optional player to assign gravity to. The two players can have different gravities.
    pub target_player: Option<TargetPlayer>,
}

impl ObjectProperties for GravityTrigger {
    fn serialise(&self) -> Vec<(u16, GDValue)> {
        let mut properties = vec![(GRAVITY, GDValue::Float(self.gravity))];

        if let Some(player) = self.target_player {
            properties.push((player as u16, GDValue::Bool(true)));
        }
        properties
    }
    fn object_id(&self) -> i32 {
        TRIGGER_GRAVITY
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Initiates level ending
pub struct EndTrigger {
    /// Optional group to spawn once the end trigger is activated
    pub spawn_id: Option<i16>,
    /// Optional group for the player to grabitate to upon finishing
    pub target_pos: Option<i16>,
    /// Disables visual end effects
    pub no_effects: bool,
    /// Instantly finishes the level when triggered
    pub instant: bool,
    /// Disable end sound effects
    pub no_sfx: bool,
}

impl ObjectProperties for EndTrigger {
    fn serialise(&self) -> Vec<(u16, GDValue)> {
        let mut properties = vec![
            (NO_END_EFFECTS, GDValue::Bool(self.no_effects)),
            (INSTANT_END, GDValue::Bool(self.instant)),
            (NO_END_SOUND_EFFECTS, GDValue::Bool(self.no_sfx)),
        ];

        if let Some(id) = self.spawn_id {
            properties.push((TARGET_ITEM, GDValue::Group(id)));
        }

        if let Some(pos) = self.target_pos {
            properties.push((TARGET_ITEM_2, GDValue::Group(pos)));
        }

        properties
    }
    fn object_id(&self) -> i32 {
        TRIGGER_END
    }
}

// items and counters

/// Visual item display. Helpful when trying to see what value an item has.
#[derive(Debug, Clone, PartialEq)]
pub struct CounterLabel {
    /// Item to display
    pub item: Item,
    /// Visual alignment of label. See [`ItemAlign`].
    pub align: ItemAlign,
    /// Only show whole seconds if the item is a timer
    pub seconds_only: bool,
}

impl ObjectProperties for CounterLabel {
    fn serialise(&self) -> Vec<(u16, GDValue)> {
        let mut properties = vec![
            (SECONDS_ONLY, GDValue::Bool(self.seconds_only)),
            (COUNTER_ALIGNMENT, GDValue::Int(self.align as i32)),
        ];

        match self.item {
            Item::Attempts | Item::MainTime | Item::Points => {
                properties.push((
                    SPECIAL_COUNTER_MODE,
                    GDValue::Int(self.item.as_special_mode_i32().unwrap()),
                ));
            }
            Item::Counter(c) => {
                properties.push((INPUT_ITEM_1, GDValue::Item(c)));
            }
            Item::Timer(t) => {
                properties.extend_from_slice(&[
                    (INPUT_ITEM_1, GDValue::Item(t)),
                    (IS_TIMER, GDValue::Bool(true)),
                ]);
            }
        }
        properties
    }

    fn object_id(&self) -> i32 {
        COUNTER
    }
}

#[derive(Debug, Clone, PartialEq)]
// TODO: verify this from the source code. I got this from looking at the tooltip in the ItemEdit trigger's window.
/// Performs an atomic operation to mutates an item's value.
///
/// The value that is set to the target item is evaluated like so:
/// 1. The value of each specified operand is fetched and an intermediate result (referred to as the operand result in the struct's remaining documentation) is computed.
///     - In the case that no operands are specified, this result is not computed.
///     - In the case that one operand is specified, this result is the value of that operand.
///     - In the case that two operands are specified, this result = `first <op> second`, where `op` is the `id_op`.
/// 2. The operand result is then modified according to the value of the modifier and `multiply_mod`. If the operand result was not computed, the operand result is now set to the value of the modifier.
/// Otherwise, the operand result becomes `operand result <op> modifier` where `op` is multiplication if `multiply_mod` is enabled and division if not.
/// 3. The operand result is rounded according to `id_rounding` and then its sign is modified according to `id_sign`.
/// 4. If `assign_op` is set to something other than `Op::Set`, the operand result becomes `current target item value <op> operand result` where `op` is `assign_op`.
/// 5. The operand result is rounded again according to `result_rounding` and then its sign is modified according to `result_sign`.
/// 6. Finally, the value of the target item is set to the operand result.
pub struct ItemEditTrigger {
    /// First operand
    pub operand1: Option<Item>,
    /// Second operand
    pub operand2: Option<Item>,
    /// Item that will be assigned to when the trigger finishes executing
    pub target: Item,
    /// An immediate value that can be used to modify the value directly. For example, using `=` as the assignment operator with both operands set to None will simply set the value of the target to the modifier.
    /// This value can also be used to multiply the result after adding the two operands together as well as a number of other things.
    pub modifier: f64,
    /// Operator for modifying target; see [`Op`] enum. This operator determines how the computed result from the operands is assigned to the target item.
    /// When set to `Op::Set` (assignment, `=` in-game), the result will simply be assigned to the target item.
    /// When set to an arithmetic operator, the target item will be set according to the equation `previous value <operator> computed value`.
    /// For example, when set to `Op::Add`, the target's existing value will be updated by adding the computed value between the operands to itself.
    pub assign_op: Op,
    /// Determines whether the modifier should multiply to the operand result or divide it. If `true`, the operand result is multiplied by the modifier; otherwise the operand result is divided by the modifier.
    ///
    /// Note: This value is also an operator, however it only works properly with `Op::Mul` and `Op::Div`. For this reason, the parameter was collapsed to a boolean.
    /// Nobody knows why RobTop didn't also support adding and subtracting the modifier from the operand result as those are also perfectly valid operations and would allow for more flexibility within this trigger.
    /// Maybe one day RobTop will regain his senses and finally add support for this.
    pub multiply_mod: bool,
    /// Operator that determines the result of the first step. Accepts all values of [`Op`] except for `Op::Set`.
    pub id_op: Option<Op>,
    /// How the result from the second step is rounded. See [`RoundMode`].
    pub id_rounding: RoundMode,
    /// How the result from the fourth step is rounded. See [`RoundMode`].
    pub result_rounding: RoundMode,
    /// How the sign of the result from the second step is set. This happens after rounding. See [`SignMode`].
    pub id_sign: SignMode,
    /// How the sign of the result from the fourth step is set. This happens after rounding. See [`SignMode`].
    pub result_sign: SignMode,
}

impl ObjectProperties for ItemEditTrigger {
    fn serialise(&self) -> Vec<(u16, GDValue)> {
        let mod_op = match self.multiply_mod {
            true => Op::Mul,
            false => Op::Div,
        };
        let id_op = match self.id_op {
            Some(op) => op,
            None => Op::Add,
        };

        let mut properties = vec![
            (TARGET_ITEM, GDValue::Item(self.target.id())),
            (
                TARGET_ITEM_TYPE,
                GDValue::Int(self.target.get_type_as_i32()),
            ),
            (MODIFIER, GDValue::Float(self.modifier)),
            (LEFT_OPERATOR, GDValue::Int(self.assign_op.to_num())),
            (RIGHT_OPERATOR, GDValue::Int(id_op.to_num())),
            (COMPARE_OPERATOR, GDValue::Int(mod_op.to_num())),
            (LEFT_ROUND_MODE, GDValue::Int(self.id_rounding as i32)),
            (RIGHT_ROUND_MODE, GDValue::Int(self.result_rounding as i32)),
            (LEFT_SIGN_MODE, GDValue::Int(self.id_sign as i32)),
            (RIGHT_SIGN_MODE, GDValue::Int(self.result_sign as i32)),
        ];

        if let Some(item) = self.operand1 {
            properties.extend_from_slice(&[
                (INPUT_ITEM_1, GDValue::Item(item.id())),
                (FIRST_ITEM_TYPE, GDValue::Int(item.get_type_as_i32())),
            ]);
        }

        if let Some(item) = self.operand2 {
            properties.extend_from_slice(&[
                (INPUT_ITEM_2, GDValue::Item(item.id())),
                (SECOND_ITEM_TYPE, GDValue::Int(item.get_type_as_i32())),
            ]);
        }
        properties
    }
    fn object_id(&self) -> i32 {
        TRIGGER_ITEM_EDIT
    }
}

/// Spawns groups based on a comparsion between two items.
///
/// The comparison evalutes both sides at the time of being called. Each side is evaluted as follows:
/// 1. The item's value is read
/// 2. Based on the modifying operator, the value is then also modified with the second operator being the modifier value. For example, it may get multiplied by 0.75 if the modifier is 0.75 and the modifying operand is multiplication.
/// 3. The result is rounded based on the [`RoundMode`]
/// 4. The result's sign is modified based on the [`SignMode`]
pub struct ItemCompareTrigger {
    /// Group that is activated when the comparison is true. Set to 0 to not spawn a group.
    pub true_id: i16,
    /// Group that is activated when the comparison is false. Set to 0 to not spawn a group.
    pub false_id: i16,
    /// Left-hand operand of comparison. See [`CompareOperand`].
    pub lhs: CompareOperand,
    /// Right-hand operand of comparison. See [`CompareOperand`].
    pub rhs: CompareOperand,
    /// Operator used to compare the two sides. See [`CompareOp`] enum.
    pub compare_op: CompareOp,
    /// Tolerant range of comparsion. Comparsion will be true if the resulting value is off by at most this value.
    pub tolerance: f64,
}

impl ObjectProperties for ItemCompareTrigger {
    fn serialise(&self) -> Vec<(u16, GDValue)> {
        vec![
            (TARGET_ITEM, GDValue::Item(self.true_id)),
            (TARGET_ITEM_2, GDValue::Item(self.false_id)),
            // ids
            (INPUT_ITEM_1, GDValue::Item(self.lhs.operand_item.id())),
            (INPUT_ITEM_2, GDValue::Item(self.rhs.operand_item.id())),
            // types
            (
                FIRST_ITEM_TYPE,
                GDValue::Int(self.lhs.operand_item.get_type_as_i32()),
            ),
            (
                SECOND_ITEM_TYPE,
                GDValue::Int(self.rhs.operand_item.get_type_as_i32()),
            ),
            // modifiers
            (MODIFIER, GDValue::Float(self.lhs.modifier)),
            (SECOND_MODIFIER, GDValue::Float(self.rhs.modifier)),
            // modifiers ops
            (LEFT_OPERATOR, GDValue::Int(self.lhs.mod_op.to_num())),
            (RIGHT_OPERATOR, GDValue::Int(self.rhs.mod_op.to_num())),
            (COMPARE_OPERATOR, GDValue::Int(self.compare_op.to_num())),
            (TOLERANCE, GDValue::Float(self.tolerance)),
            // round modes
            (LEFT_ROUND_MODE, GDValue::Int(self.lhs.rounding as i32)),
            (RIGHT_ROUND_MODE, GDValue::Int(self.rhs.rounding as i32)),
            // sign modes
            (LEFT_SIGN_MODE, GDValue::Int(self.lhs.sign as i32)),
            (RIGHT_SIGN_MODE, GDValue::Int(self.rhs.sign as i32)),
        ]
    }

    fn object_id(&self) -> i32 {
        TRIGGER_ITEM_COMPARE
    }
}

object_descriptor!(
    /// Enables the value of items to persist across attempts.
    PersistentItemTrigger: TRIGGER_PERSISTENT_ITEM => {
        /// Target item ID
        item_id: i16 => Item TARGET_ITEM,
        /// Targets a timer with the corresponding ID if enabled
        timer: bool => Bool TIMER,
        /// make this item persistent?
        persistent: bool => Bool SET_PERSISTENT_ITEM,
        /// Target all persistent items?
        target_all: bool => Bool TARGET_ALL_PERSISTENT_ITEMS,
        /// Reset item(s) to 0?
        reset: bool => Bool RESET_ITEM_TO_0
    }
);

// spawners

object_descriptor!(
    /// Randomly picks between triggering two groups.
    ///
    /// This trigger uses [`crate::core::rand::check_seed_random`] to determine the group to trigger. The first target group's chance of being spawned is determined by the `chance` parameter.
    /// If the chance is 42%, then the first target group has a 42% chance of being spawned. The second target group has a `1 - chance`, or 58% chance in this example of being toggled.
    ///
    /// If it is desirable not to activate a group, use 0 as the ID. This trigger will not activate any group ID 0.
    RandomTrigger: TRIGGER_RANDOM => {
        /// Float in the range [0.0, 1.0] to spawn the first target group
        chance: f64 => Float DURATION_GROUP_TRIGGER_CHANCE,
        /// Has a `chance` chance to be spawned
        target_group1: i16 => Group TARGET_ITEM,
        /// Has a `1 - chance` chance to be spawned
        target_group2: i16 => Group TARGET_ITEM_2
    }
);

object_descriptor!(
    /// Spawns a group
    SpawnTrigger: TRIGGER_SPAWN => {
        /// Spawns this group
        spawn_id: i16 => Group TARGET_ITEM,
        /// Delay between beign triggered and spawning the group
        delay: f64 => Float SPAWN_DELAY,
        /// Random variation on delay
        delay_variation: f64 => Float SPAWN_DELAY_VARIATION,
        /// Resets the remapping of group IDs
        reset_remap: bool => Bool RESET_REMAP,
        /// Spawns constituents of group in the order of x-position
        spawn_ordered: bool => Bool SPAWN_ORDERED,
        /// prevents the trigger's resulting spawns from being rendered in editor preview
        preview_disable: bool => Bool DISABLE_PREVIEW,
        /// List of ID remaps: (old, new). When a group is triggered with remaps, it will use new IDs
        spawn_remaps: Vec<(i16, i16)> => Remaps SPAWN_ID_REMAPS
        // spawn_remap: Vec<(i16, i16)> - special case, uses `GDValue::from_spawn_remaps()` conversion (not a plain variant), and `Vec` does not implement `Copy`, could not convert
    }
);

object_descriptor!(
    /// Activates a group on player death
    OnDeathTrigger: TRIGGER_ON_DEATH => {
        /// Spawns this group
        target_group: i16 => Group TARGET_ITEM,
        /// Activate this group instead of toggling it off
        activate_group: bool => Bool ACTIVATE_GROUP
    }
);

/// Spawns a group of particles at another group's location. I rate this trigger 7/10
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ParticleSpawnTrigger {
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
    /// Group that contains the particle objects
    pub particle_group: i16,
    /// Group at which the particles will be spawned
    pub position_group: i16,
}

impl ObjectProperties for ParticleSpawnTrigger {
    fn serialise(&self) -> Vec<(u16, GDValue)> {
        let mut properties = vec![
            (TARGET_ITEM, GDValue::Group(self.particle_group)),
            (TARGET_ITEM_2, GDValue::Group(self.position_group)),
            (
                MATCH_ROTATION_OF_SPAWNED_PARTICLES,
                GDValue::Bool(self.match_rotation),
            ),
        ];

        if let Some((x, y)) = self.position_offsets {
            properties.push((X_OFFSET_OF_SPAWNED_PARTICLES, GDValue::Int(x)));
            properties.push((Y_OFFSET_OF_SPAWNED_PARTICLES, GDValue::Int(y)));
        }

        if let Some((x, y)) = self.position_variation {
            properties.push((X_OFFSET_VARIATION_OF_SPAWNED_PARTICLES, GDValue::Int(x)));
            properties.push((Y_OFFSET_VARIATION_OF_SPAWNED_PARTICLES, GDValue::Int(y)));
        }

        if let Some((rot, var)) = self.rotation_config {
            properties.push((ROTATION_OF_SPAWNED_PARTICLES, GDValue::Int(rot)));
            properties.push((ROTATION_VARIATION_OF_SPAWNED_PARTICLES, GDValue::Int(var)));
        }

        if let Some((scale, var)) = self.scale_config {
            properties.push((SCALE_OF_SPAWNED_PARTICLES, GDValue::Float(scale)));
            properties.push((SCALE_VARIATION_OF_SPAWNED_PARTICLES, GDValue::Float(var)));
        }
        properties
    }
    fn object_id(&self) -> i32 {
        TRIGGER_SPAWN_PARTICLE
    }
}

// collision blocks

object_descriptor!(
    /// Collision block object
    CollisionBlock: COLLISION_BLOCK => {
        /// Collision block ID
        id: i16 => Item INPUT_ITEM_1,
        /// Whether this block registers collisions with other collision blocks
        dynamic: bool => Bool DYNAMIC_BLOCK
    }
);

object_descriptor!(
    /// Block that detects player input while the player is inside
    ToggleBlock: TOGGLE_BLOCK => {
        /// Group to activate/deactivate
        target_group: i16 => Group TARGET_ITEM,
        /// Activate/spawn group instead of deactivating
        activate_group: bool => Bool ACTIVATE_GROUP,
        /// Disable buffer clicking to activate this block
        claim_touch: bool => Bool CLAIM_TOUCH,
        /// Allows multiple activations
        multi_activate: bool => Bool MULTI_ACTIVATE,
        /// Spawn only without toggling
        spawn_only: bool => Bool SPAWN_ONLY
    }
);

object_descriptor!(
    /// Block that changes state based on whether the player is inside it or not.
    StateBlock: COLLISION_STATE_BLOCK => {
        /// Group that is activated when the player enters this block's hitbox
        state_on: i16 => Group TARGET_ITEM,
        /// Group that is activated when the player exits this block's hitbox
        state_off: i16 => Group TARGET_ITEM_2
    }
);

object_descriptor!(
    /// Triggers a group when it detects a collision between two collision blocks or optionally players.
    ///
    /// **Note**: At least one of the collider blocks must be dynamic for this collision to register.
    CollisionTrigger: TRIGGER_COLLISION => {
        /// ID of first collision block
        collider1: i16 => Item INPUT_ITEM_1,
        /// ID of second collision block
        collider2: i16 => Item INPUT_ITEM_2,
        /// ID of group that is activated when the two colliders collide
        target_id: i16 => Item TARGET_ITEM,
        /// Whether to check for collision with player 1 instead of collider 1
        collide_player1: bool => Bool CONTROLLING_PLAYER_1,
        /// Whether to check for collision with player 2 instead of collider 1.
        ///   Does not override collision checking with player 1 if `collide_player1` is also true.
        collide_player2: bool => Bool CONTROLLING_PLAYER_2,
        /// Whether to check for collision between the two players instead of two collision blocks
        collide_both_players: bool => Bool CONTROLLING_TARGET_PLAYER,
        /// whether this trigger will activate or deactivate the target group
        activate_group: bool => Bool ACTIVATE_GROUP,
        /// activates group when the two colliders' hitboxes stop overlapping after collision
        ///   instead of when they start colliding.
        trigger_on_exit: bool => Bool TRIGGER_ON_EXIT
    }
);

object_descriptor!(
    /// Instant collision trigger
    ///
    /// Activates a group when the two colliders collide or do not collide.
    /// This condition is only checked once and never again.
    InstantCollTrigger: TRIGGER_INSTANT_COLLISION => {
        /// ID of first collision block
        collider1: i16 => Item INPUT_ITEM_1,
        /// ID of second collision block
        collider2: i16 => Item INPUT_ITEM_2,
        /// ID of group that is activated if the two colliders collide
        true_id: i16 => Item TARGET_ITEM,
        /// ID of group that is activated if the two colliders do not collide
        false_id: i16 => Item TARGET_ITEM_2,
        /// Whether to check for collision with player 1 instead of collider 1
        collide_player1: bool => Bool CONTROLLING_PLAYER_1,
        /// Whether to check for collision with player 2 instead of collider 1.
        ///   Does not override collision checking with player 1 if `collide_player1` is also true.
        collide_player2: bool => Bool CONTROLLING_PLAYER_2,
        /// Whether to check for collision between the two players instead of two collision blocks
        collide_both_players: bool => Bool CONTROLLING_TARGET_PLAYER
    }
);

impl InstantCollTrigger {
    /// Creates a new instance of this object from two collision block IDs
    pub fn two_colliders(
        collider1_id: i16,
        collider2_id: i16,
        true_id: i16,
        false_id: i16,
    ) -> Self {
        Self {
            collider1: collider1_id,
            collider2: collider2_id,
            collide_player1: false,
            collide_player2: false,
            collide_both_players: false,
            true_id,
            false_id,
        }
    }
}

// time triggers

object_descriptor!(
    /// Time trigger
    TimeTrigger: TRIGGER_TIME => {
        /// Starting time of target timer that will be set on activation of the trigger
        start_time: f64 => Float START_TIME,
        /// Time at which to call the target group
        stop_time: f64 => Float TARGET_TIME,
        /// Whether or not to pause the timer once it reaches the stop time
        pause_when_reached: bool => Bool PAUSE_AT_TARGET_TIME,
        /// Time multiplier for this timer
        time_mod: f64 => Float TIME_VALUE_MULTIPLER,
        /// Target timer ID
        timer_id: i16 => Item INPUT_ITEM_1,
        /// Group that is activated when the timer reaches the target value
        target_group: i16 => Group TARGET_ITEM,
        /// Toggles ignoring global timewarp
        ignore_timewarp: bool => Bool IGNORE_TIMEWARP,
        /// Starts this timer paused, which allows a time control trigger to un-pause it
        start_paused: bool => Bool START_PAUSED_TIMER,
        /// Only starts the timer if any of these are met:
        ///     1. Target timer is at 0.00
        ///     2. The `start_paused` option is on
        ///     3. The timer is not currently counting
        dont_override: bool => Bool DONT_OVERRIDE
    }
);

object_descriptor!(
    /// Time control trigger
    TimeControlTrigger: TRIGGER_TIME_CONTROL => {
        /// Timer ID
        id: i16 => Item INPUT_ITEM_1,
        /// If enabled, stops the timer; otherwise, starts the timer.
        stop: bool => Bool STOP_TIME_COUNTER
    }
);

object_descriptor!(
    /// Triggers a group when a given timer reaches a specific time.
    TimeEventTrigger: TRIGGER_TIME_EVENT => {
        /// Timer ID
        id: i16 => Group INPUT_ITEM_1,
        /// If enabled, stops the timer; otherwise, starts the timer.
        target_group: i16 => Group TARGET_ITEM,
        /// At what time the timer should be to activate objects in `target_group`.
        target_time: f64 => Float TARGET_TIME,
        /// Whether this event should be triggerable multiple times
        multi_activate: bool => Bool MULTIACTIVATABLE_TIME_EVENT
    }
);

// camera triggers

/// Zooms the camera
#[derive(Debug, Clone, PartialEq)]
pub struct CameraZoomTrigger {
    /// How far to zoom the camera. Values above 1 zoom in, values below 1 zoom out. This value must be positive.
    pub zoom: f64,
    /// Time to zoom
    pub time: f64,
    /// Zoom easing
    pub easing: Option<(MoveEasing, f64)>,
}

impl ObjectProperties for CameraZoomTrigger {
    fn serialise(&self) -> Vec<(u16, GDValue)> {
        let mut properties = vec![
            (DURATION_GROUP_TRIGGER_CHANCE, GDValue::Float(self.time)),
            (CAMERA_ZOOM, GDValue::Float(self.zoom)),
        ];

        add_easing(&mut properties, self.easing);
        properties
    }
    fn object_id(&self) -> i32 {
        TRIGGER_CAMERA_ZOOM
    }
}

object_descriptor!(
    /// Visual guide for the camera in the edito
    CameraGuide: CAMERA_GUIDE => {
        /// Zoom of camera guide
        zoom: f64 => Float CAMERA_ZOOM,
        /// Center offset from this object in x axis
        offset_x: i32 => Int MOVE_UNITS_X,
        /// Center offset from this object in y axis
        offset_y: i32 => Int MOVE_UNITS_Y,
        /// Opacity of guidelines
        opacity: f64 => Float CAMERA_GUIDE_PREVIEW_OPACITY
    }
);

object_descriptor!(
    /// Makes a group of objets follow another group
    FollowTrigger: TRIGGER_FOLLOW => {
        /// Multiplier for x-axis movement of follow group
        x_mod: f64 => Float XAXIS_FOLLOW_MOD,
        /// Multiplier for y-axis movement of follow group
        y_mod: f64 => Float YAXIS_FOLLOW_MOD,
        /// Time that the follow group is followed for. -1.0 = infinite.
        follow_time: f64 => Float DURATION_GROUP_TRIGGER_CHANCE,
        /// Group that is following
        target_group: i16 => Group TARGET_ITEM,
        /// Group that is being followed
        follow_group: i16 => Group TARGET_ITEM_2
    }
);

object_descriptor!(
    /// Sets animation modes for objects with animations such as bats
    AnimateTrigger: TRIGGER_ANIMATE => {
        /// Objects to animate
        target_group: i16 => Group TARGET_ITEM,
        animation: Anim => to_i32 ANIMATION_ID
    }
);

object_descriptor!(
    /// Actiavtes/deactivates a group when an item reaches a specific count. The condition for doing so is checked every tick.
    CountTrigger: TRIGGER_COUNT => {
        /// Checks this item
        item_id: i16 => Item INPUT_ITEM_1,
        /// Target group to activate
        target_id: i16 => Group TARGET_ITEM,
        /// Target count of item at `item_id`
        target_count: i32 => Int TARGET_COUNT,
        /// Whether or not to activate the target group
        activate_group: bool => Bool ACTIVATE_GROUP,
        /// Whether or not this trigger is multi-activatable
        multi_activate: bool => Bool MULTI_ACTIVATE
    }
);

object_descriptor!(
    AdvancedRandomTrigger: TRIGGER_ADVANCED_RANDOM => {
        /// List of tuples: (target group, chance to trigger this group).
        ///
        /// Chances are considered relative to each other, meaning that they are not
        /// precentage-based. Two groups with the same relative chance will have the same
        /// (50-50) chance to be triggered
        probabilities: Vec<(i16, i32)> => ProbabilitiesList RANDOM_PROBABILITIES_LIST
    }
);

object_descriptor!(
    /// UI config trigger
    UIConfigTrigger: TRIGGER_UI_CONFIG => {
        /// the UI objects
        target_group: i16 => Group TARGET_ITEM,
        /// Group with a single object that is a reference for the center of the camera.
        ui_reference_obj: i16 => Group TARGET_ITEM_2,
        /// Reference position for the element on the X-axis
        x_reference: UIReferencePos => as_i32 X_REFERENCE_POSITION,
        /// Reference position for the element on the Y-axis
        // y_reference: UIReferencePos => Y_REFERENCE_POSITION - special case, value is `y_reference as i32 + 4`, not a plain cast, could not convert
        /// Whether or not the x-axis position scales with aspect ratio
        x_ref_relative: bool => Bool X_REFERENCE_IS_RELATIVE,
        /// Whether or not the y-axis position scales with aspect ratio
        y_ref_relative: bool => Bool Y_REFERENCE_IS_RELATIVE
    }
);

/// Returns a rotate trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `move_time`: Time to rotate the target
/// * `rotation_cfg`: Rotation specifics. See [`RotationConfig`]
/// * `easing`: optional move easing and rate. See [`MoveEasing`]
/// * `target_group`: Group that will rotate
/// * `center_group_id`: Group that is being rotated around
/// * `bounding_box`: Optional vertices of a bounding box that limit the position of the rotation group.
///
/// The tuple corresponds to the `MinX`, `MinY`, `MaxX`, `MaxY` group ids respectively in the rotate trigger.
pub fn rotate_trigger(
    config: &GDObjConfig,
    move_time: f64,
    rotation_cfg: RotationConfig,
    easing: Option<(MoveEasing, f64)>,
    target_group: i16,
    center_group_id: i16,
    bounding_box: Option<(i16, i16, i16, i16)>,
) -> GDObject {
    let mut properties = vec![
        (DURATION_GROUP_TRIGGER_CHANCE, GDValue::Float(move_time)),
        (DYNAMIC_MOVE, GDValue::Bool(rotation_cfg.dynamic_mode)),
        (
            LOCK_OBJECT_ROTATION,
            GDValue::Bool(rotation_cfg.lock_object_rotation),
        ),
        (TARGET_ITEM, GDValue::Group(target_group)),
        (TARGET_ITEM_2, GDValue::Group(center_group_id)),
    ];

    match rotation_cfg.mode {
        RotationMode::Aim(cfg) => {
            properties.extend_from_slice(&[
                (TARGET_MOVE_MODE, GDValue::Bool(true)),
                (ROTATION_TARGET_ID, GDValue::Group(cfg.aim_target)),
                (ROTATION_OFFSET, GDValue::Float(cfg.rot_offset)),
            ]);

            if let Some(player) = cfg.player_target {
                properties.push(match player {
                    RotationPlayerTarget::Player1 => (CONTROLLING_PLAYER_1, GDValue::Bool(true)),
                    RotationPlayerTarget::Player2 => (CONTROLLING_PLAYER_2, GDValue::Bool(true)),
                });
            }
        }
        RotationMode::Follow(cfg) => {
            properties.extend_from_slice(&[
                (DIRECTIONAL_MOVE_MODE, GDValue::Bool(true)),
                (ROTATION_TARGET_ID, GDValue::Group(cfg.aim_target)),
                (ROTATION_OFFSET, GDValue::Float(cfg.rot_offset)),
            ]);

            if let Some(player) = cfg.player_target {
                properties.push(match player {
                    RotationPlayerTarget::Player1 => (CONTROLLING_PLAYER_1, GDValue::Bool(true)),
                    RotationPlayerTarget::Player2 => (CONTROLLING_PLAYER_2, GDValue::Bool(true)),
                });
            }
        }
        RotationMode::Default(cfg) => {
            properties.extend_from_slice(&[
                (ROTATE_DEGREES, GDValue::Float(cfg.degrees)),
                (ROTATE_X360, GDValue::Int(cfg.x360)),
            ]);
        }
    }

    add_easing(&mut properties, easing);
    if let Some((min_x, min_y, max_x, max_y)) = bounding_box {
        properties.extend_from_slice(&[
            (MINX_ID, GDValue::Group(min_x)),
            (MINY_ID, GDValue::Group(min_y)),
            (MAXX_ID, GDValue::Group(max_x)),
            (MAXY_ID, GDValue::Group(max_y)),
        ]);
    }

    GDObject::new(TRIGGER_ROTATION, config, properties)
}

/// Returns a scale trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `scale_config`: Scaling config. See [`ScaleConfig`]
/// * `easing`: Optional move easing and rate. See [`MoveEasing`]
/// * `center_group_id`: Center of group that is being scaled. Leave as 0 to use the default center
/// * `target_group`: Group that is being scaled.
/// * `duration`: How long the scaling will be
pub fn scale_trigger(
    config: &GDObjConfig,
    scale_config: ScaleConfig,
    easing: Option<(MoveEasing, f64)>,
    center_group_id: i16,
    target_group: i16,
    duration: f64,
) -> GDObject {
    let mut properties = vec![
        (NEW_X_SCALE, GDValue::Float(scale_config.x_scale)),
        (NEW_Y_SCALE, GDValue::Float(scale_config.y_scale)),
        (DIV_BY_VALUE_X, GDValue::Bool(scale_config.div_by_value_x)),
        (DIV_BY_VALUE_Y, GDValue::Bool(scale_config.div_by_value_y)),
        (TARGET_ITEM, GDValue::Group(target_group)),
        (TARGET_ITEM_2, GDValue::Group(center_group_id)),
        (DURATION_GROUP_TRIGGER_CHANCE, GDValue::Float(duration)),
        (ONLY_MOVE, GDValue::Bool(scale_config.only_move)),
        (RELATIVE_SCALE, GDValue::Bool(scale_config.relative_scale)),
        (
            RELATIVE_ROTATION,
            GDValue::Bool(scale_config.relative_rotation),
        ),
    ];

    add_easing(&mut properties, easing);
    GDObject::new(TRIGGER_SCALE, config, properties)
}

object_descriptor!(
    /// Makes an object follow the player on the y-axis
    FollowPlayerYTrigger: TRIGGER_FOLLOW_PLAYER_Y => {
        /// Follow speed in the range \[0.0, 1.0]; 1.0 = instantaneously snaps to player y-pos
        speed: f64 => Float FOLLOW_SPEED,
        /// Delay of the following group
        delay: f64 => Float FOLLOW_DELAY,
        /// Y offset of the following group
        offset: i32 => Int FOLLOW_OFFSET,
        /// Speed limit of the following group
        max_speed: f64 => Float MAX_FOLLOW_SPEED,
        /// How long the group will follow the player
        move_time: f64 => Float DURATION_GROUP_TRIGGER_CHANCE,
        /// The group that is following the player's y-pos
        target_group: i16 => Group TARGET_ITEM
    }
);

/// Moves the middleground.
pub struct MiddleGroundConfigTrigger {
    /// Where to move it relative to its normal position
    pub offset_y: i32,
    /// Easing for moving the middleground
    pub easing: Option<(MoveEasing, f64)>,
}

impl ObjectProperties for MiddleGroundConfigTrigger {
    fn serialise(&self) -> Vec<(u16, GDValue)> {
        let mut properties = vec![(MOVE_UNITS_Y, GDValue::Int(self.offset_y))];
        add_easing(&mut properties, self.easing);
        properties
    }
    fn object_id(&self) -> i32 {
        TRIGGER_MIDDLEGROUND_CONFIG
    }
}

object_descriptor!(
    /// Event config trigger
    EventTrigger: TRIGGER_EVENT => {
        /// Group to target
        target_group: i16 => Group TARGET_ITEM,
        events: Vec<Event> => Events EVENT_LISTENERS,
        /// Activates group only if the player interacts with certain objects. For example, landing on an object with a specific material ID will activate the group if this ID is set to its material ID.
        /// If this ID is set, blocks with other material IDs will not cause the group to activate
        extra_id: i16 => Group EVENT_EXTRA_ID,
        /// Applies to a specific player. See [`ExtraID2`]
        extra_id2: ExtraID2 => as_i32 EVENT_EXTRA_ID_2
        // (IS_INTERACTABLE, GDValue::Bool(true)) - hardcoded constant, not parameter-driven, could not convert
    }
);

object_descriptor!(
    /// Changes the middle ground
    MiddleGroundTrigger: TRIGGER_MIDDLEGROUND_CHANGE => {
        /// Change to this middleground
        middleground: MiddleGround => to_i32 MIDDLEGROUND
    }
);

object_descriptor!(
    /// Toggles a group of objects when the player clicks.
    TouchTrigger: TRIGGER_TOUCH => {
        /// Group that is activated when the trigger registers a click
        target_group: i16 => Group TARGET_ITEM,
        /// Toggles target group on holding and releasing instead of clicking
        hold_mode: bool => Bool TOUCH_HOLD_MODE,
        /// Blocks 2nd player's clicks. Deprecated in favour of [`OptionalPlayerTarget::Player1`]
        dual_mode: bool => Bool TOUCH_DUAL_MODE,
        /// Toggles a specific activation mode. See [`TouchToggle`]
        toggle: TouchToggle => to_i32 TOUCH_TOGGLE_ONOFF,
        /// Only registers clicks from one player. See [`OptionalPlayerTarget`]
        target_player: OptionalPlayerTarget => to_i32 TOUCH_PLAYER_ONLY
    }
);

object_descriptor!(
    /// Stops an area effect
    AreaStopTrigger: TRIGGER_AREA_STOP => {
        effect_id: i16 => Short TARGET_ITEM
    }
);

// util fn to add easing to properties if it is specified
fn add_easing(properties: &mut Vec<(u16, GDValue)>, easing: Option<(MoveEasing, f64)>) {
    if let Some((easing, rate)) = easing {
        properties.extend_from_slice(&[
            (MOVE_EASING, GDValue::Easing(easing)),
            (EASING_RATE, GDValue::Float(rate)),
        ]);
    }
}

/* TODO: trigger constructors
 * Animation triggers
 * advanced follow
 * edit advanced follow
 * re-target advanced follow
 * keyframe setup trigger
 * keyframe setup object
 *
 * Area triggers
 * area move
 * area rotate
 * area scale
 * area fade
 * area tint
 * edit area move
 * edit area rotate
 * edit area scale
 * edit area fade
 * edit area tint
 * enter area move
 * enter area rotate
 * enter area scale
 * enter area fade
 * enter area tint
 * enter area stop
 * area stop
 *
 * Background triggers
 * switch bg
 * sdwitch ground
 *
 * Item triggers
 * instant count trigger
 * pickup trigger
 *
 * Spawner triggers
 * sequence
 *
 * Camera
 * static camera
 * offset camera
 * gameplay offset camera
 * rotate camera
 * edge camera
 * camera mode
 *
 * Gameplay triggers
 * rotate gameplay
 *
 * Sound triggers
 * song trigger
 * edit song trigger
 * sfx trigger
 * edit sfx trigger
 *
 * Misc.
 * bpm marker
 * gradient
 *
 * Player triggers
 * options
 * teleport trigger
 *
 * Shaders
 * shader setup
 * shock wave shader
 * shock line shader
 * glitch shader
 * chromatic shader
 * chromatic glitch shader
 * pixelate shader
 * lens circle shader
 * radial bulb shader
 * motion blur shader
 * bulge shader
 * pinch shader
 * gray scale shader
 * sepia shader
 * invert colour shader
 * hue shader
 * edit colour shader
 * split screen shader
 */
