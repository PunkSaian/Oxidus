use std::{ffi::c_void, sync::RwLock, time::Instant};

use hooks::{poll_event, swap_window};
use sdl_renderer::Renderer;

use crate::hook::detour::install_detour_from_symbol;

pub mod hooks;
pub mod scan_code_map;
pub mod sdl_renderer;

pub use crate::prelude::*;

pub struct OverlayState {
    context: imgui::Context,
    renderer: Renderer,
    last_frame: Instant,
    tf2_gl_ctx: *mut c_void,
    overlay_gl_ctx: *mut c_void,
}
unsafe impl Send for OverlayState {}
unsafe impl Sync for OverlayState {}

#[allow(clippy::type_complexity)]
pub static OVERLAY_STATE: RwLock<Option<OverlayState>> = const { RwLock::new(None) };

pub fn init() -> OxidusResult {
    //install_detour_from_symbol("libSDL2-2.0.so.0", "SDL_PollEvent", poll_event as *mut ())?;
    //install_detour_from_symbol(
    //    "libSDL2-2.0.so.0",
    //    "SDL_GL_SwapWindow",
    //    swap_window as *mut (),
    //)?;
    Ok(())
}

pub fn unload() {
    let mut state = OVERLAY_STATE.write().unwrap();

    if let Some(OverlayState {
        renderer,
        overlay_gl_ctx,
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
