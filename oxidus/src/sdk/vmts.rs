use std::{ffi::CStr, mem::transmute, ptr};

use super::{
    bindings::{BaseEntity, BasePlayer, TFPlayer, TFWeaponBase, TFWeaponBat},
    collidable::Collidable,
    networkable::Networkable,
};
use macros::vmt;

#[vmt]
pub struct BaseEntity {
    #[offset(4)]
    pub get_collidable: extern "c" fn() -> &Collidable,
    #[offset(79)]
    pub get_index: extern "C" fn() -> i32,
    #[offset(153)]
    pub get_max_health: extern "C" fn() -> i32,
}

impl BaseEntity {
    //pub fn as_renderable(&self) -> &mut RenderablE {
    //    unsafe{transmute(ptr::from_ref(self).byte_add(0x8))}
    //}
    pub fn as_networkable(&self) -> &mut Networkable {
        unsafe { &mut *(ptr::from_ref(self).byte_add(0x10) as *mut _) }
    }
}

impl<'a> Into<&'a BasePlayer> for &'a TFPlayer {
    fn into(self) -> &'a BasePlayer {
        unsafe { &*(self as *const TFPlayer as *const BasePlayer) }
    }
}

impl<'a> Into<&'a mut BasePlayer> for &'a mut TFPlayer {
    fn into(self) -> &'a mut BasePlayer {
        unsafe { &mut *(self as *mut TFPlayer as *mut BasePlayer) }
    }
}

#[vmt]
pub struct TFPlayer {
    #[offset(291)]
    pub get_weapon: extern "c" fn() -> &TFWeaponBase,
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
