use std::{mem::transmute, ptr};

use super::{super::bindings::BaseEntity, renderable::Renderable};
use macros::vmt;

use crate::sdk::models::networkable::Networkable;
use crate::{math::Vector3, sdk::models::collidable::Collidable};

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Team {
    Red = 2,
    Blue = 3,
}

#[vmt]
pub struct BaseEntity {
    #[offset(4)]
    pub get_collidable: extern "C" fn() -> &Collidable,
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
