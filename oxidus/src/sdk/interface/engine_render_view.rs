use macros::vmt;

use crate::math::VMatrix;

use super::client::ViewSetup;

pub struct EngineRenderView;

#[vmt]
pub struct EngineRenderView {
    #[offset(4)]
    pub set_blend: extern "C" fn(blend: f32),
    #[offset(5)]
    pub get_blend: extern "C" fn() -> f32,
    #[offset(6)]
    pub set_color_modulation: extern "C" fn(blend: &[f32;3]),
    #[offset(50)]
    pub get_marices_for_view: extern "C" fn(
        view_setup: &ViewSetup,
        w2v: *mut VMatrix,
        w2pr: *mut VMatrix,
        w2s: *mut VMatrix,
        w2px: *mut VMatrix,
    ),
}

