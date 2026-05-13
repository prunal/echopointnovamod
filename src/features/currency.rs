use hudhook::imgui::*;
use crate::state::ModState;
use crate::memory;

pub fn render_currency_ui(ui: &Ui, state: &mut ModState) {
    ui.text("Currency Settings");
    ui.separator();

    ui.checkbox("Auto Update Currency", &mut state.auto_update_currency);

    ui.separator();
    ui.text("Set Cash:");
    ui.input_int("##cash", &mut state.set_cash).build();
    if ui.button("Set Cash") {
        if let Some(story_save) = memory::get_story_save() {
            memory::write_cash(story_save, state.set_cash);
        }
    }

    ui.separator();
    ui.text("Set Energy:");
    ui.input_float("##energy", &mut state.set_energy).build();
    if ui.button("Set Energy") {
        if let Some(story_save) = memory::get_story_save() {
            memory::write_energy(story_save, state.set_energy);
        }
    }

    ui.separator();
    ui.text("Set Agility Orbs:");
    ui.input_int("##agility", &mut state.set_agility_orbs).build();
    if ui.button("Set Agility Orbs") {
        if let Some(story_save) = memory::get_story_save() {
            memory::write_agility_orbs(story_save, state.set_agility_orbs);
        }
    }

    ui.separator();
    ui.text("Set Workshop Orbs:");
    ui.input_int("##workshop", &mut state.set_workshop_orbs).build();
    if ui.button("Set Workshop Orbs") {
        if let Some(story_save) = memory::get_story_save() {
            memory::write_workshop_orbs(story_save, state.set_workshop_orbs);
        }
    }
}

pub fn update(state: &ModState) {
    if state.auto_update_currency {
        // Live update currency values here
    }
}
