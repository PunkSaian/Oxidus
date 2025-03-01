use std::ffi::{CStr, CString};

use macros::vmt;

use crate::sdk::models::material::Material;

use super::material_render_context::MaterialRenderContext;


pub struct MaterialSystem;

#[vmt]
pub struct MaterialSystem {
    #[offset(73)]
    pub find_material: extern "C" fn(name: CString, texture_group: CString, complain: bool, complain_prefix: CString) -> *mut Material,
    #[offset(98)]
    pub getn_render_context: extern "C" fn() -> &'static mut MaterialRenderContext,
}
