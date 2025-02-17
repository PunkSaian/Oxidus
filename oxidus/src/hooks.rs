use std::ptr;

use macros::vmt_hook;

use crate::{
    hook::vmt::install_vmt,
    modules::esp::ESP,
    sdk::interface::{client::FrameStage, interfaces::Interfaces},
};

#[vmt_hook]
pub unsafe extern "C" fn frame_stage_notify(this: *const (), stage: FrameStage) {
    if let FrameStage::RenderEnd = stage {
        // store esp entities
        let mut esp = ESP.write().unwrap();
        let esp = esp.as_mut().unwrap();
        esp.store_entities();
    }
    original_function(this, stage);
}

pub fn init() {
    unsafe {
        install_vmt(
            *(ptr::from_ref(Interfaces::get().client).cast()),
            35,
            frame_stage_notify as *mut (),
        );
    }
}
