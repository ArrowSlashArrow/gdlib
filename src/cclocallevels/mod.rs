//! Everything stored in CCLocalLevels.dat, the savefile that holds a player's created levels and lists.
//!
//! # How the data is organised
//!
//! ```text
//! CCLocalLevels                     the whole savefile
//!   - levels: Vec<GDLevel>          level metadata: name, creator, stats, editor state...
//!       - content.data              the level string (`k4`), parsed on demand
//!           - headers               start settings: colours, speed, gamemode...
//!           - objects: Vec<GDObject>
//!   - lists: Vec<GDList>
//! ```
//!
//! # Modules
//!
//! - [`gdlevel`]: levels and the savefile itself. Most programs start here.
//! - [`gdobj`]: objects and triggers.
//! - [`gdlist`]: level lists.
//! - [`properties`]: lookup tables for object names and property types.
//! - [`consts`]: numeric constants from the game.
//!
//! # File formats
//!
//! | Extension | Contains | Encoding | Read with | Write with | Notes |
//! |---|---|---|---|---|---|
//! | `.dat` | All levels and lists | gzip, then base64, then XOR | [`CCLocalLevels::from_local`] | [`CCLocalLevels::export_to_file`] | Only used by CCLocalLevels.dat and CCGameManager.dat |
//! | `.gmd` | One level | Plain plist | [`GDLevel::from_gmd`] | [`GDLevel::export_to_gmd`] | Unofficial file format for individual levels before GD 2.209 |
//! | `.gdlvl` | One level | gzip, then base64 | [`GDLevel::from_gdlvl`] | [`GDLevel::export_to_gdlvl`] | Official format for individual levels since GD 2.209 |
//! | `.gmdl` | One list | Plain plist | [`GDList::from_gmdl`] | [`GDList::export_to_gmdl`] | Unofficial file format for individual lists (no official format has yet been introduced) |
//!
//! GD's plists use shortened tags, such as `<k>` for `<key>` and `<s>` for `<string>`. See [`crate::core::proper_plist_tags`].
//!
//! # Serializer and deserializer functions
//!
//! * [`CCLocalLevels::from_local`]: gdlevel::CCLocalLevels::from_local (from CCLocalLevels.dat)
//! * [`CCLocalLevels::export_to_file`]: gdlevel::CCLocalLevels::export_to_file - this one has a variant called [`CCLocalLevels::export_to_file_with_backup`] which creates a backup for the savefile in case of corruption.
//! * [`GDLevel::from_gmd`]: gdlevel::GDLevel::from_gmd
//! * [`GDLevel::export_to_gmd`]: gdlevel::GDLevel::export_to_gmd
//! * [`GDLevel::from_gdlvl`]: gdlevel::GDLevel::from_gdlvl
//! * [`GDLevel::export_to_gdlvl`]: gdlevel::GDLevel::export_to_gdlvl
//! * [`GDList::from_gmdl`]: gdlist::GDList::from_gmdl
//! * [`GDList::export_to_gmdl`]: gdlist::GDList::export_to_gmdl

pub mod gdlevel;
pub mod gdlist;
pub mod gdobj;
pub mod properties;
pub mod consts {
    //! Numeric constants from GD, either derived from its source or taken directly from it.
    //!
    //! - `SPEED_*`: the player's movement speed in units per second, for each speed setting. Useful for converting between time and position in a level.
    //! - [`SPAWN_ORDER`]: how many units per second a spawn trigger moves.
    //!
    //! Reference: <https://github.com/UHDanke/gmdkit/blob/main/src/gmdkit/constants/game/speed/__init__.py>

    /// Units per second of movement at 0.5x speed
    pub const SPEED_05X: f32 = 0.7 * 5.98000200 * 60.0;
    /// Units per second of movement at 1x speed
    pub const SPEED_1X: f32 = 0.9 * 5.77000189 * 60.0;
    /// Units per second of movement at 2x speed
    pub const SPEED_2X: f32 = 1.1 * 5.87000200 * 60.0;
    /// Units per second of movement at 3x speed
    pub const SPEED_3X: f32 = 1.3 * 6.000002 * 60.0;
    /// Units per second of movement at 4x speed
    pub const SPEED_4X: f32 = 1.6 * 6.000002 * 60.0;
    /// How many units per second a spawn trigger moves
    pub const SPAWN_ORDER: f32 = 311.5801086425781;

    pub(crate) const LEVEL_HEADER_PROP_ID_OFFSET: u16 = 1000;
}
