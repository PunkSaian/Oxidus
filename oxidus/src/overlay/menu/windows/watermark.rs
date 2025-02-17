use imgui::WindowFlags;

use crate::{overlay::TEXTURES, util::consts::info_string};

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
                | WindowFlags::NO_FOCUS_ON_APPEARING
                | WindowFlags::NO_DECORATION
                | WindowFlags::NO_NAV,
        )
        .bg_alpha(0.80)
        .always_auto_resize(true)
        .build(|| {
            let textures = TEXTURES.read().unwrap();
            let logo = &textures.as_ref().unwrap().logo_trans;

            let text_height = ui.text_line_height_with_spacing();
            #[allow(deprecated)]
            imgui::ImageButton::new(logo.id, [text_height, text_height])
                .frame_padding(0)
                .build(ui);
            ui.same_line_with_spacing(0.0, 4.0); // Reduced spacing between image and text
            ui.text(info_string());
        });
}
