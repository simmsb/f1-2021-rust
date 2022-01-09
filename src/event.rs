use binrw::binread;
use uom::si::f32::Time as TimeF32;
use uom::si::f32::Velocity as VelocityF32;
use uom::si::time::second;
use uom::si::u8::Time as TimeU8;
use uom::si::velocity::kilometer_per_hour;

#[binread]
#[br(little)]
#[derive(Debug, PartialEq, Clone)]
pub struct FastestLap {
    pub vehicle_idx: u8,

    #[br(map = TimeF32::new::<second>)]
    pub lap_time: TimeF32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum PenaltyType {
    DriveThrough = 0,
    StopGo,
    GridPenalty,
    TimePenalty,
    Warning,
    Disqualified,
    RemovedFromFormationLap,
    ParkedTooLongTimer,
    TyreRegulations,
    ThisLapInvalidated,
    ThisAndNextLapInvalidated,
    ThisLapInvalidatedWithoutReason,
    ThisAndNextLapInvalidatedWithoutReason,
    ThisAndPreviousLapInvalidated,
    ThisAndPreviousLapInvalidatedWithoutReason,
    Retired,
    BlackFlagTimer,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum InfringementType {
    BlockingBySlowDriving = 0,
    BlockingByWrongWayDriving,
    ReversingOffStartLine,
    BigCollision,
    SmallCollision,
    CollisionFailedToHandBackPositionSimple,
    CollisionFailedToHandBackPositionMultiple,
    CornerCuttingGainedTime,
    CornerCuttingOvertakeSimple,
    CornerCuttingOvertakMultiple,
    CrossedPitExitLane,
    IgnoringBlueFlag,
    IgnoringYellowFlags,
    IgnoringDriveThroughs,
    TooManyDriveThroughs,
    DriveThroughReminderServeWithinNLaps,
    DriveThroughReminderServeThisLap,
    PitLaneSpeeding,
    ParkedForTooLong,
    IgnoringTyreRegulations,
    TooManyPenalties,
    MultipleWarnings,
    ApproachingDisqualification,
    TyreRegulationsSelectSingle,
    TyreRegulationsSelectMultiple,
    LapInvalidatedCornerCutting,
    LapInvalidatedRunningWide,
    CornerCuttingRanWideGainedTimeMinor,
    CornerCuttingRanWideGainedTimeSignificant,
    CornerCuttingRanWideGainedTimeExtreme,
    LapInvalidatedWallRiding,
    LapInvalidatedFlashbackUsed,
    LapInvalidatedResetToTrack,
    BlockingThePitlane,
    JumpStart,
    SafetyCarToCarCollision,
    SafetyCarIllegalOvertake,
    SafetyCarExceedingAllowedPace,
    VirtualSafetyCarExceedingAllowedPace,
    FormationLapBelowAllowedSpeed,
    RetiredMechanicalFailure,
    RetiredTerminallyDamaged,
    SafetyCarFallingTooFarBack,
    BlackFlagTimer,
    UnservedStopGoPenalty,
    UnservedDriveThroughPenalty,
    EngineComponentChange,
    GearboxChange,
    LeagueGridPenalty,
    RetryPenalty,
    IllegalTimeGain,
    MandatoryPitstop,
}

#[binread]
#[br(little)]
#[derive(Debug, PartialEq, Clone)]
pub struct Penalty {
    pub penalty_type: PenaltyType,
    pub infringement_type: InfringementType,
    pub vehicle_idx: u8,
    pub other_vehicle_idx: u8,

    #[br(map = TimeU8::new::<second>)]
    pub time: TimeU8,

    pub lap_num: u8,

    pub places_gained: u8,
}

#[binread]
#[br(little)]
#[derive(Debug, PartialEq, Clone)]
pub struct SpeedTrap {
    pub vehicle_idx: u8,

    #[br(map = VelocityF32::new::<kilometer_per_hour>)]
    pub speed: VelocityF32,

    #[br(map = |x: u8| x != 0)]
    pub overall_fastest_in_session: bool,

    #[br(map = |x: u8| x != 0)]
    pub driver_fastest_in_session: bool,
}

#[binread]
#[br(little)]
#[derive(Debug, PartialEq, Clone)]
pub struct Flashback {
    pub frame_identifier: u32,
    pub session_time: f32, // TODO: unit
}

#[derive(Debug, PartialEq, Clone)]
#[binread]
pub enum Event {
    #[br(magic = b"SSTA")]
    SessionStart,
    #[br(magic = b"SEND")]
    SessionEnd,
    #[br(magic = b"FTLP")]
    FastestLap(FastestLap),
    #[br(magic = b"RTMT")]
    Retirement { vehicle_idx: u8 },
    #[br(magic = b"DRSE")]
    DRSEnabled,
    #[br(magic = b"DRSD")]
    DRSDisabled,
    #[br(magic = b"TMPT")]
    TeammateInPits { vehicle_idx: u8 },
    #[br(magic = b"CHQF")]
    ChequeredFlag,
    #[br(magic = b"RCWN")]
    RaceWinner { vehicle_idx: u8 },
    #[br(magic = b"PENA")]
    PenaltyIssued(Penalty),
    #[br(magic = b"SPTP")]
    SpeedTrapTriggered(SpeedTrap),
    #[br(magic = b"STLG")]
    StartLights { num_lights: u8 },
    #[br(magic = b"LGOT")]
    LightsOut,
    #[br(magic = b"DTSV")]
    DriveThroughServed { vehicle_idx: u8 },
    #[br(magic = b"FLBK")]
    Flashback(Flashback),
    #[br(magic = b"BUTN")]
    ButtonStatus { button_status: u32 },
}
