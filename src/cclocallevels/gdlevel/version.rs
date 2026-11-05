//! GD release versions, and the IDs that the savefile format uses for them.
//!
//! A version has up to three numeric IDs, available through [`GDVersion`]'s methods:
//!
//! | ID | Method | Stored in |
//! |---|---|---|
//! | Game version | [`GDVersion::to_game_version_id`] | `k17` on levels |
//! | Binary version | [`GDVersion::to_binary_version_id`] | `k50` on levels, and `LLM_02` in CCLocalLevels.dat |
//! | Manifest version | [`GDVersion::to_manifest_version`] | Unknown |
//!
//! Several releases can share an ID, so converting from an ID picks the first match and may not give you the exact release. IDs that don't match any known release are kept as [`GDVersion::Unknown`].
//!
//! ```
//! use gdlib::cclocallevels::gdlevel::version::GDVersion;
//!
//! assert_eq!(GDVersion::from_binary_version(51), GDVersion::GD22091);
//! assert_eq!(GDVersion::from_binary_version(9999), GDVersion::Unknown(9999));
//! ```
//!
//! Use [`GDVersion::LATEST`] for new levels. [`GDVersion::default`] is the oldest version, not the newest.

/// Defines [`GDVersion`] and its conversion functions from a single table.
///
/// To add a new release, append one line at the bottom of the `gd_versions!` invocation below.
/// [`GDVersion::ALL`] and [`GDVersion::LATEST`] follow the order of the table, so nothing else needs updating.
macro_rules! gd_versions {
    ($(
        $(#[$attr:meta])*
        $variant:ident => game: $game:expr, binary: $bin:expr, manifest: $man:expr;
    )+) => {
        /// Enum for all released versions of Geometry Dash. The purpose of this enum is to provide a way to get game version id, `binaryVersion`, and manifest version from one object.
        ///
        /// Reference: <https://a-zalt.github.io/gdknowledge/versions.html>
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub enum GDVersion {
            $( $(#[$attr])* $variant, )+
            /// Unknown version. Internal value is `binaryVersion`.
            Unknown(i32),
        }

        impl GDVersion {
            /// Every known version of GD, oldest first. Does not include [`GDVersion::Unknown`].
            pub const ALL: &[GDVersion] = &[$(Self::$variant),+];

            /// The newest known version of GD.
            pub const LATEST: GDVersion = Self::ALL[Self::ALL.len() - 1];

            /// Converts this `GDVersion` to a game version ID. This function returns `None` if the binary version does not have a known game version.
            ///
            /// Reference: <https://a-zalt.github.io/gdknowledge/versions.html>
            pub fn to_game_version_id(&self) -> Option<i32> {
                match self {
                    $(Self::$variant => $game,)+
                    Self::Unknown(_) => None,
                }
            }

            /// Returns the binary version for a specific GD version.
            ///
            /// `binaryVersion` is a value that is present in `LLM_02` in CCLocalLevels and necessary for levels in GD to open correctly.
            /// If this value is not present in the `GDLevel.identity.binary_version` field of a `GDLevel`, GD will ignore existing level data and replace it with a new blank level.
            ///
            /// The variants of this enum correspond to different builds of GD. Not all of the values of this enum are known.
            ///
            /// **WARNING:** This function will NOT return a value for the following versions:
            /// * 2.207
            /// * 2.2071
            /// * 2.2072
            /// * 2.2073
            ///
            /// For `Self::Unknown`, the internal value is returned.
            pub fn to_binary_version_id(&self) -> Option<i32> {
                match self {
                    $(Self::$variant => $bin,)+
                    Self::Unknown(u) => Some(*u),
                }
            }

            /// Returns the manifest version of a specific GD version.
            ///
            // Maintainer's note: I have no clue where this is used.
            pub fn to_manifest_version(&self) -> Option<i32> {
                match self {
                    $(Self::$variant => $man,)+
                    Self::Unknown(_) => None,
                }
            }
        }
    };
}

// columns: game version id (`gameVersion`), binary version (`binaryVersion`), manifest version
gd_versions! {
    /// GD Version 1.000
    #[default]
    GD1000 => game: Some(1), binary: Some(0), manifest: Some(2);
    /// GD Version 1.010
    GD1010 => game: Some(1), binary: Some(0), manifest: Some(3);
    /// GD Version 1.020
    GD1020 => game: Some(1), binary: Some(0), manifest: Some(4);
    /// GD Version 1.100
    GD1100 => game: Some(2), binary: Some(0), manifest: Some(5);
    /// GD Version 1.110
    GD1110 => game: Some(2), binary: Some(1), manifest: Some(6);
    /// GD Version 1.200
    GD1200 => game: Some(3), binary: Some(2), manifest: Some(7);
    /// GD Version 1.210
    GD1210 => game: Some(3), binary: Some(2), manifest: Some(8);
    /// GD Version 1.220
    GD1220 => game: Some(3), binary: Some(2), manifest: Some(9);
    /// GD Version 1.300
    GD1300 => game: Some(4), binary: Some(4), manifest: Some(10);
    /// GD Version 1.400
    GD1400 => game: Some(5), binary: Some(5), manifest: Some(11);
    /// GD Version 1.410
    GD1410 => game: Some(5), binary: Some(5), manifest: Some(12);
    /// GD Version 1.500
    GD1500 => game: Some(6), binary: Some(6), manifest: Some(13);
    /// GD Version 1.510
    GD1510 => game: Some(6), binary: Some(6), manifest: Some(14);
    /// GD Version 1.600
    GD1600 => game: Some(7), binary: Some(7), manifest: Some(16);
    /// GD Version 1.700
    GD1700 => game: Some(10), binary: Some(10), manifest: Some(17);
    /// GD Version 1.710
    GD1710 => game: Some(10), binary: Some(11), manifest: Some(18);
    /// GD Version 1.800
    GD1800 => game: Some(11), binary: Some(12), manifest: Some(19);
    /// GD Version 1.810
    GD1810 => game: Some(18), binary: Some(13), manifest: Some(20);
    /// GD Version 1.811
    GD1811 => game: Some(18), binary: Some(14), manifest: Some(21);
    /// GD Version 1.900
    ///
    /// There are actually two (slightly) different GD 1.900s. They share a `GDVersion` and a `gameVersion` - their only (known) difference is in manifest versions.
    /// The first has a manifest version 22, and the second has a manifest version 23.
    GD1900 => game: Some(19), binary: Some(20), manifest: Some(22);
    /// GD Version 1.910
    GD1910 => game: Some(19), binary: Some(20), manifest: Some(24);
    /// GD Version 1.920
    GD1920 => game: Some(19), binary: Some(24), manifest: Some(25);
    /// GD Version 1.930
    GD1930 => game: Some(19), binary: Some(25), manifest: Some(26);
    /// GD Version 2.000
    GD2000 => game: Some(20), binary: Some(27), manifest: Some(27);
    /// GD Version 2.001
    GD2001 => game: Some(20), binary: Some(28), manifest: Some(28);
    /// GD Version 2.010
    GD2010 => game: Some(20), binary: Some(29), manifest: Some(29);
    /// GD Version 2.011
    GD2011 => game: Some(20), binary: Some(29), manifest: Some(30);
    /// GD Version 2.100
    GD2100 => game: Some(21), binary: Some(33), manifest: Some(31);
    /// GD Version 2.110
    GD2110 => game: Some(21), binary: Some(34), manifest: Some(32);
    /// GD Version 2.111
    GD2111 => game: Some(21), binary: Some(34), manifest: Some(33);
    /// GD Version 2.113
    GD2113 => game: Some(21), binary: Some(35), manifest: None;
    /// GD Version 2.200
    GD2200 => game: Some(22), binary: Some(38), manifest: Some(38);
    /// GD Version 2.201
    GD2201 => game: Some(22), binary: Some(38), manifest: None;
    /// GD Version 2.202
    GD2202 => game: Some(22), binary: Some(39), manifest: None;
    /// GD Version 2.203
    GD2203 => game: Some(22), binary: Some(40), manifest: None;
    /// GD Version 2.204
    GD2204 => game: Some(22), binary: Some(40), manifest: None;
    /// GD Version 2.205
    GD2205 => game: Some(22), binary: Some(41), manifest: None;
    /// GD Version 2.206
    GD2206 => game: Some(22), binary: Some(42), manifest: None;
    /* presumably these ones have 42 < binaryVersion < 45 */
    /// GD Version 2.207
    ///
    /// **NOTE:** This version of GD **DOES NOT** have a known binary version! This variant has a value of 0 and will not work when used in either `LLM_02` or `k50` in a level.
    GD2207 => game: Some(22), binary: None, manifest: None;
    /// GD Version 2.2071
    ///
    /// **NOTE:** This version of GD **DOES NOT** have a known binary version! This variant has a value of 0 and will not work when used in either `LLM_02` or `k50` in a level.
    GD22071 => game: Some(22), binary: None, manifest: None;
    /// GD Version 2.2072
    ///
    /// **NOTE:** This version of GD **DOES NOT** have a known binary version! This variant has a value of 0 and will not work when used in either `LLM_02` or `k50` in a level.
    GD22072 => game: Some(22), binary: None, manifest: None;
    /// GD Version 2.2073
    ///
    /// **NOTE:** This version of GD **DOES NOT** have a known binary version! This variant has a value of 0 and will not work when used in either `LLM_02` or `k50` in a level.
    GD22073 => game: Some(22), binary: None, manifest: None;
    /// GD Version 2.2074
    GD22074 => game: Some(22), binary: Some(45), manifest: None;
    /// GD Version 2.208
    GD2208 => game: Some(22), binary: Some(46), manifest: None;
    /// GD Version 2.2081
    GD22081 => game: Some(22), binary: Some(47), manifest: None;
    /// GD Version 2.2082
    GD22082 => game: Some(22), binary: Some(48), manifest: None;
    /// GD Version 2.209
    GD2209 => game: Some(22), binary: Some(49), manifest: None;
    /// GD Version 2.2091
    GD22091 => game: Some(22), binary: Some(51), manifest: None;
}

impl GDVersion {
    /// Converts a raw binary version to a `GDVersion`. Since multiple game versions share the same binary version, only the earliest game version with a specific ID will be returned.
    ///
    /// This function ensured preservation of the argument value when `to_game_version_id` is called on the returned object.
    pub fn from_binary_version(bv: i32) -> Self {
        Self::ALL
            .iter()
            .copied()
            .find(|v| v.to_binary_version_id() == Some(bv))
            .unwrap_or(Self::Unknown(bv))
    }
}

/* GDVersion data

Reference: https://a-zalt.github.io/gdknowledge/versions.html

Manifest version	gameVersion	Version	Save file GDVersion
2	1	1.000	0
3	1	1.010	0
4	1	1.020	0
5	2	1.100	0
6	2	1.110	1
7	3	1.200	2
8	3	1.210	2
9	3	1.220	2
10	4	1.300	4
11	5	1.400	5
12	5	1.410	5
13	6	1.500	6
14	6	1.510	6
16	7	1.600	7
17	10	1.700	10
18	10	1.710	11
19	11	1.800	12
20	18	1.810	13
21	18	1.811	14
22	19	1.900	20
23	19	1.900	20
24	19	1.910	20
25	19	1.920	24
26	19	1.930	25
27	20	2.000	27
28	20	2.001	28
29	20	2.010	29
30	20	2.011	29
31	21	2.100	33
32	21	2.110	34
33	21	2.111	34
-	21	2.113	35
38	22	2.200	38
-	22	2.201	38
-	22	2.202	39
-	22	2.203	40
-	22	2.204	40
?	22	2.205	41
?	22	2.206	42
?	22	2.207	??
?	22	2.2071	??
?	22	2.2072	??
?	22	2.2073	??
?	22	2.2074	45
-	22	2.208	46
-	22	2.2081	47
?	22	2.2082	48
?	22	2.209	49
?	22	2.2091	51

*/
