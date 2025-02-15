pub fn show_debug_window(ui: &mut imgui::Ui) {
    ui.window("Debug")
        .size([300.0, 200.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.text("Hello, world!");
            ui.text("This is a debug window.");
            ui.text("You can put debug information here.");
        });
}
