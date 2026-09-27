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
    /// All official songs used in GD, including in spin-off games.
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

impl OfficialSong {
    /// Returns the server ID of this official song. This is different from the client ID.
    ///
    /// Reference: <https://boomlings.dev/reference/songs>
    pub fn to_server_id(&self) -> i32 {
        self.to_num()
    }
    /// Returns the client ID of this official song. This is different from the server ID. This function returns `Some(id)` only if the song has a known corresponding client ID.
    /// This is not the case for the practice mode song or any arbitrary song IDs not accounted for in this enum.
    ///
    /// Reference: <https://boomlings.dev/reference/songs>
    pub fn to_client_id(&self) -> Option<i32> {
        match self {
            Self::StayInsideMe => None, // actually doesn't have a client ID
            Self::StereoMadness => Some(1),
            Self::BackOnTrack => Some(2),
            Self::Polargeist => Some(3),
            Self::DryOut => Some(4),
            Self::BaseAfterBase => Some(5),
            Self::CantLetGo => Some(6),
            Self::Jumper => Some(7),
            Self::TimeMachine => Some(8),
            Self::Cycles => Some(9),
            Self::Xstep => Some(10),
            Self::Clutterfunk => Some(11),
            Self::TheoryOfEverything => Some(12),
            Self::ElectromanAdventures => Some(13),
            Self::Clubstep => Some(14),
            Self::Electrodynamix => Some(15),
            Self::HexagonForce => Some(0x10),
            Self::BlastProcessing => Some(17),
            Self::TheoryOfEverything2 => Some(18),
            Self::GeometricalDominator => Some(19),
            Self::Deadlocked => Some(20),
            Self::Fingerdash => Some(21),
            Self::Dash => Some(22),
            Self::Explorers => Some(23),
            Self::TheSevenSeas => Some(1001),
            Self::VikingArena => Some(1002),
            Self::AirborneRobots => Some(1003),
            Self::Secret => Some(3001),
            Self::Payload => Some(2001),
            Self::BeastMode => Some(2002),
            Self::Machina => Some(2003),
            Self::Years => Some(2004),
            Self::Frontlines => Some(2005),
            Self::SpacePirates => Some(2006),
            Self::Striker => Some(2007),
            Self::Embers => Some(2008),
            Self::Round1 => Some(2009),
            Self::MonsterDanceOff => Some(2010),
            Self::PressStart => Some(4001),
            Self::NockEm => Some(4002),
            Self::PowerTrip => Some(4003),
            Self::Unrecognized(_) => None, // we don't know
        }
    }
}

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
