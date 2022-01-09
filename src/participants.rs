use binrw::binread;

#[binread]
#[br(little)]
#[derive(Debug, PartialEq, Clone)]
pub struct Participant {
    #[br(map = |v: u8| v != 0)]
    pub ai_controlled: bool,

    pub driver_id: u8, // TODO: drivers
    pub network_id: u8,
    pub team_id: u8, // TODO: teams

    #[br(map = |v: u8| v != 0)]
    pub my_team: bool,

    pub race_number: u8,

    pub nationality: u8, // TODO: this

    #[br(temp)]
    name_: [u8; 48],

    #[br(calc = ::std::str::from_utf8(&name_[..name_
                                             .iter()
                                             .position(|&v| v == 0)
                                             .unwrap() as usize])
         .unwrap().to_owned())]
    pub name: String,

    #[br(map = |v: u8| v != 0)]
    pub your_public_telemetry: bool,
}

#[binread]
#[br(little)]
#[derive(Debug, PartialEq, Clone)]
pub struct Participants {
    #[br(temp)]
    num_active_cars_: u8,

    #[br(temp)]
    participant_data_: [Participant; 22],

    #[br(calc = Vec::from(&participant_data_[..num_active_cars_ as usize]))]
    pub participants: Vec<Participant>,
}
