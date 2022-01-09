use binrw::binread;
use uom::si::f32::Time as TimeF32;
use uom::si::time::second;

#[derive(Debug, Clone, Copy, PartialEq)]
#[binread]
#[br(little, repr = u16)]
pub enum Format {
    F2021 = 2021,
}

#[derive(Debug, Clone, PartialEq)]
#[binread]
#[br(little)]
pub struct Header {
    pub format: Format,
    pub game_version: (u8, u8),
    pub packet_version: u8,
    pub packet_id: u8,
    pub session_uid: u64,

    #[br(map = TimeF32::new::<second>)]
    pub session_ts: TimeF32,

    pub frame_identifier: u32,
    pub player_car_idx: u8,

    #[br(map = |v: u8| if v == 255 { None } else { Some(v) })]
    pub secondary_car_idx: Option<u8>,
}
