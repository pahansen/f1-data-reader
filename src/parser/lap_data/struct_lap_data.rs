use binrw::BinRead;

#[derive(Debug, BinRead)]
#[br(little)]
pub struct LapData {
    pub m_last_lap_time_in_ms: u32,    // Last lap time in milliseconds
    pub m_current_lap_time_in_ms: u32, // Current time around the lap in milliseconds
    pub m_sector1_time_in_ms: i16,     // Sector 1 time in milliseconds
    pub m_sector2_time_in_ms: i16,     // Sector 2 time in milliseconds
    pub m_lap_distance: f32,           // Distance vehicle is around current lap in metres – could
    // be negative if line hasn’t been crossed yet
    pub m_total_distance: f32, // Total distance travelled in session in metres – could
    // be negative if line hasn’t been crossed yet
    pub m_safety_car_delta: f32,   // Delta in seconds for safety car
    pub m_car_position: u8,        // Car race position
    pub m_current_lap_num: u8,     // Current lap number
    pub m_pit_status: u8,          // 0 = none, 1 = pitting, 2 = in pit area
    pub m_num_pit_stops: u8,       // Number of pit stops taken in this race
    pub m_sector: u8,              // 0 = sector1, 1 = sector2, 2 = sector3
    pub m_current_lap_invalid: u8, // Current lap invalid - 0 = valid, 1 = invalid
    pub m_penalties: u8,           // Accumulated time penalties in seconds to be added
    pub m_warnings: u8,            // Accumulated number of warnings issued
    pub m_num_unserved_drive_through_pens: u8, // Num drive through pens left to serve
    pub m_num_unserved_stop_go_pens: u8, // Num stop go pens left to serve
    pub m_grid_position: u8,       // Grid position the vehicle started the race in
    pub m_driver_status: u8,       // Status of driver - 0 = in garage, 1 = flying lap
    // 2 = in lap, 3 = out lap, 4 = on track
    pub m_result_status: u8, // Result status - 0 = invalid, 1 = inactive, 2 = active
    // 3 = finished, 4 = didnotfinish, 5 = disqualified
    // 6 = not classified, 7 = retired
    pub m_pit_lane_timer_active: u8, // Pit lane timing, 0 = inactive, 1 = active
    pub m_pit_lane_time_in_lane_in_ms: u16, // If active, the current time spent in the pit lane in ms
    pub m_pit_stop_timer_in_ms: u16,        // Time of the actual pit stop in ms
    pub m_pit_stop_should_serve_pen: u8,    // Whether the car should serve a penalty at this stop
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct PacketLapData {
    #[br(count = 22)]
    pub m_lap_data: Vec<LapData>, // Lap data for all cars on track
    pub m_time_trial_pbcar_idx: u8, // Index of Personal Best car in time trial (255 if invalid)
    pub m_time_trial_rival_car_idx: u8, // Index of Rival car in time trial (255 if invalid)
}
