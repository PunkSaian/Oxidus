use macros::vmt;

use crate::{
    math::{Angles, Vector3},
    sdk::models::{bone_matrix::BoneMatrix, material::Material, model::Model, renderable::Renderable},
};

use super::model_info::StudioHdr;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct DrawModelState {
    pub studio_hdr: &'static StudioHdr,
    pub studio_hwdata: *const (),
    pub renderable: &'static Renderable,
    pub model_to_world: &'static BoneMatrix,
    pub decals: i32,
    pub draw_flags: i32,
    pub lod: i32,
}


#[repr(C)]
#[derive(Debug, Clone)]
pub struct ModelRenderInfo {
    pub origin: Vector3,
    pub angles: Angles,
    pub renderable: &'static Renderable,
    pub model: &'static Model,
    pub model_to_world: &'static BoneMatrix,
    pub lighting_offset: &'static BoneMatrix,
    pub lighting_origin: Vector3,
    pub flags: i32,
    pub entity_index: i32,
    pub skin: i32,
    pub body: i32,
    pub hitboxset: i32,
    pub instance: i32,
}

pub struct ModelRender;
#[repr(C)]
pub enum OverrideType
{
	Normal,
	BuildShadows,
	DepthWrite,
	SsaoDepthWrite
}

#[vmt]
pub struct ModelRender {
    #[offset(0)]
    pub draw_model: extern "C" fn(
        flags: i32,
        renderable: &'static Renderable,
        instance: i32,
        entity_index: i32,
        model: &'static Model,
        origin: Vector3,
        angles: Angles,
        skin: i32,
        body: i32,
        hitboxset: i32,
        model_to_world: &'static BoneMatrix,
        lighting_offset: &'static BoneMatrix,
    ),
    #[offset(1)]
    pub force_material_override: extern "C" fn(material: &'static Material, override_type: OverrideType),
    #[offset(16)]
    pub draw_model_ex: extern "C" fn(
        info: &ModelRenderInfo,
    ) -> i32,
    #[offset(19)]
    pub draw_model_execute: extern "C" fn(
        state: &mut DrawModelState,
        info: &ModelRenderInfo,
        custom_bone_to_world: &BoneMatrix,
    ),
}

