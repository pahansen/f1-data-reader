use binrw::BinRead;

#[derive(Debug, BinRead)]
#[br(little)]
pub struct CarTelemetryData {
    pub m_speed: u16,                // Speed of car in kilometres per hour
    pub m_throttle: f32,             // Amount of throttle applied (0.0 to 1.0)
    pub m_steer: f32,                // Steering (-1.0 (full lock left) to 1.0 (full lock right))
    pub m_brake: f32,                // Amount of brake applied (0.0 to 1.0)
    pub m_clutch: u8,                // Amount of clutch applied (0 to 100)
    pub m_gear: i8,                  // Gear selected (1-8, N=0, R=-1)
    pub m_engine_rpm: u16,           // Engine RPM
    pub m_drs: u8,                   // 0 = off, 1 = on
    pub m_rev_lights_percent: u8,    // Rev lights indicator (percentage)
    pub m_rev_lights_bit_value: u16, // Rev lights (bit 0 = leftmost LED, bit 14 = rightmost LED)
    #[br(count = 4)]
    pub m_brakes_temperature: Vec<u16>, // Brakes temperature (celsius)
    #[br(count = 4)]
    pub m_tyres_surface_temperature: Vec<u8>, // Tyres surface temperature (celsius)
    #[br(count = 4)]
    pub m_tyres_inner_temperature: Vec<u8>, // Tyres inner temperature (celsius)
    pub m_engine_temperature: u16,   // Engine temperature (celsius)
    #[br(count = 4)]
    pub m_tyres_pressure: Vec<f32>, // Tyres pressure (PSI)
    #[br(count = 4)]
    pub m_surface_type: Vec<u8>, // Driving surface, see appendices
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct PacketCarTelemetryData {
    #[br(count = 22)]
    pub m_car_telemetry_data: Vec<CarTelemetryData>,
    pub m_mfd_panel_index: u8, // Index of MFD panel open - 255 = MFD closed
    // Single player, race – 0 = Car setup, 1 = Pits
    // 2 = Damage, 3 = Engine, 4 = Temperatures
    // May vary depending on game mode
    pub m_mfd_panel_index_secondary_player: u8, // See above
    pub m_suggested_gear: u8,                   // Suggested gear for the player (1-8)
                                                // 0 if no gear suggested
}
