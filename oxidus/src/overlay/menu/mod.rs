use windows::{binds::show_binds, settings::show_settings};

pub mod windows;

pub fn show(ui: &mut imgui::Ui) {
    ui.show_demo_window(&mut false);

    show_settings(ui);
    show_binds(ui);
}
