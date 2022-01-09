use binrw::{binread, BinRead};
use uom::si::acceleration::meter_per_second_squared;
use uom::si::angle::radian;
use uom::si::angular_acceleration::degree_per_second_squared;
use uom::si::angular_velocity::{degree_per_second, revolution_per_minute};
use uom::si::f32::Acceleration as AccelerationF32;
use uom::si::f32::Angle as AngleF32;
use uom::si::f32::AngularAcceleration as AngularAccelerationF32;
use uom::si::f32::AngularVelocity as AngularVelocityF32;
use uom::si::f32::Force as ForceF32;
use uom::si::f32::Length as LengthF32;
use uom::si::f32::Velocity as VelocityF32;
use uom::si::force::kilogram_force;
use uom::si::length::centimeter;
use uom::si::velocity::centimeter_per_second;

fn map_tup<T, U>(f: impl Fn(T) -> U) -> impl Fn((T, T, T)) -> (U, U, U) {
    move |(x, y, z)| (f(x), f(y), f(z))
}

#[derive(Debug, Clone, PartialEq)]
#[binread]
#[br(little)]
pub struct Motion {
    // box this, it's huuuuge
    pub car_motions: Box<[CarMotion; 22]>,
    pub player_car_motion: PlayerCarMotion,
}

/// The motion of the player's car
#[derive(Debug, Clone, PartialEq)]
#[binread]
#[br(little)]
pub struct PlayerCarMotion {
    // TODO: confirm all of these units are correct
    #[br(map = |x: PerWheel<f32>| x.map(LengthF32::new::<centimeter>))]
    pub suspension_positions: PerWheel<LengthF32>,

    #[br(map = |x: PerWheel<f32>| x.map(VelocityF32::new::<centimeter_per_second>))]
    pub suspension_velocities: PerWheel<VelocityF32>,

    #[br(map = |x: PerWheel<f32>| x.map(AccelerationF32::new::<meter_per_second_squared>))]
    pub suspension_accelerations: PerWheel<AccelerationF32>,

    #[br(map = |x: PerWheel<f32>| x.map(AngularVelocityF32::new::<revolution_per_minute>))]
    pub wheel_speed: PerWheel<AngularVelocityF32>,

    /// Slip ratio, should be between 0 and 1
    pub wheel_slip: PerWheel<f32>,

    #[br(map = map_tup(VelocityF32::new::<centimeter_per_second>))]
    pub local_velocity: (VelocityF32, VelocityF32, VelocityF32),
    #[br(map = map_tup(AngularVelocityF32::new::<degree_per_second>))]
    pub angular_velocity: (AngularVelocityF32, AngularVelocityF32, AngularVelocityF32),
    #[br(map = map_tup(AngularAccelerationF32::new::<degree_per_second_squared>))]
    pub angular_acceleration: (
        AngularAccelerationF32,
        AngularAccelerationF32,
        AngularAccelerationF32,
    ),

    /// The angle fo the front wheels, in radians
    #[br(map = AngleF32::new::<radian>)]
    pub front_wheels_angle: AngleF32,
}

/// The motion of other cars
#[derive(Debug, Clone, PartialEq)]
#[binread]
#[br(little)]
pub struct CarMotion {
    #[br(map = map_tup(LengthF32::new::<centimeter>))]
    pub world_position: (LengthF32, LengthF32, LengthF32),

    #[br(map = map_tup(VelocityF32::new::<centimeter_per_second>))]
    pub world_velocity: (VelocityF32, VelocityF32, VelocityF32),

    /// A normalised vector pointing forwards
    #[br(map = map_tup(|x: i16| x as f32 / 32767.0))]
    pub world_forward_dir: (f32, f32, f32),

    /// A normalised vector pointing to the right
    #[br(map = map_tup(|x: i16| x as f32 / 32767.0))]
    pub world_right_dir: (f32, f32, f32),

    /// G force experienced by the car (lateral, longitudinal, vertical)
    #[br(map = map_tup(|x: f32| ForceF32::new::<kilogram_force>(x / 10.0)))]
    pub g_force: (ForceF32, ForceF32, ForceF32),

    /// The rotation of the car (yaw, pitch, roll), in radians
    #[br(map = map_tup(AngleF32::new::<radian>))]
    pub rotation: (AngleF32, AngleF32, AngleF32),
}

/// Properties that are given for each wheel
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerWheel<T> {
    pub rear_left: T,
    pub rear_right: T,
    pub front_left: T,
    pub front_right: T,
}

// derive via pls
impl<T: BinRead<Args = ()>> BinRead for PerWheel<T> {
    type Args = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        options: &binrw::ReadOptions,
        args: Self::Args,
    ) -> binrw::BinResult<Self> {
        let (rear_left, rear_right, front_left, front_right) =
            BinRead::read_options(reader, options, args)?;

        Ok(Self {
            rear_left,
            rear_right,
            front_left,
            front_right,
        })
    }
}

// at what point do I just use a functor crate
impl<T> PerWheel<T> {
    pub(crate) fn map<U>(self, f: impl Fn(T) -> U) -> PerWheel<U> {
        PerWheel {
            rear_left: f(self.rear_left),
            rear_right: f(self.rear_right),
            front_left: f(self.front_left),
            front_right: f(self.front_right),
        }
    }
}
