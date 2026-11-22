//! Types found in more than one part of GD's savefiles.
//!
//! Currently this is just [`KCEKValue`], the value of the `kCEK` key, which appears to tell GD what kind of object a dictionary holds. Levels must keep [`KCEKValue::GJGameLevel`] to be recognised as levels.
//!
//! Types used in only one area belong in that area's module instead, such as [`gdobj::structs`](crate::cclocallevels::gdobj::structs).

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
