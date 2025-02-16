use macros::{detour_hook, vmt_hook};
use sdl2_sys::{SDL_Event, SDL_Window};

use crate::{
    overlay::Overlay,
    sdk::interface::gui_surface::{EMouseCursor, GuiSurface},
};

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
pub unsafe extern "C" fn poll_event(event: &mut SDL_Event) -> i32 {
    let mut state = OVERLAY.write().unwrap();
    if let Some(state) = state.as_mut() {
        let result = original_function(event);
        if result != 0 {
            state.poll_event(event);
            if state.visible {
                event.type_ = 0;
                return result;
            }
        }
        result
    } else {
        original_function(event)
    }
}

#[vmt_hook]
pub unsafe extern "C" fn lock_cursor(this: &GuiSurface) {
    let state = OVERLAY.read().unwrap();
    if let Some(state) = state.as_ref() {
        if state.visible {
            this.unlock_cursor();
        }
    }
    original_function(this);
}

#[vmt_hook]
pub unsafe extern "C" fn set_cursor(this: *const (), cursor: EMouseCursor) {
    if let Ok(state) = OVERLAY.read() {
        if let Some(state) = state.as_ref() {
            if state.visible {
                original_function(this, EMouseCursor::dc_arrow);
                return;
            }
        }
    }
    original_function(this, cursor);
}
