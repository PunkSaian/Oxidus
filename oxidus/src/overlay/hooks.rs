use std::{ffi::c_void, time::Instant};

use imgui::MouseButton;
use macros::detour_hook;
use sdl2_sys::{
    SDL_Event, SDL_EventType, SDL_GetWindowSize, SDL_Window, SDL_BUTTON_LEFT, SDL_BUTTON_MIDDLE,
    SDL_BUTTON_RIGHT, SDL_PRESSED,
};

use crate::overlay::sdl_renderer::Renderer;

use super::{scan_code_map::sdl_scancode_to_imgui_key, IMGUI_STATE};

#[detour_hook]
pub unsafe extern "C" fn swap_window(window: *mut SDL_Window) {
    IMGUI_STATE.with(|state_cell| {
        let mut state = state_cell.borrow_mut();

        if state.is_none() {
            // Initialize ImGui context
            let mut context = imgui::Context::create();

            let tf2_gl_ctx = sdl2_sys::SDL_GL_GetCurrentContext();
            let overlay_gl_ctx = sdl2_sys::SDL_GL_CreateContext(window);

            // Get initial window size
            let mut window_width = 0;
            let mut window_height = 0;
            SDL_GetWindowSize(window, &mut window_width, &mut window_height);

            context.io_mut().display_size = [window_width as f32, window_height as f32];
            context.io_mut().display_framebuffer_scale = [1.0, 1.0];

            sdl2_sys::SDL_SetWindowTitle(window, c"Tf2 Oxidus x64 sdl".as_ptr().cast::<_>());

            // Create SDL renderer
            let sdl_renderer = sdl2_sys::SDL_CreateRenderer(
                window,
                -1,
                sdl2_sys::SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32,
            );

            let renderer = Renderer::new(sdl_renderer, &mut context);
            sdl2_sys::SDL_GL_MakeCurrent(window, tf2_gl_ctx);

            *state = Some((
                context,
                renderer,
                Instant::now(),
                tf2_gl_ctx,
                overlay_gl_ctx,
            ));
            info!("Overlay initialized");
        }

        let (context, renderer, last_frame, tf2_gl_ctx, overlay_gl_ctx) = state.as_mut().unwrap();
        sdl2_sys::SDL_GL_MakeCurrent(window, *overlay_gl_ctx);
        let now = Instant::now();
        let delta = now.duration_since(*last_frame);
        *last_frame = now;

        // Update display size every frame
        let mut window_width = 0;
        let mut window_height = 0;
        SDL_GetWindowSize(window, &mut window_width, &mut window_height);

        context.io_mut().display_size = [window_width as f32, window_height as f32];
        context.io_mut().update_delta_time(delta);

        let ui = context.new_frame();
        ui.show_demo_window(&mut true);
        ui.window("Oxidus")
            .size([300.0, 200.0], imgui::Condition::Always)
            .build(|| {
                ui.text("12345");
                ui.text((1f32 / delta.as_secs_f32()).to_string());
            });

        renderer.render(context);
        sdl2_sys::SDL_GL_MakeCurrent(window, *tf2_gl_ctx);
    });

    original_function(window);
}

#[detour_hook]
pub unsafe extern "C" fn poll_event(event: *mut c_void) -> i32 {
    let result = original_function(event);

    if result != 0 {
        IMGUI_STATE.with(|state_cell| {
            let mut state = state_cell.borrow_mut();

            if let Some((ref mut context, ..)) = state.as_mut() {
                let event_ptr = event as *const SDL_Event;
                let event = &*event_ptr;
                let io = context.io_mut();
                match unsafe { std::mem::transmute::<u32, sdl2_sys::SDL_EventType>(event.type_) } {
                    SDL_EventType::SDL_MOUSEMOTION => {
                        io.mouse_pos = [event.motion.x as f32, event.motion.y as f32];
                    }
                    SDL_EventType::SDL_MOUSEBUTTONDOWN => match u32::from(event.button.button) {
                        SDL_BUTTON_LEFT => io.mouse_down[MouseButton::Left as usize] = true,
                        SDL_BUTTON_RIGHT => io.mouse_down[MouseButton::Right as usize] = true,
                        SDL_BUTTON_MIDDLE => io.mouse_down[MouseButton::Middle as usize] = true,
                        _ => {}
                    },
                    SDL_EventType::SDL_MOUSEBUTTONUP => match u32::from(event.button.button) {
                        SDL_BUTTON_LEFT => io.mouse_down[MouseButton::Left as usize] = false,
                        SDL_BUTTON_RIGHT => io.mouse_down[MouseButton::Right as usize] = false,
                        SDL_BUTTON_MIDDLE => io.mouse_down[MouseButton::Middle as usize] = false,
                        _ => {}
                    },
                    SDL_EventType::SDL_MOUSEWHEEL => {
                        io.mouse_wheel = event.wheel.y as f32;
                    }
                    SDL_EventType::SDL_KEYDOWN | SDL_EventType::SDL_KEYUP => {
                        let pressed = u32::from(event.key.state) == SDL_PRESSED;
                        if let Some(key) = sdl_scancode_to_imgui_key(event.key.keysym.scancode) {
                            io.keys_down[key as usize] = pressed;
                        }
                    }
                    SDL_EventType::SDL_TEXTINPUT => {
                        let text =
                            std::ffi::CStr::from_ptr(event.text.text.as_ptr()).to_string_lossy();
                        for char in text.chars() {
                            io.add_input_character(char);
                        }
                    }
                    _ => {}
                }
            }
        });
    }

    result
}
