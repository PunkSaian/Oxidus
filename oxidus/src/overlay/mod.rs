use std::{
    ffi::{c_void, CStr, CString},
    sync::RwLock,
    time::Instant,
};

use hooks::{poll_event, swap_window};
use imgui::MouseButton;
use scan_code_map::sdl_scancode_to_imgui_key;
use sdl2_sys::{
    SDL_Event, SDL_EventType, SDL_GL_CreateContext, SDL_GL_GetCurrentContext, SDL_GetWindowSize,
    SDL_GetWindowTitle, SDL_SetWindowTitle, SDL_Window, SDL_BUTTON_LEFT, SDL_BUTTON_MIDDLE,
    SDL_BUTTON_RIGHT, SDL_PRESSED,
};
use sdl_renderer::Renderer;

use crate::{hook::detour::install_detour_from_symbol, util::consts};

pub mod hooks;
pub mod menu;
pub mod scan_code_map;
pub mod sdl_renderer;

pub use crate::prelude::*;

pub struct Overlay {
    context: imgui::Context,
    renderer: Renderer,
    last_frame: Instant,
    tf2_gl_ctx: *mut c_void,
    oxidus_gl_ctx: *mut c_void,
}

const IMGUI_CONFIG_FLAGS: imgui::ConfigFlags = imgui::ConfigFlags::DOCKING_ENABLE;

impl Overlay {
    pub fn initialize(window: *mut SDL_Window) -> Self {
        unsafe {
            let mut context = imgui::Context::create();

            let tf2_gl_ctx = SDL_GL_GetCurrentContext();
            let oxidus_gl_ctx = SDL_GL_CreateContext(window);

            // Get initial window size
            let mut window_width = 0;
            let mut window_height = 0;
            SDL_GetWindowSize(window, &mut window_width, &mut window_height);

            context.io_mut().display_size = [window_width as f32, window_height as f32];
            context.io_mut().display_framebuffer_scale = [1.0, 1.0];
            context.io_mut().config_flags |= IMGUI_CONFIG_FLAGS;

            let mut title = CStr::from_ptr(SDL_GetWindowTitle(window))
                .to_str()
                .unwrap()
                .to_string();
            title.push_str(
                format!(
                    " - {} v{} by {}",
                    consts::NAME,
                    consts::VERSION,
                    consts::AUTHOR
                )
                .as_str(),
            );
            let c_title = CString::new(title).unwrap();
            SDL_SetWindowTitle(window, c_title.as_ptr());

            // Create SDL renderer
            let sdl_renderer = sdl2_sys::SDL_CreateRenderer(
                window,
                -1,
                sdl2_sys::SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32,
            );

            let renderer = Renderer::new(sdl_renderer, &mut context);
            sdl2_sys::SDL_GL_MakeCurrent(window, tf2_gl_ctx);

            Self {
                context,
                renderer,
                last_frame: Instant::now(),
                tf2_gl_ctx,
                oxidus_gl_ctx,
            }
        }
    }
    pub fn run(&mut self, window: *mut SDL_Window) {
        unsafe {
            sdl2_sys::SDL_GL_MakeCurrent(window, self.oxidus_gl_ctx);
        }
        let now = Instant::now();

        // Update display size every frame
        let mut window_width = 0;
        let mut window_height = 0;
        unsafe {
            SDL_GetWindowSize(window, &mut window_width, &mut window_height);
        }
        let delta = now.duration_since(self.last_frame);
        self.context.io_mut().display_size = [window_width as f32, window_height as f32];
        self.context.io_mut().update_delta_time(delta);

        menu::show(self.context.new_frame());

        self.renderer.render(&mut self.context);
        unsafe {
            sdl2_sys::SDL_GL_MakeCurrent(window, self.tf2_gl_ctx);
        }
    }
    pub fn poll_event(&mut self, event: *mut c_void) {
        unsafe {
            let event_ptr = event as *const SDL_Event;
            let event = &*event_ptr;
            let io = self.context.io_mut();
            #[allow(non_snake_case)]
            match std::mem::transmute::<u32, sdl2_sys::SDL_EventType>(event.type_) {
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
                    let text = std::ffi::CStr::from_ptr(event.text.text.as_ptr()).to_string_lossy();
                    for char in text.chars() {
                        io.add_input_character(char);
                    }
                }
                _ => {}
            }
        }
    }
}

unsafe impl Send for Overlay {}
unsafe impl Sync for Overlay {}

#[allow(clippy::type_complexity)]
pub static OVERLAY_STATE: RwLock<Option<Overlay>> = const { RwLock::new(None) };

pub fn init() -> OxidusResult {
    install_detour_from_symbol("libSDL2-2.0.so.0", "SDL_PollEvent", poll_event as *mut ())?;
    install_detour_from_symbol(
        "libSDL2-2.0.so.0",
        "SDL_GL_SwapWindow",
        swap_window as *mut (),
    )?;
    Ok(())
}

pub fn unload() {
    let mut state = OVERLAY_STATE.write().unwrap();

    if let Some(Overlay {
        renderer,
        oxidus_gl_ctx: overlay_gl_ctx,
        ..
    }) = state.as_mut()
    {
        unsafe {
            sdl2_sys::SDL_DestroyRenderer(renderer.sdl_renderer);
            sdl2_sys::SDL_GL_DeleteContext(*overlay_gl_ctx);
        }
    }
    *state = None;
}
