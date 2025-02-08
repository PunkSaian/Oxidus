use std::{
    cell::RefCell,
    ffi::c_void,
    marker::PhantomData,
    sync::{Mutex, RwLock},
    time::Instant,
};

use hooks::{poll_event, swap_window};
use sdl_renderer::Renderer;

use crate::{hook::detour::DetourHook, util::resolve_fn, HOOKS};

pub mod hooks;
pub mod scan_code_map;
pub mod sdl_renderer;

pub use crate::prelude::*;

struct SyncWrapper<T>(T);

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
    unsafe {
        let swap_fn = resolve_fn("libSDL2-2.0.so.0", "SDL_GL_SwapWindow").unwrap();
        let swap_hook = DetourHook::new_and_install(swap_fn, swap_window as _)?;

        let poll_fn = resolve_fn("libSDL2-2.0.so.0", "SDL_PollEvent").unwrap();
        let poll_hook = DetourHook::new_and_install(poll_fn, poll_event as _)?;

        let mut hooks = HOOKS.lock().unwrap();
        hooks.push(swap_hook);
        hooks.push(poll_hook);
    }
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
