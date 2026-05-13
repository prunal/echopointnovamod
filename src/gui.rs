use hudhook::imgui::*;
use crate::state::ModState;
use crate::features;

pub fn render_ui(ui: &Ui, state: &mut ModState) {
    let [width, height] = ui.io().display_size;

    ui.window("Echo Point Nova Mod")
        .size([500.0, 600.0], Condition::FirstUseEver)
        .position([width / 2.0 - 250.0, height / 2.0 - 300.0], Condition::FirstUseEver)
        .build(|| {
            if let Some(_tab_bar) = ui.tab_bar("main_tabs") {
                if let Some(_aimbot_tab) = ui.tab_item("Aimbot") {
                    features::aimbot::render_aimbot_ui(ui, state);
                }

                if let Some(_esp_tab) = ui.tab_item("ESP") {
                    features::visuals::render_esp_ui(ui, state);
                }

                if let Some(_player_tab) = ui.tab_item("Player") {
                    features::player::render_player_ui(ui, state);
                }

                if let Some(_weapons_tab) = ui.tab_item("Weapons") {
                    features::weapons::render_weapons_ui(ui, state);
                }

                if let Some(_currency_tab) = ui.tab_item("Currency") {
                    features::currency::render_currency_ui(ui, state);
                }

                if let Some(_misc_tab) = ui.tab_item("Misc") {
                    features::misc::render_misc_ui(ui, state);
                }
            }
        });
}
