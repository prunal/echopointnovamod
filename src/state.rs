pub struct ModState {
    pub esp_enabled: bool,
    pub esp_show_box: bool,
    pub esp_show_distance: bool,
    pub esp_show_names: bool,
    pub esp_max_distance: f32,
    pub esp_min_distance: f32,
    pub esp_color: [f32; 4],

    pub debug_actor_count: i32,
    pub debug_world_addr: usize,
    pub debug_base_addr: usize,
    pub debug_level_addr: usize,
    pub debug_camera_ok: bool,
    pub debug_camera_loc: [f32; 3],
    pub debug_camera_rot: [f32; 3],
    pub debug_camera_fov: f32,
    pub debug_visible_actors: i32,
}

impl ModState {
    pub const fn new() -> Self {
        Self {
            esp_enabled: true,
            esp_show_box: true,
            esp_show_distance: true,
            esp_show_names: false,
            esp_max_distance: 200.0,
            esp_min_distance: 2.0,
            esp_color: [1.0, 0.0, 0.0, 1.0],

            debug_actor_count: 0,
            debug_world_addr: 0,
            debug_base_addr: 0,
            debug_level_addr: 0,
            debug_camera_ok: false,
            debug_camera_loc: [0.0; 3],
            debug_camera_rot: [0.0; 3],
            debug_camera_fov: 0.0,
            debug_visible_actors: 0,
        }
    }
}
