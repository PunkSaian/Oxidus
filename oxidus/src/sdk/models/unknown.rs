use macros::vmt;

use crate::sdk::bindings::BaseEntity;

#[vmt]
pub struct Unknown {
    #[offset(7)]
    pub get_base_entity: extern "C" fn() -> *const BaseEntity,
}

#[derive(Debug)]
pub struct Unknown;
