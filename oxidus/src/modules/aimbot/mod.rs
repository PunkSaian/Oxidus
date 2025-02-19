use std::ptr;

use crate::{hook::vmt::install_vmt, sdk::interface::interfaces::Interfaces};

pub mod hooks;

pub fn init() {
    unsafe {
        install_vmt(
            *(ptr::from_ref(Interfaces::get().client_mode).cast()),
            22,
            hooks::create_move as *mut (),
        );
    }
}
