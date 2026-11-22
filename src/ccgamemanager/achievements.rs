//! All vanilla GD achievements.
//!
//! [`GDAchievement`] has one variant per achievement. Convert to and from the game's string identifiers with [`GDAchievement::to_ident`] and [`GDAchievement::parse_str`]. Identifiers that aren't vanilla achievements return `None`.
//!
//! ```
//! use gdlib::ccgamemanager::achievements::GDAchievement;
//!
//! let ident = GDAchievement::Attempt01.to_ident();
//! assert_eq!(GDAchievement::parse_str(ident), Some(GDAchievement::Attempt01));
//! ```
//!
//! Progress values are read and set through [`GDStatistics`](super::structs::GDStatistics).

/// Defines [`GDAchievement`], its conversion functions, and [`MAX_ACHIEVEMENT_INDEX`] from a single table.
///
/// To add a new achievement, append one line at the **bottom** of the `gd_achievements!` invocation below.
/// A variant's repr value comes from its position in the table (the first is 1), so existing lines must not be reordered.
macro_rules! gd_achievements {
    (
        $(#[$first_attr:meta])*
        $first:ident => $first_ident:literal;
        $(
            $(#[$attr:meta])*
            $variant:ident => $ident:literal;
        )*
    ) => {
        /// The number of vanilla achievements, which is also the highest repr value of [`GDAchievement`].
        pub(crate) const MAX_ACHIEVEMENT_INDEX: usize = GDAchievement::ALL.len();

        /// All vanilla achievements in GD. All names were imported as-is, then captialized and stripped of spaces and periods with no further processing.
        ///
        /// Each variant has a repr value of the index that it will occupy in `CCGameManager.stats.achievements`.
        ///
        /// All achievement documentation was obtained from Wylie's docs: <https://wyliemaster.github.io/gddocs/#/resources/client/gamesave/achievement>
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(i32)]
        pub enum GDAchievement {
            $(#[$first_attr])*
            $first = 1,
            $( $(#[$attr])* $variant, )*
        }

        impl GDAchievement {
            /// Every vanilla achievement, in repr order.
            pub const ALL: &[GDAchievement] = &[Self::$first, $(Self::$variant),*];

            /// Parses an achievement identifier to a variant of self. This function will only successfully parse vanilla achievements.
            /// When parsing CCGameManager.dat, this function is used to identify each achievement.
            pub fn parse_str(s: &str) -> Option<Self> {
                match s {
                    $first_ident => Some(Self::$first),
                    $($ident => Some(Self::$variant),)*
                    _ => None,
                }
            }

            /// Returns the internal identifier used in the savefiles for this specific achievement.
            pub fn to_ident(&self) -> &str {
                match self {
                    Self::$first => $first_ident,
                    $(Self::$variant => $ident,)*
                }
            }
        }
    };
}

impl TryFrom<i32> for GDAchievement {
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::ALL
            .iter()
            .copied()
            .find(|a| *a as i32 == value)
            .ok_or(value)
    }
}

gd_achievements! {
    /// Beat Stereo Madness in normal Mode
    Level01b => "geometry.ach.level01b";
    /// Beat Stereo Madness in practice Mode
    Level01a => "geometry.ach.level01a";
    /// Beat Back On Track in normal Mode
    Level02b => "geometry.ach.level02b";
    /// Beat Back On Track in practice Mode
    Level02a => "geometry.ach.level02a";
    /// Beat Polargeist in normal Mode
    Level03b => "geometry.ach.level03b";
    /// Beat Polargeist in practice Mode
    Level03a => "geometry.ach.level03a";
    /// Beat Dry Out in normal Mode
    Level04b => "geometry.ach.level04b";
    /// Beat Dry Out in practice Mode
    Level04a => "geometry.ach.level04a";
    /// Beat Base after Base in normal Mode
    Level05b => "geometry.ach.level05b";
    /// Beat Base After Base in practice Mode
    Level05a => "geometry.ach.level05a";
    /// Beat Cant Let Go in normal Mode
    Level06b => "geometry.ach.level06b";
    /// Beat Cant Let Go in practice Mode
    Level06a => "geometry.ach.level06a";
    /// Beat Jumper in normal Mode
    Level07b => "geometry.ach.level07b";
    /// Beat Jumper in practice Mode
    Level07a => "geometry.ach.level07a";
    /// Beat Time Machine in normal Mode
    Level08b => "geometry.ach.level08b";
    /// Beat Time Machine in practice Mode
    Level08a => "geometry.ach.level08a";
    /// Beat Cycles in normal Mode
    Level09b => "geometry.ach.level09b";
    /// Beat Cycles in practice Mode
    Level09a => "geometry.ach.level09a";
    /// Beat Xstep in normal Mode
    Level10b => "geometry.ach.level10b";
    /// Beat Xstep in practice Mode
    Level10a => "geometry.ach.level10a";
    /// Beat Clutterfunk in normal Mode
    Level11b => "geometry.ach.level11b";
    /// Beat Clutterfunk in practice Mode
    Level11a => "geometry.ach.level11a";
    /// Beat Theory Of Everything in normal Mode
    Level12b => "geometry.ach.level12b";
    /// Beat Theory Of Everything in practice Mode
    Level12a => "geometry.ach.level12a";
    /// Beat Electroman Adventures in normal Mode
    Level13b => "geometry.ach.level13b";
    /// Beat Electroman Adventures in practice Mode
    Level13a => "geometry.ach.level13a";
    /// Beat Clubstep in normal Mode
    Level14b => "geometry.ach.level14b";
    /// Beat Clubstep in practice Mode
    Level14a => "geometry.ach.level14a";
    /// Beat Electrodynamix in normal Mode
    Level15b => "geometry.ach.level15b";
    /// Beat Electrodynamix in practice Mode
    Level15a => "geometry.ach.level15a";
    /// Beat Hexagon Force in normal Mode
    Level16b => "geometry.ach.level16b";
    /// Beat Hexagon Forece in practice Mode
    Level16a => "geometry.ach.level16a";
    /// Beat Blast Processing in normal Mode
    Level17b => "geometry.ach.level17b";
    /// Beat Blast Processing in practice Mode
    Level17a => "geometry.ach.level17a";
    /// Beat Theory Of Everything 2 in normal Mode
    Level18b => "geometry.ach.level18b";
    /// Beat Theory Of Everything 2 in practice Mode
    Level18a => "geometry.ach.level18a";
    /// Beat Geometrical Dominator in normal Mode
    Level19b => "geometry.ach.level19b";
    /// Beat Geometrical Dominator in practice Mode
    Level19a => "geometry.ach.level19a";
    /// Beat Deadlocked in normal Mode
    Level20b => "geometry.ach.level20b";
    /// Beat Deadlocked in practice Mode
    Level20a => "geometry.ach.level20a";
    /// Beat Fingerdash in normal Mode
    Level21b => "geometry.ach.level21b";
    /// Beat Fingerdash in practice Mode
    Level21a => "geometry.ach.level21a";
    /// Beat Dash in normal Mode
    Level22b => "geometry.ach.level22b";
    /// Beat Dash in practice Mode
    Level22a => "geometry.ach.level22a";
    /// Beat The Tower
    Tower01 => "geometry.ach.tower01";
    /// Beat The Sewers
    Tower02 => "geometry.ach.tower02";
    /// Beat The Cellar
    Tower03 => "geometry.ach.tower03";
    /// Beat The Secret Hollow
    Tower04 => "geometry.ach.tower04";
    /// Beat The Tower with all 3 Coins
    Tower01Coin => "geometry.ach.tower01Coin";
    /// Beat The Sewers with all 3 Coins
    Tower02Coin => "geometry.ach.tower02Coin";
    /// Beat The Cellar with all 3 Coins
    Tower03Coin => "geometry.ach.tower03Coin";
    /// Beat The Secret Hollow with all 3 Coins
    Tower04Coin => "geometry.ach.tower04Coin";
    /// Beat Stereo Madness, Back On Track and Polargeist in normal mode (steam exclusive)
    Steam01 => "geometry.ach.steam01";
    /// Beat Clubstep in normal mode (steam exclusive)
    Steam02 => "geometry.ach.steam02";
    /// Beat Clubstep in with all coins
    Demoncoin01 => "geometry.ach.demoncoin01";
    /// Beat Theory Of Everything 2 in with all coins
    Demoncoin02 => "geometry.ach.demoncoin02";
    /// Beat Deadlocked in with all coins
    Demoncoin03 => "geometry.ach.demoncoin03";
    /// Complete 1 Map Pack
    Mappacks01 => "geometry.ach.mappacks01";
    /// Complete 5 Map Packs
    Mappacks02 => "geometry.ach.mappacks02";
    /// Complete 10 Map Packs
    Mappacks03 => "geometry.ach.mappacks03";
    /// Complete 15 Map Packs
    Mappacks04 => "geometry.ach.mappacks04";
    /// Complete 20 Map Packs
    Mappacks05 => "geometry.ach.mappacks05";
    /// Complete 25 Map Packs
    Mappacks06 => "geometry.ach.mappacks06";
    /// Complete 30 Map Packs
    Mappacks07 => "geometry.ach.mappacks07";
    /// Complete 35 Map Packs
    Mappacks08 => "geometry.ach.mappacks08";
    /// Complete 40 Map Packs
    Mappacks09 => "geometry.ach.mappacks09";
    /// Complete 45 Map Packs
    Mappacks10 => "geometry.ach.mappacks10";
    /// Complete 1 Gauntlet
    Gauntlets01 => "geometry.ach.gauntlets01";
    /// Complete 5 Gauntlets
    Gauntlets02 => "geometry.ach.gauntlets02";
    /// Complete 10 Gauntlets
    Gauntlets03 => "geometry.ach.gauntlets03";
    /// Complete 15 Gauntlets
    Gauntlets04 => "geometry.ach.gauntlets04";
    /// Complete a Daily Level
    Daily01 => "geometry.ach.daily01";
    /// Complete 25 Daily Levels
    Daily02 => "geometry.ach.daily02";
    /// Complete 50 Daily Levels
    Daily03 => "geometry.ach.daily03";
    /// Complete 100 Daily Levels
    Daily04 => "geometry.ach.daily04";
    /// Complete 150 Daily Levels
    Daily05 => "geometry.ach.daily05";
    /// Complete 250 Daily Levels
    Daily06 => "geometry.ach.daily06";
    /// Complete 365 Daily Levels
    Daily07 => "geometry.ach.daily07";
    /// Complete 1 User Created Level in Normal Mode
    Custom01 => "geometry.ach.custom01";
    /// Complete 10 User Created Levels in Normal Mode
    Custom02 => "geometry.ach.custom02";
    /// Complete 50 User Created Levels in Normal Mode
    Custom03 => "geometry.ach.custom03";
    /// Complete 100 User Created Levels in Normal Mode
    Custom04 => "geometry.ach.custom04";
    /// Complete 200 User Created Levels in Normal Mode
    Custom05 => "geometry.ach.custom05";
    /// Complete 300 User Created Levels in Normal Mode
    Custom06 => "geometry.ach.custom06";
    /// Complete 500 User Created Levels in Normal Mode
    Custom07 => "geometry.ach.custom07";
    /// Complete 1,000 User Created Levels in Normal Mode
    Custom08 => "geometry.ach.custom08";
    /// Complete 1,500 User Created Levels in Normal Mode
    Custom09 => "geometry.ach.custom09";
    /// Complete 2,000 User Created Levels in Normal Mode
    Custom10 => "geometry.ach.custom10";
    /// Complete 2,500 User Created Levels in Normal Mode
    Custom11 => "geometry.ach.custom11";
    /// Complete 3,000 User Created Levels in Normal Mode
    Custom12 => "geometry.ach.custom12";
    /// Complete 4,000 User Created Levels in Normal Mode
    Custom13 => "geometry.ach.custom13";
    /// Complete 5,000 User Created Levels in Normal Mode
    Custom14 => "geometry.ach.custom14";
    /// Collect 100 stars
    Stars01 => "geometry.ach.stars01";
    /// Collect 200 stars
    Stars02 => "geometry.ach.stars02";
    /// Collect 300 stars
    Stars03 => "geometry.ach.stars03";
    /// Collect 400 stars
    Stars04 => "geometry.ach.stars04";
    /// Collect 500 stars
    Stars05 => "geometry.ach.stars05";
    /// Collect 600 stars
    Stars06 => "geometry.ach.stars06";
    /// Collect 700 stars
    Stars07 => "geometry.ach.stars07";
    /// Collect 800 stars
    Stars08 => "geometry.ach.stars08";
    /// Collect 900 stars
    Stars09 => "geometry.ach.stars09";
    /// Collect 1,000 stars
    Stars10 => "geometry.ach.stars10";
    /// Collect 1,500 stars
    Stars11 => "geometry.ach.stars11";
    /// Collect 2,000 stars
    Stars12 => "geometry.ach.stars12";
    /// Collect 2,500 stars
    Stars13 => "geometry.ach.stars13";
    /// Collect 3,000 stars
    Stars14 => "geometry.ach.stars14";
    /// Collect 3,500 stars
    Stars15 => "geometry.ach.stars15";
    /// Collect 4,000 stars
    Stars16 => "geometry.ach.stars16";
    /// Collect 4,500 stars
    Stars17 => "geometry.ach.stars17";
    /// Collect 5,000 stars
    Stars18 => "geometry.ach.stars18";
    /// Collect 5,500 stars
    Stars19 => "geometry.ach.stars19";
    /// Collect 6,000 stars
    Stars20 => "geometry.ach.stars20";
    /// Collect 6,500 stars
    Stars21 => "geometry.ach.stars21";
    /// Collect 7,000 stars
    Stars22 => "geometry.ach.stars22";
    /// Collect 7,500 stars
    Stars23 => "geometry.ach.stars23";
    /// Collect 8,000 stars
    Stars24 => "geometry.ach.stars24";
    /// Collect 9,000 stars
    Stars25 => "geometry.ach.stars25";
    /// Collect 10,000 stars
    Stars26 => "geometry.ach.stars26";
    /// Collect 12,000 stars
    Stars27 => "geometry.ach.stars27";
    /// Collect 14,000 stars
    Stars28 => "geometry.ach.stars28";
    /// Collect 16,000 stars
    Stars29 => "geometry.ach.stars29";
    /// Collect 18,000 stars
    Stars30 => "geometry.ach.stars30";
    /// Collect 20,000 stars
    Stars31 => "geometry.ach.stars31";
    /// Collect 22,500 stars
    Stars32 => "geometry.ach.stars32";
    /// Collect 25,000 stars
    Stars33 => "geometry.ach.stars33";
    /// Collect 100 moons
    Moons01 => "geometry.ach.moons01";
    /// Collect 200 moons
    Moons02 => "geometry.ach.moons02";
    /// Collect 300 moons
    Moons03 => "geometry.ach.moons03";
    /// Collect 400 moons
    Moons04 => "geometry.ach.moons04";
    /// Collect 500 moons
    Moons05 => "geometry.ach.moons05";
    /// Collect 600 moons
    Moons06 => "geometry.ach.moons06";
    /// Collect 700 moons
    Moons07 => "geometry.ach.moons07";
    /// Collect 800 moons
    Moons08 => "geometry.ach.moons08";
    /// Collect 900 moons
    Moons09 => "geometry.ach.moons09";
    /// Collect 1,000 moons
    Moons10 => "geometry.ach.moons10";
    /// Collect 1,500 moons
    Moons11 => "geometry.ach.moons11";
    /// Collect 2,000 moons
    Moons12 => "geometry.ach.moons12";
    /// Collect 2,500 moons
    Moons13 => "geometry.ach.moons13";
    /// Collect 3,000 moons
    Moons14 => "geometry.ach.moons14";
    /// Collect 3,500 moons
    Moons15 => "geometry.ach.moons15";
    /// Collect 4,000 moons
    Moons16 => "geometry.ach.moons16";
    /// Collect 4,500 moons
    Moons17 => "geometry.ach.moons17";
    /// Collect 5,000 moons
    Moons18 => "geometry.ach.moons18";
    /// Collect 5,500 moons
    Moons19 => "geometry.ach.moons19";
    /// Collect 6,000 moons
    Moons20 => "geometry.ach.moons20";
    /// Collect 6,500 moons
    Moons21 => "geometry.ach.moons21";
    /// Collect 7,000 moons
    Moons22 => "geometry.ach.moons22";
    /// Collect 7,500 moons
    Moons23 => "geometry.ach.moons23";
    /// Collect 8,000 moons
    Moons24 => "geometry.ach.moons24";
    /// Collect 9,000 moons
    Moons25 => "geometry.ach.moons25";
    /// Collect 10,000 moons
    Moons26 => "geometry.ach.moons26";
    /// Beat 1 Demon
    Demon01 => "geometry.ach.demon01";
    /// Beat 2 Demons
    Demon02 => "geometry.ach.demon02";
    /// Beat 3 Demons
    Demon03 => "geometry.ach.demon03";
    /// Beat 4 Demons
    Demon04 => "geometry.ach.demon04";
    /// Beat 5 Demons
    Demon05 => "geometry.ach.demon05";
    /// Beat 10 Demons
    Demon06 => "geometry.ach.demon06";
    /// Beat 15 Demons
    Demon07 => "geometry.ach.demon07";
    /// Beat 20 Demons
    Demon08 => "geometry.ach.demon08";
    /// Beat 30 Demons
    Demon09 => "geometry.ach.demon09";
    /// Beat 40 Demons
    Demon10 => "geometry.ach.demon10";
    /// Beat 50 Demons
    Demon11 => "geometry.ach.demon11";
    /// Beat 60 Demons
    Demon12 => "geometry.ach.demon12";
    /// Beat 70 Demons
    Demon13 => "geometry.ach.demon13";
    /// Beat 80 Demons
    Demon14 => "geometry.ach.demon14";
    /// Beat 90 Demons
    Demon15 => "geometry.ach.demon15";
    /// Beat 100 Demons
    Demon16 => "geometry.ach.demon16";
    /// Beat 120 Demons
    Demon17 => "geometry.ach.demon17";
    /// Beat 140 Demons
    Demon18 => "geometry.ach.demon18";
    /// Beat 160 Demons
    Demon19 => "geometry.ach.demon19";
    /// Beat 180 Demons
    Demon20 => "geometry.ach.demon20";
    /// Beat 200 Demons
    Demon21 => "geometry.ach.demon21";
    /// Beat 225 Demons
    Demon22 => "geometry.ach.demon22";
    /// Beat 250 Demons
    Demon23 => "geometry.ach.demon23";
    /// Beat 275 Demons
    Demon24 => "geometry.ach.demon24";
    /// Beat 300 Demons
    Demon25 => "geometry.ach.demon25";
    /// Beat 350 Demons
    Demon26 => "geometry.ach.demon26";
    /// Beat 400 Demons
    Demon27 => "geometry.ach.demon27";
    /// Beat 500 Demons
    Demon28 => "geometry.ach.demon28";
    /// Collect 5 Secret Coins
    Coins01 => "geometry.ach.coins01";
    /// Collect 10 Secret Coins
    Coins02 => "geometry.ach.coins02";
    /// Collect 15 Secret Coins
    Coins03 => "geometry.ach.coins03";
    /// Collect 20 Secret Coins
    Coins04 => "geometry.ach.coins04";
    /// Collect 25 Secret Coins
    Coins05 => "geometry.ach.coins05";
    /// Collect 30 Secret Coins
    Coins06 => "geometry.ach.coins06";
    /// Collect 35 Secret Coins
    Coins07 => "geometry.ach.coins07";
    /// Collect 40 Secret Coins
    Coins08 => "geometry.ach.coins08";
    /// Collect 45 Secret Coins
    Coins09 => "geometry.ach.coins09";
    /// Collect 50 Secret Coins
    Coins10 => "geometry.ach.coins10";
    /// Collect 55 Secret Coins
    Coins11 => "geometry.ach.coins11";
    /// Collect 60 Secret Coins
    Coins12 => "geometry.ach.coins12";
    /// Collect 65 Secret Coins
    Coins13 => "geometry.ach.coins13";
    /// Collect 70 Secret Coins
    Coins14 => "geometry.ach.coins14";
    /// Collect 75 Secret Coins
    Coins15 => "geometry.ach.coins15";
    /// Collect 80 Secret Coins
    Coins16 => "geometry.ach.coins16";
    /// Collect 85 Secret Coins
    Coins17 => "geometry.ach.coins17";
    /// Collect 90 Secret Coins
    Coins18 => "geometry.ach.coins18";
    /// Collect 95 Secret Coins
    Coins19 => "geometry.ach.coins19";
    /// Collect 100 Secret Coins
    Coins20 => "geometry.ach.coins20";
    /// Collect 105 Secret Coins
    Coins21 => "geometry.ach.coins21";
    /// Collect 110 Secret Coins
    Coins22 => "geometry.ach.coins22";
    /// Collect 115 Secret Coins
    Coins23 => "geometry.ach.coins23";
    /// Collect 120 Secret Coins
    Coins24 => "geometry.ach.coins24";
    /// Collect 130 Secret Coins
    Coins25 => "geometry.ach.coins25";
    /// Collect 140 Secret Coins
    Coins26 => "geometry.ach.coins26";
    /// Collect 150 Secret Coins
    Coins27 => "geometry.ach.coins27";
    /// Collect 160 Secret Coins
    Coins28 => "geometry.ach.coins28";
    /// Collect 1 user Coin
    Usercoins01 => "geometry.ach.usercoins01";
    /// Collect 10 user Coins
    Usercoins02 => "geometry.ach.usercoins02";
    /// Collect 20 user Coins
    Usercoins03 => "geometry.ach.usercoins03";
    /// Collect 30 user Coins
    Usercoins04 => "geometry.ach.usercoins04";
    /// Collect 40 user Coins
    Usercoins05 => "geometry.ach.usercoins05";
    /// Collect 50 user Coins
    Usercoins06 => "geometry.ach.usercoins06";
    /// Collect 60 user Coins
    Usercoins07 => "geometry.ach.usercoins07";
    /// Collect 70 user Coins
    Usercoins08 => "geometry.ach.usercoins08";
    /// Collect 80 user Coins
    Usercoins09 => "geometry.ach.usercoins09";
    /// Collect 90 user Coins
    Usercoins10 => "geometry.ach.usercoins10";
    /// Collect 100 user Coins
    Usercoins11 => "geometry.ach.usercoins11";
    /// Collect 110 user Coins
    Usercoins12 => "geometry.ach.usercoins12";
    /// Collect 120 user Coins
    Usercoins13 => "geometry.ach.usercoins13";
    /// Collect 130 user Coins
    Usercoins14 => "geometry.ach.usercoins14";
    /// Collect 140 user Coins
    Usercoins15 => "geometry.ach.usercoins15";
    /// Collect 150 user Coins
    Usercoins16 => "geometry.ach.usercoins16";
    /// Collect 160 user Coins
    Usercoins17 => "geometry.ach.usercoins17";
    /// Collect 170 user Coins
    Usercoins18 => "geometry.ach.usercoins18";
    /// Collect 180 user Coins
    Usercoins19 => "geometry.ach.usercoins19";
    /// Collect 190 user Coins
    Usercoins20 => "geometry.ach.usercoins20";
    /// Collect 200 user Coins
    Usercoins21 => "geometry.ach.usercoins21";
    /// Collect 225 user Coins
    Usercoins22 => "geometry.ach.usercoins22";
    /// Collect 250 user Coins
    Usercoins23 => "geometry.ach.usercoins23";
    /// Collect 300 user Coins
    Usercoins24 => "geometry.ach.usercoins24";
    /// Collect 350 user Coins
    Usercoins25 => "geometry.ach.usercoins25";
    /// Collect 425 user Coins
    Usercoins26 => "geometry.ach.usercoins26";
    /// Collect 500 user Coins
    Usercoins27 => "geometry.ach.usercoins27";
    /// Collect 600 user Coins
    Usercoins28 => "geometry.ach.usercoins28";
    /// Collect 700 user Coins
    Usercoins29 => "geometry.ach.usercoins29";
    /// Collect 800 user Coins
    Usercoins30 => "geometry.ach.usercoins30";
    /// Collect 900 user Coins
    Usercoins31 => "geometry.ach.usercoins31";
    /// Collect 1,000 user Coins
    Usercoins32 => "geometry.ach.usercoins32";
    /// Collect 1,200 user Coins
    Usercoins33 => "geometry.ach.usercoins33";
    /// Collect 1,400 user Coins
    Usercoins34 => "geometry.ach.usercoins34";
    /// Collect 1,600 user Coins
    Usercoins35 => "geometry.ach.usercoins35";
    /// Collect 1,800 user Coins
    Usercoins36 => "geometry.ach.usercoins36";
    /// Collect 2,000 user Coins
    Usercoins37 => "geometry.ach.usercoins37";
    /// Collect 2,300 user Coins
    Usercoins38 => "geometry.ach.usercoins38";
    /// Collect 2,600 user Coins
    Usercoins39 => "geometry.ach.usercoins39";
    /// Collect 3,000 user Coins
    Usercoins40 => "geometry.ach.usercoins40";
    /// Collect 100 Diamonds
    Diamonds01 => "geometry.ach.diamonds01";
    /// Collect 250 Diamonds
    Diamonds02 => "geometry.ach.diamonds02";
    /// Collect 500 Diamonds
    Diamonds03 => "geometry.ach.diamonds03";
    /// Collect 1,000 Diamonds
    Diamonds04 => "geometry.ach.diamonds04";
    /// Collect 1,500 Diamonds
    Diamonds05 => "geometry.ach.diamonds05";
    /// Collect 2,000 Diamonds
    Diamonds06 => "geometry.ach.diamonds06";
    /// Collect 2,500 Diamonds
    Diamonds07 => "geometry.ach.diamonds07";
    /// Collect 3,000 Diamonds
    Diamonds08 => "geometry.ach.diamonds08";
    /// Collect 4,000 Diamonds
    Diamonds09 => "geometry.ach.diamonds09";
    /// Collect 5,000 Diamonds
    Diamonds10 => "geometry.ach.diamonds10";
    /// Collect 6,000 Diamonds
    Diamonds11 => "geometry.ach.diamonds11";
    /// Collect 7,000 Diamonds
    Diamonds12 => "geometry.ach.diamonds12";
    /// Collect 9,000 Diamonds
    Diamonds13 => "geometry.ach.diamonds13";
    /// Collect 11,000 Diamonds
    Diamonds14 => "geometry.ach.diamonds14";
    /// Collect 14,000 Diamonds
    Diamonds15 => "geometry.ach.diamonds15";
    /// Collect 17,000 Diamonds
    Diamonds16 => "geometry.ach.diamonds16";
    /// Collect 20,000 Diamonds
    Diamonds17 => "geometry.ach.diamonds17";
    /// Collect 25,000 Diamonds
    Diamonds18 => "geometry.ach.diamonds18";
    /// Claim a List Reward
    Lists01 => "geometry.ach.lists01";
    /// Claim 25 List Rewards
    Lists02 => "geometry.ach.lists02";
    /// Claim 50 List Rewards
    Lists03 => "geometry.ach.lists03";
    /// Claim 100 List Rewards
    Lists04 => "geometry.ach.lists04";
    /// Claim 150 List Rewards
    Lists05 => "geometry.ach.lists05";
    /// Claim 200 List Rewards
    Lists06 => "geometry.ach.lists06";
    /// Collect 5 Fire Shards
    ShardFire01 => "geometry.ach.shardFire01";
    /// Collect 15 Fire Shards
    ShardFire02 => "geometry.ach.shardFire02";
    /// Collect 35 Fire Shards
    ShardFire03 => "geometry.ach.shardFire03";
    /// Collect 65 Fire Shards
    ShardFire04 => "geometry.ach.shardFire04";
    /// Collect 100 Fire Shards
    ShardFire05 => "geometry.ach.shardFire05";
    /// Collect 5 Ice Shards
    ShardIce01 => "geometry.ach.shardIce01";
    /// Collect 15 Ice Shards
    ShardIce02 => "geometry.ach.shardIce02";
    /// Collect 35 Ice Shards
    ShardIce03 => "geometry.ach.shardIce03";
    /// Collect 65 Ice Shards
    ShardIce04 => "geometry.ach.shardIce04";
    /// Collect 100 Ice Shards
    ShardIce05 => "geometry.ach.shardIce05";
    /// Collect 5 Poison Shards
    ShardPoison01 => "geometry.ach.shardPoison01";
    /// Collect 15 Poison Shards
    ShardPoison02 => "geometry.ach.shardPoison02";
    /// Collect 35 Poison Shards
    ShardPoison03 => "geometry.ach.shardPoison03";
    /// Collect 65 Poison Shards
    ShardPoison04 => "geometry.ach.shardPoison04";
    /// Collect 100 Poison Shards
    ShardPoison05 => "geometry.ach.shardPoison05";
    /// Collect 5 Shadow Shards
    ShardShadow01 => "geometry.ach.shardShadow01";
    /// Collect 15 Shadow Shards
    ShardShadow02 => "geometry.ach.shardShadow02";
    /// Collect 35 Shadow Shards
    ShardShadow03 => "geometry.ach.shardShadow03";
    /// Collect 65 Shadow Shards
    ShardShadow04 => "geometry.ach.shardShadow04";
    /// Collect 100 Shadow Shards
    ShardShadow05 => "geometry.ach.shardShadow05";
    /// Collect 5 Lava Shards
    ShardLava01 => "geometry.ach.shardLava01";
    /// Collect 15 Lava Shards
    ShardLava02 => "geometry.ach.shardLava02";
    /// Collect 35 Lava Shards
    ShardLava03 => "geometry.ach.shardLava03";
    /// Collect 65 Lava Shards
    ShardLava04 => "geometry.ach.shardLava04";
    /// Collect 100 Lava Shards
    ShardLava05 => "geometry.ach.shardLava05";
    /// Collect 5 of each Tier 1 Shard
    ShardBonus01 => "geometry.ach.shardBonus01";
    /// Collect 15 of each Tier 1 Shard
    ShardBonus02 => "geometry.ach.shardBonus02";
    /// Collect 35 of each Tier 1 Shard
    ShardBonus03 => "geometry.ach.shardBonus03";
    /// Collect 65 of each Tier 1 Shard
    ShardBonus04 => "geometry.ach.shardBonus04";
    /// Collect 100 of each Tier 1 Shard
    ShardBonus05 => "geometry.ach.shardBonus05";
    /// Collect 5 Earth Shards
    ShardEarth01 => "geometry.ach.shardEarth01";
    /// Collect 15 Earth Shards
    ShardEarth02 => "geometry.ach.shardEarth02";
    /// Collect 35 Earth Shards
    ShardEarth03 => "geometry.ach.shardEarth03";
    /// Collect 65 Earth Shards
    ShardEarth04 => "geometry.ach.shardEarth04";
    /// Collect 100 Earth Shards
    ShardEarth05 => "geometry.ach.shardEarth05";
    /// Collect 5 Blood Shards
    ShardBlood01 => "geometry.ach.shardBlood01";
    /// Collect 15 Blood Shards
    ShardBlood02 => "geometry.ach.shardBlood02";
    /// Collect 35 Blood Shards
    ShardBlood03 => "geometry.ach.shardBlood03";
    /// Collect 65 Blood Shards
    ShardBlood04 => "geometry.ach.shardBlood04";
    /// Collect 100 Blood Shards
    ShardBlood05 => "geometry.ach.shardBlood05";
    /// Collect 5 Metal Shards
    ShardMetal01 => "geometry.ach.shardMetal01";
    /// Collect 15 Metal Shards
    ShardMetal02 => "geometry.ach.shardMetal02";
    /// Collect 35 Metal Shards
    ShardMetal03 => "geometry.ach.shardMetal03";
    /// Collect 65 Metal Shards
    ShardMetal04 => "geometry.ach.shardMetal04";
    /// Collect 100 Metal Shards
    ShardMetal05 => "geometry.ach.shardMetal05";
    /// Collect 5 Light Shards
    ShardLight01 => "geometry.ach.shardLight01";
    /// Collect 15 Light Shards
    ShardLight02 => "geometry.ach.shardLight02";
    /// Collect 35 Light Shards
    ShardLight03 => "geometry.ach.shardLight03";
    /// Collect 65 Light Shards
    ShardLight04 => "geometry.ach.shardLight04";
    /// Collect 100 Light Shards
    ShardLight05 => "geometry.ach.shardLight05";
    /// Collect 5 Soul Shards
    ShardSoul01 => "geometry.ach.shardSoul01";
    /// Collect 15 Soul Shards
    ShardSoul02 => "geometry.ach.shardSoul02";
    /// Collect 35 Soul Shards
    ShardSoul03 => "geometry.ach.shardSoul03";
    /// Collect 65 Soul Shards
    ShardSoul04 => "geometry.ach.shardSoul04";
    /// Collect 100 Soul Shards
    ShardSoul05 => "geometry.ach.shardSoul05";
    /// Collect 5 of each Tier 2 Shard
    ShardBonusB01 => "geometry.ach.shardBonusB01";
    /// Collect 15 of each Tier 2 Shard
    ShardBonusB02 => "geometry.ach.shardBonusB02";
    /// Collect 35 of each Tier 2 Shard
    ShardBonusB03 => "geometry.ach.shardBonusB03";
    /// Collect 65 of each Tier 2 Shard
    ShardBonusB04 => "geometry.ach.shardBonusB04";
    /// Collect 100 of each Tier 2 Shard
    ShardBonusB05 => "geometry.ach.shardBonusB05";
    /// Follow 1 Creator
    FollowCreator => "geometry.ach.followCreator";
    /// Follow 10 Creators
    FollowCreator2 => "geometry.ach.followCreator2";
    /// Follow 25 Creators
    FollowCreator3 => "geometry.ach.followCreator3";
    /// Follow 50 Creators
    FollowCreator4 => "geometry.ach.followCreator4";
    /// Befriend 1 user
    Friends01 => "geometry.ach.friends01";
    /// Befriend 10 users
    Friends02 => "geometry.ach.friends02";
    /// Befriend 25 users
    Friends03 => "geometry.ach.friends03";
    /// Befriend 50 users
    Friends04 => "geometry.ach.friends04";
    /// subscribe to RobTop on YouTube
    Youtube => "geometry.ach.youtube";
    /// Follow RobTop on Twitter
    Twitter => "geometry.ach.twitter";
    /// Like RobTop on Facebook
    Facebook => "geometry.ach.facebook";
    /// Do 100 Attempts
    Attempt01 => "geometry.ach.attempt01";
    /// Do 500 Attempts
    Attempt02 => "geometry.ach.attempt02";
    /// Do 2,000 Attempts
    Attempt03 => "geometry.ach.attempt03";
    /// Do 10,000 Attempts
    Attempt04 => "geometry.ach.attempt04";
    /// Do 20,000 Attempts
    Attempt05 => "geometry.ach.attempt05";
    /// Do 30,000 Attempts
    Attempt06 => "geometry.ach.attempt06";
    /// Do 40,000 Attempts
    Attempt07 => "geometry.ach.attempt07";
    /// Do 60,000 Attempts
    Attempt08 => "geometry.ach.attempt08";
    /// Do 80,000 Attempts
    Attempt09 => "geometry.ach.attempt09";
    /// Do 100,000 Attempts
    Attempt10 => "geometry.ach.attempt10";
    /// Do 135,000 Attempts
    Attempt11 => "geometry.ach.attempt11";
    /// Do 185,000 Attempts
    Attempt12 => "geometry.ach.attempt12";
    /// Do 250,000 Attempts
    Attempt13 => "geometry.ach.attempt13";
    /// Do 300,000 Attempts
    Attempt14 => "geometry.ach.attempt14";
    /// Jump 1,000 Times
    Jump01 => "geometry.ach.jump01";
    /// Jump 10,000 Times
    Jump02 => "geometry.ach.jump02";
    /// Jump 20,000 Times
    Jump03 => "geometry.ach.jump03";
    /// Jump 50,000 Times
    Jump04 => "geometry.ach.jump04";
    /// Jump 100,000 Times
    Jump05 => "geometry.ach.jump05";
    /// Jump 200,000 Times
    Jump06 => "geometry.ach.jump06";
    /// Jump 300,000 Times
    Jump07 => "geometry.ach.jump07";
    /// Jump 400,000 Times
    Jump08 => "geometry.ach.jump08";
    /// Jump 500,000 Times
    Jump09 => "geometry.ach.jump09";
    /// Jump 750,000 Times
    Jump10 => "geometry.ach.jump10";
    /// Jump 1,000,000 Times
    Jump11 => "geometry.ach.jump11";
    /// Submit an online level
    Submit => "geometry.ach.submit";
    /// Click the supporter icon when it's locked
    Rate => "geometry.ach.rate";
    /// Rate the stars of an online level
    RateDiff => "geometry.ach.rateDiff";
    /// Rate the stars of 100 online levels
    RateDiff02 => "geometry.ach.rateDiff02";
    /// Rate the stars of 500 online levels
    RateDiff02b => "geometry.ach.rateDiff02b";
    /// Rate the stars of 1,000 online levels
    RateDiff03 => "geometry.ach.rateDiff03";
    /// Rate the stars of 2,000 online levels
    RateDiff04 => "geometry.ach.rateDiff04";
    /// Like or dislike an online level
    Like => "geometry.ach.like";
    /// Like or dislike 100 online levels
    Like02 => "geometry.ach.like02";
    /// Like or dislike 500 online levels
    Like02b => "geometry.ach.like02b";
    /// Like or dislike 1,000 online levels
    Like03 => "geometry.ach.like03";
    /// Like or dislike 2,000 online levels
    Like04 => "geometry.ach.like04";
    /// Like or dislike 3,000 online levels
    Like05 => "geometry.ach.like05";
    /// Like or dislike 4,000 online levels
    Like06 => "geometry.ach.like06";
    /// Click the "More Games" button
    MoreGames => "geometry.ach.moreGames";
    /// Die at over 95% on a main level
    Special01 => "geometry.ach.special01";
    /// Get 100 likes on your level
    Creator01 => "geometry.ach.creator01";
    /// Get a star rate on your level
    Creator02 => "geometry.ach.creator02";
    /// Get 50 likes on your level
    Creator03 => "geometry.ach.creator03";
    /// Get 250 likes on your level
    Creator04 => "geometry.ach.creator04";
    /// Get 500 likes on your level
    Creator05 => "geometry.ach.creator05";
    /// Get 1,000 likes on your level
    Creator06 => "geometry.ach.creator06";
    /// Unlock the Path of Fire
    Path0100 => "geometry.ach.path01.00";
    /// Reach Path of Fire Rank 1
    Path0101 => "geometry.ach.path01.01";
    /// Reach Path of Fire Rank 2
    Path0102 => "geometry.ach.path01.02";
    /// Reach Path of Fire Rank 3
    Path0103 => "geometry.ach.path01.03";
    /// Reach Path of Fire Rank 4
    Path0104 => "geometry.ach.path01.04";
    /// Reach Path of Fire Rank 5
    Path0105 => "geometry.ach.path01.05";
    /// Reach Path of Fire Rank 6
    Path0106 => "geometry.ach.path01.06";
    /// Reach Path of Fire Rank 7
    Path0107 => "geometry.ach.path01.07";
    /// Reach Path of Fire Rank 8
    Path0108 => "geometry.ach.path01.08";
    /// Reach Path of Fire Rank 9
    Path0109 => "geometry.ach.path01.09";
    /// Reach Path of Fire Rank 10
    Path0110 => "geometry.ach.path01.10";
    /// Unlock the Path of Ice
    Path0200 => "geometry.ach.path02.00";
    /// Reach Path of Ice Rank 1
    Path0201 => "geometry.ach.path02.01";
    /// Reach Path of Ice Rank 2
    Path0202 => "geometry.ach.path02.02";
    /// Reach Path of Ice Rank 3
    Path0203 => "geometry.ach.path02.03";
    /// Reach Path of Ice Rank 4
    Path0204 => "geometry.ach.path02.04";
    /// Reach Path of Ice Rank 5
    Path0205 => "geometry.ach.path02.05";
    /// Reach Path of Ice Rank 6
    Path0206 => "geometry.ach.path02.06";
    /// Reach Path of Ice Rank 7
    Path0207 => "geometry.ach.path02.07";
    /// Reach Path of Ice Rank 8
    Path0208 => "geometry.ach.path02.08";
    /// Reach Path of Ice Rank 9
    Path0209 => "geometry.ach.path02.09";
    /// Reach Path of Ice Rank 10
    Path0210 => "geometry.ach.path02.10";
    /// Unlock the Path of Poison
    Path0300 => "geometry.ach.path03.00";
    /// Reach Path of Poison Rank 1
    Path0301 => "geometry.ach.path03.01";
    /// Reach Path of Poison Rank 2
    Path0302 => "geometry.ach.path03.02";
    /// Reach Path of Poison Rank 3
    Path0303 => "geometry.ach.path03.03";
    /// Reach Path of Poison Rank 4
    Path0304 => "geometry.ach.path03.04";
    /// Reach Path of Poison Rank 5
    Path0305 => "geometry.ach.path03.05";
    /// Reach Path of Poison Rank 6
    Path0306 => "geometry.ach.path03.06";
    /// Reach Path of Poison Rank 7
    Path0307 => "geometry.ach.path03.07";
    /// Reach Path of Poison Rank 8
    Path0308 => "geometry.ach.path03.08";
    /// Reach Path of Poison Rank 9
    Path0309 => "geometry.ach.path03.09";
    /// Reach Path of Poison Rank 10
    Path0310 => "geometry.ach.path03.10";
    /// Unlock the Path of Shadow
    Path0400 => "geometry.ach.path04.00";
    /// Reach Path of Shadow Rank 1
    Path0401 => "geometry.ach.path04.01";
    /// Reach Path of Shadow Rank 2
    Path0402 => "geometry.ach.path04.02";
    /// Reach Path of Shadow Rank 3
    Path0403 => "geometry.ach.path04.03";
    /// Reach Path of Shadow Rank 4
    Path0404 => "geometry.ach.path04.04";
    /// Reach Path of Shadow Rank 5
    Path0405 => "geometry.ach.path04.05";
    /// Reach Path of Shadow Rank 6
    Path0406 => "geometry.ach.path04.06";
    /// Reach Path of Shadow Rank 7
    Path0407 => "geometry.ach.path04.07";
    /// Reach Path of Shadow Rank 8
    Path0408 => "geometry.ach.path04.08";
    /// Reach Path of Shadow Rank 9
    Path0409 => "geometry.ach.path04.09";
    /// Reach Path of Shadow Rank 10
    Path0410 => "geometry.ach.path04.10";
    /// Unlock the Path of Lava
    Path0500 => "geometry.ach.path05.00";
    /// Reach Path of Lava Rank 1
    Path0501 => "geometry.ach.path05.01";
    /// Reach Path of Lava Rank 2
    Path0502 => "geometry.ach.path05.02";
    /// Reach Path of Lava Rank 3
    Path0503 => "geometry.ach.path05.03";
    /// Reach Path of Lava Rank 4
    Path0504 => "geometry.ach.path05.04";
    /// Reach Path of Lava Rank 5
    Path0505 => "geometry.ach.path05.05";
    /// Reach Path of Lava Rank 6
    Path0506 => "geometry.ach.path05.06";
    /// Reach Path of Lava Rank 7
    Path0507 => "geometry.ach.path05.07";
    /// Reach Path of Lava Rank 8
    Path0508 => "geometry.ach.path05.08";
    /// Reach Path of Lava Rank 9
    Path0509 => "geometry.ach.path05.09";
    /// Reach Path of Lava Rank 10
    Path0510 => "geometry.ach.path05.10";
    /// Unlock the Path of Earth
    Path0600 => "geometry.ach.path06.00";
    /// Reach Path of Earth Rank 1
    Path0601 => "geometry.ach.path06.01";
    /// Reach Path of Earth Rank 2
    Path0602 => "geometry.ach.path06.02";
    /// Reach Path of Earth Rank 3
    Path0603 => "geometry.ach.path06.03";
    /// Reach Path of Earth Rank 4
    Path0604 => "geometry.ach.path06.04";
    /// Reach Path of Earth Rank 5
    Path0605 => "geometry.ach.path06.05";
    /// Reach Path of Earth Rank 6
    Path0606 => "geometry.ach.path06.06";
    /// Reach Path of Earth Rank 7
    Path0607 => "geometry.ach.path06.07";
    /// Reach Path of Earth Rank 8
    Path0608 => "geometry.ach.path06.08";
    /// Reach Path of Earth Rank 9
    Path0609 => "geometry.ach.path06.09";
    /// Reach Path of Earth Rank 10
    Path0610 => "geometry.ach.path06.10";
    /// Unlock the Path of Blood
    Path0700 => "geometry.ach.path07.00";
    /// Reach Path of Blood Rank 1
    Path0701 => "geometry.ach.path07.01";
    /// Reach Path of Blood Rank 2
    Path0702 => "geometry.ach.path07.02";
    /// Reach Path of Blood Rank 3
    Path0703 => "geometry.ach.path07.03";
    /// Reach Path of Blood Rank 4
    Path0704 => "geometry.ach.path07.04";
    /// Reach Path of Blood Rank 5
    Path0705 => "geometry.ach.path07.05";
    /// Reach Path of Blood Rank 6
    Path0706 => "geometry.ach.path07.06";
    /// Reach Path of Blood Rank 7
    Path0707 => "geometry.ach.path07.07";
    /// Reach Path of Blood Rank 8
    Path0708 => "geometry.ach.path07.08";
    /// Reach Path of Blood Rank 9
    Path0709 => "geometry.ach.path07.09";
    /// Reach Path of Blood Rank 10
    Path0710 => "geometry.ach.path07.10";
    /// Unlock the Path of Metal
    Path0800 => "geometry.ach.path08.00";
    /// Reach Path of Metal Rank 1
    Path0801 => "geometry.ach.path08.01";
    /// Reach Path of Metal Rank 2
    Path0802 => "geometry.ach.path08.02";
    /// Reach Path of Metal Rank 3
    Path0803 => "geometry.ach.path08.03";
    /// Reach Path of Metal Rank 4
    Path0804 => "geometry.ach.path08.04";
    /// Reach Path of Metal Rank 5
    Path0805 => "geometry.ach.path08.05";
    /// Reach Path of Metal Rank 6
    Path0806 => "geometry.ach.path08.06";
    /// Reach Path of Metal Rank 7
    Path0807 => "geometry.ach.path08.07";
    /// Reach Path of Metal Rank 8
    Path0808 => "geometry.ach.path08.08";
    /// Reach Path of Metal Rank 9
    Path0809 => "geometry.ach.path08.09";
    /// Reach Path of Metal Rank 10
    Path0810 => "geometry.ach.path08.10";
    /// Unlock the Path of Light
    Path0900 => "geometry.ach.path09.00";
    /// Reach Path of Light Rank 1
    Path0901 => "geometry.ach.path09.01";
    /// Reach Path of Light Rank 2
    Path0902 => "geometry.ach.path09.02";
    /// Reach Path of Light Rank 3
    Path0903 => "geometry.ach.path09.03";
    /// Reach Path of Light Rank 4
    Path0904 => "geometry.ach.path09.04";
    /// Reach Path of Light Rank 5
    Path0905 => "geometry.ach.path09.05";
    /// Reach Path of Light Rank 6
    Path0906 => "geometry.ach.path09.06";
    /// Reach Path of Light Rank 7
    Path0907 => "geometry.ach.path09.07";
    /// Reach Path of Light Rank 8
    Path0908 => "geometry.ach.path09.08";
    /// Reach Path of Light Rank 9
    Path0909 => "geometry.ach.path09.09";
    /// Reach Path of Light Rank 10
    Path0910 => "geometry.ach.path09.10";
    /// Unlock the Path of Souls
    Path1000 => "geometry.ach.path10.00";
    /// Reach Path of Souls Rank 1
    Path1001 => "geometry.ach.path10.01";
    /// Reach Path of Souls Rank 2
    Path1002 => "geometry.ach.path10.02";
    /// Reach Path of Souls Rank 3
    Path1003 => "geometry.ach.path10.03";
    /// Reach Path of Souls Rank 4
    Path1004 => "geometry.ach.path10.04";
    /// Reach Path of Souls Rank 5
    Path1005 => "geometry.ach.path10.05";
    /// Reach Path of Souls Rank 6
    Path1006 => "geometry.ach.path10.06";
    /// Reach Path of Souls Rank 7
    Path1007 => "geometry.ach.path10.07";
    /// Reach Path of Souls Rank 8
    Path1008 => "geometry.ach.path10.08";
    /// Reach Path of Souls Rank 9
    Path1009 => "geometry.ach.path10.09";
    /// Reach Path of Souls Rank 10
    Path1010 => "geometry.ach.path10.10";
    /// Destroy an icon on the main menu
    Secret01 => "geometry.ach.secret01";
    /// Destroy 50 icons on the main menu
    Secret02 => "geometry.ach.secret02";
    /// Destroy 100 icons on the main menu
    Secret02b => "geometry.ach.secret02b";
    /// Destroy 200 icons on the main menu
    Secret03 => "geometry.ach.secret03";
    /// Destroy 500 icons on the main menu
    Secret03b => "geometry.ach.secret03b";
    /// Found the hidden gold coin by scrolling through the level page 3 times
    Secret04 => "geometry.ach.secret04";
    /// Type 'lenny' into the 2.0 Vault
    Secret05 => "geometry.ach.secret05";
    /// Type 'sparky' into the 2.0 Vault
    Secret06 => "geometry.ach.secret06";
    /// Type 'spooky' into the 2.0 Vault
    Secret07 => "geometry.ach.secret07";
    /// Type 'blockbite' into the 2.0 Vault
    Secret08 => "geometry.ach.secret08";
    /// Type 'robotop' into the 2.0 Vault
    Secret09 => "geometry.ach.secret09";
    /// Type 'ahead' into the 2.0 Vault
    Secret10 => "geometry.ach.secret10";
    /// Destroy the 55th cube on the main menu
    Secret11 => "geometry.ach.secret11";
    /// Destroy the 50th cube on the main menu
    Secret12 => "geometry.ach.secret12";
    /// Type 'mule' into the 2.0 Vault
    Secret13 => "geometry.ach.secret13";
    /// Type 'neverending' into the 2.0 Vault
    Secret14 => "geometry.ach.secret14";
    /// Type 'gandalfpotter' into the 2.0 Vault
    Secret15 => "geometry.ach.secret15";
    /// Consecutively type '8', '16', '30', '32', '46' and '84' into the 2.0 Vault
    Secret16 => "geometry.ach.secret16";
    /// Type your username into the 2.0 Vault
    Secret17 => "geometry.ach.secret17";
    /// Destroy 750 icons on the main menu
    Secret18 => "geometry.ach.secret18";
    /// Type 'finalboss' into the 2.0 Vault
    Secret19 => "geometry.ach.secret19";
    /// Type 'brainpower' into the Vault of Secrets
    V2Secret01 => "geometry.ach.v2.secret01";
    /// Type 'cod3breaker' and the solution to the puzzle into the Vault of Secrets
    V2Secret02 => "geometry.ach.v2.secret02";
    /// Solve the 'glubfub' puzzle
    V2Secret03 => "geometry.ach.v2.secret03";
    /// Type 'octocube' into the Vault of Secrets
    V2Secret04 => "geometry.ach.v2.secret04";
    /// Type your star count into the Vault of Secrets
    V2Secret05 => "geometry.ach.v2.secret05";
    /// Type 'seven' into the Vault of Secrets
    V2Secret06 => "geometry.ach.v2.secret06";
    /// Type 'gimmiethecolor' into the Vault of Secrets
    V2Secret07 => "geometry.ach.v2.secret07";
    /// Type 'thechickenisonfire' into the Vault of Secrets
    V2Secret08 => "geometry.ach.v2.secret08";
    /// Type 'd4shg30me7ry' into the Vault of Secrets
    V2Secret09 => "geometry.ach.v2.secret09";
    /// Type 'thechickenisready' into the Vault of Secrets
    V2Secret10 => "geometry.ach.v2.secret10";
    /// Type 'darkness' into the Chamber of Time
    V3Secret01 => "geometry.ach.v3.secret01";
    /// Type 'silence' into the Chamber of Time
    V3Secret02 => "geometry.ach.v3.secret02";
    /// Type 'river' into the Chamber of Time
    V3Secret03 => "geometry.ach.v3.secret03";
    /// Type 'hunger' into the Chamber of Time
    V3Secret04 => "geometry.ach.v3.secret04";
    /// Type 'volcano' into the Chamber of Time
    V3Secret05 => "geometry.ach.v3.secret05";
    /// Type 'backontrack' into the Chamber of Time
    V3Secret06 => "geometry.ach.v3.secret06";
    /// Type 'givemehelper' into the Chamber of Time
    V3Secret07 => "geometry.ach.v3.secret07";
}
