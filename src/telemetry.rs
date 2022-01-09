use binrw::binread;
use uom::si::angular_velocity::revolution_per_minute;
use uom::si::f32::AngularVelocity as AngularVelocityF32;
use uom::si::f32::Pressure as PressureF32;
use uom::si::f32::Ratio as RatioF32;
use uom::si::pressure::psi;
use uom::si::ratio::{percent, ratio};
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::u16::ThermodynamicTemperature as TemperatureU16;
use uom::si::u16::Velocity as VelocityU16;
use uom::si::velocity::kilometer_per_hour;

use crate::PerWheel;

#[derive(Debug, PartialEq, Clone)]
#[binread]
#[br(little)]
pub struct TelemetryReading {
    #[br(map = VelocityU16::new::<kilometer_per_hour>)]
    pub speed: VelocityU16,

    #[br(map = RatioF32::new::<ratio>)]
    pub throttle: RatioF32,

    #[br(map = RatioF32::new::<ratio>)]
    pub steer: RatioF32,

    #[br(map = RatioF32::new::<ratio>)]
    pub brake: RatioF32,

    #[br(map = |x: u8| RatioF32::new::<percent>(x as f32))]
    pub clutch: RatioF32,

    pub gear: i8,

    #[br(map = |x: u16| AngularVelocityF32::new::<revolution_per_minute>(x as f32))]
    pub engine_revs: AngularVelocityF32,

    #[br(map = |x: u8| x != 0)]
    pub drs: bool,

    #[br(map = |x: u8| RatioF32::new::<percent>(x as f32))]
    pub rev_lights_percent: RatioF32,

    pub rev_lights_bit_value: u16,

    #[br(map = |x: PerWheel<u16>| x.map(TemperatureU16::new::<degree_celsius>))]
    pub brake_temp: PerWheel<TemperatureU16>,

    #[br(map = |x: PerWheel<u8>| x.map(|x| x as u16).map(TemperatureU16::new::<degree_celsius>))]
    pub tyre_surface_temp: PerWheel<TemperatureU16>,

    #[br(map = |x: PerWheel<u8>| x.map(|x| x as u16).map(TemperatureU16::new::<degree_celsius>))]
    pub tyre_inner_temp: PerWheel<TemperatureU16>,

    #[br(map = TemperatureU16::new::<degree_celsius>)]
    pub engine_temp: TemperatureU16,

    #[br(map = |x: PerWheel<f32>| x.map(PressureF32::new::<psi>))]
    pub tyre_pressure: PerWheel<PressureF32>,

    pub surface_type: PerWheel<SurfaceType>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum SurfaceType {
    Tarmac = 0,
    RumbleStrip,
    Concrete,
    Rock,
    Gravel,
    Mud,
    Sand,
    Grass,
    Water,
    Cobblestone,
    Metal,
    Ridged,
}

// #[derive(Debug, PartialEq, Clone, Copy)]
// #[binread]
// #[br(little, repr = u8)]
// pub enum MFDPanelType {
//     CarSetup = 0,
//     Pits,
//     Damage,
//     Engine,
//     Temperature,
//     MFDClosed = 255,
// }

#[derive(Debug, PartialEq, Clone)]
#[binread]
#[br(little)]
pub struct CarTelemetry {
    pub car_telemetry: [TelemetryReading; 22],

    pub mfd_panel_index: u8,
    pub mfd_panel_second_player: u8,
    pub suggested_gear: i8,
}
