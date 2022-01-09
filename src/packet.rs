use binrw::binread;

#[derive(Debug, Clone, PartialEq)]
#[binread]
pub struct Packet {
    pub header: crate::header::Header,

    #[br(args(header.packet_id))]
    pub body: Body,
}

// TODO: investigate boxing variants
#[derive(Debug, Clone, PartialEq)]
#[binread]
#[br(import(packet_id: u8))]
pub enum Body {
    #[br(pre_assert(packet_id == 0))]
    Motion(crate::motion::Motion),
    #[br(pre_assert(packet_id == 1))]
    Session(crate::session::Session),
    #[br(pre_assert(packet_id == 2))]
    LapData(crate::lap_data::LapData),
    #[br(pre_assert(packet_id == 3))]
    Event(crate::event::Event),
    #[br(pre_assert(packet_id == 4))]
    Participants(crate::participants::Participants),
    #[br(pre_assert(packet_id == 5))]
    CarSetups(crate::car_setup::CarSetups),
    #[br(pre_assert(packet_id == 6))]
    CarTelemetry(crate::telemetry::CarTelemetry),
    #[br(pre_assert(packet_id == 7))]
    CarStatus(crate::car_status::CarStatus),
    #[br(pre_assert(packet_id == 8))]
    FinalClassification(crate::final_classification::FinalClassification),
    #[br(pre_assert(packet_id == 9))]
    LobbyInfo(crate::lobby_info::LobbyInfo),
    #[br(pre_assert(packet_id == 10))]
    CarDamage(crate::car_damage::CarDamages),
    #[br(pre_assert(packet_id == 11))]
    SessionHistory(crate::session_history::SessionHistory),
}
