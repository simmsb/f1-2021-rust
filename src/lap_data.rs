use binrw::binread;
use uom::si::f32::Length as LengthF32;
use uom::si::f32::Time as TimeF32;
use uom::si::length::meter;
use uom::si::time::{millisecond, second};
use uom::si::u16::Time as TimeU16;
use uom::si::u32::Time as TimeU32;
use uom::si::u8::Time as TimeU8;

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum PitStatus {
    None = 0,
    Pitting = 1,
    InPitArea = 2,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum Sector {
    Sector1 = 0,
    Sector2 = 1,
    Sector3 = 2,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum DriverStatus {
    InGarage = 0,
    FlyingLap = 1,
    InLap = 2,
    OutLap = 3,
    OnTrack = 4,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum ResultStatus {
    Invalid = 0,
    Inactive = 1,
    Active = 2,
    Finished = 3,
    DidNotFinish = 4,
    Disqualified = 5,
    NotClassified = 6,
    Retired = 7,
}

#[binread]
#[br(little)]
#[derive(Debug, PartialEq, Clone)]
pub struct CarLapData {
    #[br(map = TimeU32::new::<millisecond>)]
    pub last_lap_time: TimeU32,

    #[br(map = TimeU32::new::<millisecond>)]
    pub current_lap_time: TimeU32,

    #[br(map = TimeU16::new::<millisecond>)]
    pub sector_1_time: TimeU16,

    #[br(map = TimeU16::new::<millisecond>)]
    pub sector_2_time: TimeU16,

    #[br(map = LengthF32::new::<meter>)]
    pub lap_distance: LengthF32,

    #[br(map = LengthF32::new::<meter>)]
    pub total_distance: LengthF32,

    #[br(map = TimeF32::new::<second>)]
    pub safety_car_delta: TimeF32,

    pub car_position: u8,
    pub current_lap_num: u8,
    pub pit_status: PitStatus,
    pub num_pit_stops: u8,
    pub sector: Sector,

    #[br(map = |x: u8| x != 0)]
    pub current_lap_invalid: bool,

    #[br(map = TimeU8::new::<second>)]
    pub penalties: TimeU8,

    pub warnings: u8,
    pub num_unserved_drive_through_pens: u8,
    pub num_unserved_stop_go_pens: u8,
    pub grid_position: u8,
    pub driver_status: DriverStatus,
    pub result_status: ResultStatus,

    #[br(map = |x: u8| x != 0)]
    pub pit_lane_timer_active: bool,

    #[br(map = TimeU16::new::<millisecond>)]
    pub pit_lane_time_in_lane: TimeU16,

    #[br(map = TimeU16::new::<millisecond>)]
    pub pit_stop_timer_in_ms: TimeU16,

    #[br(map = |x: u8| x != 0)]
    pub pit_stop_should_serve_pen: bool,
}

#[binread]
#[derive(Debug, PartialEq, Clone)]
pub struct LapData {
    pub car_lap_data: Box<[CarLapData; 22]>,
}
