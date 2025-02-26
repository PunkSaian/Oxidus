#[repr(C)]
#[derive(Debug, Clone)]
pub struct GlobalVars {
    pub realtime: f32,
    pub framecount: i32,
    pub absoluteframetime: f32,
    pub curtime: f32,
    pub frametime: f32,
    pub max_clients: i32,
    pub tick_count: i32,
    pub interval_per_tick: f32,
    pub intererpolation_amount: f32,
    pub sim_ticks_this_frame: i32,
    pub network_protocol: i32,
    pub p_save_data: *const (),
    pub m_b_client: bool,
    pub n_timestamp_networking_base: i32,
    pub n_timestamp_randomize_window: i32,
}

impl GlobalVars {
    pub fn now(&self) -> f32 {
        self.interval_per_tick * self.tick_count as f32
    }
}
