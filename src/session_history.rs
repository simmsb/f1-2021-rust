use binrw::binread;
use uom::si::time::millisecond;
use uom::si::u16::Time as TimeU16;
use uom::si::u32::Time as TimeU32;

use crate::{ActualTyreCompound, VisualTyreCompound};

#[derive(Debug, PartialEq, Clone)]
#[binread]
#[br(little)]
pub struct LapHistory {
    #[br(map = TimeU32::new::<millisecond>)]
    pub lap_time: TimeU32,

    #[br(map = TimeU16::new::<millisecond>)]
    pub sector_1_time: TimeU16,

    #[br(map = TimeU16::new::<millisecond>)]
    pub sector_2_time: TimeU16,

    #[br(map = TimeU16::new::<millisecond>)]
    pub sector_3_time: TimeU16,

    pub lap_valid_bitflags: u8,
}

#[derive(Debug, PartialEq, Clone)]
#[binread]
#[br(little)]
pub struct TyreStintHistory {
    pub end_lap: u8,
    pub actual_compound: ActualTyreCompound,
    pub visual_compound: VisualTyreCompound,
}

#[binread]
#[br(little)]
#[derive(Debug, PartialEq, Clone)]
pub struct SessionHistory {
    pub car_idx: u8,

    #[br(temp)]
    num_laps_: u8,
    #[br(temp)]
    num_tyre_stints_: u8,

    pub best_lap_time_num: u8,
    pub best_sector_1_time_num: u8,
    pub best_sector_2_time_num: u8,
    pub best_sector_3_time_num: u8,

    #[br(pad_size_to = 100, count = num_laps_)]
    pub lap_history: Vec<LapHistory>,
    #[br(pad_size_to = 8, count = num_tyre_stints_)]
    pub tyre_stint_history: Vec<TyreStintHistory>,
}
