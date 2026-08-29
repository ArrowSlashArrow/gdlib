//! This module contains all relevant objects to CCLocalLevels.dat, the savefile responsible for storing player level data.
pub mod gdlevel;
pub mod gdlist;
pub mod gdobj;
pub mod properties;
pub mod consts {
    //! Hard-coded constants in GD. These numbers are either derived from the source code of GD or directly ripped from it.
    /// Units per second of movement at 0.5x speed
    pub const SPEED_05X: f64 = 0.7 * 5.98000200 * 60.0;
    /// Units per second of movement at 1x speed
    pub const SPEED_1X: f64 = 0.9 * 5.77000189 * 60.0;
    /// Units per second of movement at 2x speed
    pub const SPEED_2X: f64 = 1.1 * 5.87000200 * 60.0;
    /// Units per second of movement at 3x speed
    pub const SPEED_3X: f64 = 1.3 * 6.000002 * 60.0;
    /// Units per second of movement at 4x speed
    pub const SPEED_4X: f64 = 1.6 * 6.000002 * 60.0;
    /// How many units per second a spawn trigger moves
    pub const SPAWN_ORDER: f64 = 311.5801086425781;
}
