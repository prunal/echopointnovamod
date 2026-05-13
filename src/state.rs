pub struct ModState {
    // Aimbot
    pub aimbot_enabled: bool,
    pub aimbot_fov: f32,
    pub aimbot_smoothness: f32,
    pub aimbot_hitbox: HitboxType,
    pub aimbot_vis_check: bool,
    pub aimbot_prediction: bool,
    pub aimbot_draw_fov: bool,
    pub silent_aim_enabled: bool,

    // ESP
    pub esp_enabled: bool,
    pub esp_enemy_enabled: bool,
    pub esp_friendly_enabled: bool,
    pub esp_show_health: bool,
    pub esp_show_distance: bool,
    pub esp_show_names: bool,
    pub esp_vis_check: bool,
    pub esp_enemy_visible_color: [f32; 3],
    pub esp_enemy_hidden_color: [f32; 3],
    pub esp_friendly_visible_color: [f32; 3],
    pub esp_friendly_hidden_color: [f32; 3],

    // Player
    pub god_mode: bool,
    pub speed_multiplier: f32,
    pub infinite_jump: bool,
    pub no_clip: bool,
    pub no_camera_shake: bool,

    // Weapons
    pub no_reload: bool,
    pub infinite_ammo: bool,
    pub recoil_slider: f32,
    pub no_spread: bool,

    // Currency
    pub auto_update_currency: bool,
    pub set_cash: i32,
    pub set_energy: f32,
    pub set_agility_orbs: i32,
    pub set_workshop_orbs: i32,

    // Misc
    pub instant_teleport: bool,
    pub unlock_all: bool,
    pub spell_cooldown_slider: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HitboxType {
    Head,
    Chest,
    Pelvis,
}

impl Default for HitboxType {
    fn default() -> Self {
        HitboxType::Head
    }
}

impl ModState {
    pub const fn new() -> Self {
        Self {
            aimbot_enabled: false,
            aimbot_fov: 100.0,
            aimbot_smoothness: 1.0,
            aimbot_hitbox: HitboxType::Head,
            aimbot_vis_check: true,
            aimbot_prediction: false,
            aimbot_draw_fov: false,
            silent_aim_enabled: false,

            esp_enabled: false,
            esp_enemy_enabled: true,
            esp_friendly_enabled: true,
            esp_show_health: true,
            esp_show_distance: true,
            esp_show_names: true,
            esp_vis_check: true,
            esp_enemy_visible_color: [1.0, 0.0, 0.0],
            esp_enemy_hidden_color: [1.0, 1.0, 0.0],
            esp_friendly_visible_color: [0.0, 1.0, 0.0],
            esp_friendly_hidden_color: [0.0, 0.5, 1.0],

            god_mode: false,
            speed_multiplier: 1.0,
            infinite_jump: false,
            no_clip: false,
            no_camera_shake: false,

            no_reload: false,
            infinite_ammo: false,
            recoil_slider: 1.0,
            no_spread: false,

            auto_update_currency: false,
            set_cash: 0,
            set_energy: 0.0,
            set_agility_orbs: 0,
            set_workshop_orbs: 0,

            instant_teleport: false,
            unlock_all: false,
            spell_cooldown_slider: 1.0,
        }
    }
}
