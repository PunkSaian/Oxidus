use std::ptr;

use macros::vmt_hook;

use crate::{
    config::Config,
    hook::vmt::install_vmt,
    i,
    math::Vector2,
    modules::{
        aimbot::{self},
        esp::ESP,
        movement::{self, rotate_movement},
    },
    sdk::interface::client::{FrameStage, ViewSetup},
};

use std::f32;

use crate::sdk::interface::client_mode::{ClientMode, UserCmd};

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
pub unsafe extern "C" fn create_move(
    client_mode: &ClientMode,
    input_sample_time: f32,
    cmd: &mut UserCmd,
) -> bool {
    let org_res = original_function(client_mode, input_sample_time, cmd);
    if cmd.tick_count == 0 {
        return org_res;
    }
    let org_cmd = *cmd;
    let overwrite_angels = !aimbot::run(cmd);

    movement::run(cmd);

    #[allow(clippy::float_cmp)]
    if org_cmd.viewangles.yaw != cmd.viewangles.yaw {
        let Vector2 { x, y } = rotate_movement(
            cmd.viewangles.yaw - org_cmd.viewangles.yaw,
            [cmd.forwardmove, cmd.sidemove].into(),
        );
        cmd.forwardmove = x;
        cmd.sidemove = y;
    }

    overwrite_angels
}

#[vmt_hook]
pub unsafe extern "C" fn override_view(this: *const (), view_setup: &mut ViewSetup) -> bool {
    let visual_settings = &Config::get_read().settings.visual;
    view_setup.fov = *visual_settings.fov.get();
    if let Some(local_player) = i!().engine.get_local_player() {
        if local_player.is_alive() {
            local_player.m_nForceTauntCam = i32::from(*visual_settings.third_person.get());
        }
    }

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
        install_vmt(
            *(ptr::from_ref(i!().client_mode).cast()),
            22,
            create_move as *mut (),
        );
    }
}
