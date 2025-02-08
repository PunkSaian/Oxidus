use imgui::WindowFlags;

pub fn show(ui: &mut imgui::Ui) {
    ui.show_demo_window(&mut true);

    show_oxidus_overlay(ui);
}

pub fn show_oxidus_overlay(ui: &mut imgui::Ui) {
    const PAD: f32 = 10.0;
    let window_pos = ui.window_pos();
    let window_size = ui.window_size();
    ui.window("Oxidus")
        .position(
            [
                window_pos[0] + window_size[0] - PAD,
                window_pos[1] + window_size[1] - PAD,
            ],
            imgui::Condition::Always,
        )
        .position_pivot([1.0, 1.0])
        .flags(
            WindowFlags::NO_MOVE
                | WindowFlags::NO_SAVED_SETTINGS
                | WindowFlags::NO_FOCUS_ON_APPEARING,
        )
        .bg_alpha(0.35)
        .always_auto_resize(true)
        .no_decoration()
        .no_nav()
        .build(|| {
            if let Some(_token) = ui.begin_popup_context_window() {
                ui.text("test");
                ui.menu_item(format!(
                    "Oxidus {} by {}",
                    env!("CARGO_PKG_VERSION"),
                    env!("CARGO_PKG_AUTHORS")
                ));
                ui.text("a test");
            };
        });
}
