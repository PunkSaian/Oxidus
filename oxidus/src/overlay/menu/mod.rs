use std::thread;

use imgui::WindowFlags;

use crate::{oxidus_cleanup, util::consts::info_string};

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

    show_watermark(ui);
}

pub fn show_watermark(ui: &mut imgui::Ui) {
    const PAD: f32 = 10.0;
    let viewport = unsafe { imgui::sys::igGetMainViewport().read() };
    let window_pos = [viewport.Pos.x, viewport.Pos.y];
    let window_size = [viewport.Size.x, viewport.Size.y];

    ui.window("watermark")
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
            ui.text(info_string());
        });
}
