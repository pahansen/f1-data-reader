use binrw::BinRead;

#[derive(Debug, BinRead)]
#[br(little)]
pub struct CarStatusData {
    pub m_traction_control: u8, // Traction control - 0 = off, 1 = medium, 2 = full
    pub m_anti_lock_brakes: u8, // 0 (off) - 1 (on)
    pub m_fuel_mix: u8,         // Fuel mix - 0 = lean, 1 = standard, 2 = rich, 3 = max
    pub m_front_brake_bias: u8, // Front brake bias (percentage)
    pub m_pit_limiter_status: u8, // Pit limiter status - 0 = off, 1 = on
    pub m_fuel_in_tank: f32,    // Current fuel mass
    pub m_fuel_capacity: f32,   // Fuel capacity
    pub m_fuel_remaining_laps: f32, // Fuel remaining in terms of laps (value on MFD)
    pub m_max_rpm: u16,         // Cars max RPM, point of rev limiter
    pub m_idle_rpm: u16,        // Cars idle RPM
    pub m_max_gears: u8,        // Maximum number of gears
    pub m_drs_allowed: u8,      // 0 = not allowed, 1 = allowed
    pub m_drs_activation_distance: u16, // 0 = DRS not available, non-zero - DRS will be available
    // in [X] metres
    pub m_actual_tyre_compound: u8, // F1 Modern - 16 = C5, 17 = C4, 18 = C3, 19 = C2, 20 = C1
    // 7 = inter, 8 = wet
    // F1 Classic - 9 = dry, 10 = wet
    // F2 – 11 = super soft, 12 = soft, 13 = medium, 14 = hard
    // 15 = wet
    pub m_visual_tyre_compound: u8, // F1 visual (can be different from actual compound)
    // 16 = soft, 17 = medium, 18 = hard, 7 = inter, 8 = wet
    // F1 Classic – same as above
    // F2 ‘19, 15 = wet, 19 – super soft, 20 = soft
    // 21 = medium , 22 = hard
    pub m_tyres_age_laps: u8,    // Age in laps of the current set of tyres
    pub m_vehicle_fia_flags: i8, // -1 = invalid/unknown, 0 = none, 1 = green
    // 2 = blue, 3 = yellow, 4 = red
    pub m_ers_store_energy: f32, // ERS energy store in Joules
    pub m_ers_deploy_mode: u8,   // ERS deployment mode, 0 = none, 1 = medium
    // 2 = hotlap, 3 = overtake
    pub m_ers_harvested_this_lap_mguk: f32, // ERS energy harvested this lap by MGU-K
    pub m_ers_harvested_this_lap_mguh: f32, // ERS energy harvested this lap by MGU-H
    pub m_ers_deployed_this_lap: f32,       // ERS energy deployed this lap
    pub m_network_paused: u8,               // Whether the car is paused in a network game
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct PacketCarStatusData {
    #[br(count = 22)]
    pub m_car_status_data: Vec<CarStatusData>,
}
