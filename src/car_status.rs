use binrw::binread;
use uom::si::angular_velocity::revolution_per_minute;
use uom::si::energy::joule;
use uom::si::f32::Energy as EnergyF32;
use uom::si::f32::Mass as MassF32;
use uom::si::length::meter;
use uom::si::mass::kilogram;
use uom::si::ratio::percent;
use uom::si::f32::AngularVelocity as AngularVelocityF32;
use uom::si::u16::Length as LengthU16;
use uom::si::f32::Ratio as RatioF32;

use crate::FlagColour;

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum TractionControl {
    Off = 0,
    Medium,
    Full,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum FuelMix {
    Lean = 0,
    Standard,
    Rich,
    Max,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum ActualTyreCompound {
    Invalid = 0,
    F1ModernC5 = 16,
    F1ModernC4 = 17,
    F1ModernC3 = 18,
    F1ModernC2 = 19,
    F1ModernC1 = 20,
    F1ModernInter = 7,
    F1ModernWet = 8,

    F1ClassicDry = 9,
    F1ClassicWet = 10,

    F2SuperSoft = 11,
    F2Soft = 12,
    F2Medium = 13,
    F2Hard = 14,
    F2Wet = 15,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum VisualTyreCompound {
    Invalid = 0,
    F1Soft = 16,
    F1Medium = 17,
    F1Hard = 18,
    F1Inter = 7,
    F1Wet = 8,

    F2SuperSoft = 19,
    F2Soft = 20,
    F2Medium = 21,
    F2Hard = 22,
    F2Wet = 15,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum ERSDeployMode {
    None = 0,
    Medium,
    Hotlap,
    Overtake,
}

#[derive(Debug, PartialEq, Clone)]
#[binread]
#[br(little)]
pub struct StatusReading {
    pub traction_control: TractionControl,

    #[br(map = |x: u8| x != 0)]
    pub anti_lock_brakes: bool,

    pub fuel_mix: FuelMix,

    #[br(map = |x: u8| RatioF32::new::<percent>(x as f32))]
    pub front_break_bias: RatioF32,

    #[br(map = |x: u8| x != 0)]
    pub put_limiter_on: bool,

    #[br(map = MassF32::new::<kilogram>)]
    pub fuel_in_tank: MassF32,

    #[br(map = MassF32::new::<kilogram>)]
    pub fuel_capacity: MassF32,

    pub fuel_remaining_laps: f32,

    #[br(map = |x: u16| AngularVelocityF32::new::<revolution_per_minute>(x as f32))]
    pub max_revs: AngularVelocityF32,

    #[br(map = |x: u16| AngularVelocityF32::new::<revolution_per_minute>(x as f32))]
    pub idle_revs: AngularVelocityF32,

    pub max_gears: u8,

    #[br(map = |x: u8| x != 0)]
    pub drs_on: bool,

    #[br(map = |x: u16| if x == 0 { None } else { Some(LengthU16::new::<meter>(x)) })]
    pub drs_distance: Option<LengthU16>,

    pub actual_tyre_compound: ActualTyreCompound,
    pub visual_tyre_compound: VisualTyreCompound,

    pub tyre_age_laps: u8,

    pub vehicle_fia_flags: FlagColour,

    #[br(map = EnergyF32::new::<joule>)]
    pub ers_energy_store: EnergyF32,

    pub ers_deploy_mode: ERSDeployMode,

    #[br(map = EnergyF32::new::<joule>)]
    pub ers_energy_this_lap_mguk: EnergyF32,

    #[br(map = EnergyF32::new::<joule>)]
    pub ers_energy_this_lap_mguh: EnergyF32,

    #[br(map = EnergyF32::new::<joule>)]
    pub ers_energy_deployed_this_lap: EnergyF32,

    #[br(map = |x: u8| x != 0)]
    pub network_paused: bool,
}

#[derive(Debug, PartialEq, Clone)]
#[binread]
pub struct CarStatus {
    pub car_status: [StatusReading; 22],
}
