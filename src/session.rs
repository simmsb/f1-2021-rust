use binrw::binread;
use uom::si::f32::Ratio as RatioF32;
use uom::si::f32::ThermodynamicTemperature as TemperatureF32;
use uom::si::i16::ThermodynamicTemperature as TemperatureI16;
use uom::si::ratio::{percent, ratio};
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::u8::Velocity as VelocityU8;
use uom::si::velocity::kilometer_per_hour;

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = i8)]
pub enum FlagColour {
    Unknown = -1,
    None = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
    Red = 4,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little)]
pub struct MarshalZone {
    #[br(map = RatioF32::new::<ratio>)]
    pub start: RatioF32,
    pub flag: FlagColour,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum SessionType {
    Unknown = 0,
    P1 = 1,
    P2 = 2,
    P3 = 3,
    ShortP = 4,
    Q1 = 5,
    Q2 = 6,
    Q3 = 7,
    ShortQ = 8,
    OSQ = 9,
    R = 10,
    R2 = 11,
    TimeTrial = 12,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum WeatherType {
    Clear = 0,
    LightCloud = 1,
    Overcast = 2,
    LightRain = 3,
    HeavyRain = 4,
    Storms = 5,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = i8)]
pub enum ChangeType {
    Up = 0,
    Down = 1,
    NoChange = 2,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little)]
pub struct WeatherForecastSample {
    pub session_type: SessionType,

    pub time_offset: u8,

    pub weather: WeatherType,

    #[br(map = |x: i8| TemperatureI16::new::<degree_celsius>(x as i16))]
    pub track_temp: TemperatureI16,
    pub track_temp_change: ChangeType,

    #[br(map = |x: i8| TemperatureI16::new::<degree_celsius>(x as i16))]
    pub air_temp: TemperatureI16,
    pub air_temp_change: ChangeType,

    #[br(map = |x: u8| RatioF32::new::<percent>(x as f32))]
    pub rain_chance: RatioF32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum FormulaType {
    F1Modern = 0,
    F1Classic = 1,
    F2 = 2,
    F1Generic = 3,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum SafetyCarStatus {
    None = 0,
    Full = 1,
    Virtual = 2,
    Formation = 3,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum OnlineStatus {
    Offline = 0,
    Online = 1,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum ForecastAccuracy {
    Perfect = 0,
    Approximate = 1,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum BrakingAssist {
    Off = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum GearboxAssist {
    Manual = 1,
    SuggestedGear = 2,
    Auto = 3,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum RacingLineMode {
    Off = 0,
    Corners = 1,
    Full = 2,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum RacingLineType {
    TwoD = 0,
    ThreeD = 1,
}

#[binread]
#[br(little)]
#[derive(Debug, PartialEq, Clone)]
pub struct Session {
    pub weather: WeatherType,

    #[br(map = |x: i8| TemperatureF32::new::<degree_celsius>(x as f32))]
    pub track_temp: TemperatureF32,
    #[br(map = |x: i8| TemperatureF32::new::<degree_celsius>(x as f32))]
    pub air_temp: TemperatureF32,

    pub total_laps: u8,
    pub track_length: u16,

    pub session_type: SessionType,

    #[br(map = |v: i8| if v == -1 { None } else { Some(v as u8) })]
    pub track_id: Option<u8>,

    pub formula: FormulaType,

    pub session_time_left: u16,
    pub session_duration: u16,

    #[br(map = VelocityU8::new::<kilometer_per_hour>)]
    pub pit_speed_limit: VelocityU8,

    #[br(map = |v: u8| v != 0)]
    pub paused: bool,
    #[br(map = |v: u8| v != 0)]
    pub is_spectating: bool,

    pub spectator_car_index: u8,

    #[br(map = |v: u8| v != 0)]
    pub sli_pro_native_suport: bool,

    #[br(temp)]
    num_marshal_zones_: u8,

    #[br(temp)]
    marshal_zones_: [MarshalZone; 21],

    #[br(calc = Vec::from(&marshal_zones_[..num_marshal_zones_ as usize]))]
    pub marshal_zones: Vec<MarshalZone>,

    pub safety_car_status: SafetyCarStatus,
    pub network_game: OnlineStatus,

    #[br(temp)]
    num_forecast_samples_: u8,

    #[br(temp)]
    forecast_samples_: [WeatherForecastSample; 56],

    #[br(calc = Vec::from(&forecast_samples_[..num_forecast_samples_ as usize]))]
    pub forecast_samples: Vec<WeatherForecastSample>,

    pub forecast_accuracy: ForecastAccuracy,

    pub ai_difficulty: u8,

    pub season_link_identifier: u32,
    pub weekend_link_identifier: u32,
    pub session_link_identifier: u32,

    pub pit_stop_window_ideal_lap: u8,
    pub pit_stop_window_latest_lap: u8,
    pub pit_stop_rejoin_position: u8,

    #[br(map = |v: u8| v != 0)]
    pub steering_assist: bool,
    pub braking_assist: BrakingAssist,
    pub gearbox_assist: GearboxAssist,

    #[br(map = |v: u8| v != 0)]
    pub pit_assist: bool,

    #[br(map = |v: u8| v != 0)]
    pub pit_release_assist: bool,

    #[br(map = |v: u8| v != 0)]
    pub ers_assist: bool,

    #[br(map = |v: u8| v != 0)]
    pub drs_assist: bool,

    pub dynamic_racing_line: RacingLineMode,
    pub dynamic_racing_line_type: RacingLineType,
}
