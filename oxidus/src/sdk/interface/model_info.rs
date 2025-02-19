use std::ops::Deref;

use macros::vmt;

use crate::{
    sdk::vmts::{BoneMatrix, Model},
    Vector3,
};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerHitboxId {
    Head,
    Pelvis,
    Spine0,
    Spine1,
    Spine2,
    Spine3,
    LeftUpperArm,
    LeftLowerArm,
    LeftHand,
    RightUpperArm,
    RightLowerArm,
    RightHand,
    LeftHip,
    LeftKnee,
    LeftFoot,
    RightHip,
    RightKnee,
    RightFoot,
}

impl Deref for PlayerHitboxId {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        unsafe { &*std::ptr::from_ref::<PlayerHitboxId>(self).cast::<i32>() }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Hitbox {
    pub bone: u32,
    pub group: i32,
    pub min: Vector3,
    pub max: Vector3,
    pub nameindex: i32,
    unused: [i32; 8],
}

#[allow(clippy::struct_field_names)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Bone {
    sznameindex: i32,
    parent: i32,
    bonecontroller: [i32; 6],
    pos: Vector3,
    quat: [f32; 4],
    rot: Vector3,
    posscale: Vector3,
    rotscale: Vector3,
    pose_to_bone: BoneMatrix,
    alignment: [f32; 4],
    flags: i32,
    proctype: i32,
    procindex: i32,
    physicsbone: i32,
    surfacepropidx: i32,
    contents: i32,
    unused: [i32; 8],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct HitboxSet {
    pub sznameindex: i32,
    pub numhitboxes: i32,
    pub hitboxindex: i32,
}

impl HitboxSet {
    pub fn get_hitbox(&self, id: i32) -> &'static Hitbox {
        unsafe {
            let ptr = (std::ptr::from_ref(self) as usize
                + self.hitboxindex as usize
                + size_of::<Hitbox>() * id as usize) as *const Hitbox;
            &*ptr
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct StudioHdr {
    pub id: i32,
    pub version: i32,
    pub checksum: i32,
    pub name: [i8; 64],
    pub length: i32,
    pub eyeposition: Vector3,
    pub illumposition: Vector3,
    pub hull_min: Vector3,
    pub hull_max: Vector3,
    pub view_bbmin: Vector3,
    pub view_bbmax: Vector3,
    pub flags: i32,
    pub numbones: i32,
    pub boneindex: i32,
    pub numbonecontrollers: i32,
    pub bonecontrollerindex: i32,
    pub numhitboxsets: i32,
    pub hitboxsetindex: i32,
}

impl StudioHdr {
    pub unsafe fn bone(&self, i: i32) -> Option<&Bone> {
        if i >= self.numbones {
            return None;
        }

        Some(&*(((std::ptr::from_ref(self) as i32) + self.boneindex + i) as *const Bone))
    }

    pub fn get_hitbox_set(&self, i: i32) -> Option<&HitboxSet> {
        unsafe {
            if i >= self.numhitboxsets {
                return None;
            }

            Some(
                &*((std::ptr::from_ref(self) as i64
                    + i64::from(self.hitboxsetindex)
                    + i64::from(i) * size_of::<HitboxSet>() as i64)
                    as *const HitboxSet),
            )
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ModelInfo {}

#[vmt]
pub struct ModelInfo {
    //#[offset(3)]
    //pub get_model_index: extern "C" fn(&CStr) -> i32,
    #[offset(29)]
    pub get_studio_model: extern "C" fn(model: *const Model) -> &'static StudioHdr,
}
