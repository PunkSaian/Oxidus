#![allow(
    clippy::missing_errors_doc,
    clippy::cast_possible_truncation,
    clippy::multiple_crate_versions,
    clippy::cast_precision_loss
)]

extern crate thiserror;

use std::{
    cell::RefCell,
    ffi::c_void,
    pin::Pin,
    ptr,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use detour::DetourHook;
use imgui::{Key, MouseButton};
use libc::{dlclose, dlopen};
use log::{error, info};
use macros::detour_hook;
use once_cell::sync::Lazy;
use prelude::*;
use sdl2_sys::{
    SDL_Event, SDL_EventType, SDL_GetWindowSize, SDL_Scancode, SDL_Window, SDL_BUTTON_LEFT,
    SDL_BUTTON_MIDDLE, SDL_BUTTON_RIGHT, SDL_PRESSED,
};
use util::resolve_fn;

#[macro_use]
extern crate log;

mod detour;
mod error;
mod prelude;
mod sdk;
mod sdl_renderer;
mod util;

use sdl_renderer::Renderer;

#[allow(clippy::type_complexity)]
static HOOKS: Lazy<Arc<Mutex<Vec<Pin<Box<DetourHook>>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

thread_local! {
    #[allow(clippy::type_complexity)]
    static IMGUI_STATE: RefCell<Option<(imgui::Context, Renderer, Instant, *mut c_void, *mut c_void)>> = const { RefCell::new(None) };
}

#[detour_hook]
unsafe extern "C" fn my_sdl_gl_swap_window(window: *mut SDL_Window) {
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
            let sdl_renderer = sdl2_sys::SDL_CreateRenderer(window, -1, 0);

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
        ui.window("Hello, world!")
            .size([300.0, 200.0], imgui::Condition::Always)
            .build(|| {
                ui.text("Hello, world!");
            });

        renderer.render(context);
        sdl2_sys::SDL_GL_MakeCurrent(window, *tf2_gl_ctx);
    });

    original_function(window);
}

#[detour_hook]
unsafe extern "C" fn my_sdl_poll_event(event: *mut c_void) -> i32 {
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

fn sdl_scancode_to_imgui_key(scancode: SDL_Scancode) -> Option<Key> {
    use SDL_Scancode::{
        SDL_SCANCODE_A, SDL_SCANCODE_BACKSPACE, SDL_SCANCODE_C, SDL_SCANCODE_DELETE,
        SDL_SCANCODE_DOWN, SDL_SCANCODE_END, SDL_SCANCODE_ESCAPE, SDL_SCANCODE_HOME,
        SDL_SCANCODE_INSERT, SDL_SCANCODE_LEFT, SDL_SCANCODE_PAGEDOWN, SDL_SCANCODE_PAGEUP,
        SDL_SCANCODE_RETURN, SDL_SCANCODE_RIGHT, SDL_SCANCODE_SPACE, SDL_SCANCODE_TAB,
        SDL_SCANCODE_UP, SDL_SCANCODE_V, SDL_SCANCODE_X, SDL_SCANCODE_Y, SDL_SCANCODE_Z,
    };
    match scancode {
        SDL_SCANCODE_TAB => Some(Key::Tab),
        SDL_SCANCODE_LEFT => Some(Key::LeftArrow),
        SDL_SCANCODE_RIGHT => Some(Key::RightArrow),
        SDL_SCANCODE_UP => Some(Key::UpArrow),
        SDL_SCANCODE_DOWN => Some(Key::DownArrow),
        SDL_SCANCODE_PAGEUP => Some(Key::PageUp),
        SDL_SCANCODE_PAGEDOWN => Some(Key::PageDown),
        SDL_SCANCODE_HOME => Some(Key::Home),
        SDL_SCANCODE_END => Some(Key::End),
        SDL_SCANCODE_INSERT => Some(Key::Insert),
        SDL_SCANCODE_DELETE => Some(Key::Delete),
        SDL_SCANCODE_BACKSPACE => Some(Key::Backspace),
        SDL_SCANCODE_SPACE => Some(Key::Space),
        SDL_SCANCODE_RETURN => Some(Key::Enter),
        SDL_SCANCODE_ESCAPE => Some(Key::Escape),
        SDL_SCANCODE_A => Some(Key::A),
        SDL_SCANCODE_C => Some(Key::C),
        SDL_SCANCODE_V => Some(Key::V),
        SDL_SCANCODE_X => Some(Key::X),
        SDL_SCANCODE_Y => Some(Key::Y),
        SDL_SCANCODE_Z => Some(Key::Z),
        //TODO: better way of doing this shit
        _ => None,
    }
}

fn install_hooks() -> OxidusResult {
    unsafe {
        let swap_fn = resolve_fn("libSDL2-2.0.so.0", "SDL_GL_SwapWindow").unwrap();
        let swap_hook = DetourHook::new_and_install(swap_fn, my_sdl_gl_swap_window as _)?;

        let poll_fn = resolve_fn("libSDL2-2.0.so.0", "SDL_PollEvent").unwrap();
        let poll_hook = DetourHook::new_and_install(poll_fn, my_sdl_poll_event as _)?;

        let mut hooks = HOOKS.lock().unwrap();
        hooks.push(swap_hook);
        hooks.push(poll_hook);
    }
    Ok(())
}

pub fn main() -> OxidusResult {
    install_hooks()?;
    Ok(())
}

// Rest of your initialization and cleanup code...
unsafe extern "C" fn load() {
    thread::spawn(|| {
        eprintln!("oxidus: before load");
        env_logger::builder()
            .filter(Some("oxidus"), log::LevelFilter::Trace)
            .try_init()
            .unwrap();
        info!("Loading");
        if let Err(e) = main() {
            error!("Failed to load\n{e}");
            unsafe {
                let handle = dlopen("/tmp/liboxidus.so".as_ptr().cast::<i8>(), 6);
                dlclose(handle);
                dlclose(handle);
            }
        } else {
            info!("Loaded sucessfully");
        }
    });
}

extern "C" fn unload() {
    info!("Unloading");
    info!("Unloaded");
}

#[link_section = ".init_array"]
#[allow(unused)]
static LOAD: unsafe extern "C" fn() = { load };

#[link_section = ".fini_array"]
#[allow(unused)]
static UNLOAD: unsafe extern "C" fn() = { unload };
