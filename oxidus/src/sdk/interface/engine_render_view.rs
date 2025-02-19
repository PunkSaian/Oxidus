use macros::vmt;

use crate::math::VMatrix;

use super::client::ViewSetup;

pub struct EngineRenderView;

#[vmt]
pub struct EngineRenderView {
    #[offset(50)]
    pub get_marices_for_view: extern "C" fn(
        view_setup: &ViewSetup,
        w2v: *mut VMatrix,
        w2pr: *mut VMatrix,
        w2s: *mut VMatrix,
        w2px: *mut VMatrix,
    ),
}

