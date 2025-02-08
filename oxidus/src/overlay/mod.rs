use std::{cell::RefCell, ffi::c_void, sync::RwLock, time::Instant};

use hooks::{poll_event, swap_window};
use sdl_renderer::Renderer;

use crate::{hook::detour::DetourHook, util::resolve_fn, HOOKS};

pub mod hooks;
pub mod scan_code_map;
pub mod sdl_renderer;

pub use crate::prelude::*;

thread_local! {
    #[allow(clippy::type_complexity)]
    pub static IMGUI_STATE: RwLock<Option<(imgui::Context, Renderer, Instant, *mut c_void, *mut c_void)>> = const { RwLock::new(None) };
}

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
    IMGUI_STATE.with(|state| {
        let mut state = state.write().unwrap();

        if let Some((_, renderer, _, _, overlay_gl_ctx)) = state.as_mut() {
            unsafe {
                sdl2_sys::SDL_DestroyRenderer(renderer.sdl_renderer);
                sdl2_sys::SDL_GL_DeleteContext(*overlay_gl_ctx);
            }
        }
        *state = None;
    });
}
