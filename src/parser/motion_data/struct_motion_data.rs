use binrw::BinRead;

#[derive(Debug, BinRead)]
#[br(little)]
pub struct CarMotionData {
    pub m_world_position_x: f32, // World space X position
    pub m_world_position_y: f32, // World space Y position
    pub m_world_position_z: f32, // World space Z position
    pub m_world_velocity_x: f32, // Velocity in world space X
    pub m_world_velocity_y: f32, // Velocity in world space Y
    pub m_world_velocity_z: f32, // Velocity in world space Z
    pub m_world_forward_dir_x: i16, // World space forward X direction (normalised)
    pub m_world_forward_dir_y: i16, // World space forward Y direction (normalised)
    pub m_world_forward_dir_z: i16, // World space forward Z direction (normalised)
    pub m_world_right_dir_x: i16, // World space right X direction (normalised)
    pub m_world_right_dir_y: i16, // World space right Y direction (normalised)
    pub m_world_right_dir_z: i16, // World space right Z direction (normalised)
    pub m_g_force_lateral: f32, // Lateral G-Force component
    pub m_g_force_longitudinal: f32, // Longitudinal G-Force component
    pub m_g_force_vertical: f32, // Vertical G-Force component
    pub m_yaw: f32, // Yaw angle in radians
    pub m_pitch: f32, // Pitch angle in radians
    pub m_roll: f32, // Roll angle in radians
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct PacketMotionData {
    #[br(count = 22)]
    pub m_car_motion_data: Vec<CarMotionData>, // Data for all cars on track
    // Extra player car ONLY data
    #[br(count = 4)]
    pub m_suspension_position: Vec<f32>, // RL, RR, FL, FR
    #[br(count = 4)]
    pub m_suspension_velocity: Vec<f32>, // RL, RR, FL, FR
    #[br(count = 4)]
    pub m_suspension_acceleration: Vec<f32>, // RL, RR, FL, FR
    #[br(count = 4)]
    pub m_wheel_speed: Vec<f32>, // Speed of each wheel
    #[br(count = 4)]
    pub m_wheel_slip: Vec<f32>, // Slip ratio for each wheel
    pub m_local_velocity_x: f32, // Velocity in local space
    pub m_local_velocity_y: f32, // Velocity in local space
    pub m_local_velocity_z: f32, // Velocity in local space
    pub m_angular_velocity_x: f32, // Angular velocity x-component
    pub m_angular_velocity_y: f32, // Angular velocity y-component
    pub m_angular_velocity_z: f32, // Angular velocity z-component
    pub m_angular_acceleration_x: f32, // Angular velocity x-component
    pub m_angular_acceleration_y: f32, // Angular velocity y-component
    pub m_angular_acceleration_z: f32, // Angular velocity z-component
    pub m_front_wheels_angle: f32, // Current front wheels angle in radians
}