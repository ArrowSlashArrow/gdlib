//! Shared structs and enums across the whole crate. The structs included here are those that are found across many different places in the savefiles.
//! Area-specific structs, for instance those that are only concerned with GD objects are not included in this module.

/// Enum for the various values of the `kCEK` key in places around the savefile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[repr(i32)]
pub enum KCEKValue {
    GJGameLevel = 4,
    SongIngoObject = 6,
    GJChallengeItem = 7,
    GJRewardItem = 8,
    GJRewardObject = 9,
    GJSmartTemplate = 10,
    GJSmartRefab = 11,
    GJLevelList = 12,
}

impl TryFrom<i32> for KCEKValue {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            4 => Ok(Self::GJGameLevel),
            6 => Ok(Self::SongIngoObject),
            7 => Ok(Self::GJChallengeItem),
            8 => Ok(Self::GJRewardItem),
            9 => Ok(Self::GJRewardObject),
            10 => Ok(Self::GJSmartTemplate),
            11 => Ok(Self::GJSmartRefab),
            12 => Ok(Self::GJLevelList),
            _ => Err(()),
        }
    }
}

/// Enum for all difficulties found in GD.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[repr(i32)]
pub enum Difficulty {
    #[default]
    NA = -1,
    Auto = 0,
    Easy = 1,
    Medium = 2,
    Hard = 3,
    Harder = 4,
    Insane = 5,
    EasyDemon = 6,
    MediumDemon = 7,
    HardDemon = 8,
    InsaneDemon = 9,
    ExtremeDemon = 10,
}
