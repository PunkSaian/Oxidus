
use macros::vmt;
use crate::math::{Angles, Vector3};

pub struct Collidable;

#[vmt]
pub struct Collidable {
    #[offset(1)]
    pub obb_mins_pre_scaled: extern "C" fn() -> &Vector3,
    #[offset(2)]
    pub obb_maxs_pre_scaled: extern "C" fn() -> &Vector3,
    #[offset(3)]
    pub obb_mins: extern "C" fn() -> &Vector3,
    #[offset(4)]
    pub obb_maxs: extern "C" fn() -> &Vector3,
    #[offset(11)]
    pub get_origin: extern "C" fn() -> &Vector3,
    #[offset(12)]
    pub get_angles: extern "C" fn() -> &Angles,
}

