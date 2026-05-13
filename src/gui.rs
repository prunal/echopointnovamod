use hudhook::imgui::*;
use crate::state::ModState;
use crate::features;

pub fn render_ui(ui: &Ui, state: &mut ModState) {
    let [width, height] = ui.io().display_size;

    ui.window("Echo Point Nova Mod")
        .size([400.0, 500.0], Condition::FirstUseEver)
        .position([width / 2.0 - 200.0, height / 2.0 - 250.0], Condition::FirstUseEver)
        .build(|| {
            features::visuals::render_esp_ui(ui, state);
        });

    features::visuals::draw_esp(ui, state);
}
