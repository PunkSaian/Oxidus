use std::thread;

use windows::{debug::show_debug_window, watermark::show_watermark};

use crate::oxidus_cleanup;

pub mod windows;

pub fn show(ui: &mut imgui::Ui) {
    crate::modules::esp::draw(ui);

    ui.show_demo_window(&mut false);

    ui.window("Oxidus").build(|| {
        if ui.button("unload") {
            thread::spawn(|| {
                oxidus_cleanup();
            });
        }
    });

    show_debug_window(ui);

    show_watermark(ui);
}
