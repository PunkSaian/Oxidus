use macros::vmt;

use super::bindings::BaseEntity;

#[vmt(Unknown)]
pub struct Unknown {
    #[offset(7)]
    pub get_base_entity: extern "C" fn() -> *const BaseEntity,
}

#[derive(Debug)]
pub struct Unknown;
