use binrw::{BinRead};

#[derive(Debug, BinRead)]
#[br(little)]
pub struct ParticipantData {
    pub m_ai_controlled: u8, // Whether the vehicle is AI (1) or Human (0) controlled
    pub m_driver_id: u8,     // Driver id - see appendix, 255 if network human
    pub m_network_id: u8,    // Network id – unique identifier for network players
    pub m_team_id: u8,       // Team id - see appendix
    pub m_my_team: u8,       // My team flag – 1 = My Team, 0 = otherwise
    pub m_race_number: u8,   // Race number of the car
    pub m_nationality: u8,
    #[br(count = 48)] // Nationality of the driver
    pub m_name: Vec<u8>, // Name of participant in UTF-8 format – null terminated                 // Will be truncated with ... (U+2026) if too long
    pub m_your_telemetry: u8, // The player's UDP setting, 0 = restricted, 1 = public
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct PacketParticipantsData {
    pub m_num_active_cars: u8, // Number of active cars in the data – should match number of
    // cars on HUD
    #[br(count = 22)]
    pub m_participants: Vec<ParticipantData>,
}
