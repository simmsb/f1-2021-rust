use binrw::binread;
use smallvec::SmallVec;
use uom::si::f64::Time as TimeF64;
use uom::si::time::{millisecond, second};
use uom::si::u32::Time as TimeU32;
use uom::si::u8::Time as TimeU8;

use crate::{ActualTyreCompound, VisualTyreCompound, ResultStatus};

#[binread]
#[br(little)]
#[derive(Debug, PartialEq, Clone)]
pub struct Classification {
    pub position: u8,
    pub num_laps: u8,
    pub grid_position: u8,
    pub points: u8,
    pub num_pit_stops: u8,
    pub result_status: ResultStatus,

    #[br(map = TimeU32::new::<millisecond>)]
    pub best_lap_time: TimeU32,

    #[br(map = TimeF64::new::<second>)]
    pub total_race_time: TimeF64,

    #[br(map = TimeU8::new::<second>)]
    pub penalties_time: TimeU8,

    pub num_penalties: u8,

    #[br(temp)]
    num_tyre_stints_: u8,
    #[br(temp)]
    tyre_stints_actual_: [u8; 8],
    #[br(temp)]
    tyre_stints_visual_: [u8; 8],

    #[br(calc = tyre_stints_actual_[..num_tyre_stints_ as usize]
         .iter()
         .map(|x| ActualTyreCompound::read(&mut ::std::io::Cursor::new(&[*x])).unwrap())
         .collect())]
    pub tyre_stints_actual: SmallVec<[ActualTyreCompound; 8]>,

    #[br(calc = tyre_stints_visual_[..num_tyre_stints_ as usize]
         .iter()
         .map(|x| VisualTyreCompound::read(&mut ::std::io::Cursor::new(&[*x])).unwrap())
         .collect())]
    pub tyre_stints_visual: SmallVec<[VisualTyreCompound; 8]>,
}

#[binread]
#[br(little)]
#[derive(Debug, PartialEq, Clone)]
pub struct FinalClassification {
    #[br(temp)]
    num_cars_: u8,

    #[br(count = num_cars_)]
    pub classifications: Vec<Classification>,
}
