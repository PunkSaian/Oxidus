use macros::detour_hook;
use sdl2_sys::SDL_Window;
use std::ffi::c_void;

use crate::overlay::Overlay;

use super::OVERLAY;

#[detour_hook]
pub unsafe extern "C" fn swap_window(window: *mut SDL_Window) {
    let mut overlay = OVERLAY.write().unwrap();

    if overlay.is_none() {
        info!("Initializing overlay");

        *overlay = Some(Overlay::new(window).unwrap());

        info!("Overlay initialized");
    }

    let state = overlay.as_mut().unwrap();
    state.run(window);

    original_function(window);
}

#[detour_hook]
pub unsafe extern "C" fn poll_event(event: *mut c_void) -> i32 {
    let result = original_function(event);

    if result != 0 {
        let mut state = OVERLAY.write().unwrap();
        if let Some(state) = state.as_mut() {
            state.poll_event(event);
        }
    }

    result
}
