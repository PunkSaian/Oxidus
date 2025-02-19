use std::{
    ffi::CStr,
    mem::{transmute, MaybeUninit},
    ptr,
};

use crate::math::{RotationVectors, Vector3};

use super::{
    bindings::{BaseEntity, BasePlayer, TFPlayer, TFWeaponBase},
    collidable::Collidable,
    interface::{engine::PlayerInfo, interfaces::Interfaces, model_info::HitboxSet},
    networkable::Networkable,
};
use macros::vmt;

//INFO: FIXED
#[vmt]
pub struct BaseEntity {
    #[offset(4)]
    pub get_collidable: extern "c" fn() -> &Collidable,
    #[offset(79)]
    pub get_entindex: extern "C" fn() -> i32,
    #[offset(153)]
    pub get_max_health: extern "C" fn() -> i32,
    #[offset(184)]
    pub is_alive: extern "C" fn() -> bool,
    #[offset(195)]
    pub get_eye_position: extern "C" fn() -> Vector3,
    #[offset(196)]
    pub get_view_vector: extern "C" fn() -> Vector3,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Team {
    Red = 2,
    Blue = 3,
}

pub const MAX_STUDIO_BONES: usize = 128;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BoneMatrix(pub [[f32; 4]; 3]);

impl BoneMatrix {
    pub fn rotation(&self) -> RotationVectors {
        RotationVectors {
            forward: Vector3 {
                x: self.0[0][0],
                y: self.0[0][1],
                z: self.0[0][2],
            },
            right: Vector3 {
                x: self.0[1][0],
                y: self.0[1][1],
                z: self.0[1][2],
            },
            up: Vector3 {
                x: self.0[2][0],
                y: self.0[2][1],
                z: self.0[2][2],
            },
        }
    }
    pub fn position(&self) -> Vector3 {
        Vector3 {
            x: self.0[0][3],
            y: self.0[1][3],
            z: self.0[2][3],
        }
    }
}

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
#[derive(Debug, Clone)]
pub struct Model {
    pub handle: &'static (),
    pub name: *const i8,
    pub load_flags: i32,
    pub server_count: i32,
    pub r#type: i32,
    pub flags: i32,
    pub vec_mins: Vector3,
    pub vec_maxs: Vector3,
    pub radius: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Renderable {}

//INFO: FIXED
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

impl BaseEntity {
    pub fn as_renderable(&self) -> &Renderable {
        unsafe { &*ptr::from_ref(self).byte_add(0x8).cast() }
    }
    pub fn as_networkable(&self) -> &Networkable {
        unsafe { &*ptr::from_ref(self).byte_add(0x10).cast() }
    }
    pub fn get_team(&self) -> Team {
        unsafe { transmute(self.m_iTeamNum) }
    }
}

impl<'a> From<&'a TFPlayer> for &'a BasePlayer {
    fn from(val: &'a TFPlayer) -> Self {
        unsafe { &*std::ptr::from_ref::<TFPlayer>(val).cast::<BasePlayer>() }
    }
}

impl<'a> From<&'a mut TFPlayer> for &'a mut BasePlayer {
    fn from(val: &'a mut TFPlayer) -> Self {
        unsafe { &mut *std::ptr::from_mut::<TFPlayer>(val).cast::<BasePlayer>() }
    }
}

//INFO: FIXED
#[vmt]
pub struct TFPlayer {
    #[offset(292)]
    pub get_weapon: extern "c" fn() -> &TFWeaponBase,
}

impl TFPlayer {
    pub fn get_info(&self) -> PlayerInfo {
        Interfaces::get()
            .engine
            .get_player_info(self.get_entindex())
    }
}

//INFO: FIXED
#[vmt]
pub struct TFWeaponBase {
    #[offset(401)]
    pub _get_print_name: extern "c" fn() -> *const i8,
}

impl TFWeaponBase {
    pub fn get_print_name(&self) -> String {
        unsafe {
            let name = self._get_print_name();
            CStr::from_ptr(name).to_string_lossy().into_owned()
        }
    }
}
