use binrw::BinRead;

#[derive(Debug, BinRead)]
#[br(little)]
pub struct PacketHeader {
    pub m_packet_format: u16,
    pub m_game_major_version: u8,
    pub m_game_minor_version: u8,
    pub m_packet_version: u8,
    pub m_packet_id: u8,
    pub m_session_uid: u64,
    pub m_session_time: f32,
    pub m_frame_identifier: u32,
    pub m_player_car_index: u8,
    pub m_secondary_player_car_index: u8,
}
