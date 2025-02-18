use std::{ffi::CStr, mem::transmute, ptr};

use crate::math::Vector3;

use super::{
    bindings::{BaseEntity, BasePlayer, TFPlayer, TFWeaponBase},
    collidable::Collidable,
    interface::{engine::PlayerInfo, interfaces::Interfaces},
    networkable::Networkable,
};
use macros::vmt;

#[vmt]
pub struct BaseEntity {
    #[offset(4)]
    pub get_collidable: extern "c" fn() -> &Collidable,
    #[offset(79)]
    pub get_entindex: extern "C" fn() -> i32,
    #[offset(153)]
    pub get_max_health: extern "C" fn() -> i32,
    #[offset(183)]
    pub is_alive: extern "C" fn() -> bool,
    #[offset(194)]
    pub get_eye_position: extern "C" fn() -> Vector3,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Team {
    Red = 2,
    Blue = 3,
}

impl BaseEntity {
    pub fn as_networkable(&mut self) -> &mut Networkable {
        unsafe { &mut *(ptr::from_ref(self).byte_add(0x10) as *mut _) }
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

#[vmt]
pub struct TFPlayer {
    #[offset(291)]
    pub get_weapon: extern "c" fn() -> &TFWeaponBase,
}

impl TFPlayer {
    pub fn get_info(&self) -> PlayerInfo {
        Interfaces::get()
            .engine
            .get_player_info(self.get_entindex())
    }
}

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
