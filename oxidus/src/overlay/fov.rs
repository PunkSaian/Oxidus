use core::f32;

use crate::{config::Config, get_setting, i};

const CUTOFF: i32 = 3000;

pub fn show_fov(ui: &mut imgui::Ui) {
    if !i!().engine.is_in_game() {
        return;
    }

    let Some(local_player) = i!().engine.get_local_player() else {
        return;
    };

    if !local_player.is_alive() {
        return;
    }

    let mut config = Config::get();
    if get_setting!(&mut config.settings, "visual", "third_person" => Bool) {
        return;
    }
    let fov = get_setting!(&mut config.settings, "aimbot", "fov" => F32);

    let visual_fov = get_setting!(&mut config.settings, "visual", "fov" => F32);

    if fov > visual_fov {
        return;
    }

    let draw_list = ui.get_background_draw_list();

    let viewport = unsafe { imgui::sys::igGetMainViewport().read() };
    let window_size = [viewport.Size.x, viewport.Size.y];

    let aspect_ratio = window_size[0] / window_size[1];

    let game_fov_rad = (visual_fov * 0.5).to_radians();
    let desired_fov_rad = (fov).to_radians();

    let radius_px = (window_size[0] * 0.5) * (desired_fov_rad.tan() / game_fov_rad.tan());

    let adjusted_radius = radius_px * (4.0 / 3.0) / aspect_ratio;

    let outline_scale = (adjusted_radius.sqrt() / 3.0).min(20.0);

    let triangle_size: f32 = outline_scale / 3.0;

    let triangle_count = (fov as usize * 5 + 10).min(100);

    for i in 0..triangle_count {
        let time_offset = (((std::time::UNIX_EPOCH.elapsed().unwrap().as_millis())
            / (fov).max(1.0) as u128)
            % CUTOFF as u128) as f32
            / CUTOFF as f32;

        let pi2 = 2.0 * std::f32::consts::PI;

        let rotation_offset = time_offset * pi2;
        let angle: f32 = (i as f32 / triangle_count as f32) * pi2 + rotation_offset;

        let center = [
            window_size[0] * 0.5 + (adjusted_radius - outline_scale / 8.0) * angle.cos(),
            window_size[1] * 0.5 + (adjusted_radius - outline_scale / 8.0) * angle.sin(),
        ];

        let point_1 = [
            center[0] + triangle_size * angle.cos(),
            center[1] + triangle_size * angle.sin(),
        ];
        let offset = f32::consts::PI * 2.0 / 3.0;

        let point_2 = [
            center[0] - triangle_size / 2.0 * (angle - offset).cos(),
            center[1] - triangle_size / 2.0 * (angle - offset).sin(),
        ];

        let point_3 = [
            center[0] - triangle_size / 2.0 * (angle + offset).cos(),
            center[1] - triangle_size / 2.0 * (angle + offset).sin(),
        ];

        let point_4 = [
            center[0] - triangle_size * angle.cos(),
            center[1] - triangle_size * angle.sin(),
        ];
        draw_list
            .add_triangle(point_1, point_2, point_3, 0x55_FF_FF_FF)
            .filled(true)
            .build();
        draw_list
            .add_triangle(point_4, point_2, point_3, 0x55_FF_FF_FF)
            .filled(true)
            .build();
    }
}
