pub struct ModState {
    pub esp_enabled: bool,
    pub esp_show_box: bool,
    pub esp_show_health: bool,
    pub esp_show_distance: bool,
    pub esp_show_names: bool,
    pub esp_max_distance: f32,
    pub esp_color: [f32; 4],

    pub debug_actor_count: i32,
    pub debug_world_addr: usize,
    pub debug_base_addr: usize,
    pub debug_level_addr: usize,
    pub debug_scan: [(usize, usize, i32); 12],
}

impl ModState {
    pub const fn new() -> Self {
        Self {
            esp_enabled: true,
            esp_show_box: true,
            esp_show_health: true,
            esp_show_distance: true,
            esp_show_names: false,
            esp_max_distance: 1000.0,
            esp_color: [1.0, 0.0, 0.0, 1.0],

            debug_actor_count: 0,
            debug_world_addr: 0,
            debug_base_addr: 0,
            debug_level_addr: 0,
            debug_scan: [(0, 0, 0); 12],
        }
    }
}
