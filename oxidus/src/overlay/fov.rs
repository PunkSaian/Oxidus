use crate::{get_entry_mut, prelude::Interfaces, config::Config};

pub fn show_fov(ui: &mut imgui::Ui) {
    if !Interfaces::get().engine.is_in_game() {
        return;
    }

    let Some(local_player) = Interfaces::get().engine.get_local_player() else {
        return;
    };

    if !local_player.is_alive() {
        return;
    }

    let settings = Config::get();
    let mut settings = settings.write().unwrap();
    let fov = get_entry_mut!(&mut settings.settings, "aimbot", "fov" => F32);

    let draw_list = ui.get_background_draw_list();

    let viewport = unsafe { imgui::sys::igGetMainViewport().read() };
    let window_size = [viewport.Size.x, viewport.Size.y];

    let game_fov = 90.0f32; // Default to 90 if unavailable
    let aspect_ratio = window_size[0] / window_size[1];

    // Convert FOV to screen space radius
    let game_fov_rad = (game_fov * 0.5).to_radians();
    let desired_fov_rad = (*fov).to_radians();

    // Calculate radius using perspective projection
    let radius_px = (window_size[0] * 0.5) * (desired_fov_rad.tan() / game_fov_rad.tan());

    // Account for aspect ratio (16:9 vs 4:3)
    let adjusted_radius = radius_px * (4.0 / 3.0) / aspect_ratio;

    draw_list
        .add_circle(
            [window_size[0] * 0.5, window_size[1] * 0.5],
            adjusted_radius,
            0xFF_FF_FF_FF,
        )
        .build();
}
