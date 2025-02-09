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
    SDL_BUTTON_RIGHT, SDL_BUTTON_X1, SDL_BUTTON_X2, SDL_PRESSED,
};
use sdl_renderer::Renderer;

use crate::{
    hook::detour::install_detour_from_symbol,
    util::consts::{self},
};

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
    #[allow(clippy::unnecessary_wraps)]
    pub fn new(window: *mut SDL_Window) -> OxidusResult<Self> {
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
            title.push_str(&consts::info_string());
            let c_title = CString::new(title).unwrap();
            SDL_SetWindowTitle(window, c_title.as_ptr());

            //TODO: fix the icon
            //let rw =
            //    sdl2_sys::SDL_RWFromConstMem(OXIDE_LOGO48.as_ptr().cast(), OXIDE_LOGO.len() as i32);

            //// Load surface from memory
            //let icon_surface = sdl2_sys::SDL_LoadBMP_RW(rw, 1); // The '1' auto-frees the RWops

            //if icon_surface.is_null() {
            //    return OxidusErrorType::Overlay("Failed to load embedded window icon".to_string())
            //        .into();
            //}
            //SDL_SetWindowIcon(window, icon_surface);
            //sdl2_sys::SDL_FreeSurface(icon_surface);

            // Create SDL renderer
            let sdl_renderer = sdl2_sys::SDL_CreateRenderer(
                window,
                -1,
                sdl2_sys::SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32,
            );

            let renderer = Renderer::new(sdl_renderer, &mut context);
            sdl2_sys::SDL_GL_MakeCurrent(window, tf2_gl_ctx);

            Ok(Self {
                context,
                renderer,
                last_frame: Instant::now(),
                tf2_gl_ctx,
                oxidus_gl_ctx,
            })
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

        self.last_frame = Instant::now();

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
                    io.add_mouse_pos_event([event.motion.x as f32, event.motion.y as f32])
                }
                SDL_EventType::SDL_MOUSEBUTTONDOWN => io.add_mouse_button_event(
                    match u32::from(event.button.button) {
                        SDL_BUTTON_LEFT => MouseButton::Left,
                        SDL_BUTTON_RIGHT => MouseButton::Right,
                        SDL_BUTTON_MIDDLE => MouseButton::Middle,
                        SDL_BUTTON_X1 => MouseButton::Extra1,
                        SDL_BUTTON_X2 => MouseButton::Extra2,
                        _ => unreachable!(),
                    },
                    true,
                ),
                SDL_EventType::SDL_MOUSEBUTTONUP => io.add_mouse_button_event(
                    match u32::from(event.button.button) {
                        SDL_BUTTON_LEFT => MouseButton::Left,
                        SDL_BUTTON_RIGHT => MouseButton::Right,
                        SDL_BUTTON_MIDDLE => MouseButton::Middle,
                        SDL_BUTTON_X1 => MouseButton::Extra1,
                        SDL_BUTTON_X2 => MouseButton::Extra2,
                        _ => unreachable!(),
                    },
                    false,
                ),
                SDL_EventType::SDL_MOUSEWHEEL => {
                    io.add_mouse_wheel_event([event.wheel.x as f32, event.wheel.y as f32]);
                }
                SDL_EventType::SDL_KEYDOWN | SDL_EventType::SDL_KEYUP => {
                    //TODO: i think the modifiers arent handled propperly
                    //let modifiers = sdl2_sys::SDL_GetModState(void);
                    let pressed = u32::from(event.key.state) == SDL_PRESSED;

                    if let Some(key) = sdl_scancode_to_imgui_key(event.key.keysym.scancode) {
                        io.add_key_event(key, pressed);
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

impl Drop for Overlay {
    fn drop(&mut self) {
        unsafe {
            sdl2_sys::SDL_DestroyRenderer(self.renderer.sdl_renderer);
            sdl2_sys::SDL_GL_DeleteContext(self.oxidus_gl_ctx);
        }
    }
}

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

    *state = None;
}
