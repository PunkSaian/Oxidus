use std::thread;

use crate::oxidus_cleanup;

pub mod windows;

pub fn show(ui: &mut imgui::Ui) {
    ui.show_demo_window(&mut false);

    ui.window("Oxidus").build(|| {
        if ui.button("unload") {
            thread::spawn(|| {
                oxidus_cleanup();
            });
        }
    });
}
