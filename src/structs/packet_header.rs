use binrw::binrw;

#[binrw]
struct PacketHeader {
    m_packetFormat: u16,
    m_gameMajorVersion: u8,
    m_gameMinorVersion: u8,
    m_packetVersion: u8,
    m_packetId: u8,
    m_sessionUID: u64,
    m_sessionTime: f64,
    m_frameIdentifier: u32,
    m_playerCarIndex: u8,
    m_secondaryPlayerCarIndex: u8,
}
