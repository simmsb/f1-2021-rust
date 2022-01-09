use crate::PerWheel;
use binrw::binread;
use uom::si::f32::Ratio as RatioF32;
use uom::si::ratio::percent;

#[derive(Debug, Clone, PartialEq)]
#[binread]
#[br(little)]
pub struct CarDamage {
    #[br(map = |x: PerWheel<f32>| x.map(RatioF32::new::<percent>))]
    pub tyre_wear: PerWheel<RatioF32>,

    #[br(map = |x: PerWheel<u8>| x.map(|x: u8| RatioF32::new::<percent>(x as f32)))]
    pub tyre_damage: PerWheel<RatioF32>,

    #[br(map = |x: PerWheel<u8>| x.map(|x: u8| RatioF32::new::<percent>(x as f32)))]
    pub brake_damage: PerWheel<RatioF32>,

    #[br(map = |x: u8| RatioF32::new::<percent>(x as f32))]
    pub front_left_wing_damage: RatioF32,

    #[br(map = |x: u8| RatioF32::new::<percent>(x as f32))]
    pub front_right_wing_damage: RatioF32,

    #[br(map = |x: u8| RatioF32::new::<percent>(x as f32))]
    pub rear_wing_damage: RatioF32,

    #[br(map = |x: u8| RatioF32::new::<percent>(x as f32))]
    pub floor_wing_damage: RatioF32,

    #[br(map = |x: u8| RatioF32::new::<percent>(x as f32))]
    pub diffuser_damage: RatioF32,

    #[br(map = |x: u8| RatioF32::new::<percent>(x as f32))]
    pub sidepod_damage: RatioF32,

    #[br(map = |x: u8| x != 0)]
    pub drs_fault: bool,

    #[br(map = |x: u8| RatioF32::new::<percent>(x as f32))]
    pub gearbox_damage: RatioF32,

    #[br(map = |x: u8| RatioF32::new::<percent>(x as f32))]
    pub engine_damage: RatioF32,

    #[br(map = |x: u8| RatioF32::new::<percent>(x as f32))]
    pub engine_mguh_wear: RatioF32,

    #[br(map = |x: u8| RatioF32::new::<percent>(x as f32))]
    pub engine_es_wear: RatioF32,

    #[br(map = |x: u8| RatioF32::new::<percent>(x as f32))]
    pub engine_ce_wear: RatioF32,

    #[br(map = |x: u8| RatioF32::new::<percent>(x as f32))]
    pub engine_ice_wear: RatioF32,

    #[br(map = |x: u8| RatioF32::new::<percent>(x as f32))]
    pub engine_mguk_wear: RatioF32,

    #[br(map = |x: u8| RatioF32::new::<percent>(x as f32))]
    pub engine_tc_wear: RatioF32,
}

#[binread]
#[br(little)]
#[derive(Debug, Clone, PartialEq)]
pub struct CarDamages {
    pub car_damages: Box<[CarDamage; 22]>,
}
