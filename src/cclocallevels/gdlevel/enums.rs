//! Enums present in GDLevel and GDList.
//!
//! Many of these were sourced from <https://github.com/UHDanke/gmdkit/blob/main/src/gmdkit/utils/enums.py>. Thanks to HDanke for providing this resource.

use crate::repr_t;

repr_t!(
    #[allow(missing_docs)]
    GDListType: i32 {
        None = 0,
        Local = 2,
        Saved = 3,
        Online = 4,
    }
    default None
);

repr_t!(
    #[allow(missing_docs)]
    /// Enum for the list difficulties
    GDListDifficulty: i32 {
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
    default NA
);

repr_t!(
    /// Type of epic rating that a level was awarded. Like [`FeatureRating`], but for Epic and above exclusively
    EpicRating: i32 {
        None = 0,
        Epic = 1,
        Legendary = 2,
        Mythic = 3,
    }
    default None
);

repr_t!(
    /// Type of level that a GDLevel is. Default is set to 2 (Local).
    GDLevelType: i32 {
        Official = 1,
        Local = 2,
        Saved = 3,
        Online = 4,
    }
    default Local
);

repr_t!(
    /// Type of demon that a level is
    DemonType: i32 {
        HardDemon = 0,
        Unknown = 1,
        EasyDemon = 3,
        MediumDemon = 4,
        InsaneDemon = 5,
        ExtremeDemon = 6,
    }
);

repr_t!(
    /// Length bracket of a level
    Length: i32 {
        Tiny = 0,
        Short = 1,
        Medium = 2,
        Long = 3,
        XL = 4,
        Platformer = 5,
    }
    default Tiny
);

repr_t!(
    /// All possible official songs a level can use
    OfficialSong: i32 {
        StayInsideMe = -1,
        StereoMadness = 0,
        BackOnTrack = 1,
        Polargeist = 2,
        DryOut = 3,
        BaseAfterBase = 4,
        CantLetGo = 5,
        Jumper = 6,
        TimeMachine = 7,
        Cycles = 8,
        Xstep = 9,
        Clutterfunk = 10,
        TheoryOfEverything = 11,
        ElectromanAdventures = 12,
        Clubstep = 13,
        Electrodynamix = 14,
        HexagonForce = 15,
        BlastProcessing = 16,
        TheoryOfEverything2 = 17,
        GeometricalDominator = 18,
        Deadlocked = 19,
        Fingerdash = 20,
        Dash = 21,
        Explorers = 22,
        TheSevenSeas = 23,
        VikingArena = 24,
        AirborneRobots = 25,
        Secret = 26,
        Payload = 27,
        BeastMode = 28,
        Machina = 29,
        Years = 30,
        Frontlines = 31,
        SpacePirates = 32,
        Striker = 33,
        Embers = 34,
        Round1 = 35,
        MonsterDanceOff = 36,
        PressStart = 37,
        NockEm = 38,
        PowerTrip = 39,
    }
);

repr_t!(
    /// Descriptor for the star rate tier of a level
    FeatureRating: i32 {
        Unrated = 0,
        Rated = 1,
        Featured = 2,
        Epic = 3,
        Legendary = 4,
        Mythic = 5,
    }
    default Unrated
);

repr_t!(
    DifficultyRating: i32 {
        None = 0,
        Easy = 10,
        Normal = 20,
        Hard = 30,
        Harder = 40,
        Insane = 50,
    }
    default None
);
