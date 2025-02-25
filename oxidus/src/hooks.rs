use std::ptr;

use macros::vmt_hook;

use crate::{
    config::Config, get_setting_mut, hook::vmt::install_vmt, i, modules::esp::ESP, sdk::interface::{
        client::{FrameStage, ViewSetup},
        interfaces::Interfaces,
    }
};

#[vmt_hook]
pub unsafe extern "C" fn frame_stage_notify(this: *const (), stage: FrameStage) {
    if let FrameStage::RenderEnd = stage {
        // store esp entities
        let mut esp = ESP.write().unwrap();
        if let Some(esp) = esp.as_mut() {
            esp.store_entities();
        }
    }
    original_function(this, stage);
}

#[vmt_hook]
pub unsafe extern "C" fn override_view(this: *const (), view_setup: &mut ViewSetup) -> bool {
    let config = Config::get();
    let mut config = config.write().unwrap();
    let fov = get_setting_mut!(&mut config.settings, "visual", "fov" => F32);
    view_setup.fov = *fov;
    original_function(this, view_setup)
}

pub fn init() {
    unsafe {
        install_vmt(
            *(ptr::from_ref(i!().client).cast()),
            35,
            frame_stage_notify as *mut (),
        );
        install_vmt(
            *(ptr::from_ref(i!().client_mode).cast()),
            17,
            override_view as *mut (),
        );
    }
}
