use hudhook::imgui::*;
use crate::state::ModState;
use crate::memory;
use crate::features::visuals;

pub fn render_misc_tab(ui: &Ui, state: &mut ModState) {
    ui.text("Player Tweaks");
    ui.separator();

    ui.checkbox("Godmode", &mut state.godmode_enabled);
    ui.text_disabled("Flips AGLGameState::bDebugGodMode. Off = damage resumes.");
}

// Runs every frame from gui.rs. Writes bDebugGodMode unconditionally each frame:
//   enabled  -> 1
//   disabled -> 0
// Writing 0 on disable is what guarantees we leave godmode if the flag was previously latched.
pub fn tick(state: &mut ModState) {
    let base = memory::get_module_base();
    let world = memory::get_gworld(base);
    let game_state = memory::get_game_state(world);
    state.debug_game_state = game_state;

    let pawn = memory::get_player_pawn(state.debug_pc);
    state.debug_player_pawn = pawn;
    if pawn != 0 {
        let class_ptr = memory::get_actor_class(pawn);
        let is_human = class_ptr != 0 && visuals::is_human_pawn_class(class_ptr);
        state.debug_player_pawn_human = is_human;
        state.debug_player_hp = memory::read_i32_at(pawn + memory::HUMAN_HP_OFFSET).unwrap_or(0);
    } else {
        state.debug_player_pawn_human = false;
        state.debug_player_hp = 0;
    }

    if game_state == 0 {
        state.debug_godmode_flag = 0;
        return;
    }

    let flag_addr = game_state + memory::GAMESTATE_DEBUG_GODMODE_OFFSET;
    let desired: u8 = if state.godmode_enabled { 1 } else { 0 };
    memory::write_u8_at(flag_addr, desired);
    state.debug_godmode_flag = memory::read_u8_at(flag_addr).unwrap_or(0);
}
