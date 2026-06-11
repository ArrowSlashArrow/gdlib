//! Shared structs and enums across the whole crate. The structs included here are those that are found across many different places in the savefiles.
//! Area-specific structs, for instance those that are only concerned with GD objects are not included in this module.

use crate::repr_t;

repr_t!(
    /// Enum for the various values of the `kCEK` key in places around the savefile.
    ///
    /// The `kCEK` key is vital to structs like `GDLevel` and `GDList`. It seems that GD uses this value to tell what kind of struct a certain dictionary is.
    #[allow(missing_docs)]
    KCEKValue: i32 {
        GJGameLevel = 4,
        SongInfoObject = 6,
        GJChallengeItem = 7,
        GJRewardItem = 8,
        GJRewardObject = 9,
        GJSmartTemplate = 10,
        GJSmartRefab = 11,
        GJLevelList = 12,
    }
);
