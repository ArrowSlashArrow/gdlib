//! GDVersion.

/// Enum for all released versions of Geometry Dash. The purpose of this enum is to provide a way to get game version id, `binaryVersion`, and manifest version from one object.
///
/// Reference: <https://a-zalt.github.io/gdknowledge/versions.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GDVersion {
    /// GD Version 1.000
    #[default]
    GD1000,
    /// GD Version 1.010
    GD1010,
    /// GD Version 1.020
    GD1020,
    /// GD Version 1.100
    GD1100,
    /// GD Version 1.110
    GD1110,
    /// GD Version 1.200
    GD1200,
    /// GD Version 1.210
    GD1210,
    /// GD Version 1.220
    GD1220,
    /// GD Version 1.300
    GD1300,
    /// GD Version 1.400
    GD1400,
    /// GD Version 1.410
    GD1410,
    /// GD Version 1.500
    GD1500,
    /// GD Version 1.510
    GD1510,
    /// GD Version 1.600
    GD1600,
    /// GD Version 1.700
    GD1700,
    /// GD Version 1.710
    GD1710,
    /// GD Version 1.800
    GD1800,
    /// GD Version 1.810
    GD1810,
    /// GD Version 1.811
    GD1811,
    /// GD Version 1.900
    ///
    /// There are actually two (slightly) different GD 1.900s. They share a `GDVersion` and a `gameVersion` - their only (known) difference is in manifest versions.
    /// The first has a manifest version 22, and the second ahs a manifest version 23.
    GD1900,
    /// GD Version 1.910
    GD1910,
    /// GD Version 1.920
    GD1920,
    /// GD Version 1.930
    GD1930,
    /// GD Version 2.000
    GD2000,
    /// GD Version 2.001
    GD2001,
    /// GD Version 2.010
    GD2010,
    /// GD Version 2.011
    GD2011,
    /// GD Version 2.100
    GD2100,
    /// GD Version 2.110
    GD2110,
    /// GD Version 2.111
    GD2111,
    /// GD Version 2.113
    GD2113,
    /// GD Version 2.200
    GD2200,
    /// GD Version 2.201
    GD2201,
    /// GD Version 2.202
    GD2202,
    /// GD Version 2.203
    GD2203,
    /// GD Version 2.204
    GD2204,
    /// GD Version 2.205
    GD2205,
    /// GD Version 2.206
    GD2206,
    /// GD Version 2.207
    ///
    /// **NOTE:** This version of GD **DOES NOT** have a known binary version! This variant has a value of 0 and will not work when used in either `LLM_02` or `k50` in a level.
    GD2207,
    /// GD Version 2.2071
    ///
    /// **NOTE:** This version of GD **DOES NOT** have a known binary version! This variant has a value of 0 and will not work when used in either `LLM_02` or `k50` in a level.
    GD22071,
    /// GD Version 2.2072
    ///
    /// **NOTE:** This version of GD **DOES NOT** have a known binary version! This variant has a value of 0 and will not work when used in either `LLM_02` or `k50` in a level.
    GD22072,
    /// GD Version 2.2073
    ///
    /// **NOTE:** This version of GD **DOES NOT** have a known binary version! This variant has a value of 0 and will not work when used in either `LLM_02` or `k50` in a level.
    GD22073,
    /// GD Version 2.2074
    GD22074,
    /// GD Version 2.208
    GD2208,
    /// GD Version 2.2081
    GD22081,
    /// GD Version 2.2082
    GD22082,
    /// GD Version 2.209
    GD2209,
    /// Unknown version. Internal value is `binaryVersion`.
    Unknown(i32),
}

impl GDVersion {
    /// Converts this `GDVersion` to a game version ID. This function returns `None` if the binary version does not have a known game version.
    ///
    /// Reference: <https://a-zalt.github.io/gdknowledge/versions.html>
    pub fn to_game_version_id(&self) -> Option<i32> {
        match &self {
            Self::GD1000 => Some(1),
            Self::GD1010 => Some(1),
            Self::GD1020 => Some(1),
            Self::GD1100 => Some(2),
            Self::GD1110 => Some(2),
            Self::GD1200 => Some(3),
            Self::GD1210 => Some(3),
            Self::GD1220 => Some(3),
            Self::GD1300 => Some(4),
            Self::GD1400 => Some(5),
            Self::GD1410 => Some(5),
            Self::GD1500 => Some(6),
            Self::GD1510 => Some(6),
            Self::GD1600 => Some(7),
            Self::GD1700 => Some(10),
            Self::GD1710 => Some(10),
            Self::GD1800 => Some(11),
            Self::GD1810 => Some(18),
            Self::GD1811 => Some(18),
            Self::GD1900 => Some(19),
            Self::GD1910 => Some(19),
            Self::GD1920 => Some(19),
            Self::GD1930 => Some(19),
            Self::GD2000 => Some(20),
            Self::GD2001 => Some(20),
            Self::GD2010 => Some(20),
            Self::GD2011 => Some(20),
            Self::GD2100 => Some(21),
            Self::GD2110 => Some(21),
            Self::GD2111 => Some(21),
            Self::GD2113 => Some(21),
            Self::GD2200 => Some(22),
            Self::GD2201 => Some(22),
            Self::GD2202 => Some(22),
            Self::GD2203 => Some(22),
            Self::GD2204 => Some(22),
            Self::GD2205 => Some(22),
            Self::GD2206 => Some(22),
            Self::GD2207 => Some(22),
            Self::GD22071 => Some(22),
            Self::GD22072 => Some(22),
            Self::GD22073 => Some(22),
            Self::GD22074 => Some(22),
            Self::GD2208 => Some(22),
            Self::GD22081 => Some(22),
            Self::GD22082 => Some(22),
            _ => None, // unknown
        }
    }

    /// Converts a raw binary version to a `GDVersion`. Since multiple game versions share the same binary version, only the earliest game version with a specific ID will be returned.
    ///
    /// This function ensured preservation of the argument value when `to_game_version_id` is called on the returned object.
    pub fn from_binary_version(bv: i32) -> Self {
        match bv {
            0 => Self::GD1000,
            // also 0: Self::GD1010,
            // also 0: Self::GD1020,
            // also 0: Self::GD1100,
            1 => Self::GD1110,
            2 => Self::GD1200,
            // also 2: Self::GD1210,
            // also 2: Self::GD1220,
            4 => Self::GD1300,
            5 => Self::GD1400,
            // also 5: Self::GD1410,
            6 => Self::GD1500,
            // also 6: Self::GD1510,
            7 => Self::GD1600,
            10 => Self::GD1700,
            11 => Self::GD1710,
            12 => Self::GD1800,
            13 => Self::GD1810,
            14 => Self::GD1811,
            20 => Self::GD1900,
            // also 20: Self::GD1910,
            24 => Self::GD1920,
            25 => Self::GD1930,
            27 => Self::GD2000,
            28 => Self::GD2001,
            29 => Self::GD2010,
            // also 29: Self::GD2011,
            33 => Self::GD2100,
            34 => Self::GD2110,
            // also 34: Self::GD2111,
            35 => Self::GD2113,
            38 => Self::GD2200,
            // also 38: Self::GD2201,
            39 => Self::GD2202,
            40 => Self::GD2203,
            // also 40: Self::GD2204,
            41 => Self::GD2205,
            42 => Self::GD2206,
            45 => Self::GD22074,
            46 => Self::GD2208,
            47 => Self::GD22081,
            48 => Self::GD22082,
            49 => Self::GD2209,
            n => Self::Unknown(n),
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
    /// - 2.207
    /// - 2.2071
    /// - 2.2072
    /// - 2.2073
    /// - `Self::Unknown`.
    pub fn to_binary_version_id(&self) -> Option<i32> {
        match self {
            Self::GD1000 => Some(0),
            Self::GD1010 => Some(0),
            Self::GD1020 => Some(0),
            Self::GD1100 => Some(0),
            Self::GD1110 => Some(1),
            Self::GD1200 => Some(2),
            Self::GD1210 => Some(2),
            Self::GD1220 => Some(2),
            Self::GD1300 => Some(4),
            Self::GD1400 => Some(5),
            Self::GD1410 => Some(5),
            Self::GD1500 => Some(6),
            Self::GD1510 => Some(6),
            Self::GD1600 => Some(7),
            Self::GD1700 => Some(10),
            Self::GD1710 => Some(11),
            Self::GD1800 => Some(12),
            Self::GD1810 => Some(13),
            Self::GD1811 => Some(14),
            Self::GD1900 => Some(20),
            Self::GD1910 => Some(20),
            Self::GD1920 => Some(24),
            Self::GD1930 => Some(25),
            Self::GD2000 => Some(27),
            Self::GD2001 => Some(28),
            Self::GD2010 => Some(29),
            Self::GD2011 => Some(29),
            Self::GD2100 => Some(33),
            Self::GD2110 => Some(34),
            Self::GD2111 => Some(34),
            Self::GD2113 => Some(35),
            Self::GD2200 => Some(38),
            Self::GD2201 => Some(38),
            Self::GD2202 => Some(39),
            Self::GD2203 => Some(40),
            Self::GD2204 => Some(40),
            Self::GD2205 => Some(41),
            Self::GD2206 => Some(42),
            Self::GD2207 => None,
            Self::GD22071 => None,
            Self::GD22072 => None,
            Self::GD22073 => None,
            Self::GD22074 => Some(45),
            Self::GD2208 => Some(46),
            Self::GD22081 => Some(47),
            Self::GD22082 => Some(48),
            Self::GD2209 => Some(49),
            Self::Unknown(_) => None,
        }
    }

    /// Returns the manifest version of a specific GD version.
    ///
    // Maintainer's note: I have no clue where this is used.
    pub fn to_manifest_version(&self) -> Option<i32> {
        match self {
            Self::GD1000 => Some(2),
            Self::GD1010 => Some(3),
            Self::GD1020 => Some(4),
            Self::GD1100 => Some(5),
            Self::GD1110 => Some(6),
            Self::GD1200 => Some(7),
            Self::GD1210 => Some(8),
            Self::GD1220 => Some(9),
            Self::GD1300 => Some(10),
            Self::GD1400 => Some(11),
            Self::GD1410 => Some(12),
            Self::GD1500 => Some(13),
            Self::GD1510 => Some(14),
            Self::GD1600 => Some(16),
            Self::GD1700 => Some(17),
            Self::GD1710 => Some(18),
            Self::GD1800 => Some(19),
            Self::GD1810 => Some(20),
            Self::GD1811 => Some(21),
            Self::GD1900 => Some(22),
            Self::GD1910 => Some(24),
            Self::GD1920 => Some(25),
            Self::GD1930 => Some(26),
            Self::GD2000 => Some(27),
            Self::GD2001 => Some(28),
            Self::GD2010 => Some(29),
            Self::GD2011 => Some(30),
            Self::GD2100 => Some(31),
            Self::GD2110 => Some(32),
            Self::GD2111 => Some(33),
            Self::GD2113 => None,
            Self::GD2200 => Some(38),
            Self::GD2201 => None,
            Self::GD2202 => None,
            Self::GD2203 => None,
            Self::GD2204 => None,
            Self::GD2205 => None,
            Self::GD2206 => None,
            Self::GD2207 => None,
            Self::GD22071 => None,
            Self::GD22072 => None,
            Self::GD22073 => None,
            Self::GD22074 => None,
            Self::GD2208 => None,
            Self::GD22081 => None,
            Self::GD22082 => None,
            Self::GD2209 => None,
            Self::Unknown(_) => None,
        }
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

*/
