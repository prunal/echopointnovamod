use hudhook::imgui::*;
use crate::state::ModState;
use crate::memory;

pub fn render_player_ui(ui: &Ui, state: &mut ModState) {
    ui.text("Player Settings");
    ui.separator();

    ui.checkbox("God Mode", &mut state.god_mode);
    ui.checkbox("Infinite Jump", &mut state.infinite_jump);
    ui.checkbox("No Clip", &mut state.no_clip);
    ui.checkbox("No Camera Shake", &mut state.no_camera_shake);

    ui.text("Speed Multiplier:");
    ui.input_float("##speed", &mut state.speed_multiplier).build();
    ui.text(format!("Current: {:.2}x", state.speed_multiplier));
}

pub fn update(state: &ModState) {
    if let Some(player) = memory::get_player() {
        if state.god_mode {
            if let Some(max_hp) = memory::read_max_hp(player) {
                memory::write_player_hp(player, max_hp);
            }
            memory::write_invincible(player, true);
        } else {
            memory::write_invincible(player, false);
        }

        // Speed multiplier would be applied to character movement
        // Infinite jump would be applied when jump input is detected
        // No clip would disable collision
        // No camera shake would zero out camera rotation delta
    }
}
