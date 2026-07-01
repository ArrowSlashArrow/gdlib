//! All achievements in GD

use crate::repr_t;

pub(crate) const MAX_ACHIEVEMENT_INDEX: usize = 537;

repr_t!(
    /// All vanilla achievements in GD. All names were imported as-is, then captialized and stripped of spaces and periods with no further processing.
    ///
    /// Each variant has a repr value of the index that it will occupy in CCGameManager.stats.achievements.
    ///
    /// All achievement documentation was obtained from Wylie's docs: <https://wyliemaster.github.io/gddocs/#/resources/client/gamesave/achievement>
    strict GDAchievement: i32 {
        /// Beat Stereo Madness in normal Mode
        Level01b = 1,
        /// Beat Stereo Madness in practice Mode
        Level01a = 2,
        /// Beat Back On Track in normal Mode
        Level02b = 3,
        /// Beat Back On Track in practice Mode
        Level02a = 4,
        /// Beat Polargeist in normal Mode
        Level03b = 5,
        /// Beat Polargeist in practice Mode
        Level03a = 6,
        /// Beat Dry Out in normal Mode
        Level04b = 7,
        /// Beat Dry Out in practice Mode
        Level04a = 8,
        /// Beat Base after Base in normal Mode
        Level05b = 9,
        /// Beat Base After Base in practice Mode
        Level05a = 10,
        /// Beat Cant Let Go in normal Mode
        Level06b = 11,
        /// Beat Cant Let Go in practice Mode
        Level06a = 12,
        /// Beat Jumper in normal Mode
        Level07b = 13,
        /// Beat Jumper in practice Mode
        Level07a = 14,
        /// Beat Time Machine in normal Mode
        Level08b = 15,
        /// Beat Time Machine in practice Mode
        Level08a = 16,
        /// Beat Cycles in normal Mode
        Level09b = 17,
        /// Beat Cycles in practice Mode
        Level09a = 18,
        /// Beat Xstep in normal Mode
        Level10b = 19,
        /// Beat Xstep in practice Mode
        Level10a = 20,
        /// Beat Clutterfunk in normal Mode
        Level11b = 21,
        /// Beat Clutterfunk in practice Mode
        Level11a = 22,
        /// Beat Theory Of Everything in normal Mode
        Level12b = 23,
        /// Beat Theory Of Everything in practice Mode
        Level12a = 24,
        /// Beat Electroman Adventures in normal Mode
        Level13b = 25,
        /// Beat Electroman Adventures in practice Mode
        Level13a = 26,
        /// Beat Clubstep in normal Mode
        Level14b = 27,
        /// Beat Clubstep in practice Mode
        Level14a = 28,
        /// Beat Electrodynamix in normal Mode
        Level15b = 29,
        /// Beat Electrodynamix in practice Mode
        Level15a = 30,
        /// Beat Hexagon Force in normal Mode
        Level16b = 31,
        /// Beat Hexagon Forece in practice Mode
        Level16a = 32,
        /// Beat Blast Processing in normal Mode
        Level17b = 33,
        /// Beat Blast Processing in practice Mode
        Level17a = 34,
        /// Beat Theory Of Everything 2 in normal Mode
        Level18b = 35,
        /// Beat Theory Of Everything 2 in practice Mode
        Level18a = 36,
        /// Beat Geometrical Dominator in normal Mode
        Level19b = 37,
        /// Beat Geometrical Dominator in practice Mode
        Level19a = 38,
        /// Beat Deadlocked in normal Mode
        Level20b = 39,
        /// Beat Deadlocked in practice Mode
        Level20a = 40,
        /// Beat Fingerdash in normal Mode
        Level21b = 41,
        /// Beat Fingerdash in practice Mode
        Level21a = 42,
        /// Beat Dash in normal Mode
        Level22b = 43,
        /// Beat Dash in practice Mode
        Level22a = 44,
        /// Beat The Tower
        Tower01 = 45,
        /// Beat The Sewers
        Tower02 = 46,
        /// Beat The Cellar
        Tower03 = 47,
        /// Beat The Secret Hollow
        Tower04 = 48,
        /// Beat The Tower with all 3 Coins
        Tower01Coin = 49,
        /// Beat The Sewers with all 3 Coins
        Tower02Coin = 50,
        /// Beat The Cellar with all 3 Coins
        Tower03Coin = 51,
        /// Beat The Secret Hollow with all 3 Coins
        Tower04Coin = 52,
        /// Beat Stereo Madness, Back On Track and Polargeist in normal mode (steam exclusive)
        Steam01 = 53,
        /// Beat Clubstep in normal mode (steam exclusive)
        Steam02 = 54,
        /// Beat Clubstep in with all coins
        Demoncoin01 = 55,
        /// Beat Theory Of Everything 2 in with all coins
        Demoncoin02 = 56,
        /// Beat Deadlocked in with all coins
        Demoncoin03 = 57,
        /// Complete 1 Map Pack
        Mappacks01 = 58,
        /// Complete 5 Map Packs
        Mappacks02 = 59,
        /// Complete 10 Map Packs
        Mappacks03 = 60,
        /// Complete 15 Map Packs
        Mappacks04 = 61,
        /// Complete 20 Map Packs
        Mappacks05 = 62,
        /// Complete 25 Map Packs
        Mappacks06 = 63,
        /// Complete 30 Map Packs
        Mappacks07 = 64,
        /// Complete 35 Map Packs
        Mappacks08 = 65,
        /// Complete 40 Map Packs
        Mappacks09 = 66,
        /// Complete 45 Map Packs
        Mappacks10 = 67,
        /// Complete 1 Gauntlet
        Gauntlets01 = 68,
        /// Complete 5 Gauntlets
        Gauntlets02 = 69,
        /// Complete 10 Gauntlets
        Gauntlets03 = 70,
        /// Complete 15 Gauntlets
        Gauntlets04 = 71,
        /// Complete a Daily Level
        Daily01 = 72,
        /// Complete 25 Daily Levels
        Daily02 = 73,
        /// Complete 50 Daily Levels
        Daily03 = 74,
        /// Complete 100 Daily Levels
        Daily04 = 75,
        /// Complete 150 Daily Levels
        Daily05 = 76,
        /// Complete 250 Daily Levels
        Daily06 = 77,
        /// Complete 365 Daily Levels
        Daily07 = 78,
        /// Complete 1 User Created Level in Normal Mode
        Custom01 = 79,
        /// Complete 10 User Created Levels in Normal Mode
        Custom02 = 80,
        /// Complete 50 User Created Levels in Normal Mode
        Custom03 = 81,
        /// Complete 100 User Created Levels in Normal Mode
        Custom04 = 82,
        /// Complete 200 User Created Levels in Normal Mode
        Custom05 = 83,
        /// Complete 300 User Created Levels in Normal Mode
        Custom06 = 84,
        /// Complete 500 User Created Levels in Normal Mode
        Custom07 = 85,
        /// Complete 1,000 User Created Levels in Normal Mode
        Custom08 = 86,
        /// Complete 1,500 User Created Levels in Normal Mode
        Custom09 = 87,
        /// Complete 2,000 User Created Levels in Normal Mode
        Custom10 = 88,
        /// Complete 2,500 User Created Levels in Normal Mode
        Custom11 = 89,
        /// Complete 3,000 User Created Levels in Normal Mode
        Custom12 = 90,
        /// Complete 4,000 User Created Levels in Normal Mode
        Custom13 = 91,
        /// Complete 5,000 User Created Levels in Normal Mode
        Custom14 = 92,
        /// Collect 100 stars
        Stars01 = 93,
        /// Collect 200 stars
        Stars02 = 94,
        /// Collect 300 stars
        Stars03 = 95,
        /// Collect 400 stars
        Stars04 = 96,
        /// Collect 500 stars
        Stars05 = 97,
        /// Collect 600 stars
        Stars06 = 98,
        /// Collect 700 stars
        Stars07 = 99,
        /// Collect 800 stars
        Stars08 = 100,
        /// Collect 900 stars
        Stars09 = 101,
        /// Collect 1,000 stars
        Stars10 = 102,
        /// Collect 1,500 stars
        Stars11 = 103,
        /// Collect 2,000 stars
        Stars12 = 104,
        /// Collect 2,500 stars
        Stars13 = 105,
        /// Collect 3,000 stars
        Stars14 = 106,
        /// Collect 3,500 stars
        Stars15 = 107,
        /// Collect 4,000 stars
        Stars16 = 108,
        /// Collect 4,500 stars
        Stars17 = 109,
        /// Collect 5,000 stars
        Stars18 = 110,
        /// Collect 5,500 stars
        Stars19 = 111,
        /// Collect 6,000 stars
        Stars20 = 112,
        /// Collect 6,500 stars
        Stars21 = 113,
        /// Collect 7,000 stars
        Stars22 = 114,
        /// Collect 7,500 stars
        Stars23 = 115,
        /// Collect 8,000 stars
        Stars24 = 116,
        /// Collect 9,000 stars
        Stars25 = 117,
        /// Collect 10,000 stars
        Stars26 = 118,
        /// Collect 12,000 stars
        Stars27 = 119,
        /// Collect 14,000 stars
        Stars28 = 120,
        /// Collect 16,000 stars
        Stars29 = 121,
        /// Collect 18,000 stars
        Stars30 = 122,
        /// Collect 20,000 stars
        Stars31 = 123,
        /// Collect 22,500 stars
        Stars32 = 124,
        /// Collect 25,000 stars
        Stars33 = 125,
        /// Collect 100 moons
        Moons01 = 126,
        /// Collect 200 moons
        Moons02 = 127,
        /// Collect 300 moons
        Moons03 = 128,
        /// Collect 400 moons
        Moons04 = 129,
        /// Collect 500 moons
        Moons05 = 130,
        /// Collect 600 moons
        Moons06 = 131,
        /// Collect 700 moons
        Moons07 = 132,
        /// Collect 800 moons
        Moons08 = 133,
        /// Collect 900 moons
        Moons09 = 134,
        /// Collect 1,000 moons
        Moons10 = 135,
        /// Collect 1,500 moons
        Moons11 = 136,
        /// Collect 2,000 moons
        Moons12 = 137,
        /// Collect 2,500 moons
        Moons13 = 138,
        /// Collect 3,000 moons
        Moons14 = 139,
        /// Collect 3,500 moons
        Moons15 = 140,
        /// Collect 4,000 moons
        Moons16 = 141,
        /// Collect 4,500 moons
        Moons17 = 142,
        /// Collect 5,000 moons
        Moons18 = 143,
        /// Collect 5,500 moons
        Moons19 = 144,
        /// Collect 6,000 moons
        Moons20 = 145,
        /// Collect 6,500 moons
        Moons21 = 146,
        /// Collect 7,000 moons
        Moons22 = 147,
        /// Collect 7,500 moons
        Moons23 = 148,
        /// Collect 8,000 moons
        Moons24 = 149,
        /// Collect 9,000 moons
        Moons25 = 150,
        /// Collect 10,000 moons
        Moons26 = 151,
        /// Beat 1 Demon
        Demon01 = 152,
        /// Beat 2 Demons
        Demon02 = 153,
        /// Beat 3 Demons
        Demon03 = 154,
        /// Beat 4 Demons
        Demon04 = 155,
        /// Beat 5 Demons
        Demon05 = 156,
        /// Beat 10 Demons
        Demon06 = 157,
        /// Beat 15 Demons
        Demon07 = 158,
        /// Beat 20 Demons
        Demon08 = 159,
        /// Beat 30 Demons
        Demon09 = 160,
        /// Beat 40 Demons
        Demon10 = 161,
        /// Beat 50 Demons
        Demon11 = 162,
        /// Beat 60 Demons
        Demon12 = 163,
        /// Beat 70 Demons
        Demon13 = 164,
        /// Beat 80 Demons
        Demon14 = 165,
        /// Beat 90 Demons
        Demon15 = 166,
        /// Beat 100 Demons
        Demon16 = 167,
        /// Beat 120 Demons
        Demon17 = 168,
        /// Beat 140 Demons
        Demon18 = 169,
        /// Beat 160 Demons
        Demon19 = 170,
        /// Beat 180 Demons
        Demon20 = 171,
        /// Beat 200 Demons
        Demon21 = 172,
        /// Beat 225 Demons
        Demon22 = 173,
        /// Beat 250 Demons
        Demon23 = 174,
        /// Beat 275 Demons
        Demon24 = 175,
        /// Beat 300 Demons
        Demon25 = 176,
        /// Beat 350 Demons
        Demon26 = 177,
        /// Beat 400 Demons
        Demon27 = 178,
        /// Beat 500 Demons
        Demon28 = 179,
        /// Collect 5 Secret Coins
        Coins01 = 180,
        /// Collect 10 Secret Coins
        Coins02 = 181,
        /// Collect 15 Secret Coins
        Coins03 = 182,
        /// Collect 20 Secret Coins
        Coins04 = 183,
        /// Collect 25 Secret Coins
        Coins05 = 184,
        /// Collect 30 Secret Coins
        Coins06 = 185,
        /// Collect 35 Secret Coins
        Coins07 = 186,
        /// Collect 40 Secret Coins
        Coins08 = 187,
        /// Collect 45 Secret Coins
        Coins09 = 188,
        /// Collect 50 Secret Coins
        Coins10 = 189,
        /// Collect 55 Secret Coins
        Coins11 = 190,
        /// Collect 60 Secret Coins
        Coins12 = 191,
        /// Collect 65 Secret Coins
        Coins13 = 192,
        /// Collect 70 Secret Coins
        Coins14 = 193,
        /// Collect 75 Secret Coins
        Coins15 = 194,
        /// Collect 80 Secret Coins
        Coins16 = 195,
        /// Collect 85 Secret Coins
        Coins17 = 196,
        /// Collect 90 Secret Coins
        Coins18 = 197,
        /// Collect 95 Secret Coins
        Coins19 = 198,
        /// Collect 100 Secret Coins
        Coins20 = 199,
        /// Collect 105 Secret Coins
        Coins21 = 200,
        /// Collect 110 Secret Coins
        Coins22 = 201,
        /// Collect 115 Secret Coins
        Coins23 = 202,
        /// Collect 120 Secret Coins
        Coins24 = 203,
        /// Collect 130 Secret Coins
        Coins25 = 204,
        /// Collect 140 Secret Coins
        Coins26 = 205,
        /// Collect 150 Secret Coins
        Coins27 = 206,
        /// Collect 160 Secret Coins
        Coins28 = 207,
        /// Collect 1 user Coin
        Usercoins01 = 208,
        /// Collect 10 user Coins
        Usercoins02 = 209,
        /// Collect 20 user Coins
        Usercoins03 = 210,
        /// Collect 30 user Coins
        Usercoins04 = 211,
        /// Collect 40 user Coins
        Usercoins05 = 212,
        /// Collect 50 user Coins
        Usercoins06 = 213,
        /// Collect 60 user Coins
        Usercoins07 = 214,
        /// Collect 70 user Coins
        Usercoins08 = 215,
        /// Collect 80 user Coins
        Usercoins09 = 216,
        /// Collect 90 user Coins
        Usercoins10 = 217,
        /// Collect 100 user Coins
        Usercoins11 = 218,
        /// Collect 110 user Coins
        Usercoins12 = 219,
        /// Collect 120 user Coins
        Usercoins13 = 220,
        /// Collect 130 user Coins
        Usercoins14 = 221,
        /// Collect 140 user Coins
        Usercoins15 = 222,
        /// Collect 150 user Coins
        Usercoins16 = 223,
        /// Collect 160 user Coins
        Usercoins17 = 224,
        /// Collect 170 user Coins
        Usercoins18 = 225,
        /// Collect 180 user Coins
        Usercoins19 = 226,
        /// Collect 190 user Coins
        Usercoins20 = 227,
        /// Collect 200 user Coins
        Usercoins21 = 228,
        /// Collect 225 user Coins
        Usercoins22 = 229,
        /// Collect 250 user Coins
        Usercoins23 = 230,
        /// Collect 300 user Coins
        Usercoins24 = 231,
        /// Collect 350 user Coins
        Usercoins25 = 232,
        /// Collect 425 user Coins
        Usercoins26 = 233,
        /// Collect 500 user Coins
        Usercoins27 = 234,
        /// Collect 600 user Coins
        Usercoins28 = 235,
        /// Collect 700 user Coins
        Usercoins29 = 236,
        /// Collect 800 user Coins
        Usercoins30 = 237,
        /// Collect 900 user Coins
        Usercoins31 = 238,
        /// Collect 1,000 user Coins
        Usercoins32 = 239,
        /// Collect 1,200 user Coins
        Usercoins33 = 240,
        /// Collect 1,400 user Coins
        Usercoins34 = 241,
        /// Collect 1,600 user Coins
        Usercoins35 = 242,
        /// Collect 1,800 user Coins
        Usercoins36 = 243,
        /// Collect 2,000 user Coins
        Usercoins37 = 244,
        /// Collect 2,300 user Coins
        Usercoins38 = 245,
        /// Collect 2,600 user Coins
        Usercoins39 = 246,
        /// Collect 3,000 user Coins
        Usercoins40 = 247,
        /// Collect 100 Diamonds
        Diamonds01 = 248,
        /// Collect 250 Diamonds
        Diamonds02 = 249,
        /// Collect 500 Diamonds
        Diamonds03 = 250,
        /// Collect 1,000 Diamonds
        Diamonds04 = 251,
        /// Collect 1,500 Diamonds
        Diamonds05 = 252,
        /// Collect 2,000 Diamonds
        Diamonds06 = 253,
        /// Collect 2,500 Diamonds
        Diamonds07 = 254,
        /// Collect 3,000 Diamonds
        Diamonds08 = 255,
        /// Collect 4,000 Diamonds
        Diamonds09 = 256,
        /// Collect 5,000 Diamonds
        Diamonds10 = 257,
        /// Collect 6,000 Diamonds
        Diamonds11 = 258,
        /// Collect 7,000 Diamonds
        Diamonds12 = 259,
        /// Collect 9,000 Diamonds
        Diamonds13 = 260,
        /// Collect 11,000 Diamonds
        Diamonds14 = 261,
        /// Collect 14,000 Diamonds
        Diamonds15 = 262,
        /// Collect 17,000 Diamonds
        Diamonds16 = 263,
        /// Collect 20,000 Diamonds
        Diamonds17 = 264,
        /// Collect 25,000 Diamonds
        Diamonds18 = 265,
        /// Claim a List Reward
        Lists01 = 266,
        /// Claim 25 List Rewards
        Lists02 = 267,
        /// Claim 50 List Rewards
        Lists03 = 268,
        /// Claim 100 List Rewards
        Lists04 = 269,
        /// Claim 150 List Rewards
        Lists05 = 270,
        /// Claim 200 List Rewards
        Lists06 = 271,
        /// Collect 5 Fire Shards
        ShardFire01 = 272,
        /// Collect 15 Fire Shards
        ShardFire02 = 273,
        /// Collect 35 Fire Shards
        ShardFire03 = 274,
        /// Collect 65 Fire Shards
        ShardFire04 = 275,
        /// Collect 100 Fire Shards
        ShardFire05 = 276,
        /// Collect 5 Ice Shards
        ShardIce01 = 277,
        /// Collect 15 Ice Shards
        ShardIce02 = 278,
        /// Collect 35 Ice Shards
        ShardIce03 = 279,
        /// Collect 65 Ice Shards
        ShardIce04 = 280,
        /// Collect 100 Ice Shards
        ShardIce05 = 281,
        /// Collect 5 Poison Shards
        ShardPoison01 = 282,
        /// Collect 15 Poison Shards
        ShardPoison02 = 283,
        /// Collect 35 Poison Shards
        ShardPoison03 = 284,
        /// Collect 65 Poison Shards
        ShardPoison04 = 285,
        /// Collect 100 Poison Shards
        ShardPoison05 = 286,
        /// Collect 5 Shadow Shards
        ShardShadow01 = 287,
        /// Collect 15 Shadow Shards
        ShardShadow02 = 288,
        /// Collect 35 Shadow Shards
        ShardShadow03 = 289,
        /// Collect 65 Shadow Shards
        ShardShadow04 = 290,
        /// Collect 100 Shadow Shards
        ShardShadow05 = 291,
        /// Collect 5 Lava Shards
        ShardLava01 = 292,
        /// Collect 15 Lava Shards
        ShardLava02 = 293,
        /// Collect 35 Lava Shards
        ShardLava03 = 294,
        /// Collect 65 Lava Shards
        ShardLava04 = 295,
        /// Collect 100 Lava Shards
        ShardLava05 = 296,
        /// Collect 5 of each Tier 1 Shard
        ShardBonus01 = 297,
        /// Collect 15 of each Tier 1 Shard
        ShardBonus02 = 298,
        /// Collect 35 of each Tier 1 Shard
        ShardBonus03 = 299,
        /// Collect 65 of each Tier 1 Shard
        ShardBonus04 = 300,
        /// Collect 100 of each Tier 1 Shard
        ShardBonus05 = 301,
        /// Collect 5 Earth Shards
        ShardEarth01 = 302,
        /// Collect 15 Earth Shards
        ShardEarth02 = 303,
        /// Collect 35 Earth Shards
        ShardEarth03 = 304,
        /// Collect 65 Earth Shards
        ShardEarth04 = 305,
        /// Collect 100 Earth Shards
        ShardEarth05 = 306,
        /// Collect 5 Blood Shards
        ShardBlood01 = 307,
        /// Collect 15 Blood Shards
        ShardBlood02 = 308,
        /// Collect 35 Blood Shards
        ShardBlood03 = 309,
        /// Collect 65 Blood Shards
        ShardBlood04 = 310,
        /// Collect 100 Blood Shards
        ShardBlood05 = 311,
        /// Collect 5 Metal Shards
        ShardMetal01 = 312,
        /// Collect 15 Metal Shards
        ShardMetal02 = 313,
        /// Collect 35 Metal Shards
        ShardMetal03 = 314,
        /// Collect 65 Metal Shards
        ShardMetal04 = 315,
        /// Collect 100 Metal Shards
        ShardMetal05 = 316,
        /// Collect 5 Light Shards
        ShardLight01 = 317,
        /// Collect 15 Light Shards
        ShardLight02 = 318,
        /// Collect 35 Light Shards
        ShardLight03 = 319,
        /// Collect 65 Light Shards
        ShardLight04 = 320,
        /// Collect 100 Light Shards
        ShardLight05 = 321,
        /// Collect 5 Soul Shards
        ShardSoul01 = 322,
        /// Collect 15 Soul Shards
        ShardSoul02 = 323,
        /// Collect 35 Soul Shards
        ShardSoul03 = 324,
        /// Collect 65 Soul Shards
        ShardSoul04 = 325,
        /// Collect 100 Soul Shards
        ShardSoul05 = 326,
        /// Collect 5 of each Tier 2 Shard
        ShardBonusB01 = 327,
        /// Collect 15 of each Tier 2 Shard
        ShardBonusB02 = 328,
        /// Collect 35 of each Tier 2 Shard
        ShardBonusB03 = 329,
        /// Collect 65 of each Tier 2 Shard
        ShardBonusB04 = 330,
        /// Collect 100 of each Tier 2 Shard
        ShardBonusB05 = 331,
        /// Follow 1 Creator
        FollowCreator = 332,
        /// Follow 10 Creators
        FollowCreator2 = 333,
        /// Follow 25 Creators
        FollowCreator3 = 334,
        /// Follow 50 Creators
        FollowCreator4 = 335,
        /// Befriend 1 user
        Friends01 = 336,
        /// Befriend 10 users
        Friends02 = 337,
        /// Befriend 25 users
        Friends03 = 338,
        /// Befriend 50 users
        Friends04 = 339,
        /// subscribe to RobTop on YouTube
        Youtube = 340,
        /// Follow RobTop on Twitter
        Twitter = 341,
        /// Like RobTop on Facebook
        Facebook = 342,
        /// Do 100 Attempts
        Attempt01 = 343,
        /// Do 500 Attempts
        Attempt02 = 344,
        /// Do 2,000 Attempts
        Attempt03 = 345,
        /// Do 10,000 Attempts
        Attempt04 = 346,
        /// Do 20,000 Attempts
        Attempt05 = 347,
        /// Do 30,000 Attempts
        Attempt06 = 348,
        /// Do 40,000 Attempts
        Attempt07 = 349,
        /// Do 60,000 Attempts
        Attempt08 = 350,
        /// Do 80,000 Attempts
        Attempt09 = 351,
        /// Do 100,000 Attempts
        Attempt10 = 352,
        /// Do 135,000 Attempts
        Attempt11 = 353,
        /// Do 185,000 Attempts
        Attempt12 = 354,
        /// Do 250,000 Attempts
        Attempt13 = 355,
        /// Do 300,000 Attempts
        Attempt14 = 356,
        /// Jump 1,000 Times
        Jump01 = 357,
        /// Jump 10,000 Times
        Jump02 = 358,
        /// Jump 20,000 Times
        Jump03 = 359,
        /// Jump 50,000 Times
        Jump04 = 360,
        /// Jump 100,000 Times
        Jump05 = 361,
        /// Jump 200,000 Times
        Jump06 = 362,
        /// Jump 300,000 Times
        Jump07 = 363,
        /// Jump 400,000 Times
        Jump08 = 364,
        /// Jump 500,000 Times
        Jump09 = 365,
        /// Jump 750,000 Times
        Jump10 = 366,
        /// Jump 1,000,000 Times
        Jump11 = 367,
        /// Submit an online level
        Submit = 368,
        /// Click the supporter icon when it's locked
        Rate = 369,
        /// Rate the stars of an online level
        RateDiff = 370,
        /// Rate the stars of 100 online levels
        RateDiff02 = 371,
        /// Rate the stars of 500 online levels
        RateDiff02b = 372,
        /// Rate the stars of 1,000 online levels
        RateDiff03 = 373,
        /// Rate the stars of 2,000 online levels
        RateDiff04 = 374,
        /// Like or dislike an online level
        Like = 375,
        /// Like or dislike 100 online levels
        Like02 = 376,
        /// Like or dislike 500 online levels
        Like02b = 377,
        /// Like or dislike 1,000 online levels
        Like03 = 378,
        /// Like or dislike 2,000 online levels
        Like04 = 379,
        /// Like or dislike 3,000 online levels
        Like05 = 380,
        /// Like or dislike 4,000 online levels
        Like06 = 381,
        /// Click the "More Games" button
        MoreGames = 382,
        /// Die at over 95% on a main level
        Special01 = 383,
        /// Get 100 likes on your level
        Creator01 = 384,
        /// Get a star rate on your level
        Creator02 = 385,
        /// Get 50 likes on your level
        Creator03 = 386,
        /// Get 250 likes on your level
        Creator04 = 387,
        /// Get 500 likes on your level
        Creator05 = 388,
        /// Get 1,000 likes on your level
        Creator06 = 389,
        /// Unlock the Path of Fire
        Path0100 = 390,
        /// Reach Path of Fire Rank 1
        Path0101 = 391,
        /// Reach Path of Fire Rank 2
        Path0102 = 392,
        /// Reach Path of Fire Rank 3
        Path0103 = 393,
        /// Reach Path of Fire Rank 4
        Path0104 = 394,
        /// Reach Path of Fire Rank 5
        Path0105 = 395,
        /// Reach Path of Fire Rank 6
        Path0106 = 396,
        /// Reach Path of Fire Rank 7
        Path0107 = 397,
        /// Reach Path of Fire Rank 8
        Path0108 = 398,
        /// Reach Path of Fire Rank 9
        Path0109 = 399,
        /// Reach Path of Fire Rank 10
        Path0110 = 400,
        /// Unlock the Path of Ice
        Path0200 = 401,
        /// Reach Path of Ice Rank 1
        Path0201 = 402,
        /// Reach Path of Ice Rank 2
        Path0202 = 403,
        /// Reach Path of Ice Rank 3
        Path0203 = 404,
        /// Reach Path of Ice Rank 4
        Path0204 = 405,
        /// Reach Path of Ice Rank 5
        Path0205 = 406,
        /// Reach Path of Ice Rank 6
        Path0206 = 407,
        /// Reach Path of Ice Rank 7
        Path0207 = 408,
        /// Reach Path of Ice Rank 8
        Path0208 = 409,
        /// Reach Path of Ice Rank 9
        Path0209 = 410,
        /// Reach Path of Ice Rank 10
        Path0210 = 411,
        /// Unlock the Path of Poison
        Path0300 = 412,
        /// Reach Path of Poison Rank 1
        Path0301 = 413,
        /// Reach Path of Poison Rank 2
        Path0302 = 414,
        /// Reach Path of Poison Rank 3
        Path0303 = 415,
        /// Reach Path of Poison Rank 4
        Path0304 = 416,
        /// Reach Path of Poison Rank 5
        Path0305 = 417,
        /// Reach Path of Poison Rank 6
        Path0306 = 418,
        /// Reach Path of Poison Rank 7
        Path0307 = 419,
        /// Reach Path of Poison Rank 8
        Path0308 = 420,
        /// Reach Path of Poison Rank 9
        Path0309 = 421,
        /// Reach Path of Poison Rank 10
        Path0310 = 422,
        /// Unlock the Path of Shadow
        Path0400 = 423,
        /// Reach Path of Shadow Rank 1
        Path0401 = 424,
        /// Reach Path of Shadow Rank 2
        Path0402 = 425,
        /// Reach Path of Shadow Rank 3
        Path0403 = 426,
        /// Reach Path of Shadow Rank 4
        Path0404 = 427,
        /// Reach Path of Shadow Rank 5
        Path0405 = 428,
        /// Reach Path of Shadow Rank 6
        Path0406 = 429,
        /// Reach Path of Shadow Rank 7
        Path0407 = 430,
        /// Reach Path of Shadow Rank 8
        Path0408 = 431,
        /// Reach Path of Shadow Rank 9
        Path0409 = 432,
        /// Reach Path of Shadow Rank 10
        Path0410 = 433,
        /// Unlock the Path of Lava
        Path0500 = 434,
        /// Reach Path of Lava Rank 1
        Path0501 = 435,
        /// Reach Path of Lava Rank 2
        Path0502 = 436,
        /// Reach Path of Lava Rank 3
        Path0503 = 437,
        /// Reach Path of Lava Rank 4
        Path0504 = 438,
        /// Reach Path of Lava Rank 5
        Path0505 = 439,
        /// Reach Path of Lava Rank 6
        Path0506 = 440,
        /// Reach Path of Lava Rank 7
        Path0507 = 441,
        /// Reach Path of Lava Rank 8
        Path0508 = 442,
        /// Reach Path of Lava Rank 9
        Path0509 = 443,
        /// Reach Path of Lava Rank 10
        Path0510 = 444,
        /// Unlock the Path of Earth
        Path0600 = 445,
        /// Reach Path of Earth Rank 1
        Path0601 = 446,
        /// Reach Path of Earth Rank 2
        Path0602 = 447,
        /// Reach Path of Earth Rank 3
        Path0603 = 448,
        /// Reach Path of Earth Rank 4
        Path0604 = 449,
        /// Reach Path of Earth Rank 5
        Path0605 = 450,
        /// Reach Path of Earth Rank 6
        Path0606 = 451,
        /// Reach Path of Earth Rank 7
        Path0607 = 452,
        /// Reach Path of Earth Rank 8
        Path0608 = 453,
        /// Reach Path of Earth Rank 9
        Path0609 = 454,
        /// Reach Path of Earth Rank 10
        Path0610 = 455,
        /// Unlock the Path of Blood
        Path0700 = 456,
        /// Reach Path of Blood Rank 1
        Path0701 = 457,
        /// Reach Path of Blood Rank 2
        Path0702 = 458,
        /// Reach Path of Blood Rank 3
        Path0703 = 459,
        /// Reach Path of Blood Rank 4
        Path0704 = 460,
        /// Reach Path of Blood Rank 5
        Path0705 = 461,
        /// Reach Path of Blood Rank 6
        Path0706 = 462,
        /// Reach Path of Blood Rank 7
        Path0707 = 463,
        /// Reach Path of Blood Rank 8
        Path0708 = 464,
        /// Reach Path of Blood Rank 9
        Path0709 = 465,
        /// Reach Path of Blood Rank 10
        Path0710 = 466,
        /// Unlock the Path of Metal
        Path0800 = 467,
        /// Reach Path of Metal Rank 1
        Path0801 = 468,
        /// Reach Path of Metal Rank 2
        Path0802 = 469,
        /// Reach Path of Metal Rank 3
        Path0803 = 470,
        /// Reach Path of Metal Rank 4
        Path0804 = 471,
        /// Reach Path of Metal Rank 5
        Path0805 = 472,
        /// Reach Path of Metal Rank 6
        Path0806 = 473,
        /// Reach Path of Metal Rank 7
        Path0807 = 474,
        /// Reach Path of Metal Rank 8
        Path0808 = 475,
        /// Reach Path of Metal Rank 9
        Path0809 = 476,
        /// Reach Path of Metal Rank 10
        Path0810 = 477,
        /// Unlock the Path of Light
        Path0900 = 478,
        /// Reach Path of Light Rank 1
        Path0901 = 479,
        /// Reach Path of Light Rank 2
        Path0902 = 480,
        /// Reach Path of Light Rank 3
        Path0903 = 481,
        /// Reach Path of Light Rank 4
        Path0904 = 482,
        /// Reach Path of Light Rank 5
        Path0905 = 483,
        /// Reach Path of Light Rank 6
        Path0906 = 484,
        /// Reach Path of Light Rank 7
        Path0907 = 485,
        /// Reach Path of Light Rank 8
        Path0908 = 486,
        /// Reach Path of Light Rank 9
        Path0909 = 487,
        /// Reach Path of Light Rank 10
        Path0910 = 488,
        /// Unlock the Path of Souls
        Path1000 = 489,
        /// Reach Path of Souls Rank 1
        Path1001 = 490,
        /// Reach Path of Souls Rank 2
        Path1002 = 491,
        /// Reach Path of Souls Rank 3
        Path1003 = 492,
        /// Reach Path of Souls Rank 4
        Path1004 = 493,
        /// Reach Path of Souls Rank 5
        Path1005 = 494,
        /// Reach Path of Souls Rank 6
        Path1006 = 495,
        /// Reach Path of Souls Rank 7
        Path1007 = 496,
        /// Reach Path of Souls Rank 8
        Path1008 = 497,
        /// Reach Path of Souls Rank 9
        Path1009 = 498,
        /// Reach Path of Souls Rank 10
        Path1010 = 499,
        /// Destroy an icon on the main menu
        Secret01 = 500,
        /// Destroy 50 icons on the main menu
        Secret02 = 501,
        /// Destroy 100 icons on the main menu
        Secret02b = 502,
        /// Destroy 200 icons on the main menu
        Secret03 = 503,
        /// Destroy 500 icons on the main menu
        Secret03b = 504,
        /// Found the hidden gold coin by scrolling through the level page 3 times
        Secret04 = 505,
        /// Type 'lenny' into the 2.0 Vault
        Secret05 = 506,
        /// Type 'sparky' into the 2.0 Vault
        Secret06 = 507,
        /// Type 'spooky' into the 2.0 Vault
        Secret07 = 508,
        /// Type 'blockbite' into the 2.0 Vault
        Secret08 = 509,
        /// Type 'robotop' into the 2.0 Vault
        Secret09 = 510,
        /// Type 'ahead' into the 2.0 Vault
        Secret10 = 511,
        /// Destroy the 55th cube on the main menu
        Secret11 = 512,
        /// Destroy the 50th cube on the main menu
        Secret12 = 513,
        /// Type 'mule' into the 2.0 Vault
        Secret13 = 514,
        /// Type 'neverending' into the 2.0 Vault
        Secret14 = 515,
        /// Type 'gandalfpotter' into the 2.0 Vault
        Secret15 = 516,
        /// Consecutively type '8', '16', '30', '32', '46' and '84' into the 2.0 Vault
        Secret16 = 517,
        /// Type your username into the 2.0 Vault
        Secret17 = 518,
        /// Destroy 750 icons on the main menu
        Secret18 = 519,
        /// Type 'finalboss' into the 2.0 Vault
        Secret19 = 520,
        /// Type 'brainpower' into the Vault of Secrets
        V2Secret01 = 521,
        /// Type 'cod3breaker' and the solution to the puzzle into the Vault of Secrets
        V2Secret02 = 522,
        /// Solve the 'glubfub' puzzle
        V2Secret03 = 523,
        /// Type 'octocube' into the Vault of Secrets
        V2Secret04 = 524,
        /// Type your star count into the Vault of Secrets
        V2Secret05 = 525,
        /// Type 'seven' into the Vault of Secrets
        V2Secret06 = 526,
        /// Type 'gimmiethecolor' into the Vault of Secrets
        V2Secret07 = 527,
        /// Type 'thechickenisonfire' into the Vault of Secrets
        V2Secret08 = 528,
        /// Type 'd4shg30me7ry' into the Vault of Secrets
        V2Secret09 = 529,
        /// Type 'thechickenisready' into the Vault of Secrets
        V2Secret10 = 530,
        /// Type 'darkness' into the Chamber of Time
        V3Secret01 = 531,
        /// Type 'silence' into the Chamber of Time
        V3Secret02 = 532,
        /// Type 'river' into the Chamber of Time
        V3Secret03 = 533,
        /// Type 'hunger' into the Chamber of Time
        V3Secret04 = 534,
        /// Type 'volcano' into the Chamber of Time
        V3Secret05 = 535,
        /// Type 'backontrack' into the Chamber of Time
        V3Secret06 = 536,
        /// Type 'givemehelper' into the Chamber of Time
        V3Secret07 = 537,
    }
);

impl GDAchievement {
    /// Parses an achievement identifier to a variant of self. This function will only successfully parse vanilla achievements.
    /// When parsing CCGameManager.dat, this function is used to identify each achievement.
    pub fn parse_str(s: &str) -> Option<Self> {
        match s {
            "geometry.ach.level01b" => Some(Self::Level01b),
            "geometry.ach.level01a" => Some(Self::Level01a),
            "geometry.ach.level02b" => Some(Self::Level02b),
            "geometry.ach.level02a" => Some(Self::Level02a),
            "geometry.ach.level03b" => Some(Self::Level03b),
            "geometry.ach.level03a" => Some(Self::Level03a),
            "geometry.ach.level04b" => Some(Self::Level04b),
            "geometry.ach.level04a" => Some(Self::Level04a),
            "geometry.ach.level05b" => Some(Self::Level05b),
            "geometry.ach.level05a" => Some(Self::Level05a),
            "geometry.ach.level06b" => Some(Self::Level06b),
            "geometry.ach.level06a" => Some(Self::Level06a),
            "geometry.ach.level07b" => Some(Self::Level07b),
            "geometry.ach.level07a" => Some(Self::Level07a),
            "geometry.ach.level08b" => Some(Self::Level08b),
            "geometry.ach.level08a" => Some(Self::Level08a),
            "geometry.ach.level09b" => Some(Self::Level09b),
            "geometry.ach.level09a" => Some(Self::Level09a),
            "geometry.ach.level10b" => Some(Self::Level10b),
            "geometry.ach.level10a" => Some(Self::Level10a),
            "geometry.ach.level11b" => Some(Self::Level11b),
            "geometry.ach.level11a" => Some(Self::Level11a),
            "geometry.ach.level12b" => Some(Self::Level12b),
            "geometry.ach.level12a" => Some(Self::Level12a),
            "geometry.ach.level13b" => Some(Self::Level13b),
            "geometry.ach.level13a" => Some(Self::Level13a),
            "geometry.ach.level14b" => Some(Self::Level14b),
            "geometry.ach.level14a" => Some(Self::Level14a),
            "geometry.ach.level15b" => Some(Self::Level15b),
            "geometry.ach.level15a" => Some(Self::Level15a),
            "geometry.ach.level16b" => Some(Self::Level16b),
            "geometry.ach.level16a" => Some(Self::Level16a),
            "geometry.ach.level17b" => Some(Self::Level17b),
            "geometry.ach.level17a" => Some(Self::Level17a),
            "geometry.ach.level18b" => Some(Self::Level18b),
            "geometry.ach.level18a" => Some(Self::Level18a),
            "geometry.ach.level19b" => Some(Self::Level19b),
            "geometry.ach.level19a" => Some(Self::Level19a),
            "geometry.ach.level20b" => Some(Self::Level20b),
            "geometry.ach.level20a" => Some(Self::Level20a),
            "geometry.ach.level21b" => Some(Self::Level21b),
            "geometry.ach.level21a" => Some(Self::Level21a),
            "geometry.ach.level22b" => Some(Self::Level22b),
            "geometry.ach.level22a" => Some(Self::Level22a),
            "geometry.ach.tower01" => Some(Self::Tower01),
            "geometry.ach.tower02" => Some(Self::Tower02),
            "geometry.ach.tower03" => Some(Self::Tower03),
            "geometry.ach.tower04" => Some(Self::Tower04),
            "geometry.ach.tower01Coin" => Some(Self::Tower01Coin),
            "geometry.ach.tower02Coin" => Some(Self::Tower02Coin),
            "geometry.ach.tower03Coin" => Some(Self::Tower03Coin),
            "geometry.ach.tower04Coin" => Some(Self::Tower04Coin),
            "geometry.ach.steam01" => Some(Self::Steam01),
            "geometry.ach.steam02" => Some(Self::Steam02),
            "geometry.ach.demoncoin01" => Some(Self::Demoncoin01),
            "geometry.ach.demoncoin02" => Some(Self::Demoncoin02),
            "geometry.ach.demoncoin03" => Some(Self::Demoncoin03),
            "geometry.ach.mappacks01" => Some(Self::Mappacks01),
            "geometry.ach.mappacks02" => Some(Self::Mappacks02),
            "geometry.ach.mappacks03" => Some(Self::Mappacks03),
            "geometry.ach.mappacks04" => Some(Self::Mappacks04),
            "geometry.ach.mappacks05" => Some(Self::Mappacks05),
            "geometry.ach.mappacks06" => Some(Self::Mappacks06),
            "geometry.ach.mappacks07" => Some(Self::Mappacks07),
            "geometry.ach.mappacks08" => Some(Self::Mappacks08),
            "geometry.ach.mappacks09" => Some(Self::Mappacks09),
            "geometry.ach.mappacks10" => Some(Self::Mappacks10),
            "geometry.ach.gauntlets01" => Some(Self::Gauntlets01),
            "geometry.ach.gauntlets02" => Some(Self::Gauntlets02),
            "geometry.ach.gauntlets03" => Some(Self::Gauntlets03),
            "geometry.ach.gauntlets04" => Some(Self::Gauntlets04),
            "geometry.ach.daily01" => Some(Self::Daily01),
            "geometry.ach.daily02" => Some(Self::Daily02),
            "geometry.ach.daily03" => Some(Self::Daily03),
            "geometry.ach.daily04" => Some(Self::Daily04),
            "geometry.ach.daily05" => Some(Self::Daily05),
            "geometry.ach.daily06" => Some(Self::Daily06),
            "geometry.ach.daily07" => Some(Self::Daily07),
            "geometry.ach.custom01" => Some(Self::Custom01),
            "geometry.ach.custom02" => Some(Self::Custom02),
            "geometry.ach.custom03" => Some(Self::Custom03),
            "geometry.ach.custom04" => Some(Self::Custom04),
            "geometry.ach.custom05" => Some(Self::Custom05),
            "geometry.ach.custom06" => Some(Self::Custom06),
            "geometry.ach.custom07" => Some(Self::Custom07),
            "geometry.ach.custom08" => Some(Self::Custom08),
            "geometry.ach.custom09" => Some(Self::Custom09),
            "geometry.ach.custom10" => Some(Self::Custom10),
            "geometry.ach.custom11" => Some(Self::Custom11),
            "geometry.ach.custom12" => Some(Self::Custom12),
            "geometry.ach.custom13" => Some(Self::Custom13),
            "geometry.ach.custom14" => Some(Self::Custom14),
            "geometry.ach.stars01" => Some(Self::Stars01),
            "geometry.ach.stars02" => Some(Self::Stars02),
            "geometry.ach.stars03" => Some(Self::Stars03),
            "geometry.ach.stars04" => Some(Self::Stars04),
            "geometry.ach.stars05" => Some(Self::Stars05),
            "geometry.ach.stars06" => Some(Self::Stars06),
            "geometry.ach.stars07" => Some(Self::Stars07),
            "geometry.ach.stars08" => Some(Self::Stars08),
            "geometry.ach.stars09" => Some(Self::Stars09),
            "geometry.ach.stars10" => Some(Self::Stars10),
            "geometry.ach.stars11" => Some(Self::Stars11),
            "geometry.ach.stars12" => Some(Self::Stars12),
            "geometry.ach.stars13" => Some(Self::Stars13),
            "geometry.ach.stars14" => Some(Self::Stars14),
            "geometry.ach.stars15" => Some(Self::Stars15),
            "geometry.ach.stars16" => Some(Self::Stars16),
            "geometry.ach.stars17" => Some(Self::Stars17),
            "geometry.ach.stars18" => Some(Self::Stars18),
            "geometry.ach.stars19" => Some(Self::Stars19),
            "geometry.ach.stars20" => Some(Self::Stars20),
            "geometry.ach.stars21" => Some(Self::Stars21),
            "geometry.ach.stars22" => Some(Self::Stars22),
            "geometry.ach.stars23" => Some(Self::Stars23),
            "geometry.ach.stars24" => Some(Self::Stars24),
            "geometry.ach.stars25" => Some(Self::Stars25),
            "geometry.ach.stars26" => Some(Self::Stars26),
            "geometry.ach.stars27" => Some(Self::Stars27),
            "geometry.ach.stars28" => Some(Self::Stars28),
            "geometry.ach.stars29" => Some(Self::Stars29),
            "geometry.ach.stars30" => Some(Self::Stars30),
            "geometry.ach.stars31" => Some(Self::Stars31),
            "geometry.ach.stars32" => Some(Self::Stars32),
            "geometry.ach.stars33" => Some(Self::Stars33),
            "geometry.ach.moons01" => Some(Self::Moons01),
            "geometry.ach.moons02" => Some(Self::Moons02),
            "geometry.ach.moons03" => Some(Self::Moons03),
            "geometry.ach.moons04" => Some(Self::Moons04),
            "geometry.ach.moons05" => Some(Self::Moons05),
            "geometry.ach.moons06" => Some(Self::Moons06),
            "geometry.ach.moons07" => Some(Self::Moons07),
            "geometry.ach.moons08" => Some(Self::Moons08),
            "geometry.ach.moons09" => Some(Self::Moons09),
            "geometry.ach.moons10" => Some(Self::Moons10),
            "geometry.ach.moons11" => Some(Self::Moons11),
            "geometry.ach.moons12" => Some(Self::Moons12),
            "geometry.ach.moons13" => Some(Self::Moons13),
            "geometry.ach.moons14" => Some(Self::Moons14),
            "geometry.ach.moons15" => Some(Self::Moons15),
            "geometry.ach.moons16" => Some(Self::Moons16),
            "geometry.ach.moons17" => Some(Self::Moons17),
            "geometry.ach.moons18" => Some(Self::Moons18),
            "geometry.ach.moons19" => Some(Self::Moons19),
            "geometry.ach.moons20" => Some(Self::Moons20),
            "geometry.ach.moons21" => Some(Self::Moons21),
            "geometry.ach.moons22" => Some(Self::Moons22),
            "geometry.ach.moons23" => Some(Self::Moons23),
            "geometry.ach.moons24" => Some(Self::Moons24),
            "geometry.ach.moons25" => Some(Self::Moons25),
            "geometry.ach.moons26" => Some(Self::Moons26),
            "geometry.ach.demon01" => Some(Self::Demon01),
            "geometry.ach.demon02" => Some(Self::Demon02),
            "geometry.ach.demon03" => Some(Self::Demon03),
            "geometry.ach.demon04" => Some(Self::Demon04),
            "geometry.ach.demon05" => Some(Self::Demon05),
            "geometry.ach.demon06" => Some(Self::Demon06),
            "geometry.ach.demon07" => Some(Self::Demon07),
            "geometry.ach.demon08" => Some(Self::Demon08),
            "geometry.ach.demon09" => Some(Self::Demon09),
            "geometry.ach.demon10" => Some(Self::Demon10),
            "geometry.ach.demon11" => Some(Self::Demon11),
            "geometry.ach.demon12" => Some(Self::Demon12),
            "geometry.ach.demon13" => Some(Self::Demon13),
            "geometry.ach.demon14" => Some(Self::Demon14),
            "geometry.ach.demon15" => Some(Self::Demon15),
            "geometry.ach.demon16" => Some(Self::Demon16),
            "geometry.ach.demon17" => Some(Self::Demon17),
            "geometry.ach.demon18" => Some(Self::Demon18),
            "geometry.ach.demon19" => Some(Self::Demon19),
            "geometry.ach.demon20" => Some(Self::Demon20),
            "geometry.ach.demon21" => Some(Self::Demon21),
            "geometry.ach.demon22" => Some(Self::Demon22),
            "geometry.ach.demon23" => Some(Self::Demon23),
            "geometry.ach.demon24" => Some(Self::Demon24),
            "geometry.ach.demon25" => Some(Self::Demon25),
            "geometry.ach.demon26" => Some(Self::Demon26),
            "geometry.ach.demon27" => Some(Self::Demon27),
            "geometry.ach.demon28" => Some(Self::Demon28),
            "geometry.ach.coins01" => Some(Self::Coins01),
            "geometry.ach.coins02" => Some(Self::Coins02),
            "geometry.ach.coins03" => Some(Self::Coins03),
            "geometry.ach.coins04" => Some(Self::Coins04),
            "geometry.ach.coins05" => Some(Self::Coins05),
            "geometry.ach.coins06" => Some(Self::Coins06),
            "geometry.ach.coins07" => Some(Self::Coins07),
            "geometry.ach.coins08" => Some(Self::Coins08),
            "geometry.ach.coins09" => Some(Self::Coins09),
            "geometry.ach.coins10" => Some(Self::Coins10),
            "geometry.ach.coins11" => Some(Self::Coins11),
            "geometry.ach.coins12" => Some(Self::Coins12),
            "geometry.ach.coins13" => Some(Self::Coins13),
            "geometry.ach.coins14" => Some(Self::Coins14),
            "geometry.ach.coins15" => Some(Self::Coins15),
            "geometry.ach.coins16" => Some(Self::Coins16),
            "geometry.ach.coins17" => Some(Self::Coins17),
            "geometry.ach.coins18" => Some(Self::Coins18),
            "geometry.ach.coins19" => Some(Self::Coins19),
            "geometry.ach.coins20" => Some(Self::Coins20),
            "geometry.ach.coins21" => Some(Self::Coins21),
            "geometry.ach.coins22" => Some(Self::Coins22),
            "geometry.ach.coins23" => Some(Self::Coins23),
            "geometry.ach.coins24" => Some(Self::Coins24),
            "geometry.ach.coins25" => Some(Self::Coins25),
            "geometry.ach.coins26" => Some(Self::Coins26),
            "geometry.ach.coins27" => Some(Self::Coins27),
            "geometry.ach.coins28" => Some(Self::Coins28),
            "geometry.ach.usercoins01" => Some(Self::Usercoins01),
            "geometry.ach.usercoins02" => Some(Self::Usercoins02),
            "geometry.ach.usercoins03" => Some(Self::Usercoins03),
            "geometry.ach.usercoins04" => Some(Self::Usercoins04),
            "geometry.ach.usercoins05" => Some(Self::Usercoins05),
            "geometry.ach.usercoins06" => Some(Self::Usercoins06),
            "geometry.ach.usercoins07" => Some(Self::Usercoins07),
            "geometry.ach.usercoins08" => Some(Self::Usercoins08),
            "geometry.ach.usercoins09" => Some(Self::Usercoins09),
            "geometry.ach.usercoins10" => Some(Self::Usercoins10),
            "geometry.ach.usercoins11" => Some(Self::Usercoins11),
            "geometry.ach.usercoins12" => Some(Self::Usercoins12),
            "geometry.ach.usercoins13" => Some(Self::Usercoins13),
            "geometry.ach.usercoins14" => Some(Self::Usercoins14),
            "geometry.ach.usercoins15" => Some(Self::Usercoins15),
            "geometry.ach.usercoins16" => Some(Self::Usercoins16),
            "geometry.ach.usercoins17" => Some(Self::Usercoins17),
            "geometry.ach.usercoins18" => Some(Self::Usercoins18),
            "geometry.ach.usercoins19" => Some(Self::Usercoins19),
            "geometry.ach.usercoins20" => Some(Self::Usercoins20),
            "geometry.ach.usercoins21" => Some(Self::Usercoins21),
            "geometry.ach.usercoins22" => Some(Self::Usercoins22),
            "geometry.ach.usercoins23" => Some(Self::Usercoins23),
            "geometry.ach.usercoins24" => Some(Self::Usercoins24),
            "geometry.ach.usercoins25" => Some(Self::Usercoins25),
            "geometry.ach.usercoins26" => Some(Self::Usercoins26),
            "geometry.ach.usercoins27" => Some(Self::Usercoins27),
            "geometry.ach.usercoins28" => Some(Self::Usercoins28),
            "geometry.ach.usercoins29" => Some(Self::Usercoins29),
            "geometry.ach.usercoins30" => Some(Self::Usercoins30),
            "geometry.ach.usercoins31" => Some(Self::Usercoins31),
            "geometry.ach.usercoins32" => Some(Self::Usercoins32),
            "geometry.ach.usercoins33" => Some(Self::Usercoins33),
            "geometry.ach.usercoins34" => Some(Self::Usercoins34),
            "geometry.ach.usercoins35" => Some(Self::Usercoins35),
            "geometry.ach.usercoins36" => Some(Self::Usercoins36),
            "geometry.ach.usercoins37" => Some(Self::Usercoins37),
            "geometry.ach.usercoins38" => Some(Self::Usercoins38),
            "geometry.ach.usercoins39" => Some(Self::Usercoins39),
            "geometry.ach.usercoins40" => Some(Self::Usercoins40),
            "geometry.ach.diamonds01" => Some(Self::Diamonds01),
            "geometry.ach.diamonds02" => Some(Self::Diamonds02),
            "geometry.ach.diamonds03" => Some(Self::Diamonds03),
            "geometry.ach.diamonds04" => Some(Self::Diamonds04),
            "geometry.ach.diamonds05" => Some(Self::Diamonds05),
            "geometry.ach.diamonds06" => Some(Self::Diamonds06),
            "geometry.ach.diamonds07" => Some(Self::Diamonds07),
            "geometry.ach.diamonds08" => Some(Self::Diamonds08),
            "geometry.ach.diamonds09" => Some(Self::Diamonds09),
            "geometry.ach.diamonds10" => Some(Self::Diamonds10),
            "geometry.ach.diamonds11" => Some(Self::Diamonds11),
            "geometry.ach.diamonds12" => Some(Self::Diamonds12),
            "geometry.ach.diamonds13" => Some(Self::Diamonds13),
            "geometry.ach.diamonds14" => Some(Self::Diamonds14),
            "geometry.ach.diamonds15" => Some(Self::Diamonds15),
            "geometry.ach.diamonds16" => Some(Self::Diamonds16),
            "geometry.ach.diamonds17" => Some(Self::Diamonds17),
            "geometry.ach.diamonds18" => Some(Self::Diamonds18),
            "geometry.ach.lists01" => Some(Self::Lists01),
            "geometry.ach.lists02" => Some(Self::Lists02),
            "geometry.ach.lists03" => Some(Self::Lists03),
            "geometry.ach.lists04" => Some(Self::Lists04),
            "geometry.ach.lists05" => Some(Self::Lists05),
            "geometry.ach.lists06" => Some(Self::Lists06),
            "geometry.ach.shardFire01" => Some(Self::ShardFire01),
            "geometry.ach.shardFire02" => Some(Self::ShardFire02),
            "geometry.ach.shardFire03" => Some(Self::ShardFire03),
            "geometry.ach.shardFire04" => Some(Self::ShardFire04),
            "geometry.ach.shardFire05" => Some(Self::ShardFire05),
            "geometry.ach.shardIce01" => Some(Self::ShardIce01),
            "geometry.ach.shardIce02" => Some(Self::ShardIce02),
            "geometry.ach.shardIce03" => Some(Self::ShardIce03),
            "geometry.ach.shardIce04" => Some(Self::ShardIce04),
            "geometry.ach.shardIce05" => Some(Self::ShardIce05),
            "geometry.ach.shardPoison01" => Some(Self::ShardPoison01),
            "geometry.ach.shardPoison02" => Some(Self::ShardPoison02),
            "geometry.ach.shardPoison03" => Some(Self::ShardPoison03),
            "geometry.ach.shardPoison04" => Some(Self::ShardPoison04),
            "geometry.ach.shardPoison05" => Some(Self::ShardPoison05),
            "geometry.ach.shardShadow01" => Some(Self::ShardShadow01),
            "geometry.ach.shardShadow02" => Some(Self::ShardShadow02),
            "geometry.ach.shardShadow03" => Some(Self::ShardShadow03),
            "geometry.ach.shardShadow04" => Some(Self::ShardShadow04),
            "geometry.ach.shardShadow05" => Some(Self::ShardShadow05),
            "geometry.ach.shardLava01" => Some(Self::ShardLava01),
            "geometry.ach.shardLava02" => Some(Self::ShardLava02),
            "geometry.ach.shardLava03" => Some(Self::ShardLava03),
            "geometry.ach.shardLava04" => Some(Self::ShardLava04),
            "geometry.ach.shardLava05" => Some(Self::ShardLava05),
            "geometry.ach.shardBonus01" => Some(Self::ShardBonus01),
            "geometry.ach.shardBonus02" => Some(Self::ShardBonus02),
            "geometry.ach.shardBonus03" => Some(Self::ShardBonus03),
            "geometry.ach.shardBonus04" => Some(Self::ShardBonus04),
            "geometry.ach.shardBonus05" => Some(Self::ShardBonus05),
            "geometry.ach.shardEarth01" => Some(Self::ShardEarth01),
            "geometry.ach.shardEarth02" => Some(Self::ShardEarth02),
            "geometry.ach.shardEarth03" => Some(Self::ShardEarth03),
            "geometry.ach.shardEarth04" => Some(Self::ShardEarth04),
            "geometry.ach.shardEarth05" => Some(Self::ShardEarth05),
            "geometry.ach.shardBlood01" => Some(Self::ShardBlood01),
            "geometry.ach.shardBlood02" => Some(Self::ShardBlood02),
            "geometry.ach.shardBlood03" => Some(Self::ShardBlood03),
            "geometry.ach.shardBlood04" => Some(Self::ShardBlood04),
            "geometry.ach.shardBlood05" => Some(Self::ShardBlood05),
            "geometry.ach.shardMetal01" => Some(Self::ShardMetal01),
            "geometry.ach.shardMetal02" => Some(Self::ShardMetal02),
            "geometry.ach.shardMetal03" => Some(Self::ShardMetal03),
            "geometry.ach.shardMetal04" => Some(Self::ShardMetal04),
            "geometry.ach.shardMetal05" => Some(Self::ShardMetal05),
            "geometry.ach.shardLight01" => Some(Self::ShardLight01),
            "geometry.ach.shardLight02" => Some(Self::ShardLight02),
            "geometry.ach.shardLight03" => Some(Self::ShardLight03),
            "geometry.ach.shardLight04" => Some(Self::ShardLight04),
            "geometry.ach.shardLight05" => Some(Self::ShardLight05),
            "geometry.ach.shardSoul01" => Some(Self::ShardSoul01),
            "geometry.ach.shardSoul02" => Some(Self::ShardSoul02),
            "geometry.ach.shardSoul03" => Some(Self::ShardSoul03),
            "geometry.ach.shardSoul04" => Some(Self::ShardSoul04),
            "geometry.ach.shardSoul05" => Some(Self::ShardSoul05),
            "geometry.ach.shardBonusB01" => Some(Self::ShardBonusB01),
            "geometry.ach.shardBonusB02" => Some(Self::ShardBonusB02),
            "geometry.ach.shardBonusB03" => Some(Self::ShardBonusB03),
            "geometry.ach.shardBonusB04" => Some(Self::ShardBonusB04),
            "geometry.ach.shardBonusB05" => Some(Self::ShardBonusB05),
            "geometry.ach.followCreator" => Some(Self::FollowCreator),
            "geometry.ach.followCreator2" => Some(Self::FollowCreator2),
            "geometry.ach.followCreator3" => Some(Self::FollowCreator3),
            "geometry.ach.followCreator4" => Some(Self::FollowCreator4),
            "geometry.ach.friends01" => Some(Self::Friends01),
            "geometry.ach.friends02" => Some(Self::Friends02),
            "geometry.ach.friends03" => Some(Self::Friends03),
            "geometry.ach.friends04" => Some(Self::Friends04),
            "geometry.ach.youtube" => Some(Self::Youtube),
            "geometry.ach.twitter" => Some(Self::Twitter),
            "geometry.ach.facebook" => Some(Self::Facebook),
            "geometry.ach.attempt01" => Some(Self::Attempt01),
            "geometry.ach.attempt02" => Some(Self::Attempt02),
            "geometry.ach.attempt03" => Some(Self::Attempt03),
            "geometry.ach.attempt04" => Some(Self::Attempt04),
            "geometry.ach.attempt05" => Some(Self::Attempt05),
            "geometry.ach.attempt06" => Some(Self::Attempt06),
            "geometry.ach.attempt07" => Some(Self::Attempt07),
            "geometry.ach.attempt08" => Some(Self::Attempt08),
            "geometry.ach.attempt09" => Some(Self::Attempt09),
            "geometry.ach.attempt10" => Some(Self::Attempt10),
            "geometry.ach.attempt11" => Some(Self::Attempt11),
            "geometry.ach.attempt12" => Some(Self::Attempt12),
            "geometry.ach.attempt13" => Some(Self::Attempt13),
            "geometry.ach.attempt14" => Some(Self::Attempt14),
            "geometry.ach.jump01" => Some(Self::Jump01),
            "geometry.ach.jump02" => Some(Self::Jump02),
            "geometry.ach.jump03" => Some(Self::Jump03),
            "geometry.ach.jump04" => Some(Self::Jump04),
            "geometry.ach.jump05" => Some(Self::Jump05),
            "geometry.ach.jump06" => Some(Self::Jump06),
            "geometry.ach.jump07" => Some(Self::Jump07),
            "geometry.ach.jump08" => Some(Self::Jump08),
            "geometry.ach.jump09" => Some(Self::Jump09),
            "geometry.ach.jump10" => Some(Self::Jump10),
            "geometry.ach.jump11" => Some(Self::Jump11),
            "geometry.ach.submit" => Some(Self::Submit),
            "geometry.ach.rate" => Some(Self::Rate),
            "geometry.ach.rateDiff" => Some(Self::RateDiff),
            "geometry.ach.rateDiff02" => Some(Self::RateDiff02),
            "geometry.ach.rateDiff02b" => Some(Self::RateDiff02b),
            "geometry.ach.rateDiff03" => Some(Self::RateDiff03),
            "geometry.ach.rateDiff04" => Some(Self::RateDiff04),
            "geometry.ach.like" => Some(Self::Like),
            "geometry.ach.like02" => Some(Self::Like02),
            "geometry.ach.like02b" => Some(Self::Like02b),
            "geometry.ach.like03" => Some(Self::Like03),
            "geometry.ach.like04" => Some(Self::Like04),
            "geometry.ach.like05" => Some(Self::Like05),
            "geometry.ach.like06" => Some(Self::Like06),
            "geometry.ach.moreGames" => Some(Self::MoreGames),
            "geometry.ach.special01" => Some(Self::Special01),
            "geometry.ach.creator01" => Some(Self::Creator01),
            "geometry.ach.creator02" => Some(Self::Creator02),
            "geometry.ach.creator03" => Some(Self::Creator03),
            "geometry.ach.creator04" => Some(Self::Creator04),
            "geometry.ach.creator05" => Some(Self::Creator05),
            "geometry.ach.creator06" => Some(Self::Creator06),
            "geometry.ach.path01.00" => Some(Self::Path0100),
            "geometry.ach.path01.01" => Some(Self::Path0101),
            "geometry.ach.path01.02" => Some(Self::Path0102),
            "geometry.ach.path01.03" => Some(Self::Path0103),
            "geometry.ach.path01.04" => Some(Self::Path0104),
            "geometry.ach.path01.05" => Some(Self::Path0105),
            "geometry.ach.path01.06" => Some(Self::Path0106),
            "geometry.ach.path01.07" => Some(Self::Path0107),
            "geometry.ach.path01.08" => Some(Self::Path0108),
            "geometry.ach.path01.09" => Some(Self::Path0109),
            "geometry.ach.path01.10" => Some(Self::Path0110),
            "geometry.ach.path02.00" => Some(Self::Path0200),
            "geometry.ach.path02.01" => Some(Self::Path0201),
            "geometry.ach.path02.02" => Some(Self::Path0202),
            "geometry.ach.path02.03" => Some(Self::Path0203),
            "geometry.ach.path02.04" => Some(Self::Path0204),
            "geometry.ach.path02.05" => Some(Self::Path0205),
            "geometry.ach.path02.06" => Some(Self::Path0206),
            "geometry.ach.path02.07" => Some(Self::Path0207),
            "geometry.ach.path02.08" => Some(Self::Path0208),
            "geometry.ach.path02.09" => Some(Self::Path0209),
            "geometry.ach.path02.10" => Some(Self::Path0210),
            "geometry.ach.path03.00" => Some(Self::Path0300),
            "geometry.ach.path03.01" => Some(Self::Path0301),
            "geometry.ach.path03.02" => Some(Self::Path0302),
            "geometry.ach.path03.03" => Some(Self::Path0303),
            "geometry.ach.path03.04" => Some(Self::Path0304),
            "geometry.ach.path03.05" => Some(Self::Path0305),
            "geometry.ach.path03.06" => Some(Self::Path0306),
            "geometry.ach.path03.07" => Some(Self::Path0307),
            "geometry.ach.path03.08" => Some(Self::Path0308),
            "geometry.ach.path03.09" => Some(Self::Path0309),
            "geometry.ach.path03.10" => Some(Self::Path0310),
            "geometry.ach.path04.00" => Some(Self::Path0400),
            "geometry.ach.path04.01" => Some(Self::Path0401),
            "geometry.ach.path04.02" => Some(Self::Path0402),
            "geometry.ach.path04.03" => Some(Self::Path0403),
            "geometry.ach.path04.04" => Some(Self::Path0404),
            "geometry.ach.path04.05" => Some(Self::Path0405),
            "geometry.ach.path04.06" => Some(Self::Path0406),
            "geometry.ach.path04.07" => Some(Self::Path0407),
            "geometry.ach.path04.08" => Some(Self::Path0408),
            "geometry.ach.path04.09" => Some(Self::Path0409),
            "geometry.ach.path04.10" => Some(Self::Path0410),
            "geometry.ach.path05.00" => Some(Self::Path0500),
            "geometry.ach.path05.01" => Some(Self::Path0501),
            "geometry.ach.path05.02" => Some(Self::Path0502),
            "geometry.ach.path05.03" => Some(Self::Path0503),
            "geometry.ach.path05.04" => Some(Self::Path0504),
            "geometry.ach.path05.05" => Some(Self::Path0505),
            "geometry.ach.path05.06" => Some(Self::Path0506),
            "geometry.ach.path05.07" => Some(Self::Path0507),
            "geometry.ach.path05.08" => Some(Self::Path0508),
            "geometry.ach.path05.09" => Some(Self::Path0509),
            "geometry.ach.path05.10" => Some(Self::Path0510),
            "geometry.ach.path06.00" => Some(Self::Path0600),
            "geometry.ach.path06.01" => Some(Self::Path0601),
            "geometry.ach.path06.02" => Some(Self::Path0602),
            "geometry.ach.path06.03" => Some(Self::Path0603),
            "geometry.ach.path06.04" => Some(Self::Path0604),
            "geometry.ach.path06.05" => Some(Self::Path0605),
            "geometry.ach.path06.06" => Some(Self::Path0606),
            "geometry.ach.path06.07" => Some(Self::Path0607),
            "geometry.ach.path06.08" => Some(Self::Path0608),
            "geometry.ach.path06.09" => Some(Self::Path0609),
            "geometry.ach.path06.10" => Some(Self::Path0610),
            "geometry.ach.path07.00" => Some(Self::Path0700),
            "geometry.ach.path07.01" => Some(Self::Path0701),
            "geometry.ach.path07.02" => Some(Self::Path0702),
            "geometry.ach.path07.03" => Some(Self::Path0703),
            "geometry.ach.path07.04" => Some(Self::Path0704),
            "geometry.ach.path07.05" => Some(Self::Path0705),
            "geometry.ach.path07.06" => Some(Self::Path0706),
            "geometry.ach.path07.07" => Some(Self::Path0707),
            "geometry.ach.path07.08" => Some(Self::Path0708),
            "geometry.ach.path07.09" => Some(Self::Path0709),
            "geometry.ach.path07.10" => Some(Self::Path0710),
            "geometry.ach.path08.00" => Some(Self::Path0800),
            "geometry.ach.path08.01" => Some(Self::Path0801),
            "geometry.ach.path08.02" => Some(Self::Path0802),
            "geometry.ach.path08.03" => Some(Self::Path0803),
            "geometry.ach.path08.04" => Some(Self::Path0804),
            "geometry.ach.path08.05" => Some(Self::Path0805),
            "geometry.ach.path08.06" => Some(Self::Path0806),
            "geometry.ach.path08.07" => Some(Self::Path0807),
            "geometry.ach.path08.08" => Some(Self::Path0808),
            "geometry.ach.path08.09" => Some(Self::Path0809),
            "geometry.ach.path08.10" => Some(Self::Path0810),
            "geometry.ach.path09.00" => Some(Self::Path0900),
            "geometry.ach.path09.01" => Some(Self::Path0901),
            "geometry.ach.path09.02" => Some(Self::Path0902),
            "geometry.ach.path09.03" => Some(Self::Path0903),
            "geometry.ach.path09.04" => Some(Self::Path0904),
            "geometry.ach.path09.05" => Some(Self::Path0905),
            "geometry.ach.path09.06" => Some(Self::Path0906),
            "geometry.ach.path09.07" => Some(Self::Path0907),
            "geometry.ach.path09.08" => Some(Self::Path0908),
            "geometry.ach.path09.09" => Some(Self::Path0909),
            "geometry.ach.path09.10" => Some(Self::Path0910),
            "geometry.ach.path10.00" => Some(Self::Path1000),
            "geometry.ach.path10.01" => Some(Self::Path1001),
            "geometry.ach.path10.02" => Some(Self::Path1002),
            "geometry.ach.path10.03" => Some(Self::Path1003),
            "geometry.ach.path10.04" => Some(Self::Path1004),
            "geometry.ach.path10.05" => Some(Self::Path1005),
            "geometry.ach.path10.06" => Some(Self::Path1006),
            "geometry.ach.path10.07" => Some(Self::Path1007),
            "geometry.ach.path10.08" => Some(Self::Path1008),
            "geometry.ach.path10.09" => Some(Self::Path1009),
            "geometry.ach.path10.10" => Some(Self::Path1010),
            "geometry.ach.secret01" => Some(Self::Secret01),
            "geometry.ach.secret02" => Some(Self::Secret02),
            "geometry.ach.secret02b" => Some(Self::Secret02b),
            "geometry.ach.secret03" => Some(Self::Secret03),
            "geometry.ach.secret03b" => Some(Self::Secret03b),
            "geometry.ach.secret04" => Some(Self::Secret04),
            "geometry.ach.secret05" => Some(Self::Secret05),
            "geometry.ach.secret06" => Some(Self::Secret06),
            "geometry.ach.secret07" => Some(Self::Secret07),
            "geometry.ach.secret08" => Some(Self::Secret08),
            "geometry.ach.secret09" => Some(Self::Secret09),
            "geometry.ach.secret10" => Some(Self::Secret10),
            "geometry.ach.secret11" => Some(Self::Secret11),
            "geometry.ach.secret12" => Some(Self::Secret12),
            "geometry.ach.secret13" => Some(Self::Secret13),
            "geometry.ach.secret14" => Some(Self::Secret14),
            "geometry.ach.secret15" => Some(Self::Secret15),
            "geometry.ach.secret16" => Some(Self::Secret16),
            "geometry.ach.secret17" => Some(Self::Secret17),
            "geometry.ach.secret18" => Some(Self::Secret18),
            "geometry.ach.secret19" => Some(Self::Secret19),
            "geometry.ach.v2.secret01" => Some(Self::V2Secret01),
            "geometry.ach.v2.secret02" => Some(Self::V2Secret02),
            "geometry.ach.v2.secret03" => Some(Self::V2Secret03),
            "geometry.ach.v2.secret04" => Some(Self::V2Secret04),
            "geometry.ach.v2.secret05" => Some(Self::V2Secret05),
            "geometry.ach.v2.secret06" => Some(Self::V2Secret06),
            "geometry.ach.v2.secret07" => Some(Self::V2Secret07),
            "geometry.ach.v2.secret08" => Some(Self::V2Secret08),
            "geometry.ach.v2.secret09" => Some(Self::V2Secret09),
            "geometry.ach.v2.secret10" => Some(Self::V2Secret10),
            "geometry.ach.v3.secret01" => Some(Self::V3Secret01),
            "geometry.ach.v3.secret02" => Some(Self::V3Secret02),
            "geometry.ach.v3.secret03" => Some(Self::V3Secret03),
            "geometry.ach.v3.secret04" => Some(Self::V3Secret04),
            "geometry.ach.v3.secret05" => Some(Self::V3Secret05),
            "geometry.ach.v3.secret06" => Some(Self::V3Secret06),
            "geometry.ach.v3.secret07" => Some(Self::V3Secret07),
            _ => None,
        }
    }

    /// Returns the internal identifier used in the savefiles for this specific achievement.
    pub fn to_ident(&self) -> &str {
        match self {
            Self::Level01b => "geometry.ach.level01b",
            Self::Level01a => "geometry.ach.level01a",
            Self::Level02b => "geometry.ach.level02b",
            Self::Level02a => "geometry.ach.level02a",
            Self::Level03b => "geometry.ach.level03b",
            Self::Level03a => "geometry.ach.level03a",
            Self::Level04b => "geometry.ach.level04b",
            Self::Level04a => "geometry.ach.level04a",
            Self::Level05b => "geometry.ach.level05b",
            Self::Level05a => "geometry.ach.level05a",
            Self::Level06b => "geometry.ach.level06b",
            Self::Level06a => "geometry.ach.level06a",
            Self::Level07b => "geometry.ach.level07b",
            Self::Level07a => "geometry.ach.level07a",
            Self::Level08b => "geometry.ach.level08b",
            Self::Level08a => "geometry.ach.level08a",
            Self::Level09b => "geometry.ach.level09b",
            Self::Level09a => "geometry.ach.level09a",
            Self::Level10b => "geometry.ach.level10b",
            Self::Level10a => "geometry.ach.level10a",
            Self::Level11b => "geometry.ach.level11b",
            Self::Level11a => "geometry.ach.level11a",
            Self::Level12b => "geometry.ach.level12b",
            Self::Level12a => "geometry.ach.level12a",
            Self::Level13b => "geometry.ach.level13b",
            Self::Level13a => "geometry.ach.level13a",
            Self::Level14b => "geometry.ach.level14b",
            Self::Level14a => "geometry.ach.level14a",
            Self::Level15b => "geometry.ach.level15b",
            Self::Level15a => "geometry.ach.level15a",
            Self::Level16b => "geometry.ach.level16b",
            Self::Level16a => "geometry.ach.level16a",
            Self::Level17b => "geometry.ach.level17b",
            Self::Level17a => "geometry.ach.level17a",
            Self::Level18b => "geometry.ach.level18b",
            Self::Level18a => "geometry.ach.level18a",
            Self::Level19b => "geometry.ach.level19b",
            Self::Level19a => "geometry.ach.level19a",
            Self::Level20b => "geometry.ach.level20b",
            Self::Level20a => "geometry.ach.level20a",
            Self::Level21b => "geometry.ach.level21b",
            Self::Level21a => "geometry.ach.level21a",
            Self::Level22b => "geometry.ach.level22b",
            Self::Level22a => "geometry.ach.level22a",
            Self::Tower01 => "geometry.ach.tower01",
            Self::Tower02 => "geometry.ach.tower02",
            Self::Tower03 => "geometry.ach.tower03",
            Self::Tower04 => "geometry.ach.tower04",
            Self::Tower01Coin => "geometry.ach.tower01Coin",
            Self::Tower02Coin => "geometry.ach.tower02Coin",
            Self::Tower03Coin => "geometry.ach.tower03Coin",
            Self::Tower04Coin => "geometry.ach.tower04Coin",
            Self::Steam01 => "geometry.ach.steam01",
            Self::Steam02 => "geometry.ach.steam02",
            Self::Demoncoin01 => "geometry.ach.demoncoin01",
            Self::Demoncoin02 => "geometry.ach.demoncoin02",
            Self::Demoncoin03 => "geometry.ach.demoncoin03",
            Self::Mappacks01 => "geometry.ach.mappacks01",
            Self::Mappacks02 => "geometry.ach.mappacks02",
            Self::Mappacks03 => "geometry.ach.mappacks03",
            Self::Mappacks04 => "geometry.ach.mappacks04",
            Self::Mappacks05 => "geometry.ach.mappacks05",
            Self::Mappacks06 => "geometry.ach.mappacks06",
            Self::Mappacks07 => "geometry.ach.mappacks07",
            Self::Mappacks08 => "geometry.ach.mappacks08",
            Self::Mappacks09 => "geometry.ach.mappacks09",
            Self::Mappacks10 => "geometry.ach.mappacks10",
            Self::Gauntlets01 => "geometry.ach.gauntlets01",
            Self::Gauntlets02 => "geometry.ach.gauntlets02",
            Self::Gauntlets03 => "geometry.ach.gauntlets03",
            Self::Gauntlets04 => "geometry.ach.gauntlets04",
            Self::Daily01 => "geometry.ach.daily01",
            Self::Daily02 => "geometry.ach.daily02",
            Self::Daily03 => "geometry.ach.daily03",
            Self::Daily04 => "geometry.ach.daily04",
            Self::Daily05 => "geometry.ach.daily05",
            Self::Daily06 => "geometry.ach.daily06",
            Self::Daily07 => "geometry.ach.daily07",
            Self::Custom01 => "geometry.ach.custom01",
            Self::Custom02 => "geometry.ach.custom02",
            Self::Custom03 => "geometry.ach.custom03",
            Self::Custom04 => "geometry.ach.custom04",
            Self::Custom05 => "geometry.ach.custom05",
            Self::Custom06 => "geometry.ach.custom06",
            Self::Custom07 => "geometry.ach.custom07",
            Self::Custom08 => "geometry.ach.custom08",
            Self::Custom09 => "geometry.ach.custom09",
            Self::Custom10 => "geometry.ach.custom10",
            Self::Custom11 => "geometry.ach.custom11",
            Self::Custom12 => "geometry.ach.custom12",
            Self::Custom13 => "geometry.ach.custom13",
            Self::Custom14 => "geometry.ach.custom14",
            Self::Stars01 => "geometry.ach.stars01",
            Self::Stars02 => "geometry.ach.stars02",
            Self::Stars03 => "geometry.ach.stars03",
            Self::Stars04 => "geometry.ach.stars04",
            Self::Stars05 => "geometry.ach.stars05",
            Self::Stars06 => "geometry.ach.stars06",
            Self::Stars07 => "geometry.ach.stars07",
            Self::Stars08 => "geometry.ach.stars08",
            Self::Stars09 => "geometry.ach.stars09",
            Self::Stars10 => "geometry.ach.stars10",
            Self::Stars11 => "geometry.ach.stars11",
            Self::Stars12 => "geometry.ach.stars12",
            Self::Stars13 => "geometry.ach.stars13",
            Self::Stars14 => "geometry.ach.stars14",
            Self::Stars15 => "geometry.ach.stars15",
            Self::Stars16 => "geometry.ach.stars16",
            Self::Stars17 => "geometry.ach.stars17",
            Self::Stars18 => "geometry.ach.stars18",
            Self::Stars19 => "geometry.ach.stars19",
            Self::Stars20 => "geometry.ach.stars20",
            Self::Stars21 => "geometry.ach.stars21",
            Self::Stars22 => "geometry.ach.stars22",
            Self::Stars23 => "geometry.ach.stars23",
            Self::Stars24 => "geometry.ach.stars24",
            Self::Stars25 => "geometry.ach.stars25",
            Self::Stars26 => "geometry.ach.stars26",
            Self::Stars27 => "geometry.ach.stars27",
            Self::Stars28 => "geometry.ach.stars28",
            Self::Stars29 => "geometry.ach.stars29",
            Self::Stars30 => "geometry.ach.stars30",
            Self::Stars31 => "geometry.ach.stars31",
            Self::Stars32 => "geometry.ach.stars32",
            Self::Stars33 => "geometry.ach.stars33",
            Self::Moons01 => "geometry.ach.moons01",
            Self::Moons02 => "geometry.ach.moons02",
            Self::Moons03 => "geometry.ach.moons03",
            Self::Moons04 => "geometry.ach.moons04",
            Self::Moons05 => "geometry.ach.moons05",
            Self::Moons06 => "geometry.ach.moons06",
            Self::Moons07 => "geometry.ach.moons07",
            Self::Moons08 => "geometry.ach.moons08",
            Self::Moons09 => "geometry.ach.moons09",
            Self::Moons10 => "geometry.ach.moons10",
            Self::Moons11 => "geometry.ach.moons11",
            Self::Moons12 => "geometry.ach.moons12",
            Self::Moons13 => "geometry.ach.moons13",
            Self::Moons14 => "geometry.ach.moons14",
            Self::Moons15 => "geometry.ach.moons15",
            Self::Moons16 => "geometry.ach.moons16",
            Self::Moons17 => "geometry.ach.moons17",
            Self::Moons18 => "geometry.ach.moons18",
            Self::Moons19 => "geometry.ach.moons19",
            Self::Moons20 => "geometry.ach.moons20",
            Self::Moons21 => "geometry.ach.moons21",
            Self::Moons22 => "geometry.ach.moons22",
            Self::Moons23 => "geometry.ach.moons23",
            Self::Moons24 => "geometry.ach.moons24",
            Self::Moons25 => "geometry.ach.moons25",
            Self::Moons26 => "geometry.ach.moons26",
            Self::Demon01 => "geometry.ach.demon01",
            Self::Demon02 => "geometry.ach.demon02",
            Self::Demon03 => "geometry.ach.demon03",
            Self::Demon04 => "geometry.ach.demon04",
            Self::Demon05 => "geometry.ach.demon05",
            Self::Demon06 => "geometry.ach.demon06",
            Self::Demon07 => "geometry.ach.demon07",
            Self::Demon08 => "geometry.ach.demon08",
            Self::Demon09 => "geometry.ach.demon09",
            Self::Demon10 => "geometry.ach.demon10",
            Self::Demon11 => "geometry.ach.demon11",
            Self::Demon12 => "geometry.ach.demon12",
            Self::Demon13 => "geometry.ach.demon13",
            Self::Demon14 => "geometry.ach.demon14",
            Self::Demon15 => "geometry.ach.demon15",
            Self::Demon16 => "geometry.ach.demon16",
            Self::Demon17 => "geometry.ach.demon17",
            Self::Demon18 => "geometry.ach.demon18",
            Self::Demon19 => "geometry.ach.demon19",
            Self::Demon20 => "geometry.ach.demon20",
            Self::Demon21 => "geometry.ach.demon21",
            Self::Demon22 => "geometry.ach.demon22",
            Self::Demon23 => "geometry.ach.demon23",
            Self::Demon24 => "geometry.ach.demon24",
            Self::Demon25 => "geometry.ach.demon25",
            Self::Demon26 => "geometry.ach.demon26",
            Self::Demon27 => "geometry.ach.demon27",
            Self::Demon28 => "geometry.ach.demon28",
            Self::Coins01 => "geometry.ach.coins01",
            Self::Coins02 => "geometry.ach.coins02",
            Self::Coins03 => "geometry.ach.coins03",
            Self::Coins04 => "geometry.ach.coins04",
            Self::Coins05 => "geometry.ach.coins05",
            Self::Coins06 => "geometry.ach.coins06",
            Self::Coins07 => "geometry.ach.coins07",
            Self::Coins08 => "geometry.ach.coins08",
            Self::Coins09 => "geometry.ach.coins09",
            Self::Coins10 => "geometry.ach.coins10",
            Self::Coins11 => "geometry.ach.coins11",
            Self::Coins12 => "geometry.ach.coins12",
            Self::Coins13 => "geometry.ach.coins13",
            Self::Coins14 => "geometry.ach.coins14",
            Self::Coins15 => "geometry.ach.coins15",
            Self::Coins16 => "geometry.ach.coins16",
            Self::Coins17 => "geometry.ach.coins17",
            Self::Coins18 => "geometry.ach.coins18",
            Self::Coins19 => "geometry.ach.coins19",
            Self::Coins20 => "geometry.ach.coins20",
            Self::Coins21 => "geometry.ach.coins21",
            Self::Coins22 => "geometry.ach.coins22",
            Self::Coins23 => "geometry.ach.coins23",
            Self::Coins24 => "geometry.ach.coins24",
            Self::Coins25 => "geometry.ach.coins25",
            Self::Coins26 => "geometry.ach.coins26",
            Self::Coins27 => "geometry.ach.coins27",
            Self::Coins28 => "geometry.ach.coins28",
            Self::Usercoins01 => "geometry.ach.usercoins01",
            Self::Usercoins02 => "geometry.ach.usercoins02",
            Self::Usercoins03 => "geometry.ach.usercoins03",
            Self::Usercoins04 => "geometry.ach.usercoins04",
            Self::Usercoins05 => "geometry.ach.usercoins05",
            Self::Usercoins06 => "geometry.ach.usercoins06",
            Self::Usercoins07 => "geometry.ach.usercoins07",
            Self::Usercoins08 => "geometry.ach.usercoins08",
            Self::Usercoins09 => "geometry.ach.usercoins09",
            Self::Usercoins10 => "geometry.ach.usercoins10",
            Self::Usercoins11 => "geometry.ach.usercoins11",
            Self::Usercoins12 => "geometry.ach.usercoins12",
            Self::Usercoins13 => "geometry.ach.usercoins13",
            Self::Usercoins14 => "geometry.ach.usercoins14",
            Self::Usercoins15 => "geometry.ach.usercoins15",
            Self::Usercoins16 => "geometry.ach.usercoins16",
            Self::Usercoins17 => "geometry.ach.usercoins17",
            Self::Usercoins18 => "geometry.ach.usercoins18",
            Self::Usercoins19 => "geometry.ach.usercoins19",
            Self::Usercoins20 => "geometry.ach.usercoins20",
            Self::Usercoins21 => "geometry.ach.usercoins21",
            Self::Usercoins22 => "geometry.ach.usercoins22",
            Self::Usercoins23 => "geometry.ach.usercoins23",
            Self::Usercoins24 => "geometry.ach.usercoins24",
            Self::Usercoins25 => "geometry.ach.usercoins25",
            Self::Usercoins26 => "geometry.ach.usercoins26",
            Self::Usercoins27 => "geometry.ach.usercoins27",
            Self::Usercoins28 => "geometry.ach.usercoins28",
            Self::Usercoins29 => "geometry.ach.usercoins29",
            Self::Usercoins30 => "geometry.ach.usercoins30",
            Self::Usercoins31 => "geometry.ach.usercoins31",
            Self::Usercoins32 => "geometry.ach.usercoins32",
            Self::Usercoins33 => "geometry.ach.usercoins33",
            Self::Usercoins34 => "geometry.ach.usercoins34",
            Self::Usercoins35 => "geometry.ach.usercoins35",
            Self::Usercoins36 => "geometry.ach.usercoins36",
            Self::Usercoins37 => "geometry.ach.usercoins37",
            Self::Usercoins38 => "geometry.ach.usercoins38",
            Self::Usercoins39 => "geometry.ach.usercoins39",
            Self::Usercoins40 => "geometry.ach.usercoins40",
            Self::Diamonds01 => "geometry.ach.diamonds01",
            Self::Diamonds02 => "geometry.ach.diamonds02",
            Self::Diamonds03 => "geometry.ach.diamonds03",
            Self::Diamonds04 => "geometry.ach.diamonds04",
            Self::Diamonds05 => "geometry.ach.diamonds05",
            Self::Diamonds06 => "geometry.ach.diamonds06",
            Self::Diamonds07 => "geometry.ach.diamonds07",
            Self::Diamonds08 => "geometry.ach.diamonds08",
            Self::Diamonds09 => "geometry.ach.diamonds09",
            Self::Diamonds10 => "geometry.ach.diamonds10",
            Self::Diamonds11 => "geometry.ach.diamonds11",
            Self::Diamonds12 => "geometry.ach.diamonds12",
            Self::Diamonds13 => "geometry.ach.diamonds13",
            Self::Diamonds14 => "geometry.ach.diamonds14",
            Self::Diamonds15 => "geometry.ach.diamonds15",
            Self::Diamonds16 => "geometry.ach.diamonds16",
            Self::Diamonds17 => "geometry.ach.diamonds17",
            Self::Diamonds18 => "geometry.ach.diamonds18",
            Self::Lists01 => "geometry.ach.lists01",
            Self::Lists02 => "geometry.ach.lists02",
            Self::Lists03 => "geometry.ach.lists03",
            Self::Lists04 => "geometry.ach.lists04",
            Self::Lists05 => "geometry.ach.lists05",
            Self::Lists06 => "geometry.ach.lists06",
            Self::ShardFire01 => "geometry.ach.shardFire01",
            Self::ShardFire02 => "geometry.ach.shardFire02",
            Self::ShardFire03 => "geometry.ach.shardFire03",
            Self::ShardFire04 => "geometry.ach.shardFire04",
            Self::ShardFire05 => "geometry.ach.shardFire05",
            Self::ShardIce01 => "geometry.ach.shardIce01",
            Self::ShardIce02 => "geometry.ach.shardIce02",
            Self::ShardIce03 => "geometry.ach.shardIce03",
            Self::ShardIce04 => "geometry.ach.shardIce04",
            Self::ShardIce05 => "geometry.ach.shardIce05",
            Self::ShardPoison01 => "geometry.ach.shardPoison01",
            Self::ShardPoison02 => "geometry.ach.shardPoison02",
            Self::ShardPoison03 => "geometry.ach.shardPoison03",
            Self::ShardPoison04 => "geometry.ach.shardPoison04",
            Self::ShardPoison05 => "geometry.ach.shardPoison05",
            Self::ShardShadow01 => "geometry.ach.shardShadow01",
            Self::ShardShadow02 => "geometry.ach.shardShadow02",
            Self::ShardShadow03 => "geometry.ach.shardShadow03",
            Self::ShardShadow04 => "geometry.ach.shardShadow04",
            Self::ShardShadow05 => "geometry.ach.shardShadow05",
            Self::ShardLava01 => "geometry.ach.shardLava01",
            Self::ShardLava02 => "geometry.ach.shardLava02",
            Self::ShardLava03 => "geometry.ach.shardLava03",
            Self::ShardLava04 => "geometry.ach.shardLava04",
            Self::ShardLava05 => "geometry.ach.shardLava05",
            Self::ShardBonus01 => "geometry.ach.shardBonus01",
            Self::ShardBonus02 => "geometry.ach.shardBonus02",
            Self::ShardBonus03 => "geometry.ach.shardBonus03",
            Self::ShardBonus04 => "geometry.ach.shardBonus04",
            Self::ShardBonus05 => "geometry.ach.shardBonus05",
            Self::ShardEarth01 => "geometry.ach.shardEarth01",
            Self::ShardEarth02 => "geometry.ach.shardEarth02",
            Self::ShardEarth03 => "geometry.ach.shardEarth03",
            Self::ShardEarth04 => "geometry.ach.shardEarth04",
            Self::ShardEarth05 => "geometry.ach.shardEarth05",
            Self::ShardBlood01 => "geometry.ach.shardBlood01",
            Self::ShardBlood02 => "geometry.ach.shardBlood02",
            Self::ShardBlood03 => "geometry.ach.shardBlood03",
            Self::ShardBlood04 => "geometry.ach.shardBlood04",
            Self::ShardBlood05 => "geometry.ach.shardBlood05",
            Self::ShardMetal01 => "geometry.ach.shardMetal01",
            Self::ShardMetal02 => "geometry.ach.shardMetal02",
            Self::ShardMetal03 => "geometry.ach.shardMetal03",
            Self::ShardMetal04 => "geometry.ach.shardMetal04",
            Self::ShardMetal05 => "geometry.ach.shardMetal05",
            Self::ShardLight01 => "geometry.ach.shardLight01",
            Self::ShardLight02 => "geometry.ach.shardLight02",
            Self::ShardLight03 => "geometry.ach.shardLight03",
            Self::ShardLight04 => "geometry.ach.shardLight04",
            Self::ShardLight05 => "geometry.ach.shardLight05",
            Self::ShardSoul01 => "geometry.ach.shardSoul01",
            Self::ShardSoul02 => "geometry.ach.shardSoul02",
            Self::ShardSoul03 => "geometry.ach.shardSoul03",
            Self::ShardSoul04 => "geometry.ach.shardSoul04",
            Self::ShardSoul05 => "geometry.ach.shardSoul05",
            Self::ShardBonusB01 => "geometry.ach.shardBonusB01",
            Self::ShardBonusB02 => "geometry.ach.shardBonusB02",
            Self::ShardBonusB03 => "geometry.ach.shardBonusB03",
            Self::ShardBonusB04 => "geometry.ach.shardBonusB04",
            Self::ShardBonusB05 => "geometry.ach.shardBonusB05",
            Self::FollowCreator => "geometry.ach.followCreator",
            Self::FollowCreator2 => "geometry.ach.followCreator2",
            Self::FollowCreator3 => "geometry.ach.followCreator3",
            Self::FollowCreator4 => "geometry.ach.followCreator4",
            Self::Friends01 => "geometry.ach.friends01",
            Self::Friends02 => "geometry.ach.friends02",
            Self::Friends03 => "geometry.ach.friends03",
            Self::Friends04 => "geometry.ach.friends04",
            Self::Youtube => "geometry.ach.youtube",
            Self::Twitter => "geometry.ach.twitter",
            Self::Facebook => "geometry.ach.facebook",
            Self::Attempt01 => "geometry.ach.attempt01",
            Self::Attempt02 => "geometry.ach.attempt02",
            Self::Attempt03 => "geometry.ach.attempt03",
            Self::Attempt04 => "geometry.ach.attempt04",
            Self::Attempt05 => "geometry.ach.attempt05",
            Self::Attempt06 => "geometry.ach.attempt06",
            Self::Attempt07 => "geometry.ach.attempt07",
            Self::Attempt08 => "geometry.ach.attempt08",
            Self::Attempt09 => "geometry.ach.attempt09",
            Self::Attempt10 => "geometry.ach.attempt10",
            Self::Attempt11 => "geometry.ach.attempt11",
            Self::Attempt12 => "geometry.ach.attempt12",
            Self::Attempt13 => "geometry.ach.attempt13",
            Self::Attempt14 => "geometry.ach.attempt14",
            Self::Jump01 => "geometry.ach.jump01",
            Self::Jump02 => "geometry.ach.jump02",
            Self::Jump03 => "geometry.ach.jump03",
            Self::Jump04 => "geometry.ach.jump04",
            Self::Jump05 => "geometry.ach.jump05",
            Self::Jump06 => "geometry.ach.jump06",
            Self::Jump07 => "geometry.ach.jump07",
            Self::Jump08 => "geometry.ach.jump08",
            Self::Jump09 => "geometry.ach.jump09",
            Self::Jump10 => "geometry.ach.jump10",
            Self::Jump11 => "geometry.ach.jump11",
            Self::Submit => "geometry.ach.submit",
            Self::Rate => "geometry.ach.rate",
            Self::RateDiff => "geometry.ach.rateDiff",
            Self::RateDiff02 => "geometry.ach.rateDiff02",
            Self::RateDiff02b => "geometry.ach.rateDiff02b",
            Self::RateDiff03 => "geometry.ach.rateDiff03",
            Self::RateDiff04 => "geometry.ach.rateDiff04",
            Self::Like => "geometry.ach.like",
            Self::Like02 => "geometry.ach.like02",
            Self::Like02b => "geometry.ach.like02b",
            Self::Like03 => "geometry.ach.like03",
            Self::Like04 => "geometry.ach.like04",
            Self::Like05 => "geometry.ach.like05",
            Self::Like06 => "geometry.ach.like06",
            Self::MoreGames => "geometry.ach.moreGames",
            Self::Special01 => "geometry.ach.special01",
            Self::Creator01 => "geometry.ach.creator01",
            Self::Creator02 => "geometry.ach.creator02",
            Self::Creator03 => "geometry.ach.creator03",
            Self::Creator04 => "geometry.ach.creator04",
            Self::Creator05 => "geometry.ach.creator05",
            Self::Creator06 => "geometry.ach.creator06",
            Self::Path0100 => "geometry.ach.path01.00",
            Self::Path0101 => "geometry.ach.path01.01",
            Self::Path0102 => "geometry.ach.path01.02",
            Self::Path0103 => "geometry.ach.path01.03",
            Self::Path0104 => "geometry.ach.path01.04",
            Self::Path0105 => "geometry.ach.path01.05",
            Self::Path0106 => "geometry.ach.path01.06",
            Self::Path0107 => "geometry.ach.path01.07",
            Self::Path0108 => "geometry.ach.path01.08",
            Self::Path0109 => "geometry.ach.path01.09",
            Self::Path0110 => "geometry.ach.path01.10",
            Self::Path0200 => "geometry.ach.path02.00",
            Self::Path0201 => "geometry.ach.path02.01",
            Self::Path0202 => "geometry.ach.path02.02",
            Self::Path0203 => "geometry.ach.path02.03",
            Self::Path0204 => "geometry.ach.path02.04",
            Self::Path0205 => "geometry.ach.path02.05",
            Self::Path0206 => "geometry.ach.path02.06",
            Self::Path0207 => "geometry.ach.path02.07",
            Self::Path0208 => "geometry.ach.path02.08",
            Self::Path0209 => "geometry.ach.path02.09",
            Self::Path0210 => "geometry.ach.path02.10",
            Self::Path0300 => "geometry.ach.path03.00",
            Self::Path0301 => "geometry.ach.path03.01",
            Self::Path0302 => "geometry.ach.path03.02",
            Self::Path0303 => "geometry.ach.path03.03",
            Self::Path0304 => "geometry.ach.path03.04",
            Self::Path0305 => "geometry.ach.path03.05",
            Self::Path0306 => "geometry.ach.path03.06",
            Self::Path0307 => "geometry.ach.path03.07",
            Self::Path0308 => "geometry.ach.path03.08",
            Self::Path0309 => "geometry.ach.path03.09",
            Self::Path0310 => "geometry.ach.path03.10",
            Self::Path0400 => "geometry.ach.path04.00",
            Self::Path0401 => "geometry.ach.path04.01",
            Self::Path0402 => "geometry.ach.path04.02",
            Self::Path0403 => "geometry.ach.path04.03",
            Self::Path0404 => "geometry.ach.path04.04",
            Self::Path0405 => "geometry.ach.path04.05",
            Self::Path0406 => "geometry.ach.path04.06",
            Self::Path0407 => "geometry.ach.path04.07",
            Self::Path0408 => "geometry.ach.path04.08",
            Self::Path0409 => "geometry.ach.path04.09",
            Self::Path0410 => "geometry.ach.path04.10",
            Self::Path0500 => "geometry.ach.path05.00",
            Self::Path0501 => "geometry.ach.path05.01",
            Self::Path0502 => "geometry.ach.path05.02",
            Self::Path0503 => "geometry.ach.path05.03",
            Self::Path0504 => "geometry.ach.path05.04",
            Self::Path0505 => "geometry.ach.path05.05",
            Self::Path0506 => "geometry.ach.path05.06",
            Self::Path0507 => "geometry.ach.path05.07",
            Self::Path0508 => "geometry.ach.path05.08",
            Self::Path0509 => "geometry.ach.path05.09",
            Self::Path0510 => "geometry.ach.path05.10",
            Self::Path0600 => "geometry.ach.path06.00",
            Self::Path0601 => "geometry.ach.path06.01",
            Self::Path0602 => "geometry.ach.path06.02",
            Self::Path0603 => "geometry.ach.path06.03",
            Self::Path0604 => "geometry.ach.path06.04",
            Self::Path0605 => "geometry.ach.path06.05",
            Self::Path0606 => "geometry.ach.path06.06",
            Self::Path0607 => "geometry.ach.path06.07",
            Self::Path0608 => "geometry.ach.path06.08",
            Self::Path0609 => "geometry.ach.path06.09",
            Self::Path0610 => "geometry.ach.path06.10",
            Self::Path0700 => "geometry.ach.path07.00",
            Self::Path0701 => "geometry.ach.path07.01",
            Self::Path0702 => "geometry.ach.path07.02",
            Self::Path0703 => "geometry.ach.path07.03",
            Self::Path0704 => "geometry.ach.path07.04",
            Self::Path0705 => "geometry.ach.path07.05",
            Self::Path0706 => "geometry.ach.path07.06",
            Self::Path0707 => "geometry.ach.path07.07",
            Self::Path0708 => "geometry.ach.path07.08",
            Self::Path0709 => "geometry.ach.path07.09",
            Self::Path0710 => "geometry.ach.path07.10",
            Self::Path0800 => "geometry.ach.path08.00",
            Self::Path0801 => "geometry.ach.path08.01",
            Self::Path0802 => "geometry.ach.path08.02",
            Self::Path0803 => "geometry.ach.path08.03",
            Self::Path0804 => "geometry.ach.path08.04",
            Self::Path0805 => "geometry.ach.path08.05",
            Self::Path0806 => "geometry.ach.path08.06",
            Self::Path0807 => "geometry.ach.path08.07",
            Self::Path0808 => "geometry.ach.path08.08",
            Self::Path0809 => "geometry.ach.path08.09",
            Self::Path0810 => "geometry.ach.path08.10",
            Self::Path0900 => "geometry.ach.path09.00",
            Self::Path0901 => "geometry.ach.path09.01",
            Self::Path0902 => "geometry.ach.path09.02",
            Self::Path0903 => "geometry.ach.path09.03",
            Self::Path0904 => "geometry.ach.path09.04",
            Self::Path0905 => "geometry.ach.path09.05",
            Self::Path0906 => "geometry.ach.path09.06",
            Self::Path0907 => "geometry.ach.path09.07",
            Self::Path0908 => "geometry.ach.path09.08",
            Self::Path0909 => "geometry.ach.path09.09",
            Self::Path0910 => "geometry.ach.path09.10",
            Self::Path1000 => "geometry.ach.path10.00",
            Self::Path1001 => "geometry.ach.path10.01",
            Self::Path1002 => "geometry.ach.path10.02",
            Self::Path1003 => "geometry.ach.path10.03",
            Self::Path1004 => "geometry.ach.path10.04",
            Self::Path1005 => "geometry.ach.path10.05",
            Self::Path1006 => "geometry.ach.path10.06",
            Self::Path1007 => "geometry.ach.path10.07",
            Self::Path1008 => "geometry.ach.path10.08",
            Self::Path1009 => "geometry.ach.path10.09",
            Self::Path1010 => "geometry.ach.path10.10",
            Self::Secret01 => "geometry.ach.secret01",
            Self::Secret02 => "geometry.ach.secret02",
            Self::Secret02b => "geometry.ach.secret02b",
            Self::Secret03 => "geometry.ach.secret03",
            Self::Secret03b => "geometry.ach.secret03b",
            Self::Secret04 => "geometry.ach.secret04",
            Self::Secret05 => "geometry.ach.secret05",
            Self::Secret06 => "geometry.ach.secret06",
            Self::Secret07 => "geometry.ach.secret07",
            Self::Secret08 => "geometry.ach.secret08",
            Self::Secret09 => "geometry.ach.secret09",
            Self::Secret10 => "geometry.ach.secret10",
            Self::Secret11 => "geometry.ach.secret11",
            Self::Secret12 => "geometry.ach.secret12",
            Self::Secret13 => "geometry.ach.secret13",
            Self::Secret14 => "geometry.ach.secret14",
            Self::Secret15 => "geometry.ach.secret15",
            Self::Secret16 => "geometry.ach.secret16",
            Self::Secret17 => "geometry.ach.secret17",
            Self::Secret18 => "geometry.ach.secret18",
            Self::Secret19 => "geometry.ach.secret19",
            Self::V2Secret01 => "geometry.ach.v2.secret01",
            Self::V2Secret02 => "geometry.ach.v2.secret02",
            Self::V2Secret03 => "geometry.ach.v2.secret03",
            Self::V2Secret04 => "geometry.ach.v2.secret04",
            Self::V2Secret05 => "geometry.ach.v2.secret05",
            Self::V2Secret06 => "geometry.ach.v2.secret06",
            Self::V2Secret07 => "geometry.ach.v2.secret07",
            Self::V2Secret08 => "geometry.ach.v2.secret08",
            Self::V2Secret09 => "geometry.ach.v2.secret09",
            Self::V2Secret10 => "geometry.ach.v2.secret10",
            Self::V3Secret01 => "geometry.ach.v3.secret01",
            Self::V3Secret02 => "geometry.ach.v3.secret02",
            Self::V3Secret03 => "geometry.ach.v3.secret03",
            Self::V3Secret04 => "geometry.ach.v3.secret04",
            Self::V3Secret05 => "geometry.ach.v3.secret05",
            Self::V3Secret06 => "geometry.ach.v3.secret06",
            Self::V3Secret07 => "geometry.ach.v3.secret07",
        }
    }
}
