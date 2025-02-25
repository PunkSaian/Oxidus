use std::mem::MaybeUninit;

use macros::vmt;

use crate::{i, prelude::*, sdk::models::client_class::UnparsedClientClass};

use super::interfaces::Interfaces;

pub struct Client;

#[repr(C)]
#[derive(Debug)]
pub struct ViewSetup {
    pub x: i32,
    pub unscaled_x: i32,
    pub y: i32,
    pub unscaled_y: i32,
    pub width: i32,
    pub unscaled_width: i32,
    pub height: i32,
    pub stereo_eye: i32,
    pub unscaled_height: i32,
    pub ortho: bool,
    pub ortho_left: f32,
    pub ortho_top: f32,
    pub ortho_right: f32,
    pub ortho_bottom: f32,
    pub fov: f32,
    pub fov_viewmodel: f32,
    pub origin: Vector3,
    pub angles: Angles,
    pub z_near: f32,
    pub z_far: f32,
    pub z_near_viewmodel: f32,
    pub z_far_viewmodel: f32,
    pub render_to_subrect_if_larger_screen: bool,
    pub aspect_ratio: f32,
    pub off_center: bool,
    pub off_center_top: f32,
    pub off_center_bottom: f32,
    pub off_center_left: f32,
    pub off_center_right: f32,
    pub do_bloom_and_tone_mapping: bool,
    pub cache_full_scene_state: bool,
    pub view_to_projection_override: bool,
    pub view_to_projection: VMatrix,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FrameStage {
    Undefined = -1,
    Start,
    NetUpdateStart,
    NetUpdatePostdataupdateStart,
    NetUpdatePostdataupdateEnd,
    NetUpdateEnd,
    RenderStart,
    RenderEnd,
}

//INFO: FIXED
#[vmt]
pub struct Client {
    #[offset(6)]
    pub level_init_post_entity: extern "C" fn(),
    #[offset(7)]
    pub level_shutdown: extern "C" fn(),
    #[offset(8)]
    pub get_all_classes: extern "C" fn() -> *const UnparsedClientClass,
    #[offset(35)]
    pub fram_stage_notify: extern "C" fn(stage: FrameStage),
    #[offset(59)]
    pub get_player_view: extern "C" fn(view: &mut ViewSetup) -> bool,
}

impl Client {
    #[allow(clippy::similar_names)]
    pub fn get_w2s_matrix(&self) -> VMatrix {
        let mut view_setup = unsafe { MaybeUninit::zeroed().assume_init() };
        self.get_player_view(&mut view_setup);

        let mut w2v: VMatrix = unsafe { MaybeUninit::zeroed().assume_init() };
        let mut v2pr: VMatrix = unsafe { MaybeUninit::zeroed().assume_init() };

        let mut w2s: VMatrix = unsafe { MaybeUninit::zeroed().assume_init() };
        let mut w2px: VMatrix = unsafe { MaybeUninit::zeroed().assume_init() };
        i!().engine_render_view.get_marices_for_view(
            &view_setup,
            &mut w2v,
            &mut v2pr,
            &mut w2s,
            &mut w2px,
        );
        w2s
    }
}
