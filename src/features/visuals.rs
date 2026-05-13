use hudhook::imgui::*;
use crate::state::ModState;
use crate::memory;

pub fn render_esp_ui(ui: &Ui, state: &mut ModState) {
    ui.text("ESP Settings");
    ui.separator();

    ui.checkbox("Enable ESP", &mut state.esp_enabled);
    ui.checkbox("Show Box", &mut state.esp_show_box);
    ui.checkbox("Show Health", &mut state.esp_show_health);
    ui.checkbox("Show Distance", &mut state.esp_show_distance);
    ui.checkbox("Show Names", &mut state.esp_show_names);

    ui.text("Max Distance:");
    ui.slider("##max_dist", 100.0, 5000.0, &mut state.esp_max_distance);

    ui.text("Color:");
    ui.color_edit4("##esp_color", &mut state.esp_color);

    ui.separator();
    ui.text("Debug Info:");
    ui.text(format!("Module Base: 0x{:X}", state.debug_base_addr));
    ui.text(format!("GWorld Addr: 0x{:X}", state.debug_world_addr));
    ui.text(format!("Actor Count: {}", state.debug_actor_count));
}

pub fn draw_esp(ui: &Ui, state: &mut ModState) {
    let base = memory::get_module_base();
    state.debug_base_addr = base;

    if !state.esp_enabled {
        return;
    }

    let world = memory::get_gworld(base);
    state.debug_world_addr = world;
    if world == 0 {
        return;
    }

    let level = memory::get_persistent_level(world);
    if level == 0 {
        return;
    }

    let actors = memory::get_actors(level);
    state.debug_actor_count = actors.count.max(0) as usize;

    if actors.count <= 0 || actors.count > 100_000 {
        return;
    }

    let draw_list = ui.get_background_draw_list();
    let [screen_w, screen_h] = ui.io().display_size;
    let color = [
        state.esp_color[0],
        state.esp_color[1],
        state.esp_color[2],
        state.esp_color[3],
    ];

    for i in 0..actors.count {
        let actor = memory::get_actor(&actors, i);
        if actor == 0 {
            continue;
        }

        let loc = match memory::get_actor_location(actor) {
            Some(l) => l,
            None => continue,
        };

        let screen_x = screen_w * 0.5 + (loc[0] * 0.01);
        let screen_y = screen_h * 0.5 - (loc[2] * 0.01);

        if screen_x < 0.0 || screen_x > screen_w || screen_y < 0.0 || screen_y > screen_h {
            continue;
        }

        if state.esp_show_box {
            draw_list
                .add_rect(
                    [screen_x - 15.0, screen_y - 30.0],
                    [screen_x + 15.0, screen_y + 30.0],
                    color,
                )
                .thickness(1.5)
                .build();
        }

        if state.esp_show_distance {
            let dist = (loc[0] * loc[0] + loc[1] * loc[1] + loc[2] * loc[2]).sqrt() * 0.01;
            draw_list.add_text(
                [screen_x - 20.0, screen_y + 32.0],
                color,
                format!("{:.0}m", dist),
            );
        }
    }
}
