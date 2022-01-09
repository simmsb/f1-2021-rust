use binrw::binread;

use crate::PerWheel;

#[binread]
#[br(little)]
#[derive(Debug, PartialEq, Clone)]
pub struct CarSetup {
    pub front_wing: u8,
    pub rear_wing: u8,

    pub on_throttle: u8,
    pub off_throttle: u8,

    pub front_camber: f32,
    pub rear_camber: f32,

    pub front_toe: f32,
    pub rear_toe: f32,

    pub front_suspension: u8,
    pub rear_suspension: u8,

    pub front_anti_roll_bar: u8,
    pub rear_anti_roll_bar: u8,

    pub front_suspension_height: u8,
    pub rear_suspension_height: u8,

    pub brake_pressure: u8,
    pub brake_bias: u8,

    pub tyre_pressure: PerWheel<f32>,

    pub ballast: u8,

    pub fuel_load: f32,
}

#[binread]
#[br(little)]
#[derive(Debug, PartialEq, Clone)]
pub struct CarSetups {
    pub setup_data: [CarSetup; 22],
}
