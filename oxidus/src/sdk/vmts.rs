use macros::vmt;
use super::bindings::*;

#[vmt(BaseEntity)]
pub struct BaseEntity {
    #[offset(2)]
    pub get_origin: extern "C" fn() -> [f32; 3],
    #[offset(3)]
    pub get_origin2: extern "C" fn(test: f32) -> [f32; 3],
}

