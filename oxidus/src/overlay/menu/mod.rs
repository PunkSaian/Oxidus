use windows::settings::show_settings;

pub mod windows;

pub fn show(ui: &mut imgui::Ui) {
    ui.show_demo_window(&mut false);

    show_settings(ui);
}
