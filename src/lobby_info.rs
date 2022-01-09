use binrw::binread;

#[derive(Debug, PartialEq, Clone, Copy)]
#[binread]
#[br(little, repr = u8)]
pub enum ReadyStatus {
    NotReady = 0,
    Ready,
    Spectating,
}

#[binread]
#[br(little)]
#[derive(Debug, PartialEq, Clone)]
pub struct LobbyPlayer {
    #[br(map = |x: u8| x != 0)]
    pub is_ai_controlled: bool,

    pub team_id: u8,
    pub nationality: u8,

    #[br(temp)]
    name_: [u8; 48],

    #[br(calc = ::std::str::from_utf8(&name_[..name_
                                             .iter()
                                             .position(|&v| v == 0)
                                             .unwrap() as usize])
         .unwrap().to_owned())]
    pub name: String,

    pub car_number: u8,
    pub ready_status: ReadyStatus,
}

#[binread]
#[br(little)]
#[derive(Debug, PartialEq, Clone)]
pub struct LobbyInfo {
    #[br(temp)]
    num_players_: u8,

    #[br(count = num_players_)]
    pub lobby_players: Vec<LobbyPlayer>,
}
