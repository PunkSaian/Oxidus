use std::mem::MaybeUninit;

use macros::vmt;

use crate::{prelude::Interfaces, sdk::interface::model_info::HitboxSet};

use super::{bone_matrix::BoneMatrix, model::Model};

pub const MAX_STUDIO_BONES: usize = 128;

#[repr(C)]
#[derive(Debug, Clone)]
pub enum BoneMask {
    Anything = 0x7FF00,
    Hitbox = 0x100,
    Attachment = 0x200,
    VertexMask = 0x3FC00,
    VertexLod0 = 0x400,
    VertexLod1 = 0x800,
    VertexLod2 = 0x1000,
    VertexLod3 = 0x2000,
    VertexLod4 = 0x4000,
    VertexLod5 = 0x8000,
    VertexLod6 = 0x10000,
    VertexLod7 = 0x20000,
    BoneMerge = 0x40000,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Renderable {}

#[vmt]
struct Renderable {
    #[offset(9)]
    get_model: extern "C" fn() -> &'static Model,
    #[offset(16)]
    _setup_bones: extern "C" fn(
        bones: &mut [BoneMatrix; MAX_STUDIO_BONES],
        max_bones: i32,
        mask: i32,
        time: f32,
    ) -> bool,
}

impl Renderable {
    pub fn get_hitbox_bones(&self) -> [BoneMatrix; MAX_STUDIO_BONES] {
        let mut bones = unsafe { MaybeUninit::zeroed().assume_init() };

        self._setup_bones(
            &mut bones,
            MAX_STUDIO_BONES as i32,
            BoneMask::Hitbox as i32,
            Interfaces::get().global_vars.now(),
        );
        bones
    }
    pub fn get_hitboxes(&self) -> &'static HitboxSet {
        let model = self.get_model();
        let hdr = Interfaces::get().model_info.get_studio_model(model);
        hdr.get_hitbox_set(0).expect("could not get hitbox set")
    }
}
